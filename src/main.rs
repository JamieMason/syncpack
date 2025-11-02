#![allow(dead_code)]

use {
  crate::{
    cli::{Cli, Subcommand},
    commands::{fix, format, json, lint, list, update},
    config::Config,
    context::Context,
    packages::Packages,
    registry_client::LiveRegistryClient,
    visit_formatting::visit_formatting,
    visit_packages::visit_packages,
  },
  log::{debug, error},
  std::{process::exit, sync::Arc},
};

#[cfg(test)]
#[path = "test/test.rs"]
mod test;

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
mod package_json;
mod packages;
mod rcfile;
mod registry_client;
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

  match packages.all.len() {
    0 => {
      error!("Found 0 package.json files");
      exit(1);
    }
    len => debug!("Found {len} package.json files"),
  }

  // PHASE 1: Create Context
  // Read all data, collect dependencies, assign to version groups.
  // At this point all instances have InstanceState::Unknown.
  // See src/context.rs for implementation.
  let ctx = Context::create(
    config,
    packages,
    if is_update_command {
      Some(Arc::new(LiveRegistryClient::new()))
    } else {
      None
    },
  );

  // PHASE 2 & 3: Inspect and Run
  // Each command chooses a visitor (visit_packages or visit_formatting) to
  // assign InstanceState, then processes instances based on those states.
  //
  // Pattern: visitor takes ownership of Context, tags instances, returns Context.
  // Command consumes Context and returns exit code (0 or 1).
  //
  // See src/visit_packages.rs and src/commands/*.rs for implementations.
  let _exit_code = match ctx.config.cli.subcommand {
    Subcommand::Fix => {
      let ctx = visit_packages(ctx); // Phase 2: Tag instances
      fix::run(ctx) // Phase 3: Fix and write files
    }
    Subcommand::Format => {
      let ctx = visit_formatting(ctx); // Phase 2: Check formatting
      format::run(ctx) // Phase 3: Fix formatting
    }
    Subcommand::Lint => {
      let ctx = visit_packages(ctx); // Phase 2: Tag instances
      lint::run(ctx) // Phase 3: Report issues
    }
    Subcommand::Update => {
      let mut ctx = ctx;
      ctx.fetch_all_updates().await; // Fetch from npm registry
      let ctx = visit_packages(ctx); // Phase 2: Tag instances
      update::run(ctx) // Phase 3: Update versions
    }
    Subcommand::List => {
      let ctx = visit_packages(ctx); // Phase 2: Tag instances
      list::run(ctx) // Phase 3: List dependencies
    }
    Subcommand::Json => {
      let ctx = visit_packages(ctx); // Phase 2: Tag instances
      json::run(ctx) // Phase 3: JSON output
    }
  };

  exit(_exit_code);
}
