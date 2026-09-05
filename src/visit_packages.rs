use crate::{
  inspection::{Inspection, RuleInput},
  registry::updates::RegistryUpdates,
  version_group::{VersionGroup, VersionGroupBehavior},
};

/// Two-stage version inspection. Stage 1 classifies every non-`SnappedTo`
/// group's instances into `out`. Stage 2 visits `SnappedTo` groups, which read
/// their targets' stage-1 computed expected values back out of `out`. The
/// stage dependency is encoded in the two passes, not a sort.
pub fn visit_packages(
  version_groups: &[VersionGroup],
  input: &RuleInput,
  registry_updates: &Option<RegistryUpdates>,
  out: &mut Inspection,
) {
  for group in version_groups.iter().filter(|group| !group.is_snapped_to()) {
    group.visit(input, registry_updates, out);
  }
  for group in version_groups.iter().filter(|group| group.is_snapped_to()) {
    group.visit_snapped(input, out);
  }
}
