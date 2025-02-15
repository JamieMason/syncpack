#[cfg(test)]
#[path = "visit_packages_test.rs"]
mod visit_packages_test;

use {
  crate::{cli::SortBy, context::Context, version_group::VersionGroupVariant},
  itertools::Itertools,
  log::debug,
  std::cmp::Ordering,
};

mod banned;
mod formatting;
mod ignored;
mod indent;
mod pinned;
mod preferred_semver;
mod same_range;
mod snapped_to;

pub fn visit_packages(ctx: Context) -> Context {
  if ctx.config.cli.inspect_mismatches {
    debug!("visit versions");

    ctx
      .version_groups
      .iter()
      // @TODO: can moving snapped to groups last be done when reading config at
      // the start?
      //
      // fix snapped to groups last, so that the packages they're snapped to
      // have any fixes applied to them first
      .sorted_by(|a, b| {
        if matches!(a.variant, VersionGroupVariant::SnappedTo) {
          Ordering::Greater
        } else if matches!(b.variant, VersionGroupVariant::SnappedTo) {
          Ordering::Less
        } else {
          Ordering::Equal
        }
      })
      .for_each(|group| {
        group.for_each_dependency(&SortBy::Name, |dependency| match dependency.variant {
          VersionGroupVariant::Banned => banned::visit(dependency),
          VersionGroupVariant::HighestSemver | VersionGroupVariant::LowestSemver => preferred_semver::visit(dependency, &ctx),
          VersionGroupVariant::Ignored => ignored::visit(dependency),
          VersionGroupVariant::Pinned => pinned::visit(dependency),
          VersionGroupVariant::SameRange => same_range::visit(dependency),
          VersionGroupVariant::SnappedTo => snapped_to::visit(dependency, &ctx),
        });
      });
  }

  if ctx.config.cli.inspect_formatting {
    formatting::visit(&ctx);
  }

  ctx
}
