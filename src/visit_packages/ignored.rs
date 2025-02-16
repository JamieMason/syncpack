use {
  super::indent::{L1, L2},
  log::debug,
};

pub fn visit(dependency: &crate::dependency::Dependency) {
  debug!("visit ignored version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  dependency.instances.borrow().iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
    instance.mark_valid(crate::instance_state::ValidInstance::IsIgnored, &instance.descriptor.specifier);
  });
}
