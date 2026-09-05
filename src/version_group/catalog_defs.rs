use {
  crate::{
    group_selector::GroupSelector,
    inspection::{Inspection, RuleInput},
    instance::{FixableInstance, Instance, InstanceState, InvalidInstance, ValidInstance, severity::SeverityMap},
    rcfile::update_group::UpdatePolicy,
    registry::updates::RegistryUpdates,
    semver_range::SemverRange,
    version_group::{DependencyIdx, L1, L2, L3, L4, L5, highest_eligible_for, sort_updates_desc},
  },
  log::debug,
  std::rc::Rc,
};

#[derive(Debug)]
pub struct CatalogDefsGroup {
  pub selector: GroupSelector,
  pub dependencies: Vec<DependencyIdx>,
  pub severity: SeverityMap,
}

impl CatalogDefsGroup {
  pub fn visit(&self, input: &RuleInput, registry_updates: &Option<RegistryUpdates>, out: &mut Inspection) {
    let arena = input.instances;
    let strict = input.cfg.strict;
    for &dep_idx in &self.dependencies {
      let dep = &input.dependencies[dep_idx.0];
      debug!("visit catalog defs version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      let sorted_desc = registry_updates
        .as_ref()
        .and_then(|r| r.updates_by_internal_name.get(&dep.internal_name))
        .map(|updates| sort_updates_desc(updates));
      for &idx in &dep.instances {
        let instance = &arena[idx.0];
        let def_specifier = &instance.descriptor.specifier;
        debug!("{L2}visit instance '{}' ({def_specifier:?})", instance.id);

        let effective_target = match &instance.preferred_update_policy {
          Some(UpdatePolicy::UpTo(t)) => Some(input.cfg.target.stricter(*t)),
          Some(UpdatePolicy::Skip) => None,
          None => Some(input.cfg.target),
        };
        let target = effective_target.and_then(|t| sorted_desc.as_deref().and_then(|s| pick_outdated_target(instance, s, t)));

        if let Some(target) = target {
          debug!("{L3}an eligible registry update applies; mark as DiffersToNpmRegistry ({target:?})");
          out.classify(
            idx,
            InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToNpmRegistry)),
            Some(Rc::clone(&target)),
            &self.severity,
            strict,
          );
          out.set_dep_expected(dep_idx, target);
        } else if instance.must_match_preferred_semver_range() && !instance.matches_preferred_semver_range() {
          if let Some(corrected) = instance.get_specifier_with_preferred_semver_range() {
            debug!("{L3}semver group prefers a different range; mark as SemverRangeMismatch ({corrected:?})");
            out.classify(
              idx,
              InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
              Some(Rc::clone(&corrected)),
              &self.severity,
              strict,
            );
            out.set_dep_expected(dep_idx, corrected);
          } else {
            debug!("{L3}semver group's preferred range cannot be applied to this specifier; mark as catalog definition");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::IsCatalogDefinition),
              Some(Rc::clone(def_specifier)),
              &self.severity,
              strict,
            );
            out.set_dep_expected(dep_idx, Rc::clone(def_specifier));
          }
        } else {
          debug!("{L3}mark as catalog definition");
          out.classify(
            idx,
            InstanceState::Valid(ValidInstance::IsCatalogDefinition),
            Some(Rc::clone(def_specifier)),
            &self.severity,
            strict,
          );
          out.set_dep_expected(dep_idx, Rc::clone(def_specifier));
        }
      }
    }
  }
}

/// Find the highest eligible registry update for a catalog def's actual
/// specifier and apply the def's existing semver range to it. Returns `None`
/// when no update is eligible or the new specifier can't be reconstructed.
fn pick_outdated_target(
  instance: &Instance,
  sorted_desc: &[Rc<crate::specifier::Specifier>],
  target: syncpack_specifier::update_target::UpdateTarget,
) -> Option<Rc<crate::specifier::Specifier>> {
  let actual_specifier = &instance.descriptor.specifier;
  let highest_update = highest_eligible_for(sorted_desc, actual_specifier, &target)?;
  debug!("{L4}an eligible update {highest_update:?} is available");
  let range = instance
    .preferred_semver_range
    .clone()
    .or_else(|| actual_specifier.get_semver_range())
    .unwrap_or(SemverRange::Exact);
  let update_version = highest_update.get_node_version()?;
  let with_updated_version = actual_specifier.with_node_version(&update_version)?;
  let with_preferred_range = with_updated_version.with_range(&range)?;
  debug!("{L5}with semver range applied update becomes {with_preferred_range:?}");
  Some(with_preferred_range)
}
