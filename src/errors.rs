use {
  crate::{context::ContextError, rcfile_loader::RcfileError},
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
  #[error(transparent)]
  CatalogError(#[from] syncpack_disk::catalogs::CatalogError),
}
