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
mod same_range;
mod snapped_to;

#[cfg(test)]
#[ctor::ctor]
fn init() {
  use crate::{logger, test::mock};
  logger::init(&mock::cli());
}

pub fn visit_packages(ctx: Context) -> Context {
  ctx.version_groups.iter().sorted_by(order_snapped_to_groups_last).for_each(|group| {
    group.dependencies.values().for_each(|dependency| match dependency.variant {
      VersionGroupVariant::Banned => banned::visit(dependency),
      VersionGroupVariant::HighestSemver | VersionGroupVariant::LowestSemver => preferred_semver::visit(dependency, &ctx),
      VersionGroupVariant::Ignored => ignored::visit(dependency),
      VersionGroupVariant::Pinned => pinned::visit(dependency),
      VersionGroupVariant::SameRange => same_range::visit(dependency),
      VersionGroupVariant::SnappedTo => snapped_to::visit(dependency, &ctx),
    });
  });
  ctx
}

/// Ensure that packages a snapped to group is snapped to has their fixes
/// applied to them first
fn order_snapped_to_groups_last(a: &&crate::version_group::VersionGroup, b: &&crate::version_group::VersionGroup) -> Ordering {
  if matches!(a.variant, VersionGroupVariant::SnappedTo) {
    Ordering::Greater
  } else if matches!(b.variant, VersionGroupVariant::SnappedTo) {
    Ordering::Less
  } else {
    Ordering::Equal
  }
}
