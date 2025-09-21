use {
  crate::{dependency_type::DependencyType, group_selector::GroupSelector, packages::Packages},
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
  Update,
  List,
  Json,
}

#[derive(Debug)]
pub enum SortBy {
  Count,
  Name,
}

#[derive(Debug)]
pub enum UpdateTarget {
  /// "*.*.*"
  Latest,
  /// "1.*.*"
  Minor,
  /// "1.2.*"
  Patch,
}

#[derive(Debug)]
pub struct Cli {
  /// Whether to check formatting instead of fixing it
  pub check: bool,
  /// The path to the root of the project
  pub cwd: PathBuf,
  /// Whether to disable ANSI color codes in terminal output
  pub disable_ansi: bool,
  /// Whether to simulate changes without writing them to disk
  pub dry_run: bool,
  /// - `--dependencies` to filter by dependency name
  pub dependencies: Vec<String>,
  /// - `--dependency-types` to filter by dependency type
  pub dependency_types: Vec<String>,
  /// - `--packages` to filter by package name
  pub packages: Vec<String>,
  /// - `--specifier-types` to filter by specifier type
  pub specifier_types: Vec<String>,
  /// Which severity levels of logging to display
  #[allow(dead_code)]
  pub log_levels: Vec<LevelFilter>,
  /// Whether to indicate that a dependency is a package developed locally
  pub show_hints: bool,
  /// Whether to output ignored dependencies regardless
  pub show_ignored: bool,
  /// Whether to list every affected instance of a dependency when listing
  /// version or semver range mismatches
  pub show_instances: bool,
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
  /// How greedy npm updates should be
  pub target: UpdateTarget,
}

impl Cli {
  /// Parse all command-line arguments from the user into a `Cli` struct
  pub fn parse() -> Self {
    fn from_arg_matches(subcommand: Subcommand, matches: &ArgMatches) -> Cli {
      Cli {
        check: (matches!(&subcommand, Subcommand::Format | Subcommand::Update)) && matches.get_flag("check"),
        cwd: env::current_dir().unwrap(),
        dependencies: get_patterns(matches, "dependencies"),
        dependency_types: get_patterns(matches, "dependency-types"),
        disable_ansi: matches.get_flag("no-ansi"),
        dry_run: (matches!(&subcommand, Subcommand::Fix | Subcommand::Format | Subcommand::Update)) && matches.get_flag("dry-run"),
        log_levels: get_log_levels(matches),
        packages: get_patterns(matches, "packages"),
        show_hints: should_show(matches, "hints"),
        show_ignored: should_show(matches, "ignored"),
        show_instances: should_show(matches, "instances"),
        show_status_codes: should_show(matches, "statuses"),
        sort: get_order_by(matches),
        source_patterns: get_patterns(matches, "source"),
        specifier_types: get_patterns(matches, "specifier-types"),
        subcommand,
        target: get_target(matches),
      }
    }

    match create().get_matches().subcommand() {
      Some(("lint", matches)) => from_arg_matches(Subcommand::Lint, matches),
      Some(("fix", matches)) => from_arg_matches(Subcommand::Fix, matches),
      Some(("format", matches)) => from_arg_matches(Subcommand::Format, matches),
      Some(("update", matches)) => from_arg_matches(Subcommand::Update, matches),
      Some(("list", matches)) => from_arg_matches(Subcommand::List, matches),
      Some(("json", matches)) => from_arg_matches(Subcommand::Json, matches),
      _ => {
        std::process::exit(1);
      }
    }
  }

  /// Create a `GroupSelector` struct based on the provided command line options
  /// which relate to filtering of packages and dependencies.
  ///
  /// `GroupSelector` is the same struct as used by `VersionGroup` and
  /// `SemverGroup` and this CLI `GroupSelector`, when configured, serves as a
  /// single `VersionGroup` which overrides all those set in config.
  pub fn get_filters(&self, packages: &Packages, all_dependency_types: &[DependencyType]) -> Option<GroupSelector> {
    if self.dependencies.is_empty() && self.dependency_types.is_empty() && self.packages.is_empty() && self.specifier_types.is_empty() {
      None
    } else {
      Some(GroupSelector::new(
        /* all_packages: */ packages,
        /* include_dependencies: */ self.dependencies.clone(),
        /* include_dependency_types: */ self.dependency_types.clone(),
        /* alias_name: */ "CLI filters".to_string(),
        /* include_packages: */ self.packages.clone(),
        /* include_specifier_types: */ self.specifier_types.clone(),
        /* all_dependency_types: */ all_dependency_types,
      ))
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
        .arg(dependencies_option("lint"))
        .arg(dependency_types_option("lint"))
        .arg(log_levels_option("lint"))
        .arg(no_ansi_option("lint"))
        .arg(show_option_versions("lint"))
        .arg(sort_option("lint"))
        .arg(source_option("lint"))
        .arg(specifier_types_option("lint")),
    )
    .subcommand(
      Command::new("fix")
        .about("Ensure that multiple packages requiring the same dependency use the same version")
        .after_long_help(additional_help())
        .arg(dependencies_option("fix"))
        .arg(dependency_types_option("fix"))
        .arg(dry_run_option("fix"))
        .arg(log_levels_option("fix"))
        .arg(no_ansi_option("fix"))
        .arg(show_option_versions("fix"))
        .arg(sort_option("fix"))
        .arg(source_option("fix"))
        .arg(specifier_types_option("fix")),
    )
    .subcommand(
      Command::new("format")
        .about("Sort package.json fields into a predictable order and nested fields alphabetically")
        .after_long_help(additional_help())
        .arg(
          Arg::new("check")
            .long("check")
            .long_help(cformat!(r#"Lint formatting instead of fixing it"#))
            .action(clap::ArgAction::SetTrue),
        )
        .arg(dry_run_option("format"))
        .arg(log_levels_option("format"))
        .arg(no_ansi_option("format"))
        .arg(source_option("format")),
    )
    .subcommand(
      Command::new("update")
        .about("Update to the latest versions on the npm registry")
        .after_long_help(additional_help())
        .arg(
          Arg::new("check")
            .long("check")
            .long_help(cformat!(r#"Check versions are up to date instead of updating them"#))
            .action(clap::ArgAction::SetTrue),
        )
        .arg(dependencies_option("update"))
        .arg(dependency_types_option("update"))
        .arg(dry_run_option("update"))
        .arg(log_levels_option("update"))
        .arg(no_ansi_option("update"))
        .arg(source_option("update"))
        .arg(specifier_types_option("update"))
        .arg(target_option("update")),
    )
    .subcommand(
      Command::new("list")
        .about("Query and inspect all dependencies in your project, both valid and invalid")
        .after_long_help(additional_help())
        .arg(dependencies_option("list"))
        .arg(dependency_types_option("list"))
        .arg(log_levels_option("list"))
        .arg(no_ansi_option("list"))
        .arg(show_option_list("list"))
        .arg(sort_option("list"))
        .arg(source_option("list"))
        .arg(specifier_types_option("list")),
    )
    .subcommand(
      Command::new("json")
        .about("Output all dependencies as flattened JSON objects")
        .after_long_help(additional_help())
        .arg(dependencies_option("json"))
        .arg(dependency_types_option("json"))
        .arg(log_levels_option("json"))
        .arg(no_ansi_option("json"))
        .arg(sort_option("json"))
        .arg(source_option("json"))
        .arg(specifier_types_option("json")),
    )
}

fn dependencies_option(command: &str) -> Arg {
  let short_help = "Only include dependencies whose name matches this glob pattern";
  Arg::new("dependencies")
    .long("dependencies")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

<bold><underline>Important:</underline></bold>
You <underline>must</> add quotes around your filter so your shell doesn't
interpret it.

<bold><underline>Examples:</underline></bold>
<dim>Exact match for "react"</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --dependencies 'react'</>
<dim>Substring match for "react"</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --dependencies '**react**'</>
<dim>All dependencies under the AWS SDK scope</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --dependencies '@aws-sdk/**'</>
<dim>Exact match for "react" or "webpack" (2 approaches)</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --dependencies 'react' --dependencies 'webpack'</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --dependencies '{has_braces}'</>
<dim>Substring match for "react" or "webpack"  (2 approaches)</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --dependencies '**react**' --dependencies '**webpack**'</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --dependencies '**{has_braces}**'</>"#,
      has_braces = "{react,webpack}"
    ))
    .action(clap::ArgAction::Append)
    .value_name("dependency-name-pattern")
}

fn show_option_versions(command: &str) -> Arg {
  let short_help = "Control what information is displayed in terminal output";
  Arg::new("show")
    .long("show")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

<bold><underline>Values:</underline></bold>
<blue>instances</>  Show every instance of every dependency
<blue>hints</>      Show a hint alongside dependencies developed in this repo
<blue>statuses</>   Show specifically how/why a dependency or instance is valid or invalid
<blue>all</>        Shorthand to enable all of the above
<blue>none</>       Shorthand to disable all of the above

<bold><underline>Examples:</underline></bold>
<dim>Only opt into showing status codes</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show statuses</>
<dim>Show all instances, not just their names</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show instances</>
<dim>Show highest level of detail</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show all</>
<dim>Show lowest level of detail</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show none</>"#
    ))
    .value_delimiter(',')
    .value_parser(["hints", "instances", "statuses", "all", "none"])
    .value_name("comma-separated-detail-names")
    .default_value("all")
}

fn show_option_list(command: &str) -> Arg {
  let short_help = "Control what information is displayed in terminal output";
  Arg::new("show")
    .long("show")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

<bold><underline>Values:</underline></bold>
<blue>instances</>  Show every instance of every dependency
<blue>hints</>      Show a hint alongside dependencies developed in this repo
<blue>statuses</>   Show specifically how/why a dependency or instance is valid or invalid
<blue>all</>        Shorthand to enable all of the above
<blue>none</>       Shorthand to disable all of the above

<bold><underline>Examples:</underline></bold>
<dim>Only opt into showing status codes</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show statuses</>
<dim>Show all instances, not just their names</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show instances</>
<dim>Show ignored dependencies and instances</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show ignored</>
<dim>Show highest level of detail</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show all</>
<dim>Choose only some values</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show hints,statuses</>
<dim>Show lowest level of detail</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show none</>"#
    ))
    .value_delimiter(',')
    .value_parser(["hints", "ignored", "instances", "statuses", "all", "none"])
    .value_name("comma-separated-detail-names")
    .default_value("hints,statuses")
}

fn sort_option(command: &str) -> Arg {
  let short_help = "Change the order in which dependencies are displayed";
  Arg::new("sort")
    .long("sort")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

<bold><underline>Examples:</underline></bold>
<dim>Sort by count, in descending order</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --sort count</>
<dim>Sort A-Z by name</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --sort name</>"#
    ))
    .action(clap::ArgAction::Set)
    .value_parser(["count", "name"])
    .value_name("choice")
    .default_value("name")
}

fn specifier_types_option(command: &str) -> Arg {
  let short_help = "Only include instances whose version specifiers are of the given type(s)";
  Arg::new("specifier-types")
    .long("specifier-types")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

<bold><underline>Values:</underline></bold>
<blue>alias</>               <yellow>npm:@preact/compat</>
<blue>exact</>               <yellow>1.2.3</>, <yellow>1.2.3-alpha</>, <yellow>1.2.3-rc.1</>
<blue>file</>                <yellow>file:./path/to/package</>
<blue>git</>                 <yellow>git+https://github.com/user/repo.git</>
<blue>latest</>              <yellow>latest</>, <yellow>*</>
<blue>link</>                <yellow>link:../package-a</>
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
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show instances --specifier-types exact
<dim>Missing or unsupported versions</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show instances --specifier-types missing,unsupported
<dim>Latest or workspace protocol only</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --show instances --specifier-types latest,workspace-protocol"#
    ))
    .value_delimiter(',')
    .value_parser([
      "alias",
      "exact",
      "file",
      "git",
      "latest",
      "link",
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
    ])
    .value_name("comma-separated-specifier-type-names")
}

fn dependency_types_option(command: &str) -> Arg {
  let short_help = "Only include dependencies of the given type(s)";
  Arg::new("dependency-types")
    .long("dependency-types")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

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
<dim>$</dim> <blue><bold>syncpack {command}</bold> --dependency-types dev
<dim>dependencies and devDependencies only</>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --dependency-types dev,prod"#
    ))
    .value_delimiter(',')
    .value_name("comma-separated-dependency-type-names")
    .default_value("dev,local,overrides,peer,pnpmOverrides,prod,resolutions")
}

fn dry_run_option(command: &str) -> Arg {
  let short_help = "Simulate changes without writing them to disk";
  Arg::new("dry-run")
    .long("dry-run")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

<bold><underline>Examples:</underline></bold>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --dry-run</>"#
    ))
    .action(clap::ArgAction::SetTrue)
}

fn log_levels_option(command: &str) -> Arg {
  let short_help = "Control how detailed the log output should be";
  Arg::new("log-levels")
    .long("log-levels")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

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
    .value_name("comma-separated-log-level-names")
    .default_value("error,warn,info")
}

fn no_ansi_option(command: &str) -> Arg {
  let short_help = "Disable ANSI colored output and terminal hyperlinks";
  Arg::new("no-ansi")
    .long("no-ansi")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

<bold><underline>Examples:</underline></bold>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --no-ansi</>"#
    ))
    .action(clap::ArgAction::SetTrue)
}

fn source_option(command: &str) -> Arg {
  let short_help = "A list of quoted glob patterns for package.json files to read from";
  Arg::new("source")
    .long("source")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

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
    .value_name("file-pattern")
}

fn target_option(command: &str) -> Arg {
  let short_help = "Limit updates to only those within the semver portion";
  Arg::new("target")
    .long("target")
    .help(short_help)
    .long_help(cformat!(
      r#"{short_help}

<bold><underline>Examples:</underline></bold>
<dim>Accept any update in latest (x.x.x)</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --target latest</>
<dim>Only update minor versions (1.x.x)</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --target minor</>
<dim>Only update patch versions (1.2.x)</dim>
<dim>$</dim> <blue><bold>syncpack {command}</bold> --target patch</>"#
    ))
    .action(clap::ArgAction::Set)
    .value_parser(["latest", "minor", "patch"])
    .value_name("greediness")
    .default_value("latest")
}

fn additional_help() -> String {
  cformat!(
    r#"<bold><underline>References:</underline></bold>
- Documentation: <blue><underline>https://jamiemason.github.io/syncpack</></>"#
  )
}

fn validate_source(value: &str) -> Result<String, String> {
  if value.ends_with("package.json") {
    Ok(value.to_string())
  } else {
    Err("must end with 'package.json'".to_string())
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

fn get_target(matches: &ArgMatches) -> UpdateTarget {
  matches
    .try_get_one::<String>("target")
    .ok()
    .flatten()
    .map(|target| match target.as_str() {
      "latest" => UpdateTarget::Latest,
      "minor" => UpdateTarget::Minor,
      "patch" => UpdateTarget::Patch,
      _ => unreachable!(),
    })
    .unwrap_or(UpdateTarget::Latest)
}

fn should_show(matches: &ArgMatches, name: &str) -> bool {
  matches
    .try_get_many::<String>("show")
    .ok()
    .flatten()
    .map(|show| {
      let names = show.collect_vec();
      if names.contains(&&"all".to_string()) {
        true
      } else if names.contains(&&"none".to_string()) {
        false
      } else {
        names.contains(&&name.to_string())
      }
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
