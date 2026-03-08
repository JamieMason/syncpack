use {
  super::indent::{L1, L2, L3, L4, L5, L6, L7, L8, L9},
  crate::{
    instance_state::{FixableInstance, UnfixableInstance, ValidInstance},
    semver_range::SemverRange,
  },
  log::debug,
  std::rc::Rc,
};

#[cfg(test)]
#[path = "same_minor_test.rs"]
mod same_minor_test;

/// Returns true if the given semver range is safe within a sameMinor group,
/// meaning it cannot resolve a version outside the MAJOR.MINOR bucket.
fn is_safe_range(range: &SemverRange) -> bool {
  matches!(range, SemverRange::Exact | SemverRange::Patch)
}

pub fn visit(dependency: &crate::dependency::Dependency) {
  debug!("visit same minor version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);

  // ── Non-semver gate ──────────────────────────────────────────────────
  // If no instance has a semver version, we fall into the non-semver branch.
  // This mirrors preferred_semver.rs: no per-instance semver/non-semver split.
  let any_has_semver = dependency
    .instances
    .iter()
    .any(|i| i.descriptor.specifier.get_node_version().is_some());

  if !any_has_semver {
    debug!("{L2}no instances have a semver version");
    if dependency.every_specifier_is_already_identical() {
      debug!("{L3}but all are identical");
      dependency.instances.iter().for_each(|instance| {
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
        instance.mark_valid(ValidInstance::IsNonSemverButIdentical, actual_specifier);
      });
    } else {
      debug!("{L3}and they differ");
      dependency.instances.iter().for_each(|instance| {
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
        instance.mark_unfixable(UnfixableInstance::NonSemverMismatch);
      });
    }
    return;
  }

  // ── Major-mismatch gate ──────────────────────────────────────────────
  // Compute once before the per-instance loop (O(n) not O(n²)).
  let all_same_major = dependency
    .instances
    .first()
    .is_some_and(|first| first.already_has_same_major_as_all(&dependency.instances));

  if !all_same_major {
    debug!("{L2}instances have differing MAJOR versions");
    dependency.instances.iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
      instance.mark_unfixable(UnfixableInstance::SameMinorHasMajorMismatch);
    });
    return;
  }

  debug!("{L2}all instances share the same MAJOR");

  // ── Minor-match gate ─────────────────────────────────────────────────
  let all_same_minor = dependency
    .instances
    .first()
    .is_some_and(|first| first.already_has_same_minor_number_as_all(&dependency.instances));

  if all_same_minor {
    debug!("{L3}all instances share the same MAJOR.MINOR");
    // No fix target needed — patch may differ, that's allowed.
    visit_instances_at_correct_minor(dependency);
    return;
  }

  debug!("{L3}instances have differing MAJOR.MINOR");

  // ── preferVersion branching ──────────────────────────────────────────
  if dependency.prefer_version.is_none() {
    debug!("{L4}preferVersion is not set");
    dependency.instances.iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L5}visit instance '{}' ({actual_specifier:?})", instance.id);
      instance.mark_unfixable(UnfixableInstance::SameMinorMismatch);
    });
    return;
  }

  // preferVersion is set — compute the fix target once.
  let fix_target = dependency.get_highest_or_lowest_minor_specifier();
  if fix_target.is_none() {
    debug!("{L4}could not determine a fix target");
    dependency.instances.iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L5}visit instance '{}' ({actual_specifier:?})", instance.id);
      instance.mark_unfixable(UnfixableInstance::SameMinorMismatch);
    });
    return;
  }
  let fix_target = fix_target.unwrap();
  let fix_target_version = fix_target.get_node_version().unwrap();
  debug!("{L4}fix target is {fix_target:?}");

  dependency.instances.iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L5}visit instance '{}' ({actual_specifier:?})", instance.id);

    let instance_version = match actual_specifier.get_node_version() {
      Some(v) => v,
      None => {
        // Non-semver instance in a mixed group — treat as mismatch
        debug!("{L6}instance has no semver version");
        instance.mark_unfixable(UnfixableInstance::NonSemverMismatch);
        return;
      }
    };

    let is_at_target_minor = instance_version.major == fix_target_version.major && instance_version.minor == fix_target_version.minor;

    if is_at_target_minor {
      debug!("{L6}instance IS at the target MAJOR.MINOR");
      // Identical to "all same MAJOR.MINOR" subtree
      visit_instance_at_correct_minor(instance, actual_specifier);
    } else {
      debug!("{L6}instance is NOT at the target MAJOR.MINOR");
      // Determine the range for the fix target
      let fix_specifier = determine_fix_specifier_for_wrong_minor(instance, actual_specifier, &fix_target);
      debug!("{L7}fix target with range applied: {fix_specifier:?}");
      instance.mark_fixable(FixableInstance::DiffersToHighestOrLowestSemverMinor, &fix_specifier);
    }
  });
}

/// Process all instances when they all share the same MAJOR.MINOR.
/// No fix target needed — just handle semver range correctness.
fn visit_instances_at_correct_minor(dependency: &crate::dependency::Dependency) {
  dependency.instances.iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
    visit_instance_at_correct_minor(instance, actual_specifier);
  });
}

/// Process a single instance that already has the correct MAJOR.MINOR.
/// Handles semver group interaction and safe/unsafe range determination.
fn visit_instance_at_correct_minor(instance: &crate::instance::Instance, actual_specifier: &Rc<crate::specifier::Specifier>) {
  if instance.must_match_preferred_semver_range() {
    debug!("{L5}it belongs to a semver group");
    let preferred_range = instance.preferred_semver_range.as_ref().unwrap();

    if is_safe_range(preferred_range) {
      debug!("{L6}preferred range is safe ({preferred_range:?})");
      // Safe preferred range always satisfies sameMinor — no conflict possible.
      if instance.matches_preferred_semver_range() {
        debug!("{L7}instance already matches preferred range");
        instance.mark_valid(ValidInstance::SatisfiesSameMinorGroup, actual_specifier);
      } else {
        debug!("{L7}instance does not match preferred range");
        instance.mark_fixable(
          FixableInstance::SemverRangeMismatch,
          &instance.get_specifier_with_preferred_semver_range().unwrap(),
        );
      }
    } else {
      debug!("{L6}preferred range is unsafe ({preferred_range:?})");
      // sameMinor policy wins unconditionally — force Patch.
      if instance.matches_preferred_semver_range() {
        debug!("{L7}instance matches preferred (unsafe) range");
        instance.mark_fixable(
          FixableInstance::SameMinorOverridesSemverRange,
          &actual_specifier.with_range(&SemverRange::Patch).unwrap(),
        );
      } else {
        debug!("{L7}instance does not match preferred (unsafe) range");
        instance.mark_fixable(
          FixableInstance::SameMinorOverridesSemverRangeMismatch,
          &actual_specifier.with_range(&SemverRange::Patch).unwrap(),
        );
      }
    }
  } else {
    debug!("{L5}it does not belong to a semver group");
    // No semver group — check on-disk range directly.
    let on_disk_range = actual_specifier.get_semver_range();
    match on_disk_range {
      Some(ref range) if is_safe_range(range) => {
        debug!("{L6}on-disk range is safe ({range:?})");
        instance.mark_valid(ValidInstance::SatisfiesSameMinorGroup, actual_specifier);
      }
      Some(ref range) => {
        debug!("{L6}on-disk range is unsafe ({range:?})");
        instance.mark_fixable(
          FixableInstance::SameMinorOverridesSemverRange,
          &actual_specifier.with_range(&SemverRange::Patch).unwrap(),
        );
      }
      None => {
        // No semver range (e.g. non-semver specifier that still has a node version)
        debug!("{L6}no semver range on disk");
        instance.mark_valid(ValidInstance::SatisfiesSameMinorGroup, actual_specifier);
      }
    }
  }
}

/// Determine the fix specifier for an instance that is at the wrong MAJOR.MINOR.
///
/// Range selection priority:
/// 1. If instance has a semver group with a safe preferred range → use preferred range
/// 2. If instance has no semver group and on-disk range is safe → preserve on-disk range
/// 3. Otherwise → force ~ (sameMinor policy wins over unsafe ranges)
fn determine_fix_specifier_for_wrong_minor(
  instance: &crate::instance::Instance,
  actual_specifier: &Rc<crate::specifier::Specifier>,
  fix_target: &Rc<crate::specifier::Specifier>,
) -> Rc<crate::specifier::Specifier> {
  let fix_target_version = fix_target.get_node_version().unwrap();

  if instance.must_match_preferred_semver_range() {
    let preferred_range = instance.preferred_semver_range.as_ref().unwrap();
    debug!("{L8}instance has semver group preferring {preferred_range:?}");

    if is_safe_range(preferred_range) {
      debug!("{L9}preferred range is safe — applying to fix target");
      // Apply preferred range to fix target version
      actual_specifier
        .with_node_version(&fix_target_version)
        .and_then(|s| s.with_range(preferred_range))
        .unwrap_or_else(|| Rc::clone(fix_target))
    } else {
      debug!("{L9}preferred range is unsafe — forcing ~ on fix target");
      // sameMinor wins: force Patch
      actual_specifier
        .with_node_version(&fix_target_version)
        .and_then(|s| s.with_range(&SemverRange::Patch))
        .unwrap_or_else(|| Rc::clone(fix_target))
    }
  } else {
    // No semver group — check on-disk range
    let on_disk_range = actual_specifier.get_semver_range();
    match on_disk_range {
      Some(ref range) if is_safe_range(range) => {
        debug!("{L8}no semver group, on-disk range is safe ({range:?}) — preserving");
        actual_specifier
          .with_node_version(&fix_target_version)
          .unwrap_or_else(|| Rc::clone(fix_target))
      }
      _ => {
        debug!("{L8}no semver group, on-disk range is unsafe or absent — forcing ~");
        actual_specifier
          .with_node_version(&fix_target_version)
          .and_then(|s| s.with_range(&SemverRange::Patch))
          .unwrap_or_else(|| Rc::clone(fix_target))
      }
    }
  }
}
