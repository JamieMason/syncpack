#![allow(dead_code)]

use {
  crate::{
    disk::LiveDiskIo,
    errors::SyncpackError,
    registry::client::{LiveRegistryClient, RegistryClient},
  },
  clap::error::ErrorKind,
  log::error,
  std::{process::exit, sync::Arc},
};

#[cfg(test)]
#[path = "test/test.rs"]
mod test;

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
pub use syncpack_specifier::{self as specifier, semver_range};
mod syncpack;
mod version_group;
mod visit_formatting;
mod visit_packages;

#[tokio::main]
async fn main() {
  let result = async {
    logger::init();
    let args: Vec<String> = std::env::args().collect();
    let io = LiveDiskIo::new();
    let registry_client: Arc<dyn RegistryClient> = Arc::new(LiveRegistryClient::new());
    let (ctx, registry_updates) = syncpack::syncpack(&args, &io, &registry_client).await?;
    syncpack::run(ctx, registry_updates, &io)
  }
  .await;

  if let Err(e) = result {
    if let SyncpackError::CliError(clap_err) = &e {
      if matches!(clap_err.kind(), ErrorKind::DisplayHelp | ErrorKind::DisplayVersion) {
        println!("{clap_err}");
        return;
      }
    }
    let msg = e.to_string();
    if !msg.is_empty() {
      error!("{e}");
    }
    exit(1);
  }
}
