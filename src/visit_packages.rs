use {
  crate::{
    context::Context,
    registry::updates::RegistryUpdates,
    version_group::{VersionGroup, VersionGroupBehavior},
  },
  itertools::Itertools,
  std::cmp::Ordering,
};

/// Iterate version groups (SnappedTo last) and assign `InstanceState` to every
/// instance via each group's `visit()`. Takes ownership of `Context` and
/// returns it with states assigned.
pub fn visit_packages(ctx: Context, registry_updates: &Option<RegistryUpdates>) -> Context {
  ctx
    .version_groups
    .iter()
    // SnappedTo groups depend on other groups' visit results, so they must be
    // visited last.
    .sorted_by(|a: &&VersionGroup, b: &&VersionGroup| -> Ordering {
      if matches!(a, VersionGroup::SnappedTo(_)) {
        Ordering::Greater
      } else if matches!(b, VersionGroup::SnappedTo(_)) {
        Ordering::Less
      } else {
        Ordering::Equal
      }
    })
    .for_each(|group| {
      group.visit(&ctx, registry_updates);
    });
  ctx
}
