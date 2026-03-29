use {
  super::{add_instance_to_dependencies, DependencyCore, L1, L2},
  crate::{
    context::Context,
    group_selector::GroupSelector,
    instance::{Instance, InstanceIdx, ValidInstance},
    registry::updates::RegistryUpdates,
  },
  log::debug,
  std::collections::BTreeMap,
};

#[cfg(test)]
#[path = "ignored_test.rs"]
mod ignored_test;

#[derive(Debug)]
pub struct IgnoredGroup {
  pub selector: GroupSelector,
  pub dependencies: BTreeMap<String, DependencyCore>,
}

impl IgnoredGroup {
  pub fn add_instance(&mut self, idx: InstanceIdx, instance: &Instance) {
    add_instance_to_dependencies(&mut self.dependencies, idx, instance);
  }

  pub fn visit(&self, ctx: &Context, _registry_updates: Option<&RegistryUpdates>) {
    let arena = &ctx.instances;
    for dep in self.dependencies.values() {
      debug!("visit ignored version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      for &idx in &dep.instances {
        let instance = &arena[idx.0];
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
        instance.mark_valid(ValidInstance::IsIgnored, &instance.descriptor.specifier);
      }
    }
  }
}
