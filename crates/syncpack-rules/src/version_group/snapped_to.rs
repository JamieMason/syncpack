use {
  crate::{
    inspection::{Inspection, RuleInput},
    version_group::{L1, L2, L3, L4, L5, L6, L7, L8, L9, L10},
  },
  log::debug,
  std::rc::Rc,
  syncpack_model::{
    dependency::{Dependency, DependencyIdx},
    group_selector::GroupSelector,
    instance::{
      FixableInstance, Instance, InstanceIdx, InstanceState, InvalidInstance, SemverGroupAndVersionConflict, SuspectInstance, ValidInstance,
    },
    sources::SourceIdx,
  },
  syncpack_specifier::Specifier,
  syncpack_vocab::severity::SeverityMap,
};

#[derive(Debug)]
pub struct SnappedToGroup {
  pub selector: GroupSelector,
  pub dependencies: Vec<DependencyIdx>,
  pub snap_to: Vec<SourceIdx>,
  pub severity: SeverityMap,
}

impl SnappedToGroup {
  /// The specifier the followers should snap to: the target instance's
  /// stage-1 computed expected value if set, otherwise its raw on-disk
  /// specifier (the self-snapped case, where the target was not visited in
  /// stage 1).
  pub fn get_snapped_to_specifier(&self, dep: &Dependency, all_instances: &[Instance], inspection: &Inspection) -> Option<Rc<Specifier>> {
    for (i, instance) in all_instances.iter().enumerate() {
      if *instance.descriptor.internal_name == *dep.internal_name {
        let instance_idx = instance.source_idx();
        for &snapped_to_idx in &self.snap_to {
          if instance_idx == snapped_to_idx {
            return Some(
              inspection
                .expected(InstanceIdx(i))
                .cloned()
                .unwrap_or_else(|| Rc::clone(&instance.descriptor.specifier)),
            );
          }
        }
      }
    }
    None
  }

  pub fn visit_snapped(&self, input: &RuleInput, out: &mut Inspection) {
    let arena = input.instances;
    let strict = input.cfg.strict;
    let severity_map = &self.severity;
    for &dep_idx in &self.dependencies {
      let dep = &input.dependencies[dep_idx.0];
      debug!("visit snapped to version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      if let Some(snapped_to_specifier) = self.get_snapped_to_specifier(dep, arena, out) {
        debug!("{L2}a target version was found ({snapped_to_specifier:?})");
        out.set_dep_expected(dep_idx, Rc::clone(&snapped_to_specifier));
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          if instance.is_local_instance && !instance.already_equals(&snapped_to_specifier) {
            debug!("{L4}it is the local instance of a package developed locally in this monorepo");
            debug!("{L5}refuse to change it");
            debug!("{L6}mark as error, user should change their config");
            out.classify(
              idx,
              InstanceState::Suspect(SuspectInstance::RefuseToSnapLocal),
              Some(Rc::clone(&snapped_to_specifier)),
              severity_map,
              strict,
            );
            continue;
          }
          debug!("{L4}it is not a local instance of a package developed locally in this monorepo");
          debug!("{L5}its version number (without a range):");
          if !instance.descriptor.specifier.has_same_version_number_as(&snapped_to_specifier) {
            debug!("{L6}differs to the target version");
            debug!("{L7}mark as error");
            let fix_target = instance
              .preferred_semver_range
              .as_ref()
              .and_then(|range| snapped_to_specifier.with_range(range))
              .unwrap_or_else(|| Rc::clone(&snapped_to_specifier));
            out.classify(
              idx,
              InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToSnapTarget)),
              Some(fix_target),
              severity_map,
              strict,
            );
            continue;
          }
          debug!("{L6}is the same as the target version");
          let range_of_snapped_to_specifier = snapped_to_specifier.get_semver_range().unwrap();
          if instance.must_match_preferred_semver_range_which_is_not(&range_of_snapped_to_specifier) {
            let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
            debug!("{L7}it is in a semver group which prefers a different semver range to the target version ({preferred_semver_range:?})");
            if instance.matches_preferred_semver_range() {
              debug!("{L8}its semver range matches its semver group");
              if instance.specifier_with_preferred_semver_range_will_satisfy(&snapped_to_specifier) {
                debug!("{L9}the semver range satisfies the target version");
                debug!("{L10}mark as suspect (the config is asking for an inexact match)");
                out.classify(
                  idx,
                  InstanceState::Valid(ValidInstance::SatisfiesSnapTarget),
                  Some(Rc::clone(actual_specifier)),
                  severity_map,
                  strict,
                );
              } else {
                debug!("{L9}the preferred semver range will not satisfy the target version");
                debug!("{L10}mark as unfixable error");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Conflict(
                    SemverGroupAndVersionConflict::MatchConflictsWithSnapTarget,
                  )),
                  Some(Rc::clone(actual_specifier)),
                  severity_map,
                  strict,
                );
              }
            } else {
              debug!("{L8}its semver range does not match its semver group");
              if instance.specifier_with_preferred_semver_range_will_satisfy(&snapped_to_specifier) {
                debug!("{L9}the preferred semver range will satisfy the target version");
                debug!("{L10}mark as fixable error");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                  Some(instance.get_specifier_with_preferred_semver_range().unwrap()),
                  severity_map,
                  strict,
                );
              } else {
                debug!("{L9}the preferred semver range will not satisfy the target version");
                debug!("{L10}mark as unfixable error");
                out.classify(
                  idx,
                  InstanceState::Invalid(InvalidInstance::Conflict(
                    SemverGroupAndVersionConflict::MismatchConflictsWithSnapTarget,
                  )),
                  Some(Rc::clone(actual_specifier)),
                  severity_map,
                  strict,
                );
              }
            }
          } else {
            debug!("{L5}it is not in a semver group which prefers a different semver range to the target version");
            if instance.already_equals(&snapped_to_specifier) {
              debug!("{L6}it is identical to the target version");
              debug!("{L7}mark as valid");
              out.classify(
                idx,
                InstanceState::Valid(ValidInstance::IsIdenticalToSnapTarget),
                Some(Rc::clone(&snapped_to_specifier)),
                severity_map,
                strict,
              );
            } else {
              debug!("{L6}it is different to the target version");
              debug!("{L7}mark as error");
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToSnapTarget)),
                Some(Rc::clone(&snapped_to_specifier)),
                severity_map,
                strict,
              );
            }
          }
        }
      } else {
        debug!("{L2}no target version was found");
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          debug!("{L3}visit instance '{}'", instance.id);
          out.classify(
            idx,
            InstanceState::Suspect(SuspectInstance::DependsOnMissingSnapTarget),
            Some(Rc::clone(&instance.descriptor.specifier)),
            severity_map,
            strict,
          );
        }
      }
    }
  }
}
