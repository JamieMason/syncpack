use {
  crate::{context::ContextError, disk::DiskIoError, rcfile::from_disk::RcfileError},
  std::fmt,
  thiserror::Error,
};

#[derive(Debug, Error)]
pub struct UnsupportedConfigErrors(pub Vec<UnsupportedConfigError>);

impl fmt::Display for UnsupportedConfigErrors {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, e) in self.0.iter().enumerate() {
      if i > 0 {
        writeln!(f)?;
      }
      write!(f, "{e}")?;
    }
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum UnsupportedConfigError {
  #[error("Config property '{property}' is deprecated\n{hint}")]
  DeprecatedProperty { property: String, hint: String },
  #[error("dependencyType '{name}' does not match any built-in or custom types")]
  InvalidDependencyType { name: String },
  #[error("Invalid semver group: must have isDisabled, isIgnored, or range")]
  InvalidSemverGroup,
  #[error("Unrecognised version group policy: '{0}'")]
  InvalidVersionGroupPolicy(String),
  #[error("customTypes.<name>.source: '{value}' is not a recognised source.\nUse 'PackageJson' or 'PnpmWorkspace'.")]
  InvalidSource { value: String },
  #[error("Config property '{path}' is not recognised")]
  UnrecognisedProperty { path: String },
}

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
