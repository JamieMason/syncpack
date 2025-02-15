use {
  super::indent::{L1, L10, L2, L3, L4, L5, L6, L7, L8, L9},
  crate::{
    context::Context,
    instance_state::{FixableInstance, SemverGroupAndVersionConflict, SuspectInstance, ValidInstance},
  },
  log::debug,
};

#[cfg(test)]
#[path = "snapped_to_test.rs"]
mod snapped_to_test;

pub fn visit(dependency: &crate::dependency::Dependency, ctx: &Context) {
  debug!("visit snapped to version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  if let Some(snapped_to_specifier) = dependency.get_snapped_to_specifier(&ctx.instances) {
    debug!("{L2}a target version was found ({snapped_to_specifier:?})");
    dependency.set_expected_specifier(&snapped_to_specifier);
    dependency.instances.iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
      if instance.is_local && !instance.already_equals(&snapped_to_specifier) {
        debug!("{L4}it is the local instance of a package developed locally in this monorepo");
        debug!("{L5}refuse to change it");
        debug!("{L6}mark as error, user should change their config");
        instance.mark_suspect(SuspectInstance::RefuseToSnapLocal);
        return;
      }
      debug!("{L4}it is not a local instance of a package developed locally in this monorepo");
      debug!("{L5}its version number (without a range):");
      if !instance.descriptor.specifier.has_same_version_number_as(&snapped_to_specifier) {
        debug!("{L6}differs to the target version");
        debug!("{L7}mark as error");
        instance.mark_fixable(FixableInstance::DiffersToSnapTarget, &snapped_to_specifier);
        return;
      }
      debug!("{L6}is the same as the target version");
      let range_of_snapped_to_specifier = snapped_to_specifier.get_semver_range().unwrap();
      if instance.must_match_preferred_semver_range_which_is_not(range_of_snapped_to_specifier) {
        let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
        debug!("{L7}it is in a semver group which prefers a different semver range to the target version ({preferred_semver_range:?})");
        if instance.matches_preferred_semver_range() {
          debug!("{L8}its semver range matches its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&snapped_to_specifier) {
            debug!("{L9}the semver range satisfies the target version");
            debug!("{L10}mark as suspect (the config is asking for an inexact match)");
            instance.mark_valid(ValidInstance::SatisfiesSnapTarget, &instance.descriptor.specifier);
          } else {
            debug!("{L9}the preferred semver range will not satisfy the target version");
            debug!("{L10}mark as unfixable error");
            instance.mark_conflict(SemverGroupAndVersionConflict::MatchConflictsWithSnapTarget);
          }
        } else {
          debug!("{L8}its semver range does not match its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&snapped_to_specifier) {
            debug!("{L9}the preferred semver range will satisfy the target version");
            debug!("{L10}mark as fixable error");
            instance.mark_fixable(
              FixableInstance::SemverRangeMismatch,
              &instance.get_specifier_with_preferred_semver_range().unwrap(),
            );
          } else {
            debug!("{L9}the preferred semver range will not satisfy the target version");
            debug!("{L10}mark as unfixable error");
            instance.mark_conflict(SemverGroupAndVersionConflict::MismatchConflictsWithSnapTarget);
          }
        }
      } else {
        debug!("{L5}it is not in a semver group which prefers a different semver range to the target version");
        if instance.already_equals(&snapped_to_specifier) {
          debug!("{L6}it is identical to the target version");
          debug!("{L7}mark as valid");
          instance.mark_valid(ValidInstance::IsIdenticalToSnapTarget, &snapped_to_specifier);
        } else {
          debug!("{L6}it is different to the target version");
          debug!("{L7}mark as error");
          instance.mark_fixable(FixableInstance::DiffersToSnapTarget, &snapped_to_specifier);
        }
      }
    });
  } else {
    debug!("{L2}no target version was found");
    dependency.instances.iter().for_each(|instance| {
      instance.mark_suspect(SuspectInstance::DependsOnMissingSnapTarget);
    });
  }
}
