pub mod instance_state;
pub mod severity;

pub use {
  instance_state::{
    FixableInstance, InstanceState, InvalidInstance, SemverGroupAndVersionConflict, SuspectInstance, UnfixableInstance, ValidInstance,
  },
  severity::Severity,
};

#[cfg(test)]
mod instance_state_test;
#[cfg(test)]
mod instance_test;
#[cfg(test)]
mod source_idx_test;

use {
  crate::{
    dependency::{DependencyType, Strategy, UpdateUrl},
    disk::Disk,
    rcfile::update_group::UpdatePolicy,
    semver_range::SemverRange,
    source::Source,
    sources::SourceIdx,
    specifier::Specifier,
  },
  log::debug,
  std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
  },
};

/// Index into the Context.instances arena.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstanceIdx(pub usize);

/// Unique identifier for an instance, format: "{dep} in {location} of
/// {package}" Examples: "react in /dependencies of package-a"
///           "pnpm in /packageManager of package-b"
pub type InstanceId = String;

/// The unchanging descriptor data for an instance. Never mutated.
#[derive(Debug)]
pub struct InstanceDescriptor {
  /// Shared via `Rc` across descriptors built from the same dep type within
  /// one iteration pass.
  pub dependency_type: Rc<DependencyType>,
  /// When a dependency group is used, its alias_name (e.g. "@aws-sdk/**")
  /// goes here in place of the actual dependency name. Only used when
  /// allocating an `Instance` to a `Dependency`; the original `name` is
  /// otherwise preserved.
  pub internal_name: String,
  /// Whether this dependency's NAME matches a local package name (e.g.
  /// package "foo" depends on "bar", and "bar" is also a local package).
  /// Distinct from `Instance::is_local_instance` (which means this instance
  /// IS a local package's own version declaration).
  pub is_local_dependency: bool,
  pub name: String,
  /// Index into `Sources::all` for the file this instance was read from. For
  /// catalog defs this points at the holding file (pnpm yaml or the Bun root
  /// pkg.json); for regular declarations it points at the consuming
  /// package.json.
  pub source_idx: SourceIdx,
  /// The original specifier, never mutated.
  pub specifier: Rc<Specifier>,
}

/// A single occurrence of a dependency in the project.
#[derive(Debug)]
pub struct Instance {
  pub descriptor: InstanceDescriptor,
  /// `None` when syncpack cannot determine the expected specifier without
  /// user intervention.
  pub expected_specifier: RefCell<Option<Rc<Specifier>>>,
  pub id: InstanceId,
  pub is_local_instance: bool,
  /// If this instance belongs to a `WithRange` semver group, the range used
  /// by Version Groups when determining the preferred version, so it tries
  /// to also satisfy any applicable semver group ranges.
  pub preferred_semver_range: Option<SemverRange>,
  /// If this instance matched an `updateGroups` entry, the policy applied
  /// when picking eligible registry updates (skip entirely or clamp the
  /// effective `UpdateTarget`). `None` when no group matched.
  pub preferred_update_policy: Option<UpdatePolicy>,
  /// Resolved by `VersionGroup::resolve_action` from the instance's `state`,
  /// the matching group's `severity` map and the rcfile's `strict` flag. Set
  /// as a side effect of `resolve_action`; remains `None` until the resolver
  /// has run. See `.plans/severity.md` §3.5.
  pub severity: RefCell<Option<Severity>>,
  /// Starts as `Unknown`, assigned during `visit_packages()`. `RefCell` so
  /// states can be assigned without `&mut Context`.
  pub state: RefCell<InstanceState>,
}

impl Instance {
  pub fn new(
    descriptor: InstanceDescriptor,
    package_name: &str,
    preferred_semver_range: Option<SemverRange>,
    preferred_update_policy: Option<UpdatePolicy>,
  ) -> Instance {
    let dependency_type_name = &descriptor.dependency_type.path;
    let id = format!("{} in {} of {}", &descriptor.name, dependency_type_name, package_name);
    let is_local_instance = dependency_type_name == "/version";
    Instance {
      descriptor,
      expected_specifier: RefCell::new(None),
      id,
      is_local_instance,
      preferred_semver_range,
      preferred_update_policy,
      severity: RefCell::new(None),
      state: RefCell::new(InstanceState::Unknown),
    }
  }

  /// Check if a link: specifier resolves to the given local package's directory.
  ///
  /// Examples:
  /// - "link:../package-a" from /packages/package-b/package.json -> /packages/package-a
  /// - "link:../../elsewhere/package-a" from /packages/package-b/package.json -> /elsewhere/package-a
  ///
  /// pnpm catalog instances do not live in a package.json, so a `Link`
  /// specifier sourced from one cannot resolve to a local package — this
  /// returns `false` in that case.
  pub fn link_resolves_to_local_package(&self, local_instance: &Instance, sources: &[Source], disk: &Disk) -> bool {
    if let Specifier::Link(link) = &*self.descriptor.specifier {
      let self_idx = self.source_idx();
      let local_idx = local_instance.source_idx();
      let Source::Package {
        file_idx: consuming_idx, ..
      } = &sources[self_idx.0]
      else {
        return false;
      };
      let Source::Package { file_idx: local_idx, .. } = &sources[local_idx.0] else {
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
        let normalized_resolved = Self::normalize_path(&resolved_link_path);
        let normalized_local = Self::normalize_path(local_package_dir);
        normalized_resolved == normalized_local
      }
    } else {
      false
    }
  }

  /// Whether this instance was sourced from a catalog (pnpm or Bun) rather
  /// than a regular package.json dependency. Reads the dep type's
  /// `is_catalog_definition` flag — set by `make_catalog_dep_types` for
  /// auto-generated `pnpmCatalog*` / `bunCatalog*` dep types.
  pub fn is_catalog_instance(&self) -> bool {
    self.descriptor.dependency_type.is_catalog_definition
  }

  /// Index into `Sources::all` for the file this instance reads from.
  pub fn source_idx(&self) -> SourceIdx {
    self.descriptor.source_idx
  }

  /// Catalog name (`"default"` or named) for catalog instances; `None` for
  /// regular non-catalog instances. Derives the name cheaply from the dep
  /// type's `name` (no allocation).
  pub fn catalog_name(&self) -> Option<&str> {
    if self.is_catalog_instance() {
      Some(crate::sources::parse_catalog_name(&self.descriptor.dependency_type.name))
    } else {
      None
    }
  }

  /// Build the `catalog:` / `catalog:{name}` target specifier this catalog
  /// definition represents. Returns `None` for non-catalog instances.
  pub fn catalog_target_specifier(&self) -> Option<Rc<Specifier>> {
    self.catalog_name().map(|name| {
      let raw = if name == "default" {
        "catalog:".to_string()
      } else {
        format!("catalog:{name}")
      };
      Specifier::new(&raw)
    })
  }

  /// Normalize a path by resolving . and .. components.
  /// Does not require the path to exist on the filesystem.
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

  /// Record what syncpack has determined the state of this instance is and what
  /// its expected specifier should be
  fn set_state(&self, state: InstanceState, expected_specifier: &Rc<Specifier>) -> &Self {
    *self.state.borrow_mut() = state;
    *self.expected_specifier.borrow_mut() = Some(Rc::clone(expected_specifier));
    self
  }

  /// Mark this instance as already having a valid specifier
  pub fn mark_valid(&self, state: ValidInstance, expected_specifier: &Rc<Specifier>) -> &Self {
    self.set_state(InstanceState::Valid(state), expected_specifier)
  }

  /// Mark this instance as having something which doesn't look quite right, but
  /// for the moment is not yet resulting in an issue
  pub fn mark_suspect(&self, state: SuspectInstance) -> &Self {
    let specifier = Rc::clone(&self.descriptor.specifier);
    self.set_state(InstanceState::Suspect(state), &specifier)
  }

  /// Like `mark_suspect` but records an expected specifier so a user can opt
  /// into routing the instance through the fix path via `severity: { ...: "fix" }`.
  /// Used by `RefuseToPinLocal` and `RefuseToSnapLocal` — both rewrite a local
  /// package's `/version` to a target specifier.
  pub fn mark_suspect_with_expected(&self, state: SuspectInstance, expected_specifier: &Rc<Specifier>) -> &Self {
    self.set_state(InstanceState::Suspect(state), expected_specifier)
  }

  /// Mark this instance as having a mismatch which can be auto-fixed
  pub fn mark_fixable(&self, state: FixableInstance, expected_specifier: &Rc<Specifier>) -> &Self {
    self.set_state(InstanceState::Invalid(InvalidInstance::Fixable(state)), expected_specifier)
  }

  /// Mark this instance as a mismatch which can't be auto-fixed, its semver
  /// group and version group config are in conflict with one another, asking
  /// for mutually exclusive versions
  pub fn mark_conflict(&self, state: SemverGroupAndVersionConflict) -> &Self {
    let specifier = Rc::clone(&self.descriptor.specifier);
    self.set_state(InstanceState::Invalid(InvalidInstance::Conflict(state)), &specifier)
  }

  /// Mark this instance as a mismatch which can't be auto-fixed without user
  /// input
  pub fn mark_unfixable(&self, state: UnfixableInstance) -> &Self {
    let specifier = Rc::clone(&self.descriptor.specifier);
    self.set_state(InstanceState::Invalid(InvalidInstance::Unfixable(state)), &specifier)
  }

  pub fn is_valid(&self) -> bool {
    self.state.borrow().is_valid()
  }

  pub fn is_invalid(&self) -> bool {
    self.state.borrow().is_invalid()
  }

  pub fn is_suspect(&self) -> bool {
    self.state.borrow().is_suspect()
  }

  pub fn is_fixable(&self) -> bool {
    self.state.borrow().is_fixable()
  }

  pub fn is_banned(&self) -> bool {
    self.state.borrow().is_banned()
  }

  pub fn is_unfixable(&self) -> bool {
    self.state.borrow().is_unfixable()
  }

  pub fn is_outdated(&self) -> bool {
    self.state.borrow().is_outdated()
  }

  pub fn has_missing_specifier(&self) -> bool {
    matches!(&*self.descriptor.specifier, Specifier::None)
  }

  /// Does this instance's actual specifier match the expected specifier?
  pub fn already_equals(&self, expected: &Rc<Specifier>) -> bool {
    self.descriptor.specifier.get_raw() == expected.get_raw()
  }

  /// Does this instance belong to a `WithRange` semver group?
  pub fn must_match_preferred_semver_range(&self) -> bool {
    self.preferred_semver_range.is_some()
  }

  /// Does this instance belong to a `WithRange` semver group and which prefers
  /// a semver range other than the given range?
  ///
  /// This is a convenience method for the common case where a preferred semver
  /// range only matters if what is preferred is not the same as the expected
  /// version of a dependency which you are trying to synchronise to
  pub fn must_match_preferred_semver_range_which_is_not(&self, needed_range: &SemverRange) -> bool {
    self.must_match_preferred_semver_range() && !self.preferred_semver_range_is(needed_range)
  }

  /// Does this instance belong to a `WithRange` semver group and which prefers
  /// a semver range other than that used by the given specifier?
  pub fn must_match_preferred_semver_range_which_differs_to(&self, other_specifier: &Rc<Specifier>) -> bool {
    other_specifier
      .get_semver_range()
      .is_some_and(|range_of_other_specifier| self.must_match_preferred_semver_range_which_is_not(&range_of_other_specifier))
  }

  /// Is the given semver range the preferred semver range for this instance?
  pub fn preferred_semver_range_is(&self, range: &SemverRange) -> bool {
    self.preferred_semver_range.as_ref().map(|r| r == range).unwrap_or(false)
  }

  /// Does this instance belong to a `WithRange` semver group and also have a
  /// specifier which matches its preferred semver range?
  pub fn matches_preferred_semver_range(&self) -> bool {
    self
      .preferred_semver_range
      .as_ref()
      .map(|preferred_semver_range| self.descriptor.specifier.get_semver_range().as_ref() == Some(preferred_semver_range))
      .unwrap_or(false)
  }

  /// Get the expected version specifier for this instance with the semver
  /// group's preferred range applied
  pub fn get_specifier_with_preferred_semver_range(&self) -> Option<Rc<Specifier>> {
    self
      .preferred_semver_range
      .as_ref()
      .and_then(|preferred_semver_range| self.descriptor.specifier.with_range(preferred_semver_range))
  }

  pub fn get_update_url(&self) -> Option<UpdateUrl> {
    if !self.is_local_instance {
      let internal_name = &self.descriptor.internal_name;
      let actual_name = &self.descriptor.name;
      let raw = self.descriptor.specifier.get_raw();
      match &*self.descriptor.specifier {
        Specifier::Alias(alias) => {
          let aliased_name = &alias.name;
          if !aliased_name.is_empty() {
            if aliased_name.starts_with("@jsr/") {
              Some(UpdateUrl {
                internal_name: internal_name.clone(),
                url: format!("https://npm.jsr.io/{aliased_name}"),
              })
            } else if aliased_name == actual_name {
              Some(UpdateUrl {
                internal_name: internal_name.clone(),
                url: format!("https://registry.npmjs.org/{actual_name}"),
              })
            } else {
              debug!(
                "'{aliased_name}' in '{raw}' does not equal the instance name '{actual_name}', skipping update as this might create mismatches"
              );
              None
            }
          } else {
            None
          }
        }
        Specifier::Exact(_) | Specifier::Range(_) | Specifier::Major(_) | Specifier::Minor(_) | Specifier::Latest(_) => {
          if actual_name.starts_with("@jsr/") {
            Some(UpdateUrl {
              internal_name: internal_name.clone(),
              url: format!("https://npm.jsr.io/{actual_name}"),
            })
          } else {
            Some(UpdateUrl {
              internal_name: internal_name.clone(),
              url: format!("https://registry.npmjs.org/{actual_name}"),
            })
          }
        }
        _ => None,
      }
    } else {
      None
    }
  }

  /// Does this instance's specifier match the specifier of every one of the
  /// given instances?
  pub fn already_satisfies_all(&self, indices: &[InstanceIdx], arena: &[Instance]) -> bool {
    !matches!(&*self.descriptor.specifier, Specifier::None)
      && self.descriptor.specifier.satisfies_all(
        &indices
          .iter()
          .map(|idx| Rc::clone(&arena[idx.0].descriptor.specifier))
          .collect::<Vec<_>>(),
      )
  }

  /// Does this instance have the same major version as all other instances?
  /// Semver ranges are not taken into account.
  pub fn already_has_same_major_as_all(&self, indices: &[InstanceIdx], arena: &[Instance]) -> bool {
    if matches!(&*self.descriptor.specifier, Specifier::None) {
      return false;
    }
    match self.descriptor.specifier.get_node_version() {
      None => false,
      Some(a) => indices.iter().all(|idx| {
        let other_instance = &arena[idx.0];
        if matches!(&*other_instance.descriptor.specifier, Specifier::None) {
          return false;
        }
        match other_instance.descriptor.specifier.get_node_version() {
          None => false,
          Some(b) => a.major == b.major,
        }
      }),
    }
  }

  /// Does this instance have the same major.minor version as all other
  /// instances? Semver ranges are not taken into account.
  pub fn already_has_same_minor_number_as_all(&self, indices: &[InstanceIdx], arena: &[Instance]) -> bool {
    if matches!(&*self.descriptor.specifier, Specifier::None) {
      return false;
    }
    match self.descriptor.specifier.get_node_version() {
      None => false,
      Some(a) => indices.iter().all(|idx| {
        let other_instance = &arena[idx.0];
        if matches!(&*other_instance.descriptor.specifier, Specifier::None) {
          return false;
        }
        match other_instance.descriptor.specifier.get_node_version() {
          None => false,
          Some(b) => a.major == b.major && a.minor == b.minor,
        }
      }),
    }
  }

  /// Will this instance's specifier, once fixed to match its semver group,
  /// satisfy the given specifier?
  pub fn specifier_with_preferred_semver_range_will_satisfy(&self, other: &Rc<Specifier>) -> bool {
    self
      .get_specifier_with_preferred_semver_range()
      .map(|specifier| {
        if let (Some(spec_range), Some(other_range)) = (specifier.get_node_range(), other.get_node_range()) {
          spec_range.allows_any(&other_range)
        } else {
          false
        }
      })
      .unwrap_or(false)
  }

  /// Delete this instance from its underlying `Source`. No-op for
  /// `PnpmYaml`-sourced instances (pnpm catalog removal lands with the
  /// Banned-catalog work).
  pub fn remove(&self, disk: &mut Disk, source: &Source) {
    let Source::Package { file_idx, .. } = source else {
      debug!("Cannot remove pnpm catalog instance from a package.json");
      return;
    };
    let file = &mut disk.package_json_files[*file_idx];
    match self.descriptor.dependency_type.strategy {
      Strategy::NameAndVersionProps | Strategy::NamedVersionString | Strategy::UnnamedVersionString => {
        let path_to_prop = &self.descriptor.dependency_type.path;
        if let Some(parent_path) = path_to_prop.rfind('/') {
          let parent_pointer = &path_to_prop[..parent_path];
          let prop_name = &path_to_prop[parent_path + 1..];
          crate::disk::remove_prop(file, parent_pointer, prop_name);
        } else if path_to_prop == "/" {
          debug!("Cannot remove root property");
        }
      }
      Strategy::VersionsByName => {
        let path_to_obj = &self.descriptor.dependency_type.path;
        crate::disk::remove_prop(file, path_to_obj, &self.descriptor.name);
      }
      Strategy::InvalidConfig => {
        unreachable!("unrecognised strategy");
      }
    };
  }
}
