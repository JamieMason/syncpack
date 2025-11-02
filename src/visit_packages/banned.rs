use {
  super::indent::{L1, L2, L3, L4, L5},
  crate::{
    instance_state::{FixableInstance, SuspectInstance},
    specifier::Specifier,
  },
  log::debug,
};

#[cfg(test)]
#[path = "banned_test.rs"]
mod banned_test;

pub fn visit(dependency: &crate::dependency::Dependency) {
  debug!("visit banned version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  dependency.instances.iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
    if instance.is_local {
      debug!("{L3}it is the local instance of a package developed locally in this monorepo");
      debug!("{L4}refuse to change it");
      debug!("{L5}mark as suspect, user should change their config");
      instance.mark_suspect(SuspectInstance::RefuseToBanLocal);
    } else {
      debug!("{L3}it should be removed");
      debug!("{L4}mark as error");
      instance.mark_fixable(FixableInstance::IsBanned, &Specifier::new(""));
    }
  });
}
