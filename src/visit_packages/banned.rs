use {
  super::indent::{L1, L2, L3, L4, L5},
  crate::specifier::Specifier,
  log::debug,
};

pub fn visit(dependency: &crate::dependency::Dependency) {
  debug!("visit banned version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  dependency.instances.borrow().iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
    if instance.is_local {
      debug!("{L3}it is the local instance of a package developed locally in this monorepo");
      debug!("{L4}refuse to change it");
      debug!("{L5}mark as suspect, user should change their config");
      instance.mark_suspect(crate::instance_state::SuspectInstance::RefuseToBanLocal);
    } else {
      debug!("{L3}it should be removed");
      debug!("{L4}mark as error");
      instance.mark_fixable(crate::instance_state::FixableInstance::IsBanned, &Specifier::None);
    }
  });
}
