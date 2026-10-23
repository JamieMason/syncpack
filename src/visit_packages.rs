use crate::{
  context::Context,
  instance::{InstanceIdx, InstanceState, InvalidInstance, UnfixableInstance},
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
  // A mismatched snap cycle has no source of truth. Freeze its members before anything
  // follows them, so packages which snap into the cycle see each member's current version
  // whatever order the groups are listed in.
  let cycle_followers = mark_circular_snap_mismatches(&ctx);
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
  // Those followers were classified against a frozen member's current version. One which
  // would be rewritten to match it is refused instead.
  for idx in cycle_followers {
    let instance = &ctx.instances[idx];
    if matches!(*instance.state.borrow(), InstanceState::Invalid(InvalidInstance::Fixable(_))) {
      instance.mark_unfixable(UnfixableInstance::DependsOnCircularSnapToMismatch);
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

/// Mark the members of every snap cycle whose version numbers disagree as
/// `CircularSnapToMismatch`; a cycle that agrees is left to normal classification.
/// Returns the instances which snap into a mismatched cycle without being part of it.
fn mark_circular_snap_mismatches(ctx: &Context) -> Vec<usize> {
  let count = ctx.instances.len();
  // One out-edge per follower: a functional graph, so cycles are simple.
  let mut target_of: Vec<Option<InstanceIdx>> = vec![None; count];
  for group in &ctx.version_groups {
    if let VersionGroup::SnappedTo(g) = group {
      for (follower, target) in g.snap_edges(&ctx.instances) {
        target_of[follower.0] = Some(target);
      }
    }
  }
  let mut seen = vec![0u8; count]; // 0 = unvisited, 1 = on current path, 2 = settled
  let mut reaches_mismatch = vec![false; count]; // in, or leads into, a mismatched cycle
  let mut in_cycle = vec![false; count];
  for start in 0..count {
    if target_of[start].is_none() || seen[start] != 0 {
      continue;
    }
    let mut path = Vec::new();
    let mut cur = start;
    let ends_in_mismatch = loop {
      match seen[cur] {
        1 => {
          // re-entered the current path: cycle runs from cur's first visit
          let at = path.iter().position(|&node| node == cur).unwrap();
          for &member in &path[at..] {
            in_cycle[member] = true;
          }
          break mark_cycle(&path[at..], ctx);
        }
        2 => break reaches_mismatch[cur],
        _ => {
          seen[cur] = 1;
          path.push(cur);
          match target_of[cur] {
            Some(next) => cur = next.0,
            None => break false,
          }
        }
      }
    };
    for &node in &path {
      seen[node] = 2;
      reaches_mismatch[node] = ends_in_mismatch;
    }
  }
  (0..count).filter(|&node| reaches_mismatch[node] && !in_cycle[node]).collect()
}

/// Flag all cycle members `CircularSnapToMismatch` when their versions disagree.
/// Returns whether they did.
fn mark_cycle(members: &[usize], ctx: &Context) -> bool {
  let first = &ctx.instances[members[0]].descriptor.specifier;
  if members
    .iter()
    .all(|&m| ctx.instances[m].descriptor.specifier.has_same_version_number_as(first))
  {
    return false;
  }
  for &m in members {
    ctx.instances[m].mark_unfixable(UnfixableInstance::CircularSnapToMismatch);
  }
  true
}
