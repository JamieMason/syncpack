#[cfg(test)]
#[path = "visit_packages_test.rs"]
mod visit_packages_test;

use {
  crate::{
    cli::SortBy,
    context::Context,
    format,
    package_json::{FormatMismatch, FormatMismatchVariant::*, PackageJson},
    version_group::VersionGroupVariant,
  },
  itertools::Itertools,
  log::debug,
  std::{cell::RefCell, cmp::Ordering, rc::Rc},
};

mod banned;
mod ignored;
mod pinned;
mod preferred_semver;
mod same_range;
mod snapped_to;

pub const L1: &str = "  ";
pub const L2: &str = "    ";
pub const L3: &str = "      ";
pub const L4: &str = "        ";
pub const L5: &str = "          ";
pub const L6: &str = "            ";
pub const L7: &str = "              ";
pub const L8: &str = "                ";
pub const L9: &str = "                  ";
pub const L10: &str = "                    ";

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
          VersionGroupVariant::Banned => banned::visit_banned(dependency),
          VersionGroupVariant::HighestSemver | VersionGroupVariant::LowestSemver => {
            preferred_semver::visit_preferred_semver(dependency, &ctx)
          }
          VersionGroupVariant::Ignored => ignored::visit_ignored(dependency),
          VersionGroupVariant::Pinned => pinned::visit_pinned(dependency),
          VersionGroupVariant::SameRange => same_range::visit_same_range(dependency),
          VersionGroupVariant::SnappedTo => snapped_to::visit_snapped_to(dependency, &ctx),
        });
      });
  }

  if ctx.config.cli.inspect_formatting {
    let add_mismatch = |package: &Rc<RefCell<PackageJson>>, mismatch: FormatMismatch| {
      let mismatch = Rc::new(mismatch);
      package.borrow().formatting_mismatches.borrow_mut().push(Rc::clone(&mismatch));
    };

    ctx.packages.all.iter().for_each(|package| {
      if ctx.config.rcfile.sort_packages || !ctx.config.rcfile.sort_first.is_empty() {
        if let Some(expected) = format::get_sorted_first(&ctx.config.rcfile, &package.borrow()) {
          add_mismatch(
            package,
            FormatMismatch {
              expected,
              package: Rc::clone(package),
              property_path: "/".to_string(),
              variant: PackagePropertiesAreNotSorted,
            },
          );
        }
      }
      if ctx.config.rcfile.format_bugs {
        if let Some(expected) = format::get_formatted_bugs(&package.borrow()) {
          add_mismatch(
            package,
            FormatMismatch {
              expected,
              package: Rc::clone(package),
              property_path: "/bugs".to_string(),
              variant: BugsPropertyIsNotFormatted,
            },
          );
        }
      }
      if ctx.config.rcfile.format_repository {
        if let Some(expected) = format::get_formatted_repository(&package.borrow()) {
          add_mismatch(
            package,
            FormatMismatch {
              expected,
              package: Rc::clone(package),
              property_path: "/repository".to_string(),
              variant: RepositoryPropertyIsNotFormatted,
            },
          );
        }
      }
      if !ctx.config.rcfile.sort_exports.is_empty() {
        if let Some(expected) = format::get_sorted_exports(&ctx.config.rcfile, &package.borrow()) {
          add_mismatch(
            package,
            FormatMismatch {
              expected,
              package: Rc::clone(package),
              property_path: "/exports".to_string(),
              variant: ExportsPropertyIsNotSorted,
            },
          );
        }
      }
      if !ctx.config.rcfile.sort_az.is_empty() {
        for key in ctx.config.rcfile.sort_az.iter() {
          if let Some(expected) = format::get_sorted_az(key, &package.borrow()) {
            add_mismatch(
              package,
              FormatMismatch {
                expected,
                package: Rc::clone(package),
                property_path: format!("/{}", key),
                variant: PropertyIsNotSortedAz,
              },
            );
          }
        }
      }
    });
  }

  ctx
}
