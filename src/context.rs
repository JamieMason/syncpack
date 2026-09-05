use {
  crate::{
    cli::Cli,
    inspection::Inspection,
    rcfile_loader::RcfileError,
    version_group::{
      BannedGroup, CatalogDefsGroup, CatalogGroup, IgnoredGroup, PinnedGroup, PreferredSemverGroup, SameMinorGroup, SameRangeGroup,
      SemverRangeOnlyGroup, SnappedToGroup, VersionGroup, VersionGroupBehavior,
    },
  },
  log::warn,
  serde_json::Value,
  std::{
    collections::{BTreeMap, HashSet},
    mem,
    path::{Path, PathBuf},
    rc::Rc,
  },
  syncpack_config::{AnyVersionGroup, Rcfile, semver_group::AnySemverGroup, update_group::AnyUpdateGroup, validate_raw_dep_types},
  syncpack_disk::{Disk, package_name},
  syncpack_io::json_view,
  syncpack_model::{
    dependency::{Dependency, DependencyIdx, add_instance_to_dep_map},
    group_selector::GroupSelector,
    instance::{Instance, InstanceDescriptor, InstanceIdx},
    source::Source,
    sources::{SourceIdx, Sources},
  },
  syncpack_specifier::{Specifier, semver_range::SemverRange, update_target::UpdateTarget},
  syncpack_vocab::{
    config_error::{UnsupportedConfigError, UnsupportedConfigErrors},
    dependency_type::{DependencyType, Strategy},
    prefer_version::PreferVersion,
    registry_updates::RegistryUpdates,
    severity::SeverityMap,
    source_kind::SourceKind,
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

/// Build the sources arena by mirroring `disk.package_json_files` 1:1. Each
/// file becomes one `Source::Package { file_idx, name }`. `user_source_indices`
/// is computed by matching each file's `filepath` against the user-pattern-
/// filtered `file_paths`. Appends `Source::PnpmYaml` at the tail when
/// `disk.pnpm_workspace` is `Some`. Discovery step — reads Disk, so it lives in
/// the bin.
pub fn sources_from_disk(disk: &Disk, file_paths: &[PathBuf]) -> Sources {
  let mut all: Vec<Source> = Vec::with_capacity(disk.package_json_files.len() + 1);
  let mut user_source_indices: Vec<usize> = Vec::new();
  for (file_idx, file) in disk.package_json_files.iter().enumerate() {
    let name = package_name(file).to_string();
    all.push(Source::Package { file_idx, name });
    if file_paths.iter().any(|p| p == &file.filepath) {
      user_source_indices.push(file_idx);
    }
  }
  let mut pnpm_yaml_source_idx = None;
  if disk.pnpm_workspace.is_some() {
    pnpm_yaml_source_idx = Some(all.len());
    all.push(Source::PnpmYaml);
  }
  Sources {
    all,
    user_source_indices,
    pnpm_yaml_source_idx,
  }
}

/// Two-pass instance discovery over the sources arena. Catalog dep types iterate
/// the canonical source regardless of user pattern membership; non-catalog dep
/// types iterate only `user_source_indices` (yaml is always included for
/// PnpmWorkspace dep types when yaml exists).
///
/// Pass 1 (catalog dep types): walk `dep_types` where `is_catalog_definition`.
/// PackageJson catalogs read from `disk.package_json_root_idx`'s slot;
/// PnpmWorkspace catalogs read from the pnpm yaml.
///
/// Pass 2 (non-catalog dep types): walk `user_source_indices` for PackageJson
/// dep types; walk yaml for PnpmWorkspace dep types.
///
/// Discovery step — reads Disk, so it lives in the bin.
pub fn iter_instances(sources: &Sources, disk: &Disk, dep_types: &[DependencyType]) -> Vec<InstanceDescriptor> {
  // Local package names are derived from package.json sources only — the
  // yaml's synthetic "name" must not poison `is_local_dependency`.
  let local_package_names: HashSet<String> = sources
    .all
    .iter()
    .filter_map(|s| match s {
      Source::Package { name, .. } => Some(name.clone()),
      Source::PnpmYaml => None,
    })
    .collect();

  // ONE yaml→json conversion per call, owned, borrowed by both passes.
  let yaml_json: Option<Value> = disk.pnpm_workspace.as_ref().map(json_view);

  // Wrap each dep type in an Rc once. Descriptors built within this call
  // share the Rc allocation; the rcfile's `Vec<DependencyType>` is never
  // converted in place.
  let dep_types_rc: Vec<Rc<DependencyType>> = dep_types.iter().map(|dt| Rc::new(dt.clone())).collect();

  let mut out: Vec<InstanceDescriptor> = Vec::new();

  // Pass 1: catalog dep types iterate the canonical source.
  for dep_type_rc in dep_types_rc.iter().filter(|d| d.is_catalog_definition) {
    match dep_type_rc.source {
      SourceKind::PackageJson => {
        // Bun catalogs always live in the root pkg.json.
        if let Some(root_idx) = disk.package_json_root_idx
          && let Some(source_idx) = sources
            .all
            .iter()
            .position(|s| matches!(s, Source::Package { file_idx, .. } if *file_idx == root_idx))
        {
          let root_file = &disk.package_json_files[root_idx];
          collect_descriptors_for_dep_type(
            dep_type_rc,
            &root_file.contents,
            SourceIdx(source_idx),
            &local_package_names,
            &mut out,
          );
        }
      }
      SourceKind::PnpmWorkspace => {
        if let (Some(yaml), Some(yaml_idx)) = (yaml_json.as_ref(), sources.pnpm_yaml_source_idx) {
          collect_descriptors_for_dep_type(dep_type_rc, yaml, SourceIdx(yaml_idx), &local_package_names, &mut out);
        }
      }
    }
  }

  // Pass 2: non-catalog dep types iterate user_source_indices (or yaml
  // when source is PnpmWorkspace).
  for dep_type_rc in dep_types_rc.iter().filter(|d| !d.is_catalog_definition) {
    match dep_type_rc.source {
      SourceKind::PackageJson => {
        for (source_idx, source) in sources.all.iter().enumerate() {
          let Source::Package { file_idx, .. } = source else { continue };
          if !sources.user_source_indices.contains(file_idx) {
            continue;
          }
          let file = &disk.package_json_files[*file_idx];
          collect_descriptors_for_dep_type(dep_type_rc, &file.contents, SourceIdx(source_idx), &local_package_names, &mut out);
        }
      }
      SourceKind::PnpmWorkspace => {
        if let (Some(yaml), Some(yaml_idx)) = (yaml_json.as_ref(), sources.pnpm_yaml_source_idx) {
          collect_descriptors_for_dep_type(dep_type_rc, yaml, SourceIdx(yaml_idx), &local_package_names, &mut out);
        }
      }
    }
  }

  out
}

/// Build an `InstanceDescriptor`. Stores `source_idx` directly (no enum
/// tagging) and shares the dep type via `Rc`. Catalog vs non-catalog branching
/// is read off `dependency_type.is_catalog_definition` at downstream use sites
/// (`Instance::is_catalog_instance`, `catalog_name`).
fn build_descriptor(
  dependency_type: &Rc<DependencyType>,
  name: &str,
  raw_specifier: &str,
  source_idx: SourceIdx,
  local_package_names: &HashSet<String>,
) -> InstanceDescriptor {
  InstanceDescriptor {
    dependency_type: Rc::clone(dependency_type),
    internal_name: name.to_string(),
    is_local_dependency: local_package_names.contains(name),
    name: name.to_string(),
    source_idx,
    specifier: Specifier::new(raw_specifier),
  }
}

/// Drive a single dep type against a single source's contents and append every
/// emitted descriptor onto `out`. Reads `contents` via JSON pointers per the
/// dep type's strategy.
fn collect_descriptors_for_dep_type(
  dep_type: &Rc<DependencyType>,
  contents: &Value,
  source_idx: SourceIdx,
  local_package_names: &HashSet<String>,
  out: &mut Vec<InstanceDescriptor>,
) {
  match dep_type.strategy {
    Strategy::NameAndVersionProps => {
      let name_path = dep_type.name_path.as_ref().expect("NameAndVersionProps requires name_path");
      let name_val = contents.pointer(name_path);
      let version_val = contents.pointer(&dep_type.path);
      let resolved_specifier: Option<&str> = match version_val {
        Some(Value::String(s)) => Some(s.as_str()),
        None if dep_type.name == "local" => Some(""),
        _ => None,
      };
      if let (Some(Value::String(name)), Some(raw_specifier)) = (name_val, resolved_specifier) {
        out.push(build_descriptor(dep_type, name, raw_specifier, source_idx, local_package_names));
      }
    }
    Strategy::NamedVersionString => {
      if let Some(Value::String(specifier)) = contents.pointer(&dep_type.path)
        && let Some((name, raw_specifier)) = specifier.split_once('@')
      {
        out.push(build_descriptor(dep_type, name, raw_specifier, source_idx, local_package_names));
      }
    }
    Strategy::UnnamedVersionString => {
      if let Some(Value::String(raw_specifier)) = contents.pointer(&dep_type.path) {
        out.push(build_descriptor(
          dep_type,
          &dep_type.name,
          raw_specifier,
          source_idx,
          local_package_names,
        ));
      }
    }
    Strategy::VersionsByName => {
      if let Some(Value::Object(versions_by_name)) = contents.pointer(&dep_type.path) {
        for (name, raw_specifier) in versions_by_name {
          if let Value::String(raw_specifier) = raw_specifier {
            out.push(build_descriptor(dep_type, name, raw_specifier, source_idx, local_package_names));
          }
        }
      }
    }
    Strategy::InvalidConfig => {
      unreachable!("unrecognised strategy");
    }
  }
}
