use {
  crate::{
    inspection::{Inspection, RuleInput},
    version_group::{L1, L2, L3, L4, highest_eligible_for, sort_updates_desc},
  },
  log::debug,
  std::rc::Rc,
  syncpack_model::{
    dependency::DependencyIdx,
    group_selector::GroupSelector,
    instance::{FixableInstance, InstanceState, InvalidInstance, ValidInstance},
  },
  syncpack_specifier::semver_range::SemverRange,
  syncpack_vocab::{registry_updates::RegistryUpdates, severity::SeverityMap, update_policy::UpdatePolicy},
};

#[derive(Debug)]
pub struct SemverRangeOnlyGroup {
  pub selector: GroupSelector,
  pub dependencies: Vec<DependencyIdx>,
  pub severity: SeverityMap,
}

impl SemverRangeOnlyGroup {
  pub fn visit(&self, input: &RuleInput, registry_updates: &Option<RegistryUpdates>, out: &mut Inspection) {
    let arena = input.instances;
    let strict = input.cfg.strict;
    for &dep_idx in &self.dependencies {
      let dep = &input.dependencies[dep_idx.0];
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
              out.classify(
                idx,
                InstanceState::Valid(ValidInstance::IsIgnored),
                Some(Rc::clone(actual_specifier)),
                &self.severity,
                strict,
              );
              continue;
            }
            Some(UpdatePolicy::UpTo(t)) => input.cfg.target.stricter(*t),
            None => input.cfg.target,
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
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToNpmRegistry)),
                Some(with_preferred_range),
                &self.severity,
                strict,
              );
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
          out.classify(
            idx,
            InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch)),
            Some(corrected),
            &self.severity,
            strict,
          );
          continue;
        }

        if instance.is_local_instance {
          debug!("{L3}local instance is valid");
          out.classify(
            idx,
            InstanceState::Valid(ValidInstance::IsLocalAndValid),
            Some(Rc::clone(actual_specifier)),
            &self.severity,
            strict,
          );
        } else {
          debug!("{L3}matches semver group");
          out.classify(
            idx,
            InstanceState::Valid(ValidInstance::MatchesSemverGroup),
            Some(Rc::clone(actual_specifier)),
            &self.severity,
            strict,
          );
        }
      }
    }
  }
}
