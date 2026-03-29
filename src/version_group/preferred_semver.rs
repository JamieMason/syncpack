use {
  super::{add_instance_to_dependencies, DependencyCore, L1, L10, L2, L3, L4, L5, L6, L7, L8, L9},
  crate::{
    cli::UpdateTarget,
    context::Context,
    group_selector::GroupSelector,
    instance::{FixableInstance, Instance, InstanceIdx, SemverGroupAndVersionConflict, SuspectInstance, UnfixableInstance, ValidInstance},
    registry::updates::RegistryUpdates,
    semver_range::SemverRange,
    specifier::Specifier,
  },
  log::debug,
  std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    rc::Rc,
  },
};

#[cfg(test)]
#[path = "preferred_semver_test.rs"]
mod preferred_semver_test;

#[derive(Debug)]
pub struct PreferredSemverGroup {
  pub selector: GroupSelector,
  pub dependencies: BTreeMap<String, DependencyCore>,
  pub prefer_highest: bool,
}

impl PreferredSemverGroup {
  pub fn add_instance(&mut self, idx: InstanceIdx, instance: &Instance) {
    add_instance_to_dependencies(&mut self.dependencies, idx, instance);
  }

  pub fn get_highest_or_lowest_specifier(&self, dep: &DependencyCore, arena: &[Instance]) -> Option<Rc<Specifier>> {
    let specifiers = dep
      .get_instances(arena)
      .filter(|instance| instance.descriptor.specifier.get_node_version().is_some())
      .map(|instance| {
        instance
          .preferred_semver_range
          .as_ref()
          .and_then(|range| instance.descriptor.specifier.with_range(range))
          .unwrap_or_else(|| Rc::clone(&instance.descriptor.specifier))
      });
    if self.prefer_highest {
      specifiers.max()
    } else {
      specifiers.min()
    }
  }

  pub fn get_eligible_registry_updates(
    &self,
    dep: &DependencyCore,
    arena: &[Instance],
    registry_updates: &RegistryUpdates,
    target: &UpdateTarget,
  ) -> Option<HashMap<String, Vec<Rc<Specifier>>>> {
    registry_updates.updates_by_internal_name.get(&dep.internal_name).map(|updates| {
      let mut specifiers_by_eligible_update: HashMap<String, Vec<Rc<Specifier>>> = HashMap::new();
      dep.get_unique_specifiers(arena).iter().for_each(|installed| {
        updates
          .iter()
          .filter(|update| update.is_eligible_update_for(installed, target))
          .filter(|update| installed.has_same_release_channel_as(update))
          .fold(None, |preferred, specifier| match preferred {
            None => Some(specifier),
            Some(preferred) => {
              if specifier.get_node_version().cmp(&preferred.get_node_version()) == Ordering::Greater {
                Some(specifier)
              } else {
                Some(preferred)
              }
            }
          })
          .inspect(|highest_update| {
            let key = highest_update.get_raw().to_string();
            let affected = specifiers_by_eligible_update.entry(key).or_default();
            affected.push(Rc::clone(installed));
          });
      });
      specifiers_by_eligible_update
    })
  }

  pub fn visit(&self, ctx: &Context, registry_updates: Option<&RegistryUpdates>) {
    let arena = &ctx.instances;
    for dep in self.dependencies.values() {
      debug!("visit standard version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      if dep.has_local_instance_with_invalid_specifier(arena) {
        debug!("{L2}it has an invalid local instance");
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          if instance.is_local_instance {
            debug!("{L4}it is the invalid local instance");
            debug!("{L5}mark as suspect");
            instance.mark_suspect(SuspectInstance::InvalidLocalVersion);
          } else {
            debug!("{L4}it depends on an unknowable version of an invalid local instance");
            debug!("{L5}mark as error");
            instance.mark_unfixable(UnfixableInstance::DependsOnInvalidLocalPackage);
          }
        }
      } else if dep.has_local_instance() {
        debug!("{L2}it is a package developed locally in this monorepo");
        let local_specifier = dep.get_local_specifier(arena).unwrap();
        dep.set_expected_specifier(&local_specifier);
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          if instance.is_local_instance {
            debug!("{L4}it is the valid local instance");
            instance.mark_valid(ValidInstance::IsLocalAndValid, &local_specifier);
            continue;
          }
          debug!("{L4}it depends on the local instance");
          if instance.descriptor.specifier.is_link() {
            debug!("{L5}it is using the link specifier");
            if let Some(local_idx) = dep.local_instance.borrow().as_ref() {
              let local_instance = &arena[local_idx.0];
              if instance.link_resolves_to_local_package(local_instance) {
                debug!("{L6}link resolves to local package directory");
                debug!("{L7}mark as satisfying local");
                instance.mark_valid(ValidInstance::SatisfiesLocal, &instance.descriptor.specifier);
                continue;
              } else {
                debug!("{L6}link resolves to a different directory");
                debug!("{L7}mark as differs to local");
                instance.mark_fixable(FixableInstance::DiffersToLocal, &local_specifier);
                continue;
              }
            }
          }
          if instance.descriptor.specifier.is_workspace_protocol() {
            debug!("{L5}it is using the workspace protocol");
            if !ctx.config.rcfile.strict {
              debug!("{L6}strict mode is off");
              debug!("{L7}mark as satisfying local");
              instance.mark_valid(ValidInstance::SatisfiesLocal, &instance.descriptor.specifier);
              continue;
            }
            debug!("{L6}strict mode is on");
          } else {
            debug!("{L5}it is not using the workspace protocol");
          }
          debug!("{L5}its version number (without a range):");
          if !instance.descriptor.specifier.has_same_version_number_as(&local_specifier) {
            debug!("{L6}differs to the local instance");
            debug!("{L7}mark as error");
            instance.mark_fixable(FixableInstance::DiffersToLocal, &local_specifier);
            continue;
          }
          debug!("{L6}is the same as the local instance");
          if instance.must_match_preferred_semver_range_which_is_not(&SemverRange::Exact) {
            let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
            debug!("{L7}it is in a semver group which prefers a different semver range to the local instance ({preferred_semver_range:?})");
            if instance.matches_preferred_semver_range() {
              debug!("{L8}its semver range matches its semver group");
              if instance.specifier_with_preferred_semver_range_will_satisfy(&local_specifier) {
                debug!("{L9}the semver range satisfies the local version");
                debug!("{L10}mark as suspect (the config is asking for an inexact match)");
                instance.mark_valid(
                  ValidInstance::SatisfiesLocal,
                  &instance.get_specifier_with_preferred_semver_range().unwrap(),
                );
              } else {
                debug!("{L9}the preferred semver range will not satisfy the local version");
                debug!("{L10}mark as unfixable error");
                instance.mark_conflict(SemverGroupAndVersionConflict::MatchConflictsWithLocal);
              }
            } else {
              debug!("{L8}its semver range does not match its semver group");
              if instance.specifier_with_preferred_semver_range_will_satisfy(&local_specifier) {
                debug!("{L9}the preferred semver range will satisfy the local version");
                debug!("{L10}mark as fixable error");
                instance.mark_fixable(
                  FixableInstance::SemverRangeMismatch,
                  &instance.get_specifier_with_preferred_semver_range().unwrap(),
                );
              } else {
                debug!("{L9}the preferred semver range will not satisfy the local version");
                debug!("{L10}mark as unfixable error");
                instance.mark_conflict(SemverGroupAndVersionConflict::MismatchConflictsWithLocal);
              }
            }
            continue;
          }
          debug!("{L7}it is not in a semver group which prefers a different semver range to the local instance");
          if instance.already_equals(&local_specifier) {
            debug!("{L8}its semver range matches the local instance");
            debug!("{L9}mark as valid");
            instance.mark_valid(ValidInstance::IsIdenticalToLocal, &local_specifier);
          } else {
            debug!("{L8}its semver range differs to the local instance");
            debug!("{L9}mark as error");
            instance.mark_fixable(FixableInstance::DiffersToLocal, &local_specifier);
          }
        }
      } else if let Some(catalog_specifier) = dep
        .instances
        .iter()
        .find(|idx| arena[idx.0].descriptor.specifier.is_catalog())
        .map(|idx| &arena[idx.0].descriptor.specifier)
      {
        debug!("{L2}one or more instances use the catalog: protocol which wins over semver");
        dep.set_expected_specifier(catalog_specifier);
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          if instance.descriptor.specifier.is_catalog() {
            debug!("{L4}it uses the catalog: protocol");
            debug!("{L5}mark as valid");
            instance.mark_valid(ValidInstance::IsCatalog, catalog_specifier);
          } else {
            debug!("{L4}it does not use the catalog: protocol");
            debug!("{L5}mark as error");
            instance.mark_fixable(FixableInstance::DiffersToCatalog, catalog_specifier);
          }
        }
      } else if let Some(specifiers_by_eligible_update) =
        registry_updates.and_then(|updates| self.get_eligible_registry_updates(dep, arena, updates, &ctx.config.cli.target))
      {
        debug!("{L2}eligible updates were found on the npm registry ({specifiers_by_eligible_update:?})");
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          specifiers_by_eligible_update.iter().for_each(|(update, affected_specifiers)| {
            if affected_specifiers.iter().any(|s| s.get_raw() == actual_specifier.get_raw()) {
              debug!("{L4}an eligible update {update:?} is available");
              let range = &instance
                .preferred_semver_range
                .clone()
                .or_else(|| actual_specifier.get_semver_range())
                .unwrap_or(SemverRange::Exact);
              let update_specifier = Specifier::new(update);
              if let Some(update_version) = update_specifier.get_node_version() {
                if let Some(with_updated_version) = actual_specifier.with_node_version(&update_version) {
                  if let Some(with_preferred_range) = with_updated_version.with_range(range) {
                    debug!("{L4}with semver group applied update becomes {with_preferred_range:?}");
                    instance.mark_fixable(FixableInstance::DiffersToNpmRegistry, &with_preferred_range);
                  }
                }
              }
            }
          });
        }
      } else if let Some(highest_specifier) = self.get_highest_or_lowest_specifier(dep, arena) {
        debug!("{L2}a highest semver version was found ({highest_specifier:?})");
        dep.set_expected_specifier(&highest_specifier);
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          debug!("{L4}its version number (without a range):");
          if !instance.descriptor.specifier.has_same_version_number_as(&highest_specifier) {
            debug!("{L5}differs to the highest semver version");
            debug!("{L6}mark as error");
            let fix_target = instance
              .preferred_semver_range
              .as_ref()
              .and_then(|range| highest_specifier.with_range(range))
              .unwrap_or_else(|| Rc::clone(&highest_specifier));
            instance.mark_fixable(FixableInstance::DiffersToHighestOrLowestSemver, &fix_target);
            continue;
          }
          debug!("{L5}is the same as the highest semver version");
          let range_of_highest_specifier = highest_specifier.get_semver_range().unwrap();
          if instance.must_match_preferred_semver_range_which_is_not(&range_of_highest_specifier) {
            let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
            debug!(
              "{L6}it is in a semver group which prefers a different semver range to the highest semver version ({preferred_semver_range:?})"
            );
            if instance.matches_preferred_semver_range() {
              debug!("{L7}its semver range matches its semver group");
              if instance.specifier_with_preferred_semver_range_will_satisfy(&highest_specifier) {
                debug!("{L8}the semver range satisfies the highest semver version");
                debug!("{L9}mark as suspect (the config is asking for an inexact match)");
                instance.mark_valid(ValidInstance::SatisfiesHighestOrLowestSemver, &instance.descriptor.specifier);
              } else {
                debug!("{L8}the preferred semver range will not satisfy the highest semver version");
                debug!("{L9}mark as unfixable error");
                instance.mark_conflict(SemverGroupAndVersionConflict::MatchConflictsWithHighestOrLowestSemver);
              }
            } else {
              debug!("{L7}its semver range does not match its semver group");
              if instance.specifier_with_preferred_semver_range_will_satisfy(&highest_specifier) {
                debug!("{L8}the preferred semver range will satisfy the highest semver version");
                debug!("{L9}mark as fixable error");
                instance.mark_fixable(
                  FixableInstance::SemverRangeMismatch,
                  &instance.get_specifier_with_preferred_semver_range().unwrap(),
                );
              } else {
                debug!("{L8}the preferred semver range will not satisfy the highest semver version");
                debug!("{L9}mark as unfixable error");
                instance.mark_conflict(SemverGroupAndVersionConflict::MismatchConflictsWithHighestOrLowestSemver);
              }
            }
          } else {
            debug!("{L4}it is not in a semver group which prefers a different semver range to the highest semver version");
            if instance.must_match_preferred_semver_range() && !instance.matches_preferred_semver_range() {
              let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
              debug!("{L5}but its actual range does not match its semver group's preferred range ({preferred_semver_range:?})");
              if instance.specifier_with_preferred_semver_range_will_satisfy(&highest_specifier) {
                debug!("{L6}the preferred semver range will satisfy the highest semver version");
                debug!("{L7}mark as fixable error");
                instance.mark_fixable(
                  FixableInstance::SemverRangeMismatch,
                  &instance.get_specifier_with_preferred_semver_range().unwrap(),
                );
              } else {
                debug!("{L6}the preferred semver range will not satisfy the highest semver version");
                debug!("{L7}mark as unfixable error");
                instance.mark_conflict(SemverGroupAndVersionConflict::MismatchConflictsWithHighestOrLowestSemver);
              }
            } else if instance.already_equals(&highest_specifier) {
              debug!("{L5}it is identical to the highest semver version");
              debug!("{L6}mark as valid");
              instance.mark_valid(ValidInstance::IsHighestOrLowestSemver, &highest_specifier);
            } else {
              debug!("{L5}it is different to the highest semver version");
              debug!("{L6}mark as error");
              instance.mark_fixable(FixableInstance::DiffersToHighestOrLowestSemver, &highest_specifier);
            }
          }
        }
      } else {
        debug!("{L2}no instances have a semver version");
        if dep.every_specifier_is_already_identical(arena) {
          debug!("{L3}but all are identical");
          for &idx in &dep.instances {
            let instance = &arena[idx.0];
            let actual_specifier = &instance.descriptor.specifier;
            debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
            debug!("{L5}it is identical to every other instance");
            debug!("{L6}mark as valid");
            instance.mark_valid(ValidInstance::IsNonSemverButIdentical, &instance.descriptor.specifier);
          }
        } else {
          debug!("{L3}and they differ");
          for &idx in &dep.instances {
            let instance = &arena[idx.0];
            let actual_specifier = &instance.descriptor.specifier;
            debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
            debug!("{L5}it depends on a currently unknowable correct version from a set of unsupported version specifiers");
            debug!("{L6}mark as error");
            instance.mark_unfixable(UnfixableInstance::NonSemverMismatch);
          }
        }
      }
    }
  }
}
