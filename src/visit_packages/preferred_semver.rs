use {
  super::indent::{L1, L10, L2, L3, L4, L5, L6, L7, L8, L9},
  crate::{context::Context, specifier::semver_range::SemverRange},
  log::debug,
};

pub fn visit(dependency: &crate::dependency::Dependency, ctx: &Context) {
  debug!("visit standard version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  if dependency.has_local_instance_with_invalid_specifier() {
    debug!("{L2}it has an invalid local instance");
    dependency.instances.borrow().iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
      if instance.is_local {
        debug!("{L4}it is the invalid local instance");
        debug!("{L5}mark as suspect");
        instance.mark_suspect(crate::instance_state::SuspectInstance::InvalidLocalVersion);
      } else {
        debug!("{L4}it depends on an unknowable version of an invalid local instance");
        debug!("{L5}mark as error");
        instance.mark_unfixable(crate::instance_state::UnfixableInstance::DependsOnInvalidLocalPackage);
      }
    });
  } else if dependency.has_local_instance() {
    debug!("{L2}it is a package developed locally in this monorepo");
    let local_specifier = dependency.get_local_specifier().unwrap();
    dependency.set_expected_specifier(&local_specifier);
    dependency.instances.borrow().iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
      if instance.is_local {
        debug!("{L4}it is the valid local instance");
        instance.mark_valid(crate::instance_state::ValidInstance::IsLocalAndValid, &local_specifier);
        return;
      }
      debug!("{L4}it depends on the local instance");
      if instance.descriptor.specifier.is_workspace_protocol() {
        debug!("{L5}it is using the workspace protocol");
        if !ctx.config.rcfile.strict {
          debug!("{L6}strict mode is off");
          debug!("{L7}mark as satisfying local");
          instance.mark_valid(crate::instance_state::ValidInstance::SatisfiesLocal, &instance.descriptor.specifier);
          return;
        }
        debug!("{L6}strict mode is on");
      } else {
        debug!("{L5}it is not using the workspace protocol");
      }
      debug!("{L5}its version number (without a range):");
      if !instance.descriptor.specifier.has_same_version_number_as(&local_specifier) {
        debug!("{L6}differs to the local instance");
        debug!("{L7}mark as error");
        instance.mark_fixable(crate::instance_state::FixableInstance::DiffersToLocal, &local_specifier);
        return;
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
              crate::instance_state::ValidInstance::SatisfiesLocal,
              &instance.get_specifier_with_preferred_semver_range().unwrap(),
            );
          } else {
            debug!("{L9}the preferred semver range will not satisfy the local version");
            debug!("{L10}mark as unfixable error");
            instance.mark_conflict(crate::instance_state::SemverGroupAndVersionConflict::MatchConflictsWithLocal);
          }
        } else {
          debug!("{L8}its semver range does not match its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&local_specifier) {
            debug!("{L9}the preferred semver range will satisfy the local version");
            debug!("{L10}mark as fixable error");
            instance.mark_fixable(
              crate::instance_state::FixableInstance::SemverRangeMismatch,
              &instance.get_specifier_with_preferred_semver_range().unwrap(),
            );
          } else {
            debug!("{L9}the preferred semver range will not satisfy the local version");
            debug!("{L10}mark as unfixable error");
            instance.mark_conflict(crate::instance_state::SemverGroupAndVersionConflict::MismatchConflictsWithLocal);
          }
        }
        return;
      }
      debug!("{L7}it is not in a semver group which prefers a different semver range to the local instance");
      if instance.already_equals(&local_specifier) {
        debug!("{L8}its semver range matches the local instance");
        debug!("{L9}mark as valid");
        instance.mark_valid(crate::instance_state::ValidInstance::IsIdenticalToLocal, &local_specifier);
      } else {
        debug!("{L8}its semver range differs to the local instance");
        debug!("{L9}mark as error");
        instance.mark_fixable(crate::instance_state::FixableInstance::DiffersToLocal, &local_specifier);
      }
    });
  } else if let Some(highest_specifier) = dependency.get_highest_or_lowest_specifier() {
    debug!("{L2}a highest semver version was found ({highest_specifier:?})");
    dependency.set_expected_specifier(&highest_specifier);
    dependency.instances.borrow().iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
      debug!("{L4}its version number (without a range):");
      if !instance.descriptor.specifier.has_same_version_number_as(&highest_specifier) {
        debug!("{L5}differs to the highest semver version");
        debug!("{L6}mark as error");
        instance.mark_fixable(
          crate::instance_state::FixableInstance::DiffersToHighestOrLowestSemver,
          &highest_specifier,
        );
        return;
      }
      debug!("{L5}is the same as the highest semver version");
      let range_of_highest_specifier = highest_specifier.get_semver_range().unwrap();
      if instance.must_match_preferred_semver_range_which_is_not(range_of_highest_specifier) {
        let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
        debug!(
          "{L6}it is in a semver group which prefers a different semver range to the highest semver version ({preferred_semver_range:?})"
        );
        if instance.matches_preferred_semver_range() {
          debug!("{L7}its semver range matches its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&highest_specifier) {
            debug!("{L8}the semver range satisfies the highest semver version");
            debug!("{L9}mark as suspect (the config is asking for an inexact match)");
            instance.mark_valid(
              crate::instance_state::ValidInstance::SatisfiesHighestOrLowestSemver,
              &instance.descriptor.specifier,
            );
          } else {
            debug!("{L8}the preferred semver range will not satisfy the highest semver version");
            debug!("{L9}mark as unfixable error");
            instance.mark_conflict(crate::instance_state::SemverGroupAndVersionConflict::MatchConflictsWithHighestOrLowestSemver);
          }
        } else {
          debug!("{L7}its semver range does not match its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&highest_specifier) {
            debug!("{L8}the preferred semver range will satisfy the highest semver version");
            debug!("{L9}mark as fixable error");
            instance.mark_fixable(
              crate::instance_state::FixableInstance::SemverRangeMismatch,
              &instance.get_specifier_with_preferred_semver_range().unwrap(),
            );
          } else {
            debug!("{L8}the preferred semver range will not satisfy the highest semver version");
            debug!("{L9}mark as unfixable error");
            instance.mark_conflict(crate::instance_state::SemverGroupAndVersionConflict::MismatchConflictsWithHighestOrLowestSemver);
          }
        }
      } else {
        debug!("{L4}it is not in a semver group which prefers a different semver range to the highest semver version");
        if instance.already_equals(&highest_specifier) {
          debug!("{L5}it is identical to the highest semver version");
          debug!("{L6}mark as valid");
          instance.mark_valid(crate::instance_state::ValidInstance::IsHighestOrLowestSemver, &highest_specifier);
        } else {
          debug!("{L5}it is different to the highest semver version");
          debug!("{L6}mark as error");
          instance.mark_fixable(
            crate::instance_state::FixableInstance::DiffersToHighestOrLowestSemver,
            &highest_specifier,
          );
        }
      }
    });
  } else {
    debug!("{L2}no instances have a semver version");
    if dependency.every_specifier_is_already_identical() {
      debug!("{L3}but all are identical");
      dependency.instances.borrow().iter().for_each(|instance| {
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
        debug!("{L5}it is identical to every other instance");
        debug!("{L6}mark as valid");
        instance.mark_valid(
          crate::instance_state::ValidInstance::IsNonSemverButIdentical,
          &instance.descriptor.specifier,
        );
      });
    } else {
      debug!("{L3}and they differ");
      dependency.instances.borrow().iter().for_each(|instance| {
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
        debug!("{L5}it depends on a currently unknowable correct version from a set of unsupported version specifiers");
        debug!("{L6}mark as error");
        instance.mark_unfixable(crate::instance_state::UnfixableInstance::NonSemverMismatch);
      });
    }
  }
}
