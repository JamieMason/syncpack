use {
  super::{L1, L2, L3, L4, L5},
  crate::{
    cli::SortBy,
    context::Context,
    format,
    package_json::{FormatMismatch, FormatMismatchVariant::*, PackageJson},
    specifier::semver_range::SemverRange,
    version_group::VersionGroupVariant,
  },
  itertools::Itertools,
  log::debug,
  std::{cell::RefCell, cmp::Ordering, rc::Rc},
};

pub fn visit_ignored(dependency: &crate::dependency::Dependency) {
  debug!("visit ignored version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  dependency.instances.borrow().iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
    instance.mark_valid(crate::instance_state::ValidInstance::IsIgnored, &instance.descriptor.specifier);
  });
}
