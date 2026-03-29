use {
  super::{add_instance_to_dependencies, DependencyCore, L1, L2, L3, L4, L5, L6, L7, L8},
  crate::{
    context::Context,
    group_selector::GroupSelector,
    instance::{FixableInstance, Instance, InstanceIdx, SuspectInstance, ValidInstance},
    registry::updates::RegistryUpdates,
    specifier::Specifier,
  },
  log::debug,
  std::{collections::BTreeMap, rc::Rc},
};

#[cfg(test)]
#[path = "pinned_test.rs"]
mod pinned_test;

#[derive(Debug)]
pub struct PinnedGroup {
  pub selector: GroupSelector,
  pub dependencies: BTreeMap<String, DependencyCore>,
  pub pin_version: Rc<Specifier>,
}

impl PinnedGroup {
  pub fn add_instance(&mut self, idx: InstanceIdx, instance: &Instance) {
    add_instance_to_dependencies(&mut self.dependencies, idx, instance);
  }

  pub fn visit(&self, ctx: &Context, _registry_updates: Option<&RegistryUpdates>) {
    let arena = &ctx.instances;
    let pinned_specifier = &self.pin_version;
    for dep in self.dependencies.values() {
      debug!("visit pinned version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      dep.set_expected_specifier(pinned_specifier);
      for &idx in &dep.instances {
        let instance = &arena[idx.0];
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
        if instance.is_local_instance {
          debug!("{L3}it is the local instance of a package developed locally in this monorepo");
          debug!("{L4}refuse to change it");
          debug!("{L5}mark as error, user should change their config");
          instance.mark_suspect(SuspectInstance::RefuseToPinLocal);
          continue;
        }
        if instance.already_equals(pinned_specifier) {
          debug!("{L3}it is identical to the pinned version");
          debug!("{L4}mark as valid");
          instance.mark_valid(ValidInstance::IsIdenticalToPin, pinned_specifier);
          continue;
        }
        debug!("{L3}it depends on the local instance");
        debug!("{L4}its version number (without a range):");
        if !instance.descriptor.specifier.has_same_version_number_as(pinned_specifier) {
          debug!("{L5}differs to the pinned version");
          debug!("{L6}mark as error");
          instance.mark_fixable(FixableInstance::DiffersToPin, pinned_specifier);
          continue;
        }
        debug!("{L5}is the same as the pinned version");
        if instance.must_match_preferred_semver_range_which_differs_to(pinned_specifier) {
          let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
          debug!("{L6}it is in a semver group which prefers a different semver range to the pinned version ({preferred_semver_range:?})");
          if instance.matches_preferred_semver_range() {
            debug!("{L7}its semver range matches its semver group");
            debug!("{L8}1. pin it and ignore the semver group");
            debug!("{L8}2. mark as suspect (the config is asking for a different range AND they want to pin it)");
            instance.mark_fixable(FixableInstance::PinOverridesSemverRange, pinned_specifier);
          } else {
            debug!("{L7}its semver range does not match its semver group or the pinned version's");
            debug!("{L8}1. pin it and ignore the semver group");
            debug!("{L8}2. mark as suspect (the config is asking for a different range AND they want to pin it)");
            instance.mark_fixable(FixableInstance::PinOverridesSemverRangeMismatch, pinned_specifier);
          }
          continue;
        }
        debug!("{L6}it is not in a semver group which prefers a different semver range to the pinned version");
        debug!("{L7}it differs to the pinned version");
        debug!("{L8}mark as error");
        instance.mark_fixable(FixableInstance::DiffersToPin, pinned_specifier);
      }
    }
  }
}
