use {
  super::indent::{L1, L2},
  crate::instance_state::ValidInstance,
  log::debug,
};

#[cfg(test)]
#[path = "ignored_test.rs"]
mod ignored_test;

pub fn visit(dependency: &crate::dependency::Dependency) {
  debug!("visit ignored version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  dependency.instances.iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
    instance.mark_valid(ValidInstance::IsIgnored, &instance.descriptor.specifier);
  });
}
