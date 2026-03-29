use {
  super::{add_instance_to_dependencies, DependencyCore, PreferVersion, L1, L2, L3, L4, L5, L6, L7, L8, L9},
  crate::{
    context::Context,
    group_selector::GroupSelector,
    instance::{FixableInstance, Instance, InstanceIdx, UnfixableInstance, ValidInstance},
    registry::updates::RegistryUpdates,
    semver_range::SemverRange,
    specifier::Specifier,
  },
  log::debug,
  std::{collections::BTreeMap, rc::Rc},
};

#[cfg(test)]
#[path = "same_minor_test.rs"]
mod same_minor_test;

#[derive(Debug)]
pub struct SameMinorGroup {
  pub selector: GroupSelector,
  pub dependencies: BTreeMap<String, DependencyCore>,
  pub prefer_version: Option<PreferVersion>,
}

fn is_safe_range(range: &SemverRange) -> bool {
  matches!(range, SemverRange::Exact | SemverRange::Patch)
}

impl SameMinorGroup {
  pub fn add_instance(&mut self, idx: InstanceIdx, instance: &Instance) {
    add_instance_to_dependencies(&mut self.dependencies, idx, instance);
  }

  pub fn get_highest_or_lowest_minor_specifier(&self, dep: &DependencyCore, arena: &[Instance]) -> Option<Rc<Specifier>> {
    let prefer_highest = match &self.prefer_version {
      Some(PreferVersion::HighestSemver) => true,
      Some(PreferVersion::LowestSemver) => false,
      None => return None,
    };
    let specifiers = dep
      .get_instances(arena)
      .filter(|instance| instance.descriptor.specifier.get_node_version().is_some())
      .map(|instance| {
        let adjusted = instance
          .preferred_semver_range
          .as_ref()
          .and_then(|range| {
            let safe_range = if matches!(range, SemverRange::Exact | SemverRange::Patch) {
              range.clone()
            } else {
              SemverRange::Patch
            };
            instance.descriptor.specifier.with_range(&safe_range)
          })
          .unwrap_or_else(|| Rc::clone(&instance.descriptor.specifier));
        adjusted
      });
    if prefer_highest {
      specifiers.max()
    } else {
      specifiers.min()
    }
  }

  pub fn visit(&self, ctx: &Context, _registry_updates: Option<&RegistryUpdates>) {
    let arena = &ctx.instances;
    for dep in self.dependencies.values() {
      debug!("visit same minor version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);

      // ── Non-semver gate ──
      let any_has_semver = dep
        .instances
        .iter()
        .any(|idx| arena[idx.0].descriptor.specifier.get_node_version().is_some());
      if !any_has_semver {
        debug!("{L2}no instances have a semver version");
        if dep.every_specifier_is_already_identical(arena) {
          debug!("{L3}but all are identical");
          for &idx in &dep.instances {
            let instance = &arena[idx.0];
            let actual_specifier = &instance.descriptor.specifier;
            debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
            instance.mark_valid(ValidInstance::IsNonSemverButIdentical, actual_specifier);
          }
        } else {
          debug!("{L3}and they differ");
          for &idx in &dep.instances {
            let instance = &arena[idx.0];
            let actual_specifier = &instance.descriptor.specifier;
            debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
            instance.mark_unfixable(UnfixableInstance::NonSemverMismatch);
          }
        }
        continue;
      }

      // ── Major-mismatch gate ──
      let all_same_major = dep
        .instances
        .first()
        .is_some_and(|idx| arena[idx.0].already_has_same_major_as_all(&dep.instances, arena));
      if !all_same_major {
        debug!("{L2}instances have differing MAJOR versions");
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
          instance.mark_unfixable(UnfixableInstance::SameMinorHasMajorMismatch);
        }
        continue;
      }

      debug!("{L2}all instances share the same MAJOR");

      // ── Minor-match gate ──
      let all_same_minor = dep
        .instances
        .first()
        .is_some_and(|idx| arena[idx.0].already_has_same_minor_number_as_all(&dep.instances, arena));
      if all_same_minor {
        debug!("{L3}all instances share the same MAJOR.MINOR");
        self.visit_instances_at_correct_minor(dep, arena);
        continue;
      }

      debug!("{L3}instances have differing MAJOR.MINOR");

      // ── preferVersion branching ──
      if self.prefer_version.is_none() {
        debug!("{L4}preferVersion is not set");
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L5}visit instance '{}' ({actual_specifier:?})", instance.id);
          instance.mark_unfixable(UnfixableInstance::SameMinorMismatch);
        }
        continue;
      }

      let fix_target = self.get_highest_or_lowest_minor_specifier(dep, arena);
      if fix_target.is_none() {
        debug!("{L4}could not determine a fix target");
        for &idx in &dep.instances {
          let instance = &arena[idx.0];
          let actual_specifier = &instance.descriptor.specifier;
          debug!("{L5}visit instance '{}' ({actual_specifier:?})", instance.id);
          instance.mark_unfixable(UnfixableInstance::SameMinorMismatch);
        }
        continue;
      }
      let fix_target = fix_target.unwrap();
      let fix_target_version = fix_target.get_node_version().unwrap();
      debug!("{L4}fix target is {fix_target:?}");

      for &idx in &dep.instances {
        let instance = &arena[idx.0];
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L5}visit instance '{}' ({actual_specifier:?})", instance.id);

        let instance_version = match actual_specifier.get_node_version() {
          Some(v) => v,
          None => {
            debug!("{L6}instance has no semver version");
            instance.mark_unfixable(UnfixableInstance::NonSemverMismatch);
            continue;
          }
        };

        let is_at_target_minor = instance_version.major == fix_target_version.major && instance_version.minor == fix_target_version.minor;

        if is_at_target_minor {
          debug!("{L6}instance IS at the target MAJOR.MINOR");
          Self::visit_instance_at_correct_minor(instance, actual_specifier);
        } else {
          debug!("{L6}instance is NOT at the target MAJOR.MINOR");
          let fix_specifier = Self::determine_fix_specifier_for_wrong_minor(instance, actual_specifier, &fix_target);
          debug!("{L7}fix target with range applied: {fix_specifier:?}");
          instance.mark_fixable(FixableInstance::DiffersToHighestOrLowestSemverMinor, &fix_specifier);
        }
      }
    }
  }

  fn visit_instances_at_correct_minor(&self, dep: &DependencyCore, arena: &[Instance]) {
    for &idx in &dep.instances {
      let instance = &arena[idx.0];
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
      Self::visit_instance_at_correct_minor(instance, actual_specifier);
    }
  }

  fn visit_instance_at_correct_minor(instance: &Instance, actual_specifier: &Rc<Specifier>) {
    if instance.must_match_preferred_semver_range() {
      debug!("{L5}it belongs to a semver group");
      let preferred_range = instance.preferred_semver_range.as_ref().unwrap();
      if is_safe_range(preferred_range) {
        debug!("{L6}preferred range is safe ({preferred_range:?})");
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
          debug!("{L6}no semver range on disk");
          instance.mark_valid(ValidInstance::SatisfiesSameMinorGroup, actual_specifier);
        }
      }
    }
  }

  fn determine_fix_specifier_for_wrong_minor(
    instance: &Instance,
    actual_specifier: &Rc<Specifier>,
    fix_target: &Rc<Specifier>,
  ) -> Rc<Specifier> {
    let fix_target_version = fix_target.get_node_version().unwrap();
    if instance.must_match_preferred_semver_range() {
      let preferred_range = instance.preferred_semver_range.as_ref().unwrap();
      debug!("{L8}instance has semver group preferring {preferred_range:?}");
      if is_safe_range(preferred_range) {
        debug!("{L9}preferred range is safe — applying to fix target");
        actual_specifier
          .with_node_version(&fix_target_version)
          .and_then(|s| s.with_range(preferred_range))
          .unwrap_or_else(|| Rc::clone(fix_target))
      } else {
        debug!("{L9}preferred range is unsafe — forcing ~ on fix target");
        actual_specifier
          .with_node_version(&fix_target_version)
          .and_then(|s| s.with_range(&SemverRange::Patch))
          .unwrap_or_else(|| Rc::clone(fix_target))
      }
    } else {
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
}
