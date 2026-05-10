#![allow(dead_code)]

use {
  crate::{
    cli::Cli,
    disk::LiveDiskIo,
    errors::SyncpackError,
    registry::{
      cache::default_cache_filepath,
      cached_client::CachedRegistryClient,
      client::{LiveRegistryClient, RegistryClient},
    },
    tui::LiveTui,
  },
  clap::error::ErrorKind,
  log::{debug, error},
  std::{process::exit, sync::Arc},
};

mod catalogs;
#[cfg(test)]
#[path = "catalogs_test.rs"]
mod catalogs_test;
mod cli;
mod commands;
mod context;
mod dependency;
mod disk;
mod errors;
mod file_paths;
mod group_selector;
mod instance;
mod logger;
mod rcfile;
mod registry;
mod source;
mod source_patterns;
mod sources;
#[cfg(test)]
#[path = "test/test.rs"]
mod test;
pub use syncpack_specifier::{self as specifier, semver_range};
mod syncpack;
mod tui;
mod version_group;
mod visit_formatting;
mod visit_packages;

#[tokio::main]
async fn main() {
  let result = async {
    logger::init();
    let args: Vec<String> = std::env::args().collect();
    let cli = Cli::parse(&args)?;
    let io = Arc::new(LiveDiskIo::new());
    let registry_client: Arc<dyn RegistryClient> = if cli.no_cache {
      Arc::new(LiveRegistryClient::new())
    } else {
      Arc::new(CachedRegistryClient::new(
        LiveRegistryClient::new(),
        Arc::clone(&io),
        default_cache_filepath(),
      ))
    };
    let tui = LiveTui::new();
    let (ctx, registry_updates) = syncpack::syncpack(cli, &*io, &registry_client).await?;
    debug!("config: {:#?}", ctx.config);
    syncpack::run(ctx, registry_updates, &*io, &tui)
  }
  .await;

  if let Err(e) = result {
    if let SyncpackError::CliError(clap_err) = &e
      && matches!(clap_err.kind(), ErrorKind::DisplayHelp | ErrorKind::DisplayVersion)
    {
      println!("{clap_err}");
      return;
    }
    let msg = e.to_string();
    if !msg.is_empty() {
      error!("{e}");
    }
    exit(1);
  }
}
