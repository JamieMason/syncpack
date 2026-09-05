use {
  crate::{
    inspection::{Inspection, RuleInput},
    version_group::{L1, L2, L3, L4, L5},
  },
  log::debug,
  std::rc::Rc,
  syncpack_model::{
    dependency::DependencyIdx,
    group_selector::GroupSelector,
    instance::{FixableInstance, InstanceState, InvalidInstance, SuspectInstance},
  },
  syncpack_specifier::Specifier,
  syncpack_vocab::{registry_updates::RegistryUpdates, severity::SeverityMap},
};

#[derive(Debug)]
pub struct BannedGroup {
  pub selector: GroupSelector,
  pub dependencies: Vec<DependencyIdx>,
  pub severity: SeverityMap,
}

impl BannedGroup {
  pub fn visit(&self, input: &RuleInput, _registry_updates: &Option<RegistryUpdates>, out: &mut Inspection) {
    let arena = input.instances;
    let strict = input.cfg.strict;
    for &dep_idx in &self.dependencies {
      let dep = &input.dependencies[dep_idx.0];
      debug!("visit banned version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      for &idx in &dep.instances {
        let instance = &arena[idx.0];
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
        if instance.is_local_instance {
          debug!("{L3}it is the local instance of a package developed locally in this monorepo");
          debug!("{L4}refuse to change it");
          debug!("{L5}mark as suspect, user should change their config");
          out.classify(
            idx,
            InstanceState::Suspect(SuspectInstance::RefuseToBanLocal),
            Some(Rc::clone(actual_specifier)),
            &self.severity,
            strict,
          );
        } else {
          debug!("{L3}it should be removed");
          debug!("{L4}mark as error");
          out.classify(
            idx,
            InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::IsBanned)),
            Some(Specifier::new("")),
            &self.severity,
            strict,
          );
        }
      }
    }
  }
}
