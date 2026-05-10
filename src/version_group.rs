use {
  crate::{
    cli::SortBy,
    context::Context,
    dependency::UpdateUrl,
    errors::UnsupportedConfigError,
    group_selector::GroupSelector,
    instance::{Instance, InstanceIdx, InstanceState},
    registry::updates::RegistryUpdates,
    source::Source,
    sources::Sources,
    specifier::Specifier,
  },
  itertools::Itertools,
  log::warn,
  serde::Deserialize,
  serde_json::Value,
  std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    rc::Rc,
  },
  syncpack_specifier::update_target::UpdateTarget,
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
mod snapped_to;

/// When a version group has `preferVersion` set, this determines the direction
/// used to pick a winner among differing versions.
#[derive(Clone, Debug)]
pub enum PreferVersion {
  HighestSemver,
  LowestSemver,
}

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
  #[serde(flatten)]
  pub unknown_fields: HashMap<String, Value>,
}

pub use {
  banned::BannedGroup, catalog::CatalogGroup, catalog_defs::CatalogDefsGroup, ignored::IgnoredGroup, pinned::PinnedGroup,
  preferred_semver::PreferredSemverGroup, same_minor::SameMinorGroup, same_range::SameRangeGroup, snapped_to::SnappedToGroup,
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

#[derive(Debug)]
pub struct DependencyCore {
  pub expected: RefCell<Option<Rc<Specifier>>>,
  pub has_alias: bool,
  pub instances: Vec<InstanceIdx>,
  pub local_instance: RefCell<Option<InstanceIdx>>,
  pub internal_name: String,
}

impl DependencyCore {
  fn new(internal_name: String) -> Self {
    Self {
      expected: RefCell::new(None),
      has_alias: false,
      instances: vec![],
      local_instance: RefCell::new(None),
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

  pub fn get_state(&self, arena: &[Instance]) -> InstanceState {
    self
      .instances
      .iter()
      .fold(InstanceState::Unknown, |acc, idx| acc.max(arena[idx.0].state.borrow().clone()))
  }

  pub fn get_states(&self, arena: &[Instance]) -> Vec<InstanceState> {
    self.instances.iter().map(|idx| arena[idx.0].state.borrow().clone()).collect()
  }

  pub fn set_expected_specifier(&self, specifier: &Rc<Specifier>) -> &Self {
    *self.expected.borrow_mut() = Some(Rc::clone(specifier));
    self
  }

  pub fn get_local_specifier(&self, arena: &[Instance]) -> Option<Rc<Specifier>> {
    self
      .local_instance
      .borrow()
      .as_ref()
      .map(|idx| Rc::clone(&arena[idx.0].descriptor.specifier))
  }

  pub fn internal_name_is_supported(&self) -> bool {
    !self.internal_name.contains('>') && self.internal_name.rfind('@').unwrap_or(0) == 0
  }

  pub fn has_local_instance(&self) -> bool {
    self.local_instance.borrow().is_some()
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
    instances: &'a [Instance],
    sources: &'a [Source],
  ) -> impl Iterator<Item = (InstanceIdx, &'a Instance)> {
    self.get_instances(instances).sorted_by(|(_, a), (_, b)| {
      if a.is_valid() && !b.is_valid() {
        return Ordering::Less;
      }
      if b.is_valid() && !a.is_valid() {
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

pub(crate) fn add_instance_to_dependencies(dependencies: &mut BTreeMap<String, DependencyCore>, idx: InstanceIdx, instance: &Instance) {
  let dep = dependencies
    .entry(instance.descriptor.internal_name.clone())
    .or_insert_with(|| DependencyCore::new(instance.descriptor.internal_name.clone()));
  dep.instances.push(idx);
  if instance.is_local_instance {
    *dep.local_instance.borrow_mut() = Some(idx);
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
mod dependency_core_test {
  use super::DependencyCore;

  #[test]
  fn internal_name_is_supported() {
    let scenarios = vec![
      (true, "@fluid-private/changelog-generator-wrapper"),
      (true, "@fluid-tools/markdown-magic"),
      (true, "@types/events_pkg"),
      (true, "@types/node"),
      (true, "get-tsconfig"),
      (true, "node-fetch"),
      (true, "nodegit"),
      (true, "qs"),
      (true, "sharp"),
      (true, "socket.io-client"),
      (true, "socket.io-parser"),
      (false, "@fluentui/react-positioning>@floating-ui/dom"),
      (false, "@types/node@<18"),
      (false, "good-fences>nodegit"),
      (false, "json5@<1.0.2"),
      (false, "json5@>=2.0.0 <2.2.2"),
      (false, "oclif>@aws-sdk/client-cloudfront"),
      (false, "oclif>@aws-sdk/client-s3"),
      (false, "simplemde>codemirror"),
      (false, "simplemde>marked"),
    ];
    for (expected, name) in scenarios {
      let dep = DependencyCore::new(name.to_string());
      assert_eq!(expected, dep.internal_name_is_supported(), "failed for {name}");
    }
  }
}

pub trait VersionGroupBehavior {
  fn selector(&self) -> &GroupSelector;
  fn dependencies(&self) -> &BTreeMap<String, DependencyCore>;
  fn add_instance(&mut self, idx: InstanceIdx, instance: &Instance);
  fn visit(&self, ctx: &Context, registry_updates: &Option<RegistryUpdates>);
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
      Self::SnappedTo(g) => &g.selector,
    }
  }

  fn dependencies(&self) -> &BTreeMap<String, DependencyCore> {
    match self {
      Self::Banned(g) => &g.dependencies,
      Self::Catalog(g) => &g.dependencies,
      Self::CatalogDefs(g) => &g.dependencies,
      Self::Ignored(g) => &g.dependencies,
      Self::Pinned(g) => &g.dependencies,
      Self::PreferredSemver(g) => &g.dependencies,
      Self::SameMinor(g) => &g.dependencies,
      Self::SameRange(g) => &g.dependencies,
      Self::SnappedTo(g) => &g.dependencies,
    }
  }

  fn add_instance(&mut self, idx: InstanceIdx, instance: &Instance) {
    match self {
      Self::Banned(g) => g.add_instance(idx, instance),
      Self::Catalog(g) => g.add_instance(idx, instance),
      Self::CatalogDefs(g) => g.add_instance(idx, instance),
      Self::Ignored(g) => g.add_instance(idx, instance),
      Self::Pinned(g) => g.add_instance(idx, instance),
      Self::PreferredSemver(g) => g.add_instance(idx, instance),
      Self::SameMinor(g) => g.add_instance(idx, instance),
      Self::SameRange(g) => g.add_instance(idx, instance),
      Self::SnappedTo(g) => g.add_instance(idx, instance),
    }
  }

  fn visit(&self, ctx: &Context, registry_updates: &Option<RegistryUpdates>) {
    match self {
      Self::Banned(g) => g.visit(ctx, registry_updates),
      Self::Catalog(g) => g.visit(ctx, registry_updates),
      Self::CatalogDefs(g) => g.visit(ctx, registry_updates),
      Self::Ignored(g) => g.visit(ctx, registry_updates),
      Self::Pinned(g) => g.visit(ctx, registry_updates),
      Self::PreferredSemver(g) => g.visit(ctx, registry_updates),
      Self::SameMinor(g) => g.visit(ctx, registry_updates),
      Self::SameRange(g) => g.visit(ctx, registry_updates),
      Self::SnappedTo(g) => g.visit(ctx, registry_updates),
    }
  }
}

impl VersionGroup {
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
      Self::SnappedTo(_) => "SnappedTo",
    }
  }

  pub fn is_ignored(&self) -> bool {
    matches!(self, Self::Ignored(_))
  }

  pub fn get_sorted_dependencies(&self, sort: &SortBy) -> impl Iterator<Item = &DependencyCore> {
    self.dependencies().values().sorted_by(|a, b| match sort {
      SortBy::Count => b.instances.len().cmp(&a.instances.len()),
      SortBy::Name => Ordering::Equal,
    })
  }

  pub fn get_update_urls(&self, arena: &[Instance]) -> Option<Vec<UpdateUrl>> {
    match self {
      Self::PreferredSemver(g) if g.prefer_highest => Some(g.dependencies.values().filter_map(|d| d.get_update_url(arena)).collect()),
      Self::CatalogDefs(g) => Some(g.dependencies.values().filter_map(|d| d.get_update_url(arena)).collect()),
      _ => None,
    }
  }

  pub fn get_catch_all() -> Self {
    Self::PreferredSemver(PreferredSemverGroup {
      selector: GroupSelector::new(vec![], vec![], "Default Version Group".into(), vec![], vec![]),
      dependencies: BTreeMap::new(),
      prefer_highest: true,
    })
  }

  pub fn from_config(group: AnyVersionGroup, sources: &Sources) -> Result<Self, UnsupportedConfigError> {
    let selector = GroupSelector::new(
      group.dependencies,
      group.dependency_types,
      group.label,
      group.packages,
      group.specifier_types,
    );

    if let Some(true) = group.is_banned {
      return Ok(Self::Banned(BannedGroup {
        selector,
        dependencies: BTreeMap::new(),
      }));
    }
    if let Some(true) = group.is_ignored {
      return Ok(Self::Ignored(IgnoredGroup {
        selector,
        dependencies: BTreeMap::new(),
      }));
    }
    if let Some(pin_version) = &group.pin_version {
      return Ok(Self::Pinned(PinnedGroup {
        selector,
        dependencies: BTreeMap::new(),
        pin_version: Specifier::new(pin_version),
      }));
    }
    if let Some(policy) = &group.policy {
      if policy == "sameRange" {
        return Ok(Self::SameRange(SameRangeGroup {
          selector,
          dependencies: BTreeMap::new(),
        }));
      } else if policy == "sameMinor" {
        let prefer_version = group.prefer_version.as_ref().map(|pv| {
          if pv == "lowestSemver" {
            PreferVersion::LowestSemver
          } else {
            PreferVersion::HighestSemver
          }
        });
        return Ok(Self::SameMinor(SameMinorGroup {
          selector,
          dependencies: BTreeMap::new(),
          prefer_version,
        }));
      } else if policy == "catalog" {
        return Ok(Self::Catalog(CatalogGroup {
          selector,
          dependencies: BTreeMap::new(),
        }));
      } else {
        return Err(UnsupportedConfigError::InvalidVersionGroupPolicy(policy.clone()));
      }
    }
    if let Some(snap_to) = &group.snap_to {
      return Ok(Self::SnappedTo(SnappedToGroup {
        selector,
        dependencies: BTreeMap::new(),
        snap_to: snap_to
          .iter()
          .flat_map(|name| {
            sources.find_package(name).or_else(|| {
              warn!("Invalid Snapped To Version Group: No package.json file found with a name property of '{name}'");
              None
            })
          })
          .collect(),
      }));
    }
    if let Some(prefer_version) = &group.prefer_version {
      return Ok(Self::PreferredSemver(PreferredSemverGroup {
        selector,
        dependencies: BTreeMap::new(),
        prefer_highest: prefer_version != "lowestSemver",
      }));
    }
    Ok(Self::PreferredSemver(PreferredSemverGroup {
      selector,
      dependencies: BTreeMap::new(),
      prefer_highest: true,
    }))
  }
}

#[cfg(test)]
pub(crate) fn visit_groups(ctx: &Context, version_group_configs: &[serde_json::Value]) {
  visit_groups_with_registry(ctx, version_group_configs, &None);
}

#[cfg(test)]
pub(crate) fn visit_groups_with_registry(
  ctx: &Context,
  version_group_configs: &[serde_json::Value],
  registry_updates: &Option<RegistryUpdates>,
) {
  let mut groups: Vec<VersionGroup> = version_group_configs
    .iter()
    .map(|json| {
      let cfg: AnyVersionGroup = serde_json::from_value(json.clone()).unwrap();
      VersionGroup::from_config(cfg, &ctx.sources).unwrap()
    })
    .collect();
  groups.push(VersionGroup::get_catch_all());

  for (i, instance) in ctx.instances.iter().enumerate() {
    let idx = InstanceIdx(i);
    let package_name = ctx.sources.all[instance.source_idx().0].name();
    if let Some(group) = groups.iter_mut().find(|g| g.selector().can_add(&instance.descriptor, package_name)) {
      group.add_instance(idx, instance);
    }
  }

  let mut snapped_to_indices = Vec::new();
  let mut other_indices = Vec::new();
  for (i, group) in groups.iter().enumerate() {
    if matches!(group, VersionGroup::SnappedTo(_)) {
      snapped_to_indices.push(i);
    } else {
      other_indices.push(i);
    }
  }
  for &i in other_indices.iter().chain(snapped_to_indices.iter()) {
    groups[i].visit(ctx, registry_updates);
  }
}
