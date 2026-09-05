use {
  crate::{
    inspection::{Inspection, RuleInput},
    version_group::{L1, L2},
  },
  log::debug,
  std::rc::Rc,
  syncpack_model::{
    dependency::DependencyIdx,
    group_selector::GroupSelector,
    instance::{InstanceState, ValidInstance},
  },
  syncpack_vocab::{registry_updates::RegistryUpdates, severity::SeverityMap},
};

#[cfg(test)]
#[path = "ignored_test.rs"]
mod ignored_test;

#[derive(Debug)]
pub struct IgnoredGroup {
  pub selector: GroupSelector,
  pub dependencies: Vec<DependencyIdx>,
  pub severity: SeverityMap,
}

impl IgnoredGroup {
  pub fn visit(&self, input: &RuleInput, _registry_updates: &Option<RegistryUpdates>, out: &mut Inspection) {
    let arena = input.instances;
    let strict = input.cfg.strict;
    for &dep_idx in &self.dependencies {
      let dep = &input.dependencies[dep_idx.0];
      debug!("visit ignored version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      for &idx in &dep.instances {
        let instance = &arena[idx.0];
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
        out.classify(
          idx,
          InstanceState::Valid(ValidInstance::IsIgnored),
          Some(Rc::clone(actual_specifier)),
          &self.severity,
          strict,
        );
      }
    }
  }
}
