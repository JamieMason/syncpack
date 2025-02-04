use {
  crate::{group_selector::GroupSelector, packages::Packages},
  clap::{builder::ValueParser, crate_description, crate_name, crate_version, Arg, ArgMatches, Command},
  color_print::cformat,
  itertools::Itertools,
  log::LevelFilter,
  std::{env, path::PathBuf},
};

#[derive(Debug)]
pub enum Subcommand {
  Lint,
  Fix,
  Format,
}

#[derive(Debug)]
pub enum SortBy {
  Count,
  Name,
}

#[derive(Debug)]
pub struct Cli {
  /// Whether to check formatting instead of fixing it
  pub check: bool,
  /// The path to the root of the project
  pub cwd: PathBuf,
  /// A set of filters made up of filter options passed in as CLI arguments:
  /// - `--dependencies` to filter by dependency name
  /// - `--dependency-types` to filter by dependency type
  /// - `--specifier-types` to filter by specifier type
  /// - `--packages` to filter by package name
  pub filter: Option<GroupSelector>,
  /// Whether to disable ANSI color codes in terminal output
  pub disable_ansi: bool,
  /// Whether to inspect formatting of package.json files
  pub inspect_formatting: bool,
  /// Whether to inspect semver ranges and versions
  pub inspect_mismatches: bool,
  /// Which severity levels of logging to display
  pub log_levels: Vec<LevelFilter>,
  /// Whether to output ignored dependencies regardless
  pub show_ignored: bool,
  /// Whether to indicate that a dependency is a package developed locally
  pub show_hints: bool,
  /// Whether to list every affected instance of a dependency when listing
  /// version or semver range mismatches
  pub show_instances: bool,
  /// Whether to list every affected package.json file when listing formatting
  /// mismatches
  pub show_packages: bool,
  /// Whether to show the name of the status code for each dependency and
  /// instance, such as `HighestSemverMismatch`
  pub show_status_codes: bool,
  /// Whether to sort dependencies and instances by name A-Z or by descending
  /// count
  pub sort: SortBy,
  /// Glob patterns for package.json files to inspect
  pub source_patterns: Vec<String>,
  /// The subcommand that the user is running
  pub subcommand: Subcommand,
}

impl Cli {
  pub fn parse() -> Cli {
    match create().get_matches().subcommand() {
      Some(("lint", matches)) => Cli::from_arg_matches(Subcommand::Lint, matches),
      Some(("fix", matches)) => Cli::from_arg_matches(Subcommand::Fix, matches),
      Some(("format", matches)) => Cli::from_arg_matches(Subcommand::Format, matches),
      _ => {
        std::process::exit(1);
      }
    }
  }

  /// Create a new `Cli` from CLI arguments provided by the user
  fn from_arg_matches(subcommand: Subcommand, matches: &ArgMatches) -> Self {
    Self {
      check: matches!(&subcommand, Subcommand::Lint) || matches!(&subcommand, Subcommand::Format) && matches.get_flag("check"),
      cwd: env::current_dir().unwrap(),
      // @TODO
      filter: get_filters(matches),
      disable_ansi: matches.get_flag("no-ansi"),
      inspect_formatting: matches!(&subcommand, Subcommand::Format),
      inspect_mismatches: matches!(&subcommand, Subcommand::Lint) || matches!(&subcommand, Subcommand::Fix),
      log_levels: get_log_levels(matches),
      sort: get_order_by(matches),
      show_ignored: should_show(matches, "ignored"),
      show_hints: should_show(matches, "hints"),
      show_instances: should_show(matches, "instances"),
      show_packages: should_show(matches, "packages"),
      show_status_codes: should_show(matches, "statuses"),
      source_patterns: get_patterns(matches, "source"),
      subcommand,
    }
  }
}

fn create() -> Command {
  Command::new(crate_name!())
    .about(crate_description!())
    .version(crate_version!())
    .subcommand(
      Command::new("lint")
        .about("Lint all versions and ranges and exit with 0 or 1 based on whether all files match your Syncpack configuration file")
        .after_long_help(additional_help())
        .arg(
          Arg::new("dependencies")
            .long("dependencies")
            .long_help(cformat!(
              r#"Only display dependencies whose <bold>name</bold> matches this <bold>glob pattern</bold>

<bold><underline>Important:</underline></bold>
1. You <underline>must</> add quotes around your filter so your shell doesn't
   interpret it.
2. --dependencies only affects what syncpack will display, it will
   still inspect and exit 1/0 based on every dependency in your project.

<bold><underline>Examples:</underline></bold>
<dim>Exact match for "react"</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --dependencies 'react'</>
<dim>Substring match for "react"</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --dependencies '**react**'</>
<dim>All dependencies under the AWS SDK scope</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --dependencies '@aws-sdk/**'</>
<dim>Exact match for "react" or "webpack" (2 approaches)</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --dependencies 'react' --dependencies 'webpack'</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --dependencies '{has_braces}'</>
<dim>Substring match for "react" or "webpack"  (2 approaches)</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --dependencies '**react**' --dependencies '**webpack**'</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --dependencies '**{has_braces}**'</>"#, has_braces="{react,webpack}"
            ))
            .action(clap::ArgAction::Append),
        )
        .arg(
          Arg::new("dependency-types")
            .long("dependency-types")
            .long_help(cformat!(
              r#"Only display dependencies of the given type(s)

<bold><underline>Important:</underline></bold>
--dependency-types only affects what syncpack will display, it will
still inspect and exit 1/0 based on every dependency in your project.

<bold><underline>Default Values:</underline></bold>
<blue>dev</>            devDependencies
<blue>local</>          version
<blue>overrides</>      overrides
<blue>peer</>           peerDependencies
<blue>pnpmOverrides</>  pnpm.overrides
<blue>prod</>           dependencies
<blue>resolutions</>    resolutions

<bold><underline>Custom Values:</underline></bold>
See <blue>https://jamiemason.github.io/syncpack/config/custom-types/</>

<bold><underline>Examples:</underline></bold>
<dim>devDependencies only</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --dependency-types dev
<dim>dependencies and devDependencies only</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --dependency-types dev,prod"#
            ))
            .value_delimiter(','),
        )
        .arg(
          Arg::new("specifier-types")
            .long("specifier-types")
            .long_help(cformat!(
              r#"Only display instances whose version specifiers are of the given type(s)

<bold><underline>Important:</underline></bold>
--specifier-types only affects what syncpack will display, it will
still inspect and exit 1/0 based on every dependency in your project.

<bold><underline>Values:</underline></bold>
<blue>alias</>               <yellow>npm:@preact/compat</>
<blue>exact</>               <yellow>1.2.3</>, <yellow>1.2.3-alpha</>, <yellow>1.2.3-rc.1</>
<blue>file</>                <yellow>file:./path/to/package</>
<blue>git</>                 <yellow>git+https://github.com/user/repo.git</>
<blue>latest</>              <yellow>latest</>, <yellow>*</>
<blue>major</>               <yellow>1</>
<blue>minor</>               <yellow>1.2</>
<blue>missing</>             A local package.json with a missing .version
<blue>range</>               <yellow>^1.2.3</>, <yellow>^1.2.3-alpha</>, <yellow>^1.2.3-rc.1</>
<blue>range-complex</>       <yellow>^1.2.3-alpha || ~1.2.3-rc.1</>
<blue>range-major</>         <yellow>^1</>
<blue>range-minor</>         <yellow>^1.2</>
<blue>tag</>                 <yellow>alpha</>
<blue>unsupported</>         <yellow>wtf|#|broken</>
<blue>url</>                 <yellow>https://example.com/package</>
<blue>workspace-protocol</>  <yellow>workspace:*</>

<bold><underline>Examples:</underline></bold>
<dim>Exact versions only</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --show instances --specifier-types exact
<dim>Missing or unsupported versions</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --show instances --specifier-types missing,unsupported
<dim>Latest or workspace protocol only</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --show instances --specifier-types latest,workspace-protocol"#
            ))
            .value_delimiter(',')
            .value_parser([
              "alias",
              "exact",
              "file",
              "git",
              "latest",
              "major",
              "minor",
              "missing",
              "range",
              "range-complex",
              "range-major",
              "range-minor",
              "tag",
              "unsupported",
              "url",
              "workspace-protocol",
            ]),
        )
        .arg(log_levels_option("lint"))
        .arg(no_ansi_option("lint"))
        .arg(
          Arg::new("sort")
            .long("sort")
            .long_help(cformat!(
              r#"Change the order in which dependencies are displayed

<bold><underline>Examples:</underline></bold>
<dim>Sort by count, in descending order</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --sort count</>
<dim>Sort A-Z by name</>
<dim>$</dim> <blue><bold>syncpack lint</bold> --sort name</>"#
            ))
            .action(clap::ArgAction::Set)
            .value_parser(["count", "name"])
            .default_value("name"),
        )
        .arg(
          Arg::new("show")
            .long("show")
            .long_help(cformat!(
              r#"Control what information is displayed in terminal output

<bold><underline>Values:</underline></bold>
<blue>ignored</>    Show instances and dependencies which syncpack is ignoring
<blue>instances</>  Show every instance of every dependency
<blue>hints</>      Show a hint alongside dependencies developed in this repo
<blue>statuses</>   Show specifically how/why a dependency or instance is valid or invalid
<blue>all</>        Shorthand to enable all of the above

<bold><underline>Examples:</underline></bold>
<dim>Only opt into showing status codes</dim>
<dim>$</dim> <blue><bold>syncpack lint</bold> --show statuses</>
<dim>Show all instances, including ignored</dim>
<dim>$</dim> <blue><bold>syncpack lint</bold> --show ignored,instances</>
<dim>Show highest level of detail</dim>
<dim>$</dim> <blue><bold>syncpack lint</bold> --show all</>"#
            ))
            .value_delimiter(',')
            .value_parser(["ignored", "instances", "hints", "statuses", "all"])
            .default_value("hints,statuses"),
        )
        .arg(source_option("lint")),
    )
    .subcommand(
      Command::new("fix")
        .about("Ensure that multiple packages requiring the same dependency define the same version, so that every package requires eg. `react@16.4.2`, instead of a combination of `react@16.4.2`, `react@0.15.9`, and `react@16.0.0`")
        .after_long_help(additional_help())
        .arg(log_levels_option("fix"))
        .arg(no_ansi_option("fix"))
        .arg(source_option("fix")),
    )
    .subcommand(
      Command::new("format")
        .about("Ensure that package.json files follow a conventional format, where fields appear in a predictable order and nested fields are ordered alphabetically. Shorthand properties are used where available")
        .after_long_help(additional_help())
        .arg(Arg::new("check").long("check").long_help(cformat!(r#"Lint formatting instead of fixing it"#)).action(clap::ArgAction::SetTrue))
        .arg(log_levels_option("format"))
        .arg(no_ansi_option("format"))
        .arg(
          Arg::new("show")
            .long("show")
            .long_help(cformat!(
              r#"Control what information is displayed in terminal output

<bold><underline>Values:</underline></bold>
<blue>packages</>   Show formatting status of each package.json file

<bold><underline>Examples:</underline></bold>
<dim>Show highest level of detail</dim>
<dim>$</dim> <blue><bold>syncpack format</bold> --show packages</>"#
            ))
            .value_delimiter(',')
            .value_parser([
              "packages",
            ])
        )
        .arg(source_option("format")),
    )
}

fn get_filters(matches: &ArgMatches) -> Option<GroupSelector> {
  let dependencies = get_patterns(matches, "dependencies");
  let dependency_types = get_patterns(matches, "dependency-types");
  let label = "CLI filters".to_string();
  let all_packages = &Packages::new();
  let packages = get_patterns(matches, "packages");
  let specifier_types = get_patterns(matches, "specifier-types");
  if dependencies.is_empty() && dependency_types.is_empty() && packages.is_empty() && specifier_types.is_empty() {
    None
  } else {
    Some(GroupSelector::new(
      all_packages,
      dependencies,
      dependency_types,
      label,
      packages,
      specifier_types,
    ))
  }
}

fn get_order_by(matches: &ArgMatches) -> SortBy {
  matches
    .try_get_one::<String>("sort")
    .ok()
    .flatten()
    .map(|sort| match sort.as_str() {
      "count" => SortBy::Count,
      "name" => SortBy::Name,
      _ => unreachable!(),
    })
    .unwrap_or(SortBy::Name)
}

fn get_patterns(matches: &ArgMatches, option_name: &str) -> Vec<String> {
  matches
    .try_get_many::<String>(option_name)
    .ok()
    .flatten()
    .map(|source| source.into_iter().map(|source| source.to_owned()).collect_vec())
    .unwrap_or_default()
}

fn should_show(matches: &ArgMatches, name: &str) -> bool {
  matches
    .try_get_many::<String>("show")
    .ok()
    .flatten()
    .map(|show| {
      show
        .collect_vec()
        .iter()
        .any(|value| value == &&"all".to_string() || value == &&name.to_string())
    })
    .unwrap_or(false)
}

fn get_log_levels(matches: &ArgMatches) -> Vec<LevelFilter> {
  let chosen_values = matches
    .try_get_many::<String>("log-levels")
    .ok()
    .flatten()
    .unwrap_or_default()
    .collect_vec();
  vec![
    ("off", LevelFilter::Off),
    ("error", LevelFilter::Error),
    ("warn", LevelFilter::Warn),
    ("info", LevelFilter::Info),
    ("debug", LevelFilter::Debug),
  ]
  .into_iter()
  .filter(|(name, _)| {
    chosen_values
      .iter()
      .any(|choice| choice == &&"all".to_string() || choice == &&name.to_string())
  })
  .map(|(_, level)| level)
  .collect_vec()
}

fn log_levels_option(command: &str) -> Arg {
  Arg::new("log-levels")
    .long("log-levels")
    .long_help(cformat!(
      r#"Control how detailed the log output should be

<bold><underline>Examples:</underline></bold>
<dim>Turn off logging completely</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --log-levels off</>
<dim>Only show verbose debugging logs</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --log-levels debug</>
<dim>Show everything</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --log-levels error,warn,info,debug</>"#
    ))
    .value_delimiter(',')
    .value_parser(["off", "error", "warn", "info", "debug"])
    .default_value("error,warn,info")
}

fn no_ansi_option(command: &str) -> Arg {
  Arg::new("no-ansi")
    .long("no-ansi")
    .long_help(cformat!(
      r#"Disable ANSI colored output and terminal hyperlinks

<bold><underline>Examples:</underline></bold>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --no-ansi</>"#
    ))
    .action(clap::ArgAction::SetTrue)
}

fn source_option(command: &str) -> Arg {
  Arg::new("source")
    .long("source")
    .long_help(cformat!(
      r#"A list of quoted glob patterns for package.json files to read from

<bold><underline>Examples:</underline></bold>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --source 'package.json' --source 'apps/*/package.json'</>

<bold><underline>Resolving Packages:</underline></bold>
Patterns are discovered in the following order, first one wins:

1. <blue>--source</> CLI options
2. <blue>.source</> property of your syncpack config file
3. <blue>.workspaces.packages</> property of package.json (yarn)
4. <blue>.workspaces</> property of package.json (npm and yarn)
5. <blue>.packages</> property of pnpm-workspace.yaml
6. <blue>.packages</> property of lerna.json
7. Default to <blue>["package.json","packages/*/package.json"]</>"#
    ))
    .action(clap::ArgAction::Append)
    .value_parser(ValueParser::new(validate_source))
}

fn additional_help() -> String {
  cformat!(
    r#"<bold><underline>References:</underline></bold>
- Documentation: <blue><underline>https://jamiemason.github.io/syncpack</></>
- Learn glob patterns: <blue><underline>https://github.com/isaacs/node-glob#glob-primer</></>
- lerna.json: <blue><underline>https://github.com/lerna/lerna#lernajson</></>
- Yarn Workspaces: <blue><underline>https://yarnpkg.com/lang/en/docs/workspaces</></>
- Pnpm Workspaces: <blue><underline>https://pnpm.js.org/en/workspaces</></>"#
  )
}

fn validate_source(value: &str) -> Result<String, String> {
  if value.ends_with("package.json") {
    Ok(value.to_string())
  } else {
    Err("must end with 'package.json'".to_string())
  }
}
