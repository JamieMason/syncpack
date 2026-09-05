use {
  crate::{
    cli::Cli,
    disk::Disk,
    inspection::Inspection,
    instance::{Instance, InstanceDescriptor, InstanceIdx},
    rcfile::{Rcfile, from_disk::RcfileError, validate_raw_dep_types},
    source::Source,
    sources::Sources,
    specifier::Specifier,
    version_group::{Dependency, DependencyIdx, VersionGroup, VersionGroupBehavior, add_instance_to_dep_map},
  },
  std::{
    collections::BTreeMap,
    mem,
    path::{Path, PathBuf},
  },
  syncpack_vocab::{
    config_error::{UnsupportedConfigError, UnsupportedConfigErrors},
    dependency_type::DependencyType,
    registry_updates::RegistryUpdates,
  },
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
  pub fn create(mut config: Config, disk: Disk, sources: Sources, dep_types: Vec<DependencyType>) -> Result<Self, ContextError> {
    // Append catalog dep types BEFORE validation or version-group construction
    // — user-referenced catalog dep types (e.g. `pnpmCatalog:react18`) must
    // resolve here.
    config.rcfile.all_dependency_types.extend(dep_types);

    let mut errors: Vec<UnsupportedConfigError> = validate_post_discovery(&config.rcfile);

    let dependency_groups = mem::take(&mut config.rcfile.dependency_groups);
    let semver_groups = mem::take(&mut config.rcfile.semver_groups);
    let update_groups = mem::take(&mut config.rcfile.update_groups);
    let (mut version_groups, version_group_errors) = config.rcfile.get_version_groups(&sources);
    errors.extend(version_group_errors);
    let all_dependency_types = config.rcfile.all_dependency_types.clone();
    if let Some(ref filters) = config.cli.filters
      && let Err(error) = filters.validate_dependency_types(&all_dependency_types)
    {
      errors.push(error);
    }
    if !errors.is_empty() {
      return Err(ContextError::RcfileError(RcfileError::UnsupportedConfig(UnsupportedConfigErrors(
        errors,
      ))));
    }

    // Single uniform iteration. Each dep type's `source` field gates which
    // source kinds it's invoked against. Descriptors are owned and consumed
    // directly; package_name is looked up at each use site.
    let descriptors: Vec<InstanceDescriptor> = sources.iter_instances(&disk, &all_dependency_types).collect();

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

/// Resolve, at discovery time, whether each `link:` instance of a
/// locally-developed dependency resolves to that package's directory, so
/// inspection needs no disk I/O.
fn compute_link_resolution(
  dependencies: &[Dependency],
  instances: &[Instance],
  sources: &Sources,
  disk: &Disk,
) -> Vec<(InstanceIdx, bool)> {
  let mut resolutions = Vec::new();
  for dep in dependencies {
    if let Some(local_idx) = dep.local_instance {
      let local_instance = &instances[local_idx.0];
      for &idx in &dep.instances {
        let instance = &instances[idx.0];
        if instance.descriptor.specifier.is_link() {
          resolutions.push((idx, link_resolves_to_local(instance, local_instance, &sources.all, disk)));
        }
      }
    }
  }
  resolutions
}

/// Whether a `link:` instance of a locally-developed dependency resolves to
/// that package's directory. Discovery-time filesystem read, so inspection
/// stays pure. `false` for non-link instances and pnpm-catalog sources (which
/// do not live in a package.json).
fn link_resolves_to_local(instance: &Instance, local_instance: &Instance, sources: &[Source], disk: &Disk) -> bool {
  let Specifier::Link(link) = &*instance.descriptor.specifier else {
    return false;
  };
  let Source::Package {
    file_idx: consuming_idx, ..
  } = &sources[instance.source_idx().0]
  else {
    return false;
  };
  let Source::Package { file_idx: local_idx, .. } = &sources[local_instance.source_idx().0] else {
    return false;
  };
  let consuming_package_path = &disk.package_json_files[*consuming_idx].filepath;
  let consuming_package_dir = consuming_package_path.parent().unwrap_or_else(|| Path::new(""));
  let link_path = link.raw.strip_prefix("link:").unwrap_or(&link.raw);
  let resolved_link_path = consuming_package_dir.join(link_path);
  let local_package_path = &disk.package_json_files[*local_idx].filepath;
  let local_package_dir = local_package_path.parent().unwrap_or_else(|| Path::new(""));

  if let (Ok(resolved_canonical), Ok(local_canonical)) = (resolved_link_path.canonicalize(), local_package_dir.canonicalize()) {
    resolved_canonical == local_canonical
  } else {
    normalize_path(&resolved_link_path) == normalize_path(local_package_dir)
  }
}

/// Normalize a path by resolving `.` and `..` components without touching the
/// filesystem.
fn normalize_path(path: &Path) -> PathBuf {
  let mut components = Vec::new();
  for component in path.components() {
    match component {
      std::path::Component::ParentDir => {
        components.pop();
      }
      std::path::Component::CurDir => {}
      _ => components.push(component),
    }
  }
  components.iter().collect()
}

/// Validate every dep-type-filter selector against the post-discovery list of
/// dependency types, collecting every error rather than failing on the first.
fn validate_post_discovery(rcfile: &Rcfile) -> Vec<UnsupportedConfigError> {
  let mut errors = Vec::new();
  for selector in &rcfile.dependency_groups {
    if let Err(error) = selector.validate_dependency_types(&rcfile.all_dependency_types) {
      errors.push(error);
    }
  }
  for group in &rcfile.semver_groups {
    if let Err(error) = group.selector.validate_dependency_types(&rcfile.all_dependency_types) {
      errors.push(error);
    }
  }
  for group in &rcfile.update_groups {
    if let Err(error) = group.selector.validate_dependency_types(&rcfile.all_dependency_types) {
      errors.push(error);
    }
  }
  for group in &rcfile.version_groups {
    if let Err(error) = validate_raw_dep_types(&group.dependency_types, &rcfile.all_dependency_types) {
      errors.push(error);
    }
  }
  errors
}
