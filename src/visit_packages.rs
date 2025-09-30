use {
  crate::{context::Context, version_group::VersionGroupVariant},
  itertools::Itertools,
  std::cmp::Ordering,
};

mod banned;
mod ignored;
mod indent;
mod pinned;
mod preferred_semver;
mod same_minor;
mod same_range;
mod snapped_to;

#[cfg(test)]
#[ctor::ctor]
fn init() {
  use crate::{logger, test::mock};
  logger::init(&mock::cli());
}

/// Phase 2 of the 3-phase pipeline: Inspect Context (assign InstanceState).
///
/// This function iterates through all version groups and their dependencies,
/// delegating to specific visitor modules to assign InstanceState to each
/// instance based on the version group's policy.
///
/// This is where validation happens - each visitor examines instances and tags
/// them as Valid, Invalid (Fixable/Unfixable/Conflict), or Suspect.
///
/// Important: This function only TAGS instances - it never modifies
/// package.json files. File modifications happen in Phase 3 (commands).
///
/// Takes ownership of Context and returns it with states assigned.
///
/// Called from: src/main.rs (after Context::create)
/// Next step: Command functions in src/commands/*.rs
/// See also: Each visitor module (banned.rs, pinned.rs, etc.)
pub fn visit_packages(ctx: Context) -> Context {
  ctx.version_groups.iter().sorted_by(order_snapped_to_groups_last).for_each(|group| {
    group.dependencies.values().for_each(|dependency| match dependency.variant {
      VersionGroupVariant::Banned => banned::visit(dependency),
      VersionGroupVariant::HighestSemver | VersionGroupVariant::LowestSemver => preferred_semver::visit(dependency, &ctx),
      VersionGroupVariant::Ignored => ignored::visit(dependency),
      VersionGroupVariant::Pinned => pinned::visit(dependency),
      VersionGroupVariant::SameRange => same_range::visit(dependency),
      VersionGroupVariant::SameMinor => same_minor::visit(dependency),
      VersionGroupVariant::SnappedTo => snapped_to::visit(dependency, &ctx),
    });
  });
  ctx
}

/// Ensure that packages a snapped to group is snapped to has their fixes
/// applied to them first.
///
/// SnappedTo groups depend on other packages' versions, so those packages
/// must be validated first. This ordering ensures the target packages have
/// their correct versions assigned before SnappedTo groups try to match them.
fn order_snapped_to_groups_last(a: &&crate::version_group::VersionGroup, b: &&crate::version_group::VersionGroup) -> Ordering {
  if matches!(a.variant, VersionGroupVariant::SnappedTo) {
    Ordering::Greater
  } else if matches!(b.variant, VersionGroupVariant::SnappedTo) {
    Ordering::Less
  } else {
    Ordering::Equal
  }
}
