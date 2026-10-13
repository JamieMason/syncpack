use {
  crate::{
    inspection::{Inspection, RuleInput},
    version_group::{VersionGroup, VersionGroupBehavior},
  },
  std::rc::Rc,
  syncpack_model::instance::{InstanceIdx, InstanceState, InvalidInstance, UnfixableInstance},
  syncpack_vocab::{registry_updates::RegistryUpdates, severity::SeverityMap},
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
  mark_circular_snap_mismatches(version_groups, input, out);
}

/// Snap cycle members whose version numbers disagree are unfixable; a cycle
/// that agrees is left to normal classification. Runs last so its verdict wins.
fn mark_circular_snap_mismatches(version_groups: &[VersionGroup], input: &RuleInput, out: &mut Inspection) {
  let count = input.instances.len();
  // One out-edge per follower: a functional graph, so cycles are simple.
  let mut target_of: Vec<Option<InstanceIdx>> = vec![None; count];
  let mut severity_of: Vec<Option<&SeverityMap>> = vec![None; count];
  for group in version_groups {
    if let VersionGroup::SnappedTo(g) = group {
      for (follower, target) in g.snap_edges(input.dependencies, input.instances) {
        target_of[follower.0] = Some(target);
        severity_of[follower.0] = Some(&g.severity);
      }
    }
  }
  let mut seen = vec![0u8; count]; // 0 = unvisited, 1 = on current path, 2 = settled
  for start in 0..count {
    if target_of[start].is_none() || seen[start] != 0 {
      continue;
    }
    let mut path = Vec::new();
    let mut cur = start;
    loop {
      match seen[cur] {
        1 => {
          // re-entered the current path: cycle runs from cur's first visit
          let at = path.iter().position(|&node| node == cur).unwrap();
          mark_cycle(&path[at..], input, &severity_of, out);
          break;
        }
        2 => break,
        _ => {
          seen[cur] = 1;
          path.push(cur);
          match target_of[cur] {
            Some(next) => cur = next.0,
            None => break,
          }
        }
      }
    }
    for &node in &path {
      seen[node] = 2;
    }
  }
}

/// Flag all cycle members `CircularSnapToMismatch` when their versions disagree.
fn mark_cycle(members: &[usize], input: &RuleInput, severity_of: &[Option<&SeverityMap>], out: &mut Inspection) {
  let first = input.instances[members[0]].descriptor.specifier.as_ref();
  if members
    .iter()
    .all(|&m| input.instances[m].descriptor.specifier.has_same_version_number_as(first))
  {
    return;
  }
  for &m in members {
    out.classify(
      InstanceIdx(m),
      InstanceState::Invalid(InvalidInstance::Unfixable(UnfixableInstance::CircularSnapToMismatch)),
      Some(Rc::clone(&input.instances[m].descriptor.specifier)),
      severity_of[m].expect("a cycle member is a snap follower with a recorded severity map"),
      input.cfg.strict,
    );
  }
}
