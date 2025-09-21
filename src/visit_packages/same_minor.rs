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
    if instance.already_has_same_minor_as_all(&dependency.instances) {
      debug!("{L3}its specifier satisfies all other instances in the group");
      if instance.must_match_preferred_semver_range() {
        debug!("{L4}it belongs to a semver group");
        if instance.matches_preferred_semver_range() {
          debug!("{L5}its specifier matches its semver group");
          instance.mark_valid(ValidInstance::SatisfiesSameMinorGroup, actual_specifier);
        } else {
          debug!("{L5}its specifier mismatches its semver group");
          instance.mark_fixable(
            FixableInstance::SemverRangeMismatch,
            &instance.get_specifier_with_preferred_semver_range().unwrap(),
          );
        }
      } else {
        debug!("{L4}it does not belong to a semver group");
        instance.mark_valid(ValidInstance::SatisfiesSameMinorGroup, actual_specifier);
      }
    } else {
      debug!("{L3}its specifier does not satisfy all other instances in the group");
      instance.mark_unfixable(UnfixableInstance::SameMinorMismatch);
    }
  });
}
