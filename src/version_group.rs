use {
  crate::{
    cli::SortBy,
    group_selector::GroupSelector,
    inspection::{Inspection, RuleInput},
    instance::{Instance, InstanceIdx, InstanceState},
    source::Source,
    sources::Sources,
    specifier::Specifier,
  },
  itertools::Itertools,
  log::warn,
  serde::Deserialize,
  serde_json::Value,
  std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    rc::Rc,
  },
  syncpack_specifier::update_target::UpdateTarget,
  syncpack_vocab::{
    config_error::UnsupportedConfigError,
    prefer_version::PreferVersion,
    registry_updates::RegistryUpdates,
    severity::{Severity, SeverityMap},
    update_url::UpdateUrl,
  },
};

mod banned;
#[cfg(test)]
#[path = "version_group/bun_catalog_test.rs"]
mod bun_catalog_test;
mod catalog;
mod catalog_defs;
#[cfg(test)]
#[path = "version_group/catalog_defs_test.rs"]
mod catalog_defs_test;
#[cfg(test)]
#[path = "version_group/catalog_test.rs"]
mod catalog_test;
mod ignored;
mod pinned;
mod preferred_semver;
mod same_minor;
mod same_range;
mod semver_range_only;
mod snapped_to;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyVersionGroup {
  #[serde(default)]
  pub dependencies: Vec<String>,
  #[serde(default)]
  pub dependency_types: Vec<String>,
  #[serde(default)]
  pub label: String,
  #[serde(default)]
  pub packages: Vec<String>,
  #[serde(default)]
  pub specifier_types: Vec<String>,
  //
  pub is_banned: Option<bool>,
  pub is_ignored: Option<bool>,
  pub pin_version: Option<String>,
  pub policy: Option<String>,
  pub snap_to: Option<Vec<String>>,
  pub prefer_version: Option<String>,
  #[serde(default)]
  pub severity: SeverityMap,
  #[serde(flatten)]
  pub unknown_fields: HashMap<String, Value>,
}

pub use {
  banned::BannedGroup, catalog::CatalogGroup, catalog_defs::CatalogDefsGroup, ignored::IgnoredGroup, pinned::PinnedGroup,
  preferred_semver::PreferredSemverGroup, same_minor::SameMinorGroup, same_range::SameRangeGroup, semver_range_only::SemverRangeOnlyGroup,
  snapped_to::SnappedToGroup,
};

pub(crate) const L1: &str = "  ";
pub(crate) const L2: &str = "    ";
pub(crate) const L3: &str = "      ";
pub(crate) const L4: &str = "        ";
pub(crate) const L5: &str = "          ";
pub(crate) const L6: &str = "            ";
pub(crate) const L7: &str = "              ";
pub(crate) const L8: &str = "                ";
pub(crate) const L9: &str = "                  ";
pub(crate) const L10: &str = "                    ";

/// Index into the `Discovered.dependencies` arena.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DependencyIdx(pub usize);

/// One dependency within one version group: the set of instances of a single
/// `internal_name` that group claims. Immutable discovery data — inspection
/// outputs (expected specifier, states) are held in `Inspection`, keyed by
/// `DependencyIdx` / `InstanceIdx`.
#[derive(Debug)]
pub struct Dependency {
  pub has_alias: bool,
  pub instances: Vec<InstanceIdx>,
  /// The `/version` instance of this dependency when the dependency is a
  /// package developed locally in this monorepo.
  pub local_instance: Option<InstanceIdx>,
  pub internal_name: String,
}

impl Dependency {
  fn new(internal_name: String) -> Self {
    Self {
      has_alias: false,
      instances: vec![],
      local_instance: None,
      internal_name,
    }
  }

  pub fn get_update_url(&self, arena: &[Instance]) -> Option<UpdateUrl> {
    if self.internal_name_is_supported() {
      self.instances.iter().find_map(|idx| arena[idx.0].get_update_url())
    } else {
      None
    }
  }

  pub fn get_state(&self, inspection: &Inspection) -> InstanceState {
    self
      .instances
      .iter()
      .fold(InstanceState::Unknown, |acc, idx| acc.max(inspection.state(*idx).clone()))
  }

  pub fn get_states(&self, inspection: &Inspection) -> Vec<InstanceState> {
    self.instances.iter().map(|idx| inspection.state(*idx).clone()).collect()
  }

  pub fn get_local_specifier(&self, arena: &[Instance]) -> Option<Rc<Specifier>> {
    self.local_instance.map(|idx| Rc::clone(&arena[idx.0].descriptor.specifier))
  }

  pub fn internal_name_is_supported(&self) -> bool {
    !self.internal_name.contains('>') && self.internal_name.rfind('@').unwrap_or(0) == 0
  }

  pub fn has_local_instance(&self) -> bool {
    self.local_instance.is_some()
  }

  pub fn has_local_instance_with_invalid_specifier(&self, arena: &[Instance]) -> bool {
    self
      .get_local_specifier(arena)
      .is_some_and(|local| !matches!(&*local, Specifier::Exact(_)))
  }

  pub fn every_specifier_is_already_identical(&self, arena: &[Instance]) -> bool {
    if let Some(first_actual) = self.instances.first().map(|idx| &arena[idx.0].descriptor.specifier) {
      self.instances.iter().all(|idx| {
        let instance = &arena[idx.0];
        Rc::ptr_eq(&instance.descriptor.specifier, first_actual) || instance.descriptor.specifier.get_raw() == first_actual.get_raw()
      })
    } else {
      false
    }
  }

  pub fn get_unique_specifiers(&self, arena: &[Instance]) -> Vec<Rc<Specifier>> {
    let mut unique = Vec::new();
    for idx in &self.instances {
      let spec = &arena[idx.0].descriptor.specifier;
      if !unique.iter().any(|s: &Rc<Specifier>| s.get_raw() == spec.get_raw()) {
        unique.push(Rc::clone(spec));
      }
    }
    unique
  }

  pub fn get_instances<'a>(&'a self, arena: &'a [Instance]) -> impl Iterator<Item = (InstanceIdx, &'a Instance)> {
    self.instances.iter().map(move |idx| (*idx, &arena[idx.0]))
  }

  pub fn get_sorted_instances<'a>(
    &'a self,
    inspection: &Inspection,
    instances: &'a [Instance],
    sources: &'a [Source],
  ) -> impl Iterator<Item = (InstanceIdx, &'a Instance)> {
    self.get_instances(instances).sorted_by(|(a_idx, a), (b_idx, b)| {
      let a_valid = inspection.state(*a_idx).is_valid();
      let b_valid = inspection.state(*b_idx).is_valid();
      if a_valid && !b_valid {
        return Ordering::Less;
      }
      if b_valid && !a_valid {
        return Ordering::Greater;
      }
      if a.has_missing_specifier() {
        return Ordering::Greater;
      }
      if b.has_missing_specifier() {
        return Ordering::Less;
      }
      let specifier_order = b.descriptor.specifier.cmp(&a.descriptor.specifier);
      if matches!(specifier_order, Ordering::Equal) {
        let a_name = sources[a.source_idx().0].name();
        let b_name = sources[b.source_idx().0].name();
        a_name.cmp(b_name)
      } else {
        specifier_order
      }
    })
  }
}

/// Group discovered instances into a per-group map of `Dependency`, keyed by
/// `internal_name`. Called at discovery time; the map is later flattened into
/// the global dependency arena.
pub(crate) fn add_instance_to_dep_map(dependencies: &mut BTreeMap<String, Dependency>, idx: InstanceIdx, instance: &Instance) {
  let dep = dependencies
    .entry(instance.descriptor.internal_name.clone())
    .or_insert_with(|| Dependency::new(instance.descriptor.internal_name.clone()));
  dep.instances.push(idx);
  if instance.is_local_instance {
    dep.local_instance = Some(idx);
  }
  if instance.descriptor.name != dep.internal_name {
    dep.has_alias = true;
  }
}

/// Sort registry updates DESC by node version. Pre-computed once per
/// dependency so per-instance lookups can early-exit on first match.
pub(super) fn sort_updates_desc(updates: &[Rc<Specifier>]) -> Vec<Rc<Specifier>> {
  let mut v: Vec<Rc<Specifier>> = updates.to_vec();
  v.sort_by_key(|u| std::cmp::Reverse(u.get_node_version()));
  v
}

/// Find the highest eligible registry update for `installed` from a
/// pre-sorted DESC list. Returns `None` when no update is eligible.
pub(super) fn highest_eligible_for<'a>(
  sorted_desc: &'a [Rc<Specifier>],
  installed: &Rc<Specifier>,
  target: &UpdateTarget,
) -> Option<&'a Rc<Specifier>> {
  sorted_desc
    .iter()
    .find(|u| u.is_eligible_update_for(installed, target) && installed.has_same_release_channel_as(u))
}

#[cfg(test)]
#[path = "version_group/dependency_test.rs"]
mod dependency_test;

pub trait VersionGroupBehavior {
  fn selector(&self) -> &GroupSelector;
  fn dependency_indices(&self) -> &[DependencyIdx];
  fn set_dependency_indices(&mut self, indices: Vec<DependencyIdx>);
  fn visit(&self, input: &RuleInput, registry_updates: &Option<RegistryUpdates>, out: &mut Inspection);
}

#[derive(Debug)]
pub enum VersionGroup {
  Banned(BannedGroup),
  /// User-opt-in `policy: "catalog"` group. Enforces that claimed instances
  /// either ARE catalog definitions or use the `catalog:` protocol.
  Catalog(CatalogGroup),
  /// Auto-injected catch-all that claims catalog definition instances. Sits
  /// one slot above the existing `PreferredSemver` catch-all so user-defined
  /// groups can still claim catalog defs first (first-match-wins).
  CatalogDefs(CatalogDefsGroup),
  Ignored(IgnoredGroup),
  Pinned(PinnedGroup),
  PreferredSemver(PreferredSemverGroup),
  SameMinor(SameMinorGroup),
  SameRange(SameRangeGroup),
  SemverRangeOnly(SemverRangeOnlyGroup),
  SnappedTo(SnappedToGroup),
}

impl VersionGroupBehavior for VersionGroup {
  fn selector(&self) -> &GroupSelector {
    match self {
      Self::Banned(g) => &g.selector,
      Self::Catalog(g) => &g.selector,
      Self::CatalogDefs(g) => &g.selector,
      Self::Ignored(g) => &g.selector,
      Self::Pinned(g) => &g.selector,
      Self::PreferredSemver(g) => &g.selector,
      Self::SameMinor(g) => &g.selector,
      Self::SameRange(g) => &g.selector,
      Self::SemverRangeOnly(g) => &g.selector,
      Self::SnappedTo(g) => &g.selector,
    }
  }

  fn dependency_indices(&self) -> &[DependencyIdx] {
    match self {
      Self::Banned(g) => &g.dependencies,
      Self::Catalog(g) => &g.dependencies,
      Self::CatalogDefs(g) => &g.dependencies,
      Self::Ignored(g) => &g.dependencies,
      Self::Pinned(g) => &g.dependencies,
      Self::PreferredSemver(g) => &g.dependencies,
      Self::SameMinor(g) => &g.dependencies,
      Self::SameRange(g) => &g.dependencies,
      Self::SemverRangeOnly(g) => &g.dependencies,
      Self::SnappedTo(g) => &g.dependencies,
    }
  }

  fn set_dependency_indices(&mut self, indices: Vec<DependencyIdx>) {
    match self {
      Self::Banned(g) => g.dependencies = indices,
      Self::Catalog(g) => g.dependencies = indices,
      Self::CatalogDefs(g) => g.dependencies = indices,
      Self::Ignored(g) => g.dependencies = indices,
      Self::Pinned(g) => g.dependencies = indices,
      Self::PreferredSemver(g) => g.dependencies = indices,
      Self::SameMinor(g) => g.dependencies = indices,
      Self::SameRange(g) => g.dependencies = indices,
      Self::SemverRangeOnly(g) => g.dependencies = indices,
      Self::SnappedTo(g) => g.dependencies = indices,
    }
  }

  /// Stage-1 inspection: every non-`SnappedTo` group classifies its
  /// instances. `SnappedTo` is visited via `visit_snapped` after this pass.
  fn visit(&self, input: &RuleInput, registry_updates: &Option<RegistryUpdates>, out: &mut Inspection) {
    match self {
      Self::Banned(g) => g.visit(input, registry_updates, out),
      Self::Catalog(g) => g.visit(input, registry_updates, out),
      Self::CatalogDefs(g) => g.visit(input, registry_updates, out),
      Self::Ignored(g) => g.visit(input, registry_updates, out),
      Self::Pinned(g) => g.visit(input, registry_updates, out),
      Self::PreferredSemver(g) => g.visit(input, registry_updates, out),
      Self::SameMinor(g) => g.visit(input, registry_updates, out),
      Self::SameRange(g) => g.visit(input, registry_updates, out),
      Self::SemverRangeOnly(g) => g.visit(input, registry_updates, out),
      Self::SnappedTo(_) => unreachable!("SnappedTo groups are visited via visit_snapped in stage 2"),
    }
  }
}

impl VersionGroup {
  pub fn is_snapped_to(&self) -> bool {
    matches!(self, Self::SnappedTo(_))
  }

  /// Stage-2 inspection: `SnappedTo` groups snap their followers to the
  /// target's stage-1 computed expected value, read from `out`.
  pub fn visit_snapped(&self, input: &RuleInput, out: &mut Inspection) {
    match self {
      Self::SnappedTo(g) => g.visit_snapped(input, out),
      _ => unreachable!("visit_snapped is only valid for SnappedTo groups"),
    }
  }

  pub fn variant_label(&self) -> &str {
    match self {
      Self::Banned(_) => "Banned",
      Self::Catalog(_) => "Catalog",
      Self::CatalogDefs(_) => "CatalogDefs",
      Self::Ignored(_) => "Ignored",
      Self::Pinned(_) => "Pinned",
      Self::PreferredSemver(g) => {
        if g.prefer_highest {
          "HighestSemver"
        } else {
          "LowestSemver"
        }
      }
      Self::SameMinor(_) => "SameMinor",
      Self::SameRange(_) => "SameRange",
      Self::SemverRangeOnly(_) => "SemverRangeOnly",
      Self::SnappedTo(_) => "SnappedTo",
    }
  }

  pub fn is_ignored(&self) -> bool {
    matches!(self, Self::Ignored(_))
  }

  pub fn severity_map(&self) -> &SeverityMap {
    match self {
      Self::Banned(g) => &g.severity,
      Self::Catalog(g) => &g.severity,
      Self::CatalogDefs(g) => &g.severity,
      Self::Ignored(g) => &g.severity,
      Self::Pinned(g) => &g.severity,
      Self::PreferredSemver(g) => &g.severity,
      Self::SameMinor(g) => &g.severity,
      Self::SameRange(g) => &g.severity,
      Self::SemverRangeOnly(g) => &g.severity,
      Self::SnappedTo(g) => &g.severity,
    }
  }

  pub fn get_sorted_dependencies(&self, arena: &[Dependency], sort: &SortBy) -> Vec<DependencyIdx> {
    self
      .dependency_indices()
      .iter()
      .copied()
      .sorted_by(|a, b| match sort {
        SortBy::Count => arena[b.0].instances.len().cmp(&arena[a.0].instances.len()),
        SortBy::Name => Ordering::Equal,
      })
      .collect()
  }

  pub fn get_update_urls(&self, deps: &[Dependency], arena: &[Instance]) -> Option<Vec<UpdateUrl>> {
    let collect = |group_deps: &[DependencyIdx]| group_deps.iter().filter_map(|d| deps[d.0].get_update_url(arena)).collect();
    match self {
      Self::PreferredSemver(g) if g.prefer_highest => Some(collect(&g.dependencies)),
      Self::CatalogDefs(g) => Some(collect(&g.dependencies)),
      Self::SemverRangeOnly(g) => Some(collect(&g.dependencies)),
      _ => None,
    }
  }

  pub fn get_catch_all() -> Self {
    Self::PreferredSemver(PreferredSemverGroup {
      selector: GroupSelector::new(vec![], vec![], "Default Version Group".into(), vec![], vec![]),
      dependencies: vec![],
      prefer_highest: true,
      severity: SeverityMap::new(),
    })
  }

  pub fn from_config(group: AnyVersionGroup, index: usize, sources: &Sources) -> Result<Self, UnsupportedConfigError> {
    let selector = GroupSelector::new(
      group.dependencies,
      group.dependency_types,
      group.label,
      group.packages,
      group.specifier_types,
    );

    if let Some(true) = group.is_banned {
      let severity = validate_severity(group.severity, "Banned", index, BANNED_KEYS)?;
      return Ok(Self::Banned(BannedGroup {
        selector,
        dependencies: vec![],
        severity,
      }));
    }
    if let Some(true) = group.is_ignored {
      // Ignored produces no statuses; severity is accepted and discarded.
      return Ok(Self::Ignored(IgnoredGroup {
        selector,
        dependencies: vec![],
        severity: SeverityMap::new(),
      }));
    }
    if let Some(pin_version) = &group.pin_version {
      let severity = validate_severity(group.severity, "Pinned", index, PINNED_KEYS)?;
      return Ok(Self::Pinned(PinnedGroup {
        selector,
        dependencies: vec![],
        pin_version: Specifier::new(pin_version),
        severity,
      }));
    }
    if let Some(policy) = &group.policy {
      if policy == "sameRange" {
        let severity = validate_severity(group.severity, "SameRange", index, SAME_RANGE_KEYS)?;
        return Ok(Self::SameRange(SameRangeGroup {
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
        return Ok(Self::SameMinor(SameMinorGroup {
          selector,
          dependencies: vec![],
          prefer_version,
          severity,
        }));
      } else if policy == "catalog" {
        let severity = validate_severity(group.severity, "Catalog", index, CATALOG_KEYS)?;
        return Ok(Self::Catalog(CatalogGroup {
          selector,
          dependencies: vec![],
          severity,
        }));
      } else if policy == "semverRangeOnly" {
        let severity = validate_severity(group.severity, "SemverRangeOnly", index, SEMVER_RANGE_ONLY_KEYS)?;
        return Ok(Self::SemverRangeOnly(SemverRangeOnlyGroup {
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
      return Ok(Self::SnappedTo(SnappedToGroup {
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
      return Ok(Self::PreferredSemver(PreferredSemverGroup {
        selector,
        dependencies: vec![],
        prefer_highest,
        severity,
      }));
    }
    let severity = validate_severity(group.severity, "HighestSemver", index, PREFERRED_SEMVER_KEYS)?;
    Ok(Self::PreferredSemver(PreferredSemverGroup {
      selector,
      dependencies: vec![],
      prefer_highest: true,
      severity,
    }))
  }
}

/// Map a resolved severity to the action a command should take. Pure function
/// of `severity` — commands branch on this at report/apply time.
pub fn action_of(severity: Severity) -> InstanceAction {
  match severity {
    Severity::None => InstanceAction::Valid,
    Severity::Fix => InstanceAction::Fix(Severity::Fix),
    Severity::Warn | Severity::Error => InstanceAction::Render(severity),
  }
}

/// Resolver output consumed by each command's iteration loop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstanceAction {
  /// Do nothing — instance is Valid or Unknown.
  Valid,
  /// Render as warning or error; never apply the fix.
  Render(Severity),
  /// Apply the fix; render under `fix`/`lint`.
  Fix(Severity),
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
