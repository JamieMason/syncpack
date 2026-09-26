use {
  crate::{
    cli::Cli,
    discovery::{
      groups::{build_semver_groups, build_update_groups, build_version_groups, validate_post_discovery},
      link::compute_link_resolution,
    },
    rcfile_loader::RcfileError,
  },
  std::{collections::BTreeMap, mem},
  syncpack_config::Rcfile,
  syncpack_disk::Disk,
  syncpack_model::{
    dependency::{Dependency, DependencyIdx, add_instance_to_dep_map},
    group_selector::GroupSelector,
    instance::{Instance, InstanceDescriptor, InstanceIdx},
    sources::Sources,
  },
  syncpack_rules::{
    inspection::Inspection,
    version_group::{VersionGroup, VersionGroupBehavior},
  },
  syncpack_vocab::{
    config_error::{UnsupportedConfigError, UnsupportedConfigErrors},
    dependency_type::DependencyType,
    registry_updates::RegistryUpdates,
  },
  thiserror::Error,
};

#[cfg(test)]
#[path = "../discovery_test.rs"]
mod discovery_test;

mod groups;
mod instances;
mod link;

pub use crate::discovery::instances::{iter_instances, sources_from_disk};

#[derive(Debug, Error)]
pub enum DiscoveryError {
  #[error(transparent)]
  RcfileError(RcfileError),
}

#[derive(Debug)]
pub struct Config {
  pub cli: Cli,
  pub rcfile: Rcfile,
}

/// The project after discovery: config, package files, every instance and
/// dependency, and the version groups they belong to. Immutable for the
/// purposes of inspection — `inspect` reads it and returns an `Inspection`
/// value; only `apply` (in the bin) mutates `disk`.
#[derive(Debug)]
pub struct Discovered {
  pub config: Config,
  /// Mutation goes through `disk.package_json_files[idx]` or
  /// `disk.pnpm_workspace` at apply time only.
  pub disk: Disk,
  pub instances: Vec<Instance>,
  /// The global dependency arena; version groups hold `Vec<DependencyIdx>`
  /// into this.
  pub dependencies: Vec<Dependency>,
  /// Sole owner of every `Source` reference. Pnpm yaml is a unit-variant
  /// slot; the actual file lives on `disk.pnpm_workspace`.
  pub sources: Sources,
  pub version_groups: Vec<VersionGroup>,
}

/// The project after inspection: the discovered data, the `Inspection` value
/// carrying every per-entity result, and any registry payload. Binding these
/// together stops a caller reading results early or pairing the wrong
/// `Inspection` with the wrong data.
#[derive(Debug)]
pub struct Inspected {
  pub data: Discovered,
  pub inspection: Inspection,
  pub updates: Option<RegistryUpdates>,
}

impl Discovered {
  /// Read all configuration and package.json files, collect all dependency
  /// instances, group them into version groups, and precompute link
  /// resolution. Reports every config error at once (B4).
  pub fn create(mut config: Config, disk: Disk, sources: Sources, dep_types: Vec<DependencyType>) -> Result<Self, DiscoveryError> {
    // Append catalog dep types BEFORE validation or version-group construction
    // — user-referenced catalog dep types (e.g. `pnpmCatalog:react18`) must
    // resolve here.
    config.rcfile.all_dependency_types.extend(dep_types);

    // Design C (§5.6): compile the raw group configs into runtime structures at
    // discovery time. Selectors are `model::GroupSelector`; construction reads
    // config — so the config crate itself never names a rules/model group type.
    let dependency_groups: Vec<GroupSelector> = mem::take(&mut config.rcfile.dependency_groups)
      .into_iter()
      .map(|dg| GroupSelector::new(dg.dependencies, dg.dependency_types, dg.alias_name, dg.packages, dg.specifier_types))
      .collect();
    let (semver_groups, semver_group_errors) = build_semver_groups(mem::take(&mut config.rcfile.semver_groups));
    let (update_groups, update_group_errors) = build_update_groups(mem::take(&mut config.rcfile.update_groups));

    let mut errors: Vec<UnsupportedConfigError> =
      validate_post_discovery(&config.rcfile, &dependency_groups, &semver_groups, &update_groups);
    errors.extend(semver_group_errors);
    errors.extend(update_group_errors);

    let (mut version_groups, version_group_errors) = build_version_groups(&mut config.rcfile, &sources);
    errors.extend(version_group_errors);

    let all_dependency_types = config.rcfile.all_dependency_types.clone();
    if let Some(ref filters) = config.cli.filters
      && let Err(error) = filters.validate_dependency_types(&all_dependency_types)
    {
      errors.push(error);
    }
    if !errors.is_empty() {
      return Err(DiscoveryError::RcfileError(RcfileError::UnsupportedConfig(
        UnsupportedConfigErrors(errors),
      )));
    }

    // Single uniform iteration. Each dep type's `source` field gates which
    // source kinds it's invoked against. Descriptors are owned and consumed
    // directly; package_name is looked up at each use site.
    let descriptors: Vec<InstanceDescriptor> = iter_instances(&sources, &disk, &all_dependency_types);

    let mut instances: Vec<Instance> = Vec::with_capacity(descriptors.len());
    let mut group_dep_maps: Vec<BTreeMap<String, Dependency>> = version_groups.iter().map(|_| BTreeMap::new()).collect();
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

      let group_index = version_groups
        .iter()
        .position(|group| group.selector().can_add(&descriptor, package_name));

      let instance = Instance::new(descriptor, package_name, preferred_semver_range, preferred_update_policy);
      let idx = InstanceIdx(instances.len());
      instances.push(instance);

      if let Some(group_index) = group_index {
        add_instance_to_dep_map(&mut group_dep_maps[group_index], idx, &instances[idx.0]);
      }
    }

    // Flatten the per-group dependency maps into one global arena, preserving
    // the BTreeMap's sorted-by-internal-name order within each group, and give
    // each group the indices of the dependencies it claims.
    let mut dependencies: Vec<Dependency> = Vec::new();
    for (group_index, dep_map) in group_dep_maps.into_iter().enumerate() {
      let mut dep_indices: Vec<DependencyIdx> = Vec::with_capacity(dep_map.len());
      for (_internal_name, dep) in dep_map {
        dep_indices.push(DependencyIdx(dependencies.len()));
        dependencies.push(dep);
      }
      version_groups[group_index].set_dependency_indices(dep_indices);
    }

    for (idx, resolves) in compute_link_resolution(&dependencies, &instances, &sources, &disk) {
      instances[idx.0].link_resolves_to_local = resolves;
    }

    Ok(Self {
      config,
      disk,
      instances,
      dependencies,
      sources,
      version_groups,
    })
  }

  /// Detected package manager (lives on Disk; accessor preserves call-site
  /// ergonomics).
  pub fn package_manager(&self) -> Option<syncpack_vocab::package_manager::PackageManager> {
    self.disk.package_manager
  }
}
