use {
  crate::{
    cli::Cli,
    disk::Disk,
    group_selector::GroupSelector,
    inspection::Inspection,
    instance::{Instance, InstanceDescriptor, InstanceIdx},
    rcfile::{Rcfile, from_disk::RcfileError, semver_group::AnySemverGroup, update_group::AnyUpdateGroup, validate_raw_dep_types},
    semver_range::SemverRange,
    source::Source,
    sources::Sources,
    specifier::Specifier,
    version_group::{
      AnyVersionGroup, BannedGroup, CatalogDefsGroup, CatalogGroup, Dependency, DependencyIdx, IgnoredGroup, PinnedGroup,
      PreferredSemverGroup, SameMinorGroup, SameRangeGroup, SemverRangeOnlyGroup, SnappedToGroup, VersionGroup, VersionGroupBehavior,
      add_instance_to_dep_map,
    },
  },
  log::warn,
  std::{
    collections::BTreeMap,
    mem,
    path::{Path, PathBuf},
  },
  syncpack_specifier::update_target::UpdateTarget,
  syncpack_vocab::{
    config_error::{UnsupportedConfigError, UnsupportedConfigErrors},
    dependency_type::DependencyType,
    prefer_version::PreferVersion,
    registry_updates::RegistryUpdates,
    severity::SeverityMap,
    update_policy::UpdatePolicy,
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
/// The selectors are compiled at discovery time (Design C), so they are passed
/// in rather than read off `Rcfile`.
fn validate_post_discovery(
  rcfile: &Rcfile,
  dependency_groups: &[GroupSelector],
  semver_groups: &[SemverGroup],
  update_groups: &[UpdateGroup],
) -> Vec<UnsupportedConfigError> {
  let mut errors = Vec::new();
  for selector in dependency_groups {
    if let Err(error) = selector.validate_dependency_types(&rcfile.all_dependency_types) {
      errors.push(error);
    }
  }
  for group in semver_groups {
    if let Err(error) = group.selector.validate_dependency_types(&rcfile.all_dependency_types) {
      errors.push(error);
    }
  }
  for group in update_groups {
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

// Group construction (Design C, §5.6). Semver / update / version groups are
// built here at discovery time from the raw rcfile config: the compiled
// `GroupSelector` (model) and version-group variants (rules) are constructed by
// the bin, so `config` depends on neither.

/// A semver group after selector compilation: which instances it claims and the
/// range they should use. Discovery-transient.
#[derive(Debug)]
pub struct SemverGroup {
  pub selector: GroupSelector,
  pub range: Option<SemverRange>,
}

impl SemverGroup {
  /// Built-in group ensuring local packages are an exact version.
  fn get_exact_local_specifiers() -> SemverGroup {
    SemverGroup {
      selector: GroupSelector::new(
        vec![],
        vec!["local".to_string()],
        "Local package versions must be exact".to_string(),
        vec![],
        vec![],
      ),
      range: None,
    }
  }

  /// Built-in catch-all applying to any instance.
  fn get_catch_all() -> SemverGroup {
    SemverGroup {
      selector: GroupSelector::new(vec![], vec![], "Default Semver Group".to_string(), vec![], vec![]),
      range: None,
    }
  }

  /// Build one semver group from a raw rcfile config item.
  pub fn from_config(group: AnySemverGroup) -> Result<SemverGroup, UnsupportedConfigError> {
    let selector = GroupSelector::new(
      group.dependencies,
      group.dependency_types,
      group.label,
      group.packages,
      group.specifier_types,
    );
    if let Some(true) = group.is_disabled {
      Ok(SemverGroup { selector, range: None })
    } else if let Some(true) = group.is_ignored {
      Ok(SemverGroup { selector, range: None })
    } else if let Some(range) = &group.range {
      Ok(SemverGroup {
        selector,
        range: SemverRange::new(range),
      })
    } else {
      Err(UnsupportedConfigError::InvalidSemverGroup)
    }
  }
}

/// An update group after selector compilation. Discovery-transient.
#[derive(Debug)]
pub struct UpdateGroup {
  pub selector: GroupSelector,
  pub policy: UpdatePolicy,
}

impl UpdateGroup {
  /// Build one update group from a raw rcfile config item.
  pub fn from_config(group: AnyUpdateGroup) -> Result<UpdateGroup, UnsupportedConfigError> {
    let selector = GroupSelector::new(
      group.dependencies,
      group.dependency_types,
      group.label,
      group.packages,
      group.specifier_types,
    );
    let policy = match (group.is_ignored, group.target.as_deref()) {
      (Some(true), None) => UpdatePolicy::Skip,
      (None | Some(false), Some("patch")) => UpdatePolicy::UpTo(UpdateTarget::Patch),
      (None | Some(false), Some("minor")) => UpdatePolicy::UpTo(UpdateTarget::Minor),
      (None | Some(false), Some("latest")) => UpdatePolicy::UpTo(UpdateTarget::Latest),
      (None | Some(false), Some(_)) => return Err(UnsupportedConfigError::InvalidUpdateGroup),
      (Some(true), Some(_)) => return Err(UnsupportedConfigError::InvalidUpdateGroup),
      (None | Some(false), None) => return Err(UnsupportedConfigError::InvalidUpdateGroup),
    };
    Ok(UpdateGroup { selector, policy })
  }
}

/// Build the semver group list, preserving the order `[exact-local built-in,
/// ...user groups, catch-all built-in]`. User groups that fail validation are
/// skipped and their errors collected (B4).
fn build_semver_groups(raw: Vec<AnySemverGroup>) -> (Vec<SemverGroup>, Vec<UnsupportedConfigError>) {
  let mut groups = vec![SemverGroup::get_exact_local_specifiers()];
  let mut errors = Vec::new();
  for group_config in raw {
    match SemverGroup::from_config(group_config) {
      Ok(group) => groups.push(group),
      Err(error) => errors.push(error),
    }
  }
  groups.push(SemverGroup::get_catch_all());
  (groups, errors)
}

/// Build the update group list, collecting any per-group errors (B4).
fn build_update_groups(raw: Vec<AnyUpdateGroup>) -> (Vec<UpdateGroup>, Vec<UnsupportedConfigError>) {
  let mut groups = Vec::new();
  let mut errors = Vec::new();
  for group_config in raw {
    match UpdateGroup::from_config(group_config) {
      Ok(group) => groups.push(group),
      Err(error) => errors.push(error),
    }
  }
  (groups, errors)
}

/// Build every version group from the raw rcfile config, collecting *all* config
/// errors rather than failing on the first (B4). Auto-injects a `CatalogDefs`
/// catch-all immediately before the default `PreferredSemver` catch-all when at
/// least one catalog dep type exists.
fn build_version_groups(rcfile: &mut Rcfile, sources: &Sources) -> (Vec<VersionGroup>, Vec<UnsupportedConfigError>) {
  let mut all_groups: Vec<VersionGroup> = Vec::new();
  let mut errors: Vec<UnsupportedConfigError> = Vec::new();
  for (index, group_config) in mem::take(&mut rcfile.version_groups).into_iter().enumerate() {
    match version_group_from_config(group_config, index, sources) {
      Ok(group) => all_groups.push(group),
      Err(error) => errors.push(error),
    }
  }
  let catalog_dep_type_names = rcfile.catalog_dep_type_names();
  if !catalog_dep_type_names.is_empty() {
    all_groups.push(VersionGroup::CatalogDefs(CatalogDefsGroup {
      selector: GroupSelector::new(vec![], catalog_dep_type_names, "Catalog Definitions".into(), vec![], vec![]),
      dependencies: vec![],
      severity: SeverityMap::new(),
    }));
  }
  all_groups.push(VersionGroup::get_catch_all());
  (all_groups, errors)
}

/// Build one version group from a raw rcfile config item. Reads config
/// (`AnyVersionGroup`) and constructs a rules `VersionGroup`, so it lives in the
/// bin (neither `config` nor `rules` may name the other).
pub fn version_group_from_config(group: AnyVersionGroup, index: usize, sources: &Sources) -> Result<VersionGroup, UnsupportedConfigError> {
  let selector = GroupSelector::new(
    group.dependencies,
    group.dependency_types,
    group.label,
    group.packages,
    group.specifier_types,
  );

  if let Some(true) = group.is_banned {
    let severity = validate_severity(group.severity, "Banned", index, BANNED_KEYS)?;
    return Ok(VersionGroup::Banned(BannedGroup {
      selector,
      dependencies: vec![],
      severity,
    }));
  }
  if let Some(true) = group.is_ignored {
    // Ignored produces no statuses; severity is accepted and discarded.
    return Ok(VersionGroup::Ignored(IgnoredGroup {
      selector,
      dependencies: vec![],
      severity: SeverityMap::new(),
    }));
  }
  if let Some(pin_version) = &group.pin_version {
    let severity = validate_severity(group.severity, "Pinned", index, PINNED_KEYS)?;
    return Ok(VersionGroup::Pinned(PinnedGroup {
      selector,
      dependencies: vec![],
      pin_version: Specifier::new(pin_version),
      severity,
    }));
  }
  if let Some(policy) = &group.policy {
    if policy == "sameRange" {
      let severity = validate_severity(group.severity, "SameRange", index, SAME_RANGE_KEYS)?;
      return Ok(VersionGroup::SameRange(SameRangeGroup {
        selector,
        dependencies: vec![],
        severity,
      }));
    } else if policy == "sameMinor" {
      let severity = validate_severity(group.severity, "SameMinor", index, SAME_MINOR_KEYS)?;
      let prefer_version = group.prefer_version.as_ref().map(|pv| {
        if pv == "lowestSemver" {
          PreferVersion::LowestSemver
        } else {
          PreferVersion::HighestSemver
        }
      });
      return Ok(VersionGroup::SameMinor(SameMinorGroup {
        selector,
        dependencies: vec![],
        prefer_version,
        severity,
      }));
    } else if policy == "catalog" {
      let severity = validate_severity(group.severity, "Catalog", index, CATALOG_KEYS)?;
      return Ok(VersionGroup::Catalog(CatalogGroup {
        selector,
        dependencies: vec![],
        severity,
      }));
    } else if policy == "semverRangeOnly" {
      let severity = validate_severity(group.severity, "SemverRangeOnly", index, SEMVER_RANGE_ONLY_KEYS)?;
      return Ok(VersionGroup::SemverRangeOnly(SemverRangeOnlyGroup {
        selector,
        dependencies: vec![],
        severity,
      }));
    } else {
      return Err(UnsupportedConfigError::InvalidVersionGroupPolicy(policy.clone()));
    }
  }
  if let Some(snap_to) = &group.snap_to {
    let severity = validate_severity(group.severity, "SnappedTo", index, SNAPPED_TO_KEYS)?;
    return Ok(VersionGroup::SnappedTo(SnappedToGroup {
      selector,
      dependencies: vec![],
      snap_to: snap_to
        .iter()
        .flat_map(|name| {
          sources.find_package(name).or_else(|| {
            warn!("Invalid Snapped To Version Group: No package.json file found with a name property of '{name}'");
            None
          })
        })
        .collect(),
      severity,
    }));
  }
  if let Some(prefer_version) = &group.prefer_version {
    let prefer_highest = prefer_version != "lowestSemver";
    let group_type = if prefer_highest { "HighestSemver" } else { "LowestSemver" };
    let severity = validate_severity(group.severity, group_type, index, PREFERRED_SEMVER_KEYS)?;
    return Ok(VersionGroup::PreferredSemver(PreferredSemverGroup {
      selector,
      dependencies: vec![],
      prefer_highest,
      severity,
    }));
  }
  let severity = validate_severity(group.severity, "HighestSemver", index, PREFERRED_SEMVER_KEYS)?;
  Ok(VersionGroup::PreferredSemver(PreferredSemverGroup {
    selector,
    dependencies: vec![],
    prefer_highest: true,
    severity,
  }))
}

const BANNED_KEYS: &[&str] = &["IsBanned"];
const PINNED_KEYS: &[&str] = &[
  "DiffersToPin",
  "PinOverridesSemverRange",
  "PinOverridesSemverRangeMismatch",
  "RefuseToPinLocal",
];
const PREFERRED_SEMVER_KEYS: &[&str] = &[
  "SemverRangeMismatch",
  "DiffersToLocal",
  "DiffersToCatalog",
  "DiffersToHighestOrLowestSemver",
];
const SAME_RANGE_KEYS: &[&str] = &["SemverRangeMismatch"];
const SEMVER_RANGE_ONLY_KEYS: &[&str] = &["SemverRangeMismatch"];
const SAME_MINOR_KEYS: &[&str] = &[
  "DiffersToHighestOrLowestSemverMinor",
  "SemverRangeMismatch",
  "SameMinorOverridesSemverRange",
  "SameMinorOverridesSemverRangeMismatch",
];
const SNAPPED_TO_KEYS: &[&str] = &["DiffersToSnapTarget", "SemverRangeMismatch", "RefuseToSnapLocal"];
const CATALOG_KEYS: &[&str] = &["NotUsingCatalog", "MissingFromCatalog"];

fn validate_severity(
  severity: SeverityMap,
  group_type: &'static str,
  index: usize,
  permitted: &'static [&'static str],
) -> Result<SeverityMap, UnsupportedConfigError> {
  for key in severity.keys() {
    if !permitted.contains(&key.as_str()) {
      return Err(UnsupportedConfigError::InvalidSeverityKey {
        path: format!("versionGroups[{index}].severity.{key}"),
        group_type: group_type.to_string(),
        key: key.clone(),
        permitted: permitted.to_vec(),
      });
    }
  }
  Ok(severity)
}
