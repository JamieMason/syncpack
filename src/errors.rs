use {crate::rcfile::error::RcfileError, std::fmt, thiserror::Error};

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error("Config property '{property}' is deprecated\n{hint}")]
  DeprecatedProperty { property: String, hint: String },
  #[error("Config property '{path}' is not recognised")]
  UnrecognisedProperty { path: String },
  #[error("dependencyType '{name}' does not match any built-in or custom types")]
  InvalidDependencyType { name: String },
  #[error("Invalid semver group: must have isDisabled, isIgnored, or range")]
  InvalidSemverGroup,
  #[error("Unrecognised version group policy: '{0}'")]
  InvalidVersionGroupPolicy(String),
}

#[derive(Debug)]
pub enum SyncpackError {
  DeprecatedCommand,
  InvalidConfig(Vec<ConfigError>),
  IssuesFound,
  NoSubcommand,
  RcfileError(RcfileError),
}

impl fmt::Display for SyncpackError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::DeprecatedCommand => Ok(()),
      Self::InvalidConfig(errors) => {
        for (i, e) in errors.iter().enumerate() {
          if i > 0 {
            writeln!(f)?;
          }
          write!(f, "{e}")?;
        }
        write!(f, "\ncheck your syncpack config file, see https://syncpack.dev for documentation")
      }
      Self::IssuesFound => Ok(()),
      Self::NoSubcommand => write!(f, "No subcommand specified"),
      Self::RcfileError(e) => write!(f, "{e}"),
    }
  }
}

impl std::error::Error for SyncpackError {}

impl From<RcfileError> for SyncpackError {
  fn from(e: RcfileError) -> Self {
    Self::RcfileError(e)
  }
}

impl From<ConfigError> for SyncpackError {
  fn from(e: ConfigError) -> Self {
    Self::InvalidConfig(vec![e])
  }
}

impl From<Vec<ConfigError>> for SyncpackError {
  fn from(errors: Vec<ConfigError>) -> Self {
    Self::InvalidConfig(errors)
  }
}
