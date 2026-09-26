use {
  log::warn,
  std::mem,
  syncpack_config::{AnyVersionGroup, Rcfile, semver_group::AnySemverGroup, update_group::AnyUpdateGroup, validate_raw_dep_types},
  syncpack_model::{group_selector::GroupSelector, sources::Sources},
  syncpack_rules::version_group::{
    BannedGroup, CatalogDefsGroup, CatalogGroup, IgnoredGroup, PinnedGroup, PreferredSemverGroup, SameMinorGroup, SameRangeGroup,
    SemverRangeOnlyGroup, SnappedToGroup, VersionGroup,
  },
  syncpack_specifier::{Specifier, semver_range::SemverRange, update_target::UpdateTarget},
  syncpack_vocab::{
    config_error::UnsupportedConfigError, prefer_version::PreferVersion, severity::SeverityMap, update_policy::UpdatePolicy,
  },
};

/// Validate every dep-type-filter selector against the post-discovery list of
/// dependency types, collecting every error rather than failing on the first.
/// The selectors are compiled at discovery time (Design C), so they are passed
/// in rather than read off `Rcfile`.
pub(super) fn validate_post_discovery(
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
pub(super) struct SemverGroup {
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
  fn from_config(group: AnySemverGroup) -> Result<SemverGroup, UnsupportedConfigError> {
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
pub(super) struct UpdateGroup {
  pub selector: GroupSelector,
  pub policy: UpdatePolicy,
}

impl UpdateGroup {
  /// Build one update group from a raw rcfile config item.
  fn from_config(group: AnyUpdateGroup) -> Result<UpdateGroup, UnsupportedConfigError> {
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
pub(super) fn build_semver_groups(raw: Vec<AnySemverGroup>) -> (Vec<SemverGroup>, Vec<UnsupportedConfigError>) {
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
pub(super) fn build_update_groups(raw: Vec<AnyUpdateGroup>) -> (Vec<UpdateGroup>, Vec<UnsupportedConfigError>) {
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
pub(super) fn build_version_groups(rcfile: &mut Rcfile, sources: &Sources) -> (Vec<VersionGroup>, Vec<UnsupportedConfigError>) {
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
fn version_group_from_config(group: AnyVersionGroup, index: usize, sources: &Sources) -> Result<VersionGroup, UnsupportedConfigError> {
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
