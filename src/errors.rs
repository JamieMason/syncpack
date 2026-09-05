use {
  crate::{context::ContextError, rcfile::from_disk::RcfileError},
  syncpack_io::DiskIoError,
  thiserror::Error,
};

#[derive(Debug, Error)]
pub enum SyncpackError {
  #[error(transparent)]
  ContextError(ContextError),
  #[error(transparent)]
  DiskIoError(#[from] DiskIoError),
  #[error("Deprecated command")]
  DeprecatedCommand,
  #[error("Issues found")]
  IssuesFound,
  #[error("Cancelled, no changes written")]
  Cancelled,
  #[error("{0}")]
  CliError(clap::Error),
  #[error("No subcommand specified")]
  NoSubcommand,
  #[error(transparent)]
  RcfileError(RcfileError),
  #[error(
    "Bun catalog blocks were found at both top-level (/catalog or /catalogs/*) and nested under /workspaces (/workspaces/catalog or /workspaces/catalogs/*) in the root package.json. Only one location can be used."
  )]
  BunDualCatalogPath,
}
