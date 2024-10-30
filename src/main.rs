#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use {
  crate::{
    cli::{Cli, Subcommand},
    config::Config,
    effects::{fix, lint},
    packages::Packages,
    visit_packages::visit_packages,
  },
  context::Context,
  log::{debug, error},
};

#[cfg(test)]
#[path = "test/test.rs"]
mod test;

mod cli;
mod config;
mod context;
mod dependency;
mod dependency_type;
mod effects;
mod format;
mod group_selector;
mod instance;
mod instance_state;
mod logger;
mod package_json;
mod packages;
mod rcfile;
mod semver_group;
mod specifier;
mod version_group;
mod visit_packages;

fn main() {
  let cli = Cli::parse();

  logger::init(&cli);

  let config = Config::from_cli(cli);

  debug!("Command: {:?}", config.cli.subcommand);
  debug!("{:#?}", config.cli);
  debug!("{:#?}", config.rcfile);

  let packages = Packages::from_config(&config);

  match packages.all.len() {
    0 => {
      error!("Found 0 package.json files");
      std::process::exit(1);
    }
    len => debug!("Found {len} package.json files"),
  }

  let ctx = Context::create(config, packages);
  let ctx = visit_packages(ctx);

  match ctx.config.cli.subcommand {
    Subcommand::Fix => {
      let ctx = fix::run(ctx);
      ctx.exit_program();
    }
    Subcommand::Format => {
      let ctx = if ctx.config.cli.check { lint::run(ctx) } else { fix::run(ctx) };
      ctx.exit_program();
    }
    Subcommand::Lint => {
      let ctx = lint::run(ctx);
      ctx.exit_program();
    }
  };
}
