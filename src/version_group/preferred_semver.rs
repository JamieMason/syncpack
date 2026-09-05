use {
  crate::{
    group_selector::GroupSelector,
    inspection::{Inspection, RuleInput},
    instance::{
      FixableInstance, Instance, InstanceState, InvalidInstance, SemverGroupAndVersionConflict, SuspectInstance, UnfixableInstance,
      ValidInstance,
    },
    semver_range::SemverRange,
    specifier::Specifier,
    version_group::{Dependency, DependencyIdx, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, highest_eligible_for, sort_updates_desc},
  },
  log::debug,
  std::rc::Rc,
  syncpack_vocab::{registry_updates::RegistryUpdates, severity::SeverityMap, update_policy::UpdatePolicy},
};

#[cfg(test)]
#[path = "preferred_semver_test.rs"]
mod preferred_semver_test;

#[derive(Debug)]
pub struct PreferredSemverGroup {
  pub selector: GroupSelector,
  pub dependencies: Vec<DependencyIdx>,
  pub prefer_highest: bool,
  pub severity: SeverityMap,
}

impl PreferredSemverGroup {
  pub fn get_highest_or_lowest_specifier(&self, dep: &Dependency, arena: &[Instance]) -> Option<Rc<Specifier>> {
    let specifiers = dep
      .get_instances(arena)
      .filter(|(_, instance)| instance.descriptor.specifier.get_node_version().is_some())
      .map(|(_, instance)| {
        instance
          .preferred_semver_range
          .as_ref()
          .and_then(|range| instance.descriptor.specifier.with_range(range))
          .unwrap_or_else(|| Rc::clone(&instance.descriptor.specifier))
      });
    if self.prefer_highest { specifiers.max() } else { specifiers.min() }
  }

  pub fn visit(&self, input: &RuleInput, registry_updates: &Option<RegistryUpdates>, out: &mut Inspection) {
    let arena = input.instances;
    let strict = input.cfg.strict;
    let severity_map = &self.severity;
    for &dep_idx in &self.dependencies {
      let dep = &input.dependencies[dep_idx.0];
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
            out.classify(
              idx,
              InstanceState::Suspect(SuspectInstance::InvalidLocalVersion),
              Some(Rc::clone(actual_specifier)),
              severity_map,
              strict,
            );
          } else {
            debug!("{L4}it depends on an unknowable version of an invalid local instance");
            debug!("{L5}mark as error");
            out.classify(
              idx,
              InstanceState::Invalid(InvalidInstance::Unfixable(UnfixableInstance::DependsOnInvalidLocalPackage)),
              Some(Rc::clone(actual_specifier)),
              severity_map,
              strict,
            );
          }
        }
      } else if dep.has_local_instance() {
        debug!("{L2}it is a package developed locally in this monorepo");
        let local_specifier = dep.get_local_specifier(arena).unwrap();
        out.set_dep_expected(dep_idx, Rc::clone(&local_specifier));
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          if instance.is_catalog_instance() {
            debug!("{L4}it is a catalog definition (defensive short-circuit)");
            if instance.must_match_preferred_semver_range()
              && !instance.matches_preferred_semver_range()
              && let Some(corrected) = instance.get_specifier_with_preferred_semver_range()
            {
              debug!("{L5}semver group prefers a different range; mark as SemverRangeMismatch ({corrected:?})");
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                Some(corrected),
                severity_map,
                strict,
              );
              continue;
            }
            debug!("{L5}mark as catalog definition");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::IsCatalogDefinition),
              Some(Rc::clone(actual_specifier)),
              severity_map,
              strict,
            );
            continue;
          }
          if instance.is_local_instance {
            debug!("{L4}it is the valid local instance");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::IsLocalAndValid),
              Some(Rc::clone(&local_specifier)),
              severity_map,
              strict,
            );
            continue;
          }
          debug!("{L4}it depends on the local instance");
          if instance.descriptor.specifier.is_link() {
            debug!("{L5}it is using the link specifier");
            if dep.local_instance.is_some() {
              if instance.link_resolves_to_local {
                debug!("{L6}link resolves to local package directory");
                debug!("{L7}mark as satisfying local");
                out.classify(
                  idx,
                  InstanceState::Valid(ValidInstance::SatisfiesLocal),
                  Some(Rc::clone(actual_specifier)),
                  severity_map,
                  strict,
                );
                continue;
              } else {
                debug!("{L6}link resolves to a different directory");
                debug!("{L7}mark as differs to local");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToLocal)),
                  Some(Rc::clone(&local_specifier)),
                  severity_map,
                  strict,
                );
                continue;
              }
            }
          }
          if instance.descriptor.specifier.is_workspace_protocol() {
            debug!("{L5}it is using the workspace protocol");
            if !strict {
              debug!("{L6}strict mode is off");
              debug!("{L7}mark as satisfying local");
              out.classify(
                idx,
                InstanceState::Valid(ValidInstance::SatisfiesLocal),
                Some(Rc::clone(actual_specifier)),
                severity_map,
                strict,
              );
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
            out.classify(
              idx,
              InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToLocal)),
              Some(Rc::clone(&local_specifier)),
              severity_map,
              strict,
            );
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
                out.classify(
                  idx,
                  InstanceState::Valid(ValidInstance::SatisfiesLocal),
                  Some(instance.get_specifier_with_preferred_semver_range().unwrap()),
                  severity_map,
                  strict,
                );
              } else {
                debug!("{L9}the preferred semver range will not satisfy the local version");
                debug!("{L10}mark as unfixable error");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Conflict(SemverGroupAndVersionConflict::MatchConflictsWithLocal)),
                  Some(Rc::clone(actual_specifier)),
                  severity_map,
                  strict,
                );
              }
            } else {
              debug!("{L8}its semver range does not match its semver group");
              if instance.specifier_with_preferred_semver_range_will_satisfy(&local_specifier) {
                debug!("{L9}the preferred semver range will satisfy the local version");
                debug!("{L10}mark as fixable error");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                  Some(instance.get_specifier_with_preferred_semver_range().unwrap()),
                  severity_map,
                  strict,
                );
              } else {
                debug!("{L9}the preferred semver range will not satisfy the local version");
                debug!("{L10}mark as unfixable error");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Conflict(SemverGroupAndVersionConflict::MismatchConflictsWithLocal)),
                  Some(Rc::clone(actual_specifier)),
                  severity_map,
                  strict,
                );
              }
            }
            continue;
          }
          debug!("{L7}it is not in a semver group which prefers a different semver range to the local instance");
          if instance.already_equals(&local_specifier) {
            debug!("{L8}its semver range matches the local instance");
            debug!("{L9}mark as valid");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::IsIdenticalToLocal),
              Some(Rc::clone(&local_specifier)),
              severity_map,
              strict,
            );
          } else {
            debug!("{L8}its semver range differs to the local instance");
            debug!("{L9}mark as error");
            out.classify(
              idx,
              InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToLocal)),
              Some(Rc::clone(&local_specifier)),
              severity_map,
              strict,
            );
          }
        }
      } else if let Some(catalog_specifier) = dep
        .instances
        .iter()
        .find(|idx| arena[idx.0].descriptor.specifier.is_catalog())
        .map(|idx| &arena[idx.0].descriptor.specifier)
      {
        debug!("{L2}one or more instances use the catalog: protocol which wins over semver");
        out.set_dep_expected(dep_idx, Rc::clone(catalog_specifier));
        let catalog_specifier = Rc::clone(catalog_specifier);
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          if instance.is_catalog_instance() {
            debug!("{L4}it is a catalog definition (defensive short-circuit)");
            if instance.must_match_preferred_semver_range()
              && !instance.matches_preferred_semver_range()
              && let Some(corrected) = instance.get_specifier_with_preferred_semver_range()
            {
              debug!("{L5}semver group prefers a different range; mark as SemverRangeMismatch ({corrected:?})");
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                Some(corrected),
                severity_map,
                strict,
              );
              continue;
            }
            debug!("{L5}mark as catalog definition");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::IsCatalogDefinition),
              Some(Rc::clone(actual_specifier)),
              severity_map,
              strict,
            );
          } else if instance.descriptor.specifier.is_catalog() {
            debug!("{L4}it uses the catalog: protocol");
            debug!("{L5}mark as valid");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::IsCatalog),
              Some(Rc::clone(&catalog_specifier)),
              severity_map,
              strict,
            );
          } else {
            debug!("{L4}it does not use the catalog: protocol");
            debug!("{L5}mark as error");
            out.classify(
              idx,
              InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToCatalog)),
              Some(Rc::clone(&catalog_specifier)),
              severity_map,
              strict,
            );
          }
        }
      } else if let Some(updates) = registry_updates
        .as_ref()
        .and_then(|r| r.updates_by_internal_name.get(&dep.internal_name))
      {
        let sorted_desc = sort_updates_desc(updates);
        debug!(
          "{L2}registry updates available for '{}' ({} versions)",
          dep.internal_name,
          sorted_desc.len()
        );
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          if instance.is_catalog_instance() {
            debug!("{L4}it is a catalog definition (defensive short-circuit)");
            if instance.must_match_preferred_semver_range()
              && !instance.matches_preferred_semver_range()
              && let Some(corrected) = instance.get_specifier_with_preferred_semver_range()
            {
              debug!("{L5}semver group prefers a different range; mark as SemverRangeMismatch ({corrected:?})");
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                Some(corrected),
                severity_map,
                strict,
              );
              continue;
            }
            debug!("{L5}mark as catalog definition");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::IsCatalogDefinition),
              Some(Rc::clone(actual_specifier)),
              severity_map,
              strict,
            );
            continue;
          }

          let effective = match &instance.preferred_update_policy {
            Some(UpdatePolicy::Skip) => {
              debug!("{L4}updateGroup policy is Skip; mark as ignored");
              out.classify(
                idx,
                InstanceState::Valid(ValidInstance::IsIgnored),
                Some(Rc::clone(actual_specifier)),
                severity_map,
                strict,
              );
              continue;
            }
            Some(UpdatePolicy::UpTo(t)) => input.cfg.target.stricter(*t),
            None => input.cfg.target,
          };

          if let Some(highest_update) = highest_eligible_for(&sorted_desc, actual_specifier, &effective) {
            debug!("{L4}an eligible update {highest_update:?} is available");
            let range = &instance
              .preferred_semver_range
              .clone()
              .or_else(|| actual_specifier.get_semver_range())
              .unwrap_or(SemverRange::Exact);
            if let Some(update_version) = highest_update.get_node_version()
              && let Some(with_updated_version) = actual_specifier.with_node_version(&update_version)
              && let Some(with_preferred_range) = with_updated_version.with_range(range)
            {
              debug!("{L4}with semver group applied update becomes {with_preferred_range:?}");
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToNpmRegistry)),
                Some(with_preferred_range),
                severity_map,
                strict,
              );
            }
          }
        }
      } else if let Some(highest_specifier) = self.get_highest_or_lowest_specifier(dep, arena) {
        debug!("{L2}a highest semver version was found ({highest_specifier:?})");
        out.set_dep_expected(dep_idx, Rc::clone(&highest_specifier));
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          if instance.is_catalog_instance() {
            debug!("{L4}it is a catalog definition (defensive short-circuit)");
            if instance.must_match_preferred_semver_range()
              && !instance.matches_preferred_semver_range()
              && let Some(corrected) = instance.get_specifier_with_preferred_semver_range()
            {
              debug!("{L5}semver group prefers a different range; mark as SemverRangeMismatch ({corrected:?})");
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                Some(corrected),
                severity_map,
                strict,
              );
              continue;
            }
            debug!("{L5}mark as catalog definition");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::IsCatalogDefinition),
              Some(Rc::clone(actual_specifier)),
              severity_map,
              strict,
            );
            continue;
          }
          debug!("{L4}its version number (without a range):");
          if !instance.descriptor.specifier.has_same_version_number_as(&highest_specifier) {
            debug!("{L5}differs to the highest semver version");
            debug!("{L6}mark as error");
            let fix_target = instance
              .preferred_semver_range
              .as_ref()
              .and_then(|range| highest_specifier.with_range(range))
              .unwrap_or_else(|| Rc::clone(&highest_specifier));
            out.classify(
              idx,
              InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToHighestOrLowestSemver)),
              Some(fix_target),
              severity_map,
              strict,
            );
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
                out.classify(
                  idx,
                  InstanceState::Valid(ValidInstance::SatisfiesHighestOrLowestSemver),
                  Some(Rc::clone(actual_specifier)),
                  severity_map,
                  strict,
                );
              } else {
                debug!("{L8}the preferred semver range will not satisfy the highest semver version");
                debug!("{L9}mark as unfixable error");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Conflict(
                    SemverGroupAndVersionConflict::MatchConflictsWithHighestOrLowestSemver,
                  )),
                  Some(Rc::clone(actual_specifier)),
                  severity_map,
                  strict,
                );
              }
            } else {
              debug!("{L7}its semver range does not match its semver group");
              if instance.specifier_with_preferred_semver_range_will_satisfy(&highest_specifier) {
                debug!("{L8}the preferred semver range will satisfy the highest semver version");
                debug!("{L9}mark as fixable error");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                  Some(instance.get_specifier_with_preferred_semver_range().unwrap()),
                  severity_map,
                  strict,
                );
              } else {
                debug!("{L8}the preferred semver range will not satisfy the highest semver version");
                debug!("{L9}mark as unfixable error");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Conflict(
                    SemverGroupAndVersionConflict::MismatchConflictsWithHighestOrLowestSemver,
                  )),
                  Some(Rc::clone(actual_specifier)),
                  severity_map,
                  strict,
                );
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
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                  Some(instance.get_specifier_with_preferred_semver_range().unwrap()),
                  severity_map,
                  strict,
                );
              }
            } else if instance.already_equals(&highest_specifier) {
              debug!("{L5}it is identical to the highest semver version");
              debug!("{L6}mark as valid");
              out.classify(
                idx,
                InstanceState::Valid(ValidInstance::IsHighestOrLowestSemver),
                Some(Rc::clone(&highest_specifier)),
                severity_map,
                strict,
              );
            } else {
              debug!("{L5}it is different to the highest semver version");
              debug!("{L6}mark as error");
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToHighestOrLowestSemver)),
                Some(Rc::clone(&highest_specifier)),
                severity_map,
                strict,
              );
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
            if instance.is_catalog_instance() {
              debug!("{L5}it is a catalog definition (defensive short-circuit)");
              if instance.must_match_preferred_semver_range()
                && !instance.matches_preferred_semver_range()
                && let Some(corrected) = instance.get_specifier_with_preferred_semver_range()
              {
                debug!("{L6}semver group prefers a different range; mark as SemverRangeMismatch ({corrected:?})");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                  Some(corrected),
                  severity_map,
                  strict,
                );
                continue;
              }
              debug!("{L6}mark as catalog definition");
              out.classify(
                idx,
                InstanceState::Valid(ValidInstance::IsCatalogDefinition),
                Some(Rc::clone(actual_specifier)),
                severity_map,
                strict,
              );
              continue;
            }
            debug!("{L5}it is identical to every other instance");
            debug!("{L6}mark as valid");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::IsNonSemverButIdentical),
              Some(Rc::clone(actual_specifier)),
              severity_map,
              strict,
            );
          }
        } else {
          debug!("{L3}and they differ");
          for &idx in &dep.instances {
            let instance = &arena[idx.0];
            let actual_specifier = &instance.descriptor.specifier;
            debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
            if instance.is_catalog_instance() {
              debug!("{L5}it is a catalog definition (defensive short-circuit)");
              if instance.must_match_preferred_semver_range()
                && !instance.matches_preferred_semver_range()
                && let Some(corrected) = instance.get_specifier_with_preferred_semver_range()
              {
                debug!("{L6}semver group prefers a different range; mark as SemverRangeMismatch ({corrected:?})");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                  Some(corrected),
                  severity_map,
                  strict,
                );
                continue;
              }
              debug!("{L6}mark as catalog definition");
              out.classify(
                idx,
                InstanceState::Valid(ValidInstance::IsCatalogDefinition),
                Some(Rc::clone(actual_specifier)),
                severity_map,
                strict,
              );
              continue;
            }
            debug!("{L5}it depends on a currently unknowable correct version from a set of unsupported version specifiers");
            debug!("{L6}mark as error");
            out.classify(
              idx,
              InstanceState::Invalid(InvalidInstance::Unfixable(UnfixableInstance::NonSemverMismatch)),
              Some(Rc::clone(actual_specifier)),
              severity_map,
              strict,
            );
          }
        }
      }
    }
  }
}
