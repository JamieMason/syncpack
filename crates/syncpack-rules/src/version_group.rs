use {
  crate::inspection::{Inspection, RuleInput},
  itertools::Itertools,
  std::{cmp::Ordering, collections::HashSet, rc::Rc},
  syncpack_model::{
    dependency::{Dependency, DependencyIdx},
    group_selector::GroupSelector,
    instance::{Instance, InstanceIdx, InstanceState},
    source::Source,
  },
  syncpack_specifier::{Specifier, update_target::UpdateTarget},
  syncpack_vocab::{registry_updates::RegistryUpdates, severity::SeverityMap, update_url::UpdateUrl},
};

mod banned;
mod catalog;
mod catalog_defs;
mod ignored;
mod pinned;
mod preferred_semver;
mod same_minor;
mod same_range;
mod semver_range_only;
mod snapped_to;

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

/// Every registry URL that should be queried for updates across all version
/// groups. Deduplicated by `UpdateUrl`; internal names support dependency-group
/// aliasing (many dependencies aliased as one). The registry fetcher consumes
/// this set (`registry::updates::fetch_all`), keeping `registry` free of any
/// dependency on the rule/model types.
pub fn get_unique_update_urls(version_groups: &[VersionGroup], dependencies: &[Dependency], arena: &[Instance]) -> HashSet<UpdateUrl> {
  version_groups.iter().fold(HashSet::new(), |mut unique_update_urls, group| {
    group.get_update_urls(dependencies, arena).inspect(|update_urls| {
      update_urls.iter().for_each(|url| {
        unique_update_urls.insert(url.clone());
      });
    });
    unique_update_urls
  })
}

/// The dependency's overall state: the highest (worst) instance state across
/// its instances, read from `inspection`.
pub fn get_state(dep: &Dependency, inspection: &Inspection) -> InstanceState {
  dep
    .instances
    .iter()
    .fold(InstanceState::Unknown, |acc, idx| acc.max(inspection.state(*idx).clone()))
}

/// Every instance's state for this dependency, read from `inspection`.
pub fn get_states(dep: &Dependency, inspection: &Inspection) -> Vec<InstanceState> {
  dep.instances.iter().map(|idx| inspection.state(*idx).clone()).collect()
}

/// The dependency's instances sorted valid-first, then by descending specifier,
/// then by source name. Reads validity from `inspection`.
pub fn get_sorted_instances<'a>(
  dep: &'a Dependency,
  inspection: &Inspection,
  instances: &'a [Instance],
  sources: &'a [Source],
) -> impl Iterator<Item = (InstanceIdx, &'a Instance)> {
  dep.get_instances(instances).sorted_by(|(a_idx, a), (b_idx, b)| {
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
}
