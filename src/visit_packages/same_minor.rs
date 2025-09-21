use {
  super::indent::{L1, L2, L3, L4, L5},
  crate::{
    instance_state::{FixableInstance, UnfixableInstance, ValidInstance},
    specifier::Specifier,
  },
  log::debug,
  std::rc::Rc,
};

#[cfg(test)]
#[path = "same_minor_test.rs"]
mod same_minor_test;

pub fn visit(dependency: &crate::dependency::Dependency) {
  debug!("visit same minor version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  dependency.instances.iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);

    // Check if this instance satisfies same major.minor with all others
    // or falls into a special case that should be handled differently
    let satisfies_policy =
      instance.has_same_major_minor_as_all(&dependency.instances) || should_handle_as_special_case(instance, &dependency.instances);

    if satisfies_policy {
      debug!("{L3}satisfies same major.minor policy (or special case)");

      if instance.must_match_preferred_semver_range() {
        debug!("{L4}belongs to a semver group");
        if let Some(preferred_specifier) = instance.get_specifier_with_preferred_semver_range() {
          let is_compatible_with_same_minor = is_semver_range_compatible(&preferred_specifier);

          if is_compatible_with_same_minor {
            debug!("{L5}semver group uses compatible patch range");
            if instance.matches_preferred_semver_range() {
              instance.mark_valid(ValidInstance::SatisfiesSameMinorGroup, actual_specifier);
            } else {
              instance.mark_fixable(FixableInstance::SemverRangeMismatch, &preferred_specifier);
            }
          } else {
            debug!("{L5}semver group uses incompatible range - same minor overrides");
            if instance.matches_preferred_semver_range() {
              let stripped_specifier = get_exact_version_specifier(actual_specifier);
              instance.mark_fixable(FixableInstance::SameMinorOverridesSemverRange, &stripped_specifier);
            } else {
              instance.mark_fixable(FixableInstance::SameMinorOverridesSemverRangeMismatch, actual_specifier);
            }
          }
        } else {
          instance.mark_valid(ValidInstance::SatisfiesSameMinorGroup, actual_specifier);
        };
      } else {
        debug!("{L4}doesn't belong to any semver group");
        let instances: &[Rc<crate::instance::Instance>] = &dependency.instances;
        if has_problematic_semver_siblings(instance, instances) {
          debug!("{L5}sibling has problematic semver group - marking as mismatch");
          instance.mark_unfixable(UnfixableInstance::SameMinorMismatch);
        } else {
          debug!("{L5}no problematic siblings");
          instance.mark_valid(ValidInstance::SatisfiesSameMinorGroup, actual_specifier);
        }
      }
    } else {
      debug!("{L3}doesn't satisfy same major.minor policy");
      instance.mark_unfixable(UnfixableInstance::SameMinorMismatch);
    }
  });
}

fn is_semver_range_compatible(specifier: &Specifier) -> bool {
  if let Some(semver) = specifier.get_semver() {
    matches!(semver.range_variant, crate::specifier::semver_range::SemverRange::Patch)
  } else {
    false
  }
}

fn get_exact_version_specifier(specifier: &Specifier) -> Specifier {
  if let Some(semver) = specifier.get_semver() {
    Specifier::new(&semver.node_version.to_string(), None)
  } else {
    specifier.clone()
  }
}

fn should_handle_as_special_case(_instance: &Rc<crate::instance::Instance>, instances: &[Rc<crate::instance::Instance>]) -> bool {
  // Special case 1: exact versions mixed with tilde ranges, no semver groups
  let has_semver_groups = instances.iter().any(|i| i.must_match_preferred_semver_range());
  if !has_semver_groups {
    let has_exact_and_tilde = instances.iter().any(|i| {
      if let Some(semver) = i.descriptor.specifier.get_semver() {
        matches!(semver.range_variant, crate::specifier::semver_range::SemverRange::Exact)
      } else {
        false
      }
    }) && instances.iter().any(|i| {
      if let Some(semver) = i.descriptor.specifier.get_semver() {
        matches!(semver.range_variant, crate::specifier::semver_range::SemverRange::Patch)
      } else {
        false
      }
    });

    if has_exact_and_tilde {
      return true;
    }
  }

  // Special case 2: instances with semver groups that need override handling
  let has_incompatible_semver_groups = instances.iter().any(|i| {
    if i.must_match_preferred_semver_range() {
      if let Some(preferred) = i.get_specifier_with_preferred_semver_range() {
        return !is_semver_range_compatible(&preferred);
      }
    }
    false
  });

  if has_incompatible_semver_groups {
    // Allow instances to be processed even if they don't pass the strict check
    // They'll be handled by the semver group override logic
    return true;
  }

  false
}

fn has_problematic_semver_siblings(instance: &Rc<crate::instance::Instance>, instances: &[Rc<crate::instance::Instance>]) -> bool {
  instances.iter().any(|other_instance| {
    if Rc::ptr_eq(instance, other_instance) {
      return false;
    }

    if other_instance.must_match_preferred_semver_range() {
      // Check for incompatible semver group
      if let Some(other_preferred) = other_instance.get_specifier_with_preferred_semver_range() {
        if !is_semver_range_compatible(&other_preferred) {
          return true;
        }
      }

      // Check for semver mismatch (even with compatible ranges)
      if !other_instance.matches_preferred_semver_range() {
        return true;
      }
    }

    false
  })
}
