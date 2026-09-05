use {
  crate::{
    group_selector::GroupSelector,
    inspection::{Inspection, RuleInput},
    instance::{FixableInstance, InstanceState, InvalidInstance, UnfixableInstance, ValidInstance, severity::SeverityMap},
    registry::updates::RegistryUpdates,
    version_group::{DependencyIdx, L1, L2, L3, L4, L5},
  },
  log::debug,
  std::rc::Rc,
};

#[cfg(test)]
#[path = "same_range_test.rs"]
mod same_range_test;

#[derive(Debug)]
pub struct SameRangeGroup {
  pub selector: GroupSelector,
  pub dependencies: Vec<DependencyIdx>,
  pub severity: SeverityMap,
}

impl SameRangeGroup {
  pub fn visit(&self, input: &RuleInput, _registry_updates: &Option<RegistryUpdates>, out: &mut Inspection) {
    let arena = input.instances;
    let strict = input.cfg.strict;
    for &dep_idx in &self.dependencies {
      let dep = &input.dependencies[dep_idx.0];
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
              out.classify(
                idx,
                InstanceState::Valid(ValidInstance::SatisfiesSameRangeGroup),
                Some(Rc::clone(actual_specifier)),
                &self.severity,
                strict,
              );
            } else {
              debug!("{L5}its specifier mismatches its semver group");
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
                Some(instance.get_specifier_with_preferred_semver_range().unwrap()),
                &self.severity,
                strict,
              );
            }
          } else {
            debug!("{L4}it does not belong to a semver group");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::SatisfiesSameRangeGroup),
              Some(Rc::clone(actual_specifier)),
              &self.severity,
              strict,
            );
          }
        } else {
          debug!("{L3}its specifier does not satisfy all other instances in the group");
          out.classify(
            idx,
            InstanceState::Invalid(InvalidInstance::Unfixable(UnfixableInstance::SameRangeMismatch)),
            Some(Rc::clone(actual_specifier)),
            &self.severity,
            strict,
          );
        }
      }
    }
  }
}
