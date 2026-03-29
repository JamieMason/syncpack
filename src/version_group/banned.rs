use {
  super::{add_instance_to_dependencies, DependencyCore, L1, L2, L3, L4, L5},
  crate::{
    context::Context,
    group_selector::GroupSelector,
    instance::{FixableInstance, Instance, InstanceIdx, SuspectInstance},
    registry::updates::RegistryUpdates,
    specifier::Specifier,
  },
  log::debug,
  std::collections::BTreeMap,
};

#[cfg(test)]
#[path = "banned_test.rs"]
mod banned_test;

#[derive(Debug)]
pub struct BannedGroup {
  pub selector: GroupSelector,
  pub dependencies: BTreeMap<String, DependencyCore>,
}

impl BannedGroup {
  pub fn add_instance(&mut self, idx: InstanceIdx, instance: &Instance) {
    add_instance_to_dependencies(&mut self.dependencies, idx, instance);
  }

  pub fn visit(&self, ctx: &Context, _registry_updates: Option<&RegistryUpdates>) {
    let arena = &ctx.instances;
    for dep in self.dependencies.values() {
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
          instance.mark_suspect(SuspectInstance::RefuseToBanLocal);
        } else {
          debug!("{L3}it should be removed");
          debug!("{L4}mark as error");
          instance.mark_fixable(FixableInstance::IsBanned, &Specifier::new(""));
        }
      }
    }
  }
}
