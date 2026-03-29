use {
  super::{add_instance_to_dependencies, DependencyCore, L1, L2, L3, L4, L5},
  crate::{
    context::Context,
    group_selector::GroupSelector,
    instance::{FixableInstance, Instance, InstanceIdx, UnfixableInstance, ValidInstance},
    registry::updates::RegistryUpdates,
  },
  log::debug,
  std::collections::BTreeMap,
};

#[cfg(test)]
#[path = "same_range_test.rs"]
mod same_range_test;

#[derive(Debug)]
pub struct SameRangeGroup {
  pub selector: GroupSelector,
  pub dependencies: BTreeMap<String, DependencyCore>,
}

impl SameRangeGroup {
  pub fn add_instance(&mut self, idx: InstanceIdx, instance: &Instance) {
    add_instance_to_dependencies(&mut self.dependencies, idx, instance);
  }

  pub fn visit(&self, ctx: &Context, _registry_updates: Option<&RegistryUpdates>) {
    let arena = &ctx.instances;
    for dep in self.dependencies.values() {
      debug!("visit same range version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      for &idx in &dep.instances {
        let instance = &arena[idx.0];
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
        if instance.already_satisfies_all(&dep.instances, arena) {
          debug!("{L3}its specifier satisfies all other instances in the group");
          if instance.must_match_preferred_semver_range() {
            debug!("{L4}it belongs to a semver group");
            if instance.matches_preferred_semver_range() {
              debug!("{L5}its specifier matches its semver group");
              instance.mark_valid(ValidInstance::SatisfiesSameRangeGroup, actual_specifier);
            } else {
              debug!("{L5}its specifier mismatches its semver group");
              instance.mark_fixable(
                FixableInstance::SemverRangeMismatch,
                &instance.get_specifier_with_preferred_semver_range().unwrap(),
              );
            }
          } else {
            debug!("{L4}it does not belong to a semver group");
            instance.mark_valid(ValidInstance::SatisfiesSameRangeGroup, actual_specifier);
          }
        } else {
          debug!("{L3}its specifier does not satisfy all other instances in the group");
          instance.mark_unfixable(UnfixableInstance::SameRangeMismatch);
        }
      }
    }
  }
}
