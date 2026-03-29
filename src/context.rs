use {
  crate::{
    catalogs::CatalogsByName,
    cli::Cli,
    instance::{Instance, InstanceIdx},
    packages::Packages,
    rcfile::{error::RcfileError, Rcfile},
    version_group::{VersionGroup, VersionGroupBehavior},
  },
  log::debug,
  std::{fmt, mem},
  thiserror::Error,
};

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

#[derive(Debug)]
pub struct Config {
  pub cli: Cli,
  pub rcfile: Rcfile,
}

impl Config {
  /// Read the rcfile from stdin and fall back to defaults if none was sent
  pub fn from_cli(cli: Cli) -> Result<Config, SyncpackError> {
    Ok(Config {
      rcfile: Rcfile::from_disk(&cli)?,
      cli,
    })
  }
}

/// The central data structure that owns all project data.
#[derive(Debug)]
pub struct Context {
  /// If present, the contents of each bun or pnpm catalog. The default catalog
  /// is keyed under "default" and named by their names.
  ///
  /// - https://pnpm.io/catalogs
  /// - https://bun.sh/docs/pm/catalogs
  #[allow(dead_code)]
  pub catalogs: Option<CatalogsByName>,
  /// All default configuration with user config applied
  pub config: Config,
  /// Every instance in the project (arena — owns all instances).
  pub instances: Vec<Instance>,
  /// Every package.json in the project
  pub packages: Packages,
  /// All version groups, their dependencies, and their instances
  pub version_groups: Vec<VersionGroup>,
}

impl Context {
  /// Read all configuration and package.json files, collect all dependency
  /// instances, and assign them to version groups.
  pub fn create(mut config: Config, packages: Packages, catalogs: Option<CatalogsByName>) -> Result<Self, SyncpackError> {
    let mut instances = vec![];
    let dependency_groups = mem::take(&mut config.rcfile.dependency_groups);
    let semver_groups = mem::take(&mut config.rcfile.semver_groups);
    let mut version_groups = config.rcfile.get_version_groups(&packages)?;
    let all_dependency_types = &config.rcfile.all_dependency_types;
    if let Some(ref filters) = config.cli.filters {
      filters.validate_dependency_types(all_dependency_types)?;
    }

    packages.get_all_instances(all_dependency_types, |mut descriptor| {
      let dependency_group = dependency_groups.iter().find(|alias| alias.can_add(&descriptor));

      if let Some(group) = dependency_group {
        descriptor.internal_name = group.label.clone();
      }

      descriptor.matches_cli_filter = config.cli.filters.as_ref().is_none_or(|f| f.can_add(&descriptor));

      if !descriptor.matches_cli_filter {
        return;
      }

      let preferred_semver_range = semver_groups
        .iter()
        .find(|group| group.selector.can_add(&descriptor))
        .and_then(|group| group.range.clone());

      let version_group = version_groups.iter_mut().find(|group| group.selector().can_add(&descriptor));

      let instance = Instance::new(descriptor, preferred_semver_range);
      let idx = InstanceIdx(instances.len());
      instances.push(instance);

      if let Some(group) = version_group {
        group.add_instance(idx, &instances[idx.0]);
      }
    });

    Ok(Self {
      catalogs,
      config,
      instances,
      packages,
      version_groups,
    })
  }

  pub fn from_cli(cli: Cli) -> Result<Self, SyncpackError> {
    let config = Config::from_cli(cli)?;

    debug!("Command: {:?}", config.cli.subcommand);
    debug!("{:#?}", config.cli);
    debug!("{:#?}", config.rcfile);

    let packages = Packages::from_config(&config);
    let catalogs: Option<CatalogsByName> = None; // catalogs::from_config(&config);

    Context::create(config, packages, catalogs)
  }
}
