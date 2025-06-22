use {
  crate::{
    cli::{Cli, Subcommand},
    config::Config,
    effects::{fix, format, lint, update},
    packages::Packages,
  },
  context::Context,
  effects::list,
  log::{debug, error},
  std::process::exit,
  visit_formatting::visit_formatting,
  visit_packages::visit_packages,
};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[cfg(test)]
#[path = "test/test.rs"]
mod test;

mod cli;
mod config;
mod context;
mod dependency;
mod dependency_type;
mod effects;
mod group_selector;
mod instance;
mod instance_state;
mod logger;
mod package_json;
mod packages;
mod rcfile;
mod registry_client;
mod semver_group;
mod specifier;
mod version_group;
mod visit_formatting;
mod visit_packages;

#[tokio::main]
async fn main() {
  #[cfg(feature = "dhat-heap")]
  let _profiler = dhat::Profiler::new_heap();

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
      exit(1);
    }
    len => debug!("Found {len} package.json files"),
  }

  let ctx = Context::create(config, packages, None);

  match ctx.config.cli.subcommand {
    Subcommand::Fix => {
      let ctx = visit_packages(ctx);
      fix::run(ctx);
    }
    Subcommand::Format => {
      let ctx = visit_formatting(ctx);
      format::run(ctx);
    }
    Subcommand::Lint => {
      let ctx = visit_packages(ctx);
      lint::run(ctx);
    }
    Subcommand::Update => {
      let mut ctx = ctx;
      ctx.fetch_all_updates().await;
      let ctx = visit_packages(ctx);
      update::run(ctx);
    }
    Subcommand::List => {
      let ctx = visit_packages(ctx);
      list::run(ctx);
    }
  };

  #[cfg(feature = "dhat-heap")]
  drop(_profiler);
}
