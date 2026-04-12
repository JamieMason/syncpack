#![allow(dead_code)]

use {
  crate::{
    disk::LiveDiskIo,
    registry::client::{LiveRegistryClient, RegistryClient},
  },
  log::error,
  std::{process::exit, sync::Arc},
};

#[cfg(test)]
#[path = "test/test.rs"]
mod test;

mod catalogs;
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
mod package_json;
mod packages;
mod rcfile;
mod registry;
mod source_patterns;
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
    syncpack::syncpack(&args, &io, &registry_client).await
  }
  .await;

  if let Err(e) = result {
    let msg = e.to_string();
    if !msg.is_empty() {
      error!("{e}");
    }
    exit(1);
  }
}
