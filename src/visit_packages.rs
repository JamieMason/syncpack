use crate::{
  context::Context,
  registry::updates::RegistryUpdates,
  version_group::{VersionGroup, VersionGroupBehavior},
};

/// Assign `InstanceState` to every instance via each group's `visit()`.
/// SnappedTo groups read their targets' expected specifiers, so they are
/// visited after every other group. Then call `resolve_action` for every
/// instance against its claiming group so `instance.severity` is populated and
/// reporters / JSON output / test assertions can read it directly without
/// re-invoking the resolver. Takes ownership of `Context` and returns it.
pub fn visit_packages(ctx: Context, registry_updates: &Option<RegistryUpdates>) -> Context {
  let (snap_groups, other_groups): (Vec<_>, Vec<_>) = ctx
    .version_groups
    .iter()
    .partition(|group| matches!(group, VersionGroup::SnappedTo(_)));
  for group in &other_groups {
    group.visit(&ctx, registry_updates);
  }
  // A snap target can itself be a follower, so repeat until a pass changes no
  // expected specifier: a chain of N snap groups settles in at most N passes,
  // and the fixed bound can't loop on a cycle.
  for _ in 0..snap_groups.len() {
    let before = ctx
      .instances
      .iter()
      .map(|instance| instance.expected_specifier.borrow().clone())
      .collect::<Vec<_>>();
    for group in &snap_groups {
      group.visit(&ctx, registry_updates);
    }
    let settled =
      ctx.instances.iter().zip(&before).all(|(instance, before)| {
        instance.expected_specifier.borrow().as_ref().map(|s| s.get_raw()) == before.as_ref().map(|s| s.get_raw())
      });
    if settled {
      break;
    }
  }

  let strict = ctx.config.rcfile.strict;
  for group in ctx.version_groups.iter() {
    for dep in group.dependencies().values() {
      for &idx in &dep.instances {
        let instance = &ctx.instances[idx.0];
        group.resolve_action(instance, strict);
      }
    }
  }
  ctx
}
