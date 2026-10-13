use {
  crate::{
    inspection::{Inspection, RuleInput},
    version_group::{VersionGroup, VersionGroupBehavior},
  },
  syncpack_vocab::registry_updates::RegistryUpdates,
};

/// Two-stage version inspection: stage 1 classifies non-`SnappedTo` groups,
/// then stage 2 resolves `SnappedTo` groups, which read their targets' expected
/// values back out of `out`.
pub fn visit_packages(
  version_groups: &[VersionGroup],
  input: &RuleInput,
  registry_updates: &Option<RegistryUpdates>,
  out: &mut Inspection,
) {
  for group in version_groups.iter().filter(|group| !group.is_snapped_to()) {
    group.visit(input, registry_updates, out);
  }
  // A snap target can itself be a follower, so repeat: a chain of N snap groups
  // resolves in at most N passes, and the fixed bound can't loop on a cycle.
  let snap_groups = version_groups.iter().filter(|group| group.is_snapped_to()).collect::<Vec<_>>();
  for _ in 0..snap_groups.len() {
    for group in &snap_groups {
      group.visit_snapped(input, out);
    }
  }
}
