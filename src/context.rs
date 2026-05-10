use {
  crate::{
    cli::Cli,
    dependency::DependencyType,
    disk::{Disk, PackageManager},
    errors::{UnsupportedConfigError, UnsupportedConfigErrors},
    instance::{Instance, InstanceDescriptor, InstanceIdx},
    rcfile::{Rcfile, from_disk::RcfileError, validate_raw_dep_types},
    sources::Sources,
    version_group::{VersionGroup, VersionGroupBehavior},
  },
  std::mem,
  thiserror::Error,
};

#[cfg(test)]
#[path = "context_test.rs"]
mod context_test;

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
  pub config: Config,
  /// Mutation goes through `ctx.disk.package_json_files[idx]` or
  /// `ctx.disk.pnpm_workspace`.
  pub disk: Disk,
  pub instances: Vec<Instance>,
  /// Sole owner of every `Source` reference. Pnpm yaml is a unit-variant
  /// slot; the actual file lives on `disk.pnpm_workspace`.
  pub sources: Sources,
  pub version_groups: Vec<VersionGroup>,
}

impl Context {
  /// Read all configuration and package.json files, collect all dependency
  /// instances, and assign them to version groups.
  pub fn create(mut config: Config, disk: Disk, sources: Sources, dep_types: Vec<DependencyType>) -> Result<Self, ContextError> {
    // Append catalog dep types BEFORE validation or version-group construction
    // — user-referenced catalog dep types (e.g. `pnpmCatalog:react18`) must
    // resolve here.
    config.rcfile.all_dependency_types.extend(dep_types);

    // Validate every dep-type-filter selector against the post-discovery list.
    validate_post_discovery(&config.rcfile)
      .map_err(|err| vec![err])
      .map_err(UnsupportedConfigErrors)
      .map_err(RcfileError::UnsupportedConfig)
      .map_err(ContextError::RcfileError)?;

    // Auto-injects `CatalogDefs` when catalog dep types exist.
    let dependency_groups = mem::take(&mut config.rcfile.dependency_groups);
    let semver_groups = mem::take(&mut config.rcfile.semver_groups);
    let update_groups = mem::take(&mut config.rcfile.update_groups);
    let mut version_groups = config
      .rcfile
      .get_version_groups(&sources) // @TODO: Return every error
      .map_err(|err| vec![err])
      .map_err(UnsupportedConfigErrors)
      .map_err(RcfileError::UnsupportedConfig)
      .map_err(ContextError::RcfileError)?;
    let all_dependency_types = config.rcfile.all_dependency_types.clone();
    if let Some(ref filters) = config.cli.filters {
      filters
        .validate_dependency_types(&all_dependency_types) // @TODO: Return every error
        .map_err(|err| vec![err])
        .map_err(UnsupportedConfigErrors)
        .map_err(RcfileError::UnsupportedConfig)
        .map_err(ContextError::RcfileError)?;
    }

    // Single uniform iteration. Each dep type's `source` field gates which
    // source kinds it's invoked against. Catalog vs non-catalog branching
    // reads `dependency_type.is_catalog_definition` at use sites. Descriptors
    // are owned and consumed directly; package_name is looked up at each use
    // site via `sources.all[idx].name()` (no String clone per descriptor).
    let descriptors: Vec<InstanceDescriptor> = sources.iter_instances(&disk, &all_dependency_types).collect();

    let mut instances: Vec<Instance> = Vec::with_capacity(descriptors.len());
    for mut descriptor in descriptors {
      let package_name = sources.all[descriptor.source_idx.0].name();
      let dependency_group = dependency_groups.iter().find(|alias| alias.can_add(&descriptor, package_name));

      if let Some(group) = dependency_group {
        descriptor.internal_name = group.label.clone();
      }

      let matches = config.cli.filters.as_ref().is_none_or(|f| f.can_add(&descriptor, package_name));
      if !matches {
        continue;
      }

      let preferred_semver_range = semver_groups
        .iter()
        .find(|group| group.selector.can_add(&descriptor, package_name))
        .and_then(|group| group.range.clone());

      let preferred_update_policy = update_groups
        .iter()
        .find(|group| group.selector.can_add(&descriptor, package_name))
        .map(|group| group.policy.clone());

      let version_group = version_groups
        .iter_mut()
        .find(|group| group.selector().can_add(&descriptor, package_name));

      let instance = Instance::new(descriptor, package_name, preferred_semver_range, preferred_update_policy);
      let idx = InstanceIdx(instances.len());
      instances.push(instance);

      if let Some(group) = version_group {
        group.add_instance(idx, &instances[idx.0]);
      }
    }

    Ok(Self {
      config,
      disk,
      instances,
      sources,
      version_groups,
    })
  }

  /// Detected package manager (lives on Disk; accessor preserves call-site
  /// ergonomics through the field migration).
  pub fn package_manager(&self) -> Option<PackageManager> {
    self.disk.package_manager
  }

  /// All catalog-def instances for this internal dep name (one per catalog the
  /// dep appears in across pnpm and Bun). Keys by `descriptor.internal_name`
  /// so callers inside a `DependencyCore` visit can pass `dep.internal_name`
  /// directly — this works under `dependency_groups` aliasing because the
  /// def and its consumers share the same alias label.
  pub fn catalog_defs_for<'a>(&'a self, internal_name: &'a str) -> impl Iterator<Item = &'a Instance> + 'a {
    self
      .instances
      .iter()
      .filter(move |i| i.is_catalog_instance() && i.descriptor.internal_name == internal_name)
  }

  /// Distinct catalog names across the whole project (`"default"` + named).
  pub fn distinct_catalog_names(&self) -> Vec<&str> {
    let mut names: Vec<&str> = self.instances.iter().filter_map(|i| i.catalog_name()).collect();
    names.sort_unstable();
    names.dedup();
    names
  }
}

/// Validate every dep-type-filter selector (`dependency_groups`,
/// `semver_groups`, raw `version_groups`) against the post-discovery list of
/// dependency types.
fn validate_post_discovery(rcfile: &Rcfile) -> Result<(), UnsupportedConfigError> {
  for selector in &rcfile.dependency_groups {
    selector.validate_dependency_types(&rcfile.all_dependency_types)?;
  }
  for group in &rcfile.semver_groups {
    group.selector.validate_dependency_types(&rcfile.all_dependency_types)?;
  }
  for group in &rcfile.update_groups {
    group.selector.validate_dependency_types(&rcfile.all_dependency_types)?;
  }
  for group in &rcfile.version_groups {
    validate_raw_dep_types(&group.dependency_types, &rcfile.all_dependency_types)?;
  }
  Ok(())
}
