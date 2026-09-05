#![allow(dead_code)]

use {
  crate::{cli::Cli, errors::SyncpackError, tui::LiveTui},
  clap::error::ErrorKind,
  log::{debug, error},
  std::{process::exit, sync::Arc},
  syncpack_io::LiveDiskIo,
  syncpack_registry::{
    cache::default_cache_filepath,
    cached_client::CachedRegistryClient,
    client::{LiveRegistryClient, RegistryClient},
  },
};

#[cfg(test)]
#[path = "version_group/banned_test.rs"]
mod banned_test;
#[cfg(test)]
#[path = "version_group/bun_catalog_test.rs"]
mod bun_catalog_test;
#[cfg(test)]
#[path = "version_group/catalog_defs_test.rs"]
mod catalog_defs_test;
#[cfg(test)]
#[path = "version_group/catalog_test.rs"]
mod catalog_test;
#[cfg(test)]
#[path = "catalogs_test.rs"]
mod catalogs_test;
mod cli;
mod commands;
mod context;
mod errors;
#[cfg(test)]
#[path = "visit_formatting/format_test.rs"]
mod format_test;
#[cfg(test)]
#[path = "version_group/ignored_test.rs"]
mod ignored_test;
#[cfg(test)]
#[path = "instance/instance_test.rs"]
mod instance_test;
mod javascript;
mod logger;
#[cfg(test)]
#[path = "version_group/pinned_test.rs"]
mod pinned_test;
#[cfg(test)]
#[path = "version_group/preferred_semver_test.rs"]
mod preferred_semver_test;
mod rcfile_loader;
#[cfg(test)]
#[path = "version_group/same_minor_test.rs"]
mod same_minor_test;
#[cfg(test)]
#[path = "version_group/same_range_test.rs"]
mod same_range_test;
#[cfg(test)]
#[path = "version_group/semver_range_only_test.rs"]
mod semver_range_only_test;
#[cfg(test)]
#[path = "version_group/snapped_to_test.rs"]
mod snapped_to_test;
#[cfg(test)]
#[path = "instance/source_idx_test.rs"]
mod source_idx_test;
mod source_patterns;
#[cfg(test)]
#[path = "sources_test.rs"]
mod sources_test;
#[cfg(test)]
#[path = "test/test.rs"]
mod test;
pub use syncpack_specifier::{self as specifier, semver_range};
mod syncpack;
mod tui;

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
    let inspected = syncpack::syncpack(cli, &*io, &registry_client).await?;
    debug!("config: {:#?}", inspected.data.config);
    syncpack::run(inspected, &*io, &tui)
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
