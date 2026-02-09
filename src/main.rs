use {
  crate::{
    cli::{Cli, Subcommand},
    commands::{fix, fix_mismatches, format, json, lint, lint_semver_ranges, list, list_mismatches, prompt, set_semver_ranges, update},
    config::Config,
    context::Context,
    packages::Packages,
    registry_client::{LiveRegistryClient, RegistryClient},
    visit_formatting::visit_formatting,
    visit_packages::visit_packages,
  },
  log::{debug, error},
  std::{process::exit, sync::Arc},
};

#[cfg(test)]
#[path = "test/test.rs"]
mod test;

mod catalogs;
mod cli;
mod commands;
mod config;
mod context;
mod dependency;
mod dependency_type;
mod group_selector;
mod instance;
mod instance_state;
mod logger;
#[cfg(test)]
mod npmrc_integration_test;
mod package_json;
mod packages;
#[cfg(test)]
mod packages_test;
mod pattern_matcher;
mod rcfile;
#[cfg(test)]
mod rcfile_test;
mod registry_client;
#[cfg(test)]
mod registry_client_test;
mod semver_group;
mod semver_range;
mod specifier;
mod version_group;
mod visit_formatting;
mod visit_packages;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  logger::init(&cli);

  let config = Config::from_cli(cli);
  let is_update_command = matches!(&config.cli.subcommand, Subcommand::Update);

  debug!("Command: {:?}", config.cli.subcommand);
  debug!("{:#?}", config.cli);
  debug!("{:#?}", config.rcfile);

  let packages = Packages::from_config(&config);
  let catalogs = None; // catalogs::from_config(&config);

  match packages.all.len() {
    0 => {
      error!("Found 0 package.json files");
      exit(1);
    }
    len => debug!("Found {len} package.json files"),
  }

  let registry_client = if is_update_command {
    let npmrc = npmrc_config_rs::NpmrcConfig::load().unwrap_or_else(|e| {
      error!("Failed to load .npmrc config: {e}");
      exit(1);
    });
    Some(Arc::new(LiveRegistryClient::new(npmrc)) as Arc<dyn RegistryClient>)
  } else {
    None
  };

  let ctx = Context::create(config, packages, registry_client, catalogs);

  let _exit_code = match ctx.config.cli.subcommand {
    Subcommand::Fix => {
      let ctx = visit_packages(ctx);
      fix::run(ctx)
    }
    Subcommand::Format => {
      let ctx = visit_formatting(ctx);
      format::run(ctx)
    }
    Subcommand::Lint => {
      let ctx = visit_packages(ctx);
      lint::run(ctx)
    }
    Subcommand::Update => {
      let mut ctx = ctx;
      ctx.fetch_all_updates().await;
      let ctx = visit_packages(ctx);
      update::run(ctx)
    }
    Subcommand::List => {
      let ctx = visit_packages(ctx);
      list::run(ctx)
    }
    Subcommand::Json => {
      let ctx = visit_packages(ctx);
      json::run(ctx)
    }
    Subcommand::ListMismatches => list_mismatches::run(),
    Subcommand::LintSemverRanges => lint_semver_ranges::run(),
    Subcommand::FixMismatches => fix_mismatches::run(),
    Subcommand::SetSemverRanges => set_semver_ranges::run(),
    Subcommand::Prompt => prompt::run(),
  };

  exit(_exit_code);
}
