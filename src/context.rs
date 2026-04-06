use {
  crate::{
    catalogs::CatalogsByName,
    cli::Cli,
    errors::UnsupportedConfigErrors,
    instance::{Instance, InstanceIdx},
    packages::Packages,
    rcfile::{from_disk::RcfileError, Rcfile},
    version_group::{VersionGroup, VersionGroupBehavior},
  },
  std::mem,
  thiserror::Error,
};

#[derive(Debug, Error)]
pub enum ContextError {
  #[error(transparent)]
  RcfileError(RcfileError),
}

#[derive(Debug)]
pub struct Config {
  pub cli: Cli,
  pub rcfile: Rcfile,
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
  pub fn create(mut config: Config, packages: Packages, catalogs: Option<CatalogsByName>) -> Result<Self, ContextError> {
    let mut instances = vec![];
    let dependency_groups = mem::take(&mut config.rcfile.dependency_groups);
    let semver_groups = mem::take(&mut config.rcfile.semver_groups);
    let mut version_groups = config
      .rcfile
      .get_version_groups(&packages) // @TODO: Return every error
      .map_err(|err| vec![err])
      .map_err(UnsupportedConfigErrors)
      .map_err(RcfileError::UnsupportedConfig)
      .map_err(ContextError::RcfileError)?;
    let all_dependency_types = &config.rcfile.all_dependency_types;
    if let Some(ref filters) = config.cli.filters {
      filters
        .validate_dependency_types(all_dependency_types) // @TODO: Return every error
        .map_err(|err| vec![err])
        .map_err(UnsupportedConfigErrors)
        .map_err(RcfileError::UnsupportedConfig)
        .map_err(ContextError::RcfileError)?;
    }

    packages.get_all_instances(all_dependency_types, |mut descriptor| {
      let package_name = &packages.all[descriptor.package_idx.0].name;
      let dependency_group = dependency_groups.iter().find(|alias| alias.can_add(&descriptor, package_name));

      if let Some(group) = dependency_group {
        descriptor.internal_name = group.label.clone();
      }

      descriptor.matches_cli_filter = config.cli.filters.as_ref().is_none_or(|f| f.can_add(&descriptor, package_name));

      if !descriptor.matches_cli_filter {
        return;
      }

      let preferred_semver_range = semver_groups
        .iter()
        .find(|group| group.selector.can_add(&descriptor, package_name))
        .and_then(|group| group.range.clone());

      let version_group = version_groups.iter_mut().find(|group| group.selector().can_add(&descriptor, package_name));

      let instance = Instance::new(descriptor, package_name, preferred_semver_range);
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
}
