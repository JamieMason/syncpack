use {
  super::{DependencyCore, L1, L2, L3, L4, add_instance_to_dependencies, highest_eligible_for, sort_updates_desc},
  crate::{
    context::Context,
    group_selector::GroupSelector,
    instance::{FixableInstance, Instance, InstanceIdx, ValidInstance, severity::SeverityMap},
    rcfile::update_group::UpdatePolicy,
    registry::updates::RegistryUpdates,
    semver_range::SemverRange,
  },
  log::debug,
  std::collections::BTreeMap,
};

#[cfg(test)]
#[path = "semver_range_only_test.rs"]
mod semver_range_only_test;

#[derive(Debug)]
pub struct SemverRangeOnlyGroup {
  pub selector: GroupSelector,
  pub dependencies: BTreeMap<String, DependencyCore>,
  pub severity: SeverityMap,
}

impl SemverRangeOnlyGroup {
  pub fn add_instance(&mut self, idx: InstanceIdx, instance: &Instance) {
    add_instance_to_dependencies(&mut self.dependencies, idx, instance);
  }

  pub fn visit(&self, ctx: &Context, registry_updates: &Option<RegistryUpdates>) {
    let arena = &ctx.instances;
    for dep in self.dependencies.values() {
      debug!("visit semver range only version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      let sorted_desc = registry_updates
        .as_ref()
        .and_then(|r| r.updates_by_internal_name.get(&dep.internal_name))
        .map(|u| sort_updates_desc(u));
      for &idx in &dep.instances {
        let instance = &arena[idx.0];
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);

        if !instance.is_local_instance
          && let Some(sorted_desc) = &sorted_desc
        {
          let effective = match &instance.preferred_update_policy {
            Some(UpdatePolicy::Skip) => {
              debug!("{L3}updateGroup policy is Skip; mark as ignored");
              instance.mark_valid(ValidInstance::IsIgnored, &instance.descriptor.specifier);
              continue;
            }
            Some(UpdatePolicy::UpTo(t)) => ctx.config.cli.target.stricter(*t),
            None => ctx.config.cli.target,
          };
          if let Some(highest_update) = highest_eligible_for(sorted_desc, actual_specifier, &effective) {
            debug!("{L3}an eligible update {highest_update:?} is available");
            let range = &instance
              .preferred_semver_range
              .clone()
              .or_else(|| actual_specifier.get_semver_range())
              .unwrap_or(SemverRange::Exact);
            if let Some(update_version) = highest_update.get_node_version()
              && let Some(with_updated_version) = actual_specifier.with_node_version(&update_version)
              && let Some(with_preferred_range) = with_updated_version.with_range(range)
            {
              debug!("{L4}with semver group applied update becomes {with_preferred_range:?}");
              instance.mark_fixable(FixableInstance::DiffersToNpmRegistry, &with_preferred_range);
              continue;
            }
          }
        }

        if instance.must_match_preferred_semver_range()
          && !instance.matches_preferred_semver_range()
          && actual_specifier.get_node_version().is_some()
          && let Some(corrected) = instance.get_specifier_with_preferred_semver_range()
        {
          debug!("{L3}semver group prefers a different range; mark as SemverRangeMismatch ({corrected:?})");
          instance.mark_fixable(FixableInstance::SemverRangeMismatch, &corrected);
          continue;
        }

        if instance.is_local_instance {
          debug!("{L3}local instance is valid");
          instance.mark_valid(ValidInstance::IsLocalAndValid, actual_specifier);
        } else {
          debug!("{L3}matches semver group");
          instance.mark_valid(ValidInstance::MatchesSemverGroup, actual_specifier);
        }
      }
    }
  }
}
