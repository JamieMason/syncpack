use crate::{
  context::Context,
  package_json::{FormatMismatch, FormatMismatchVariant::*},
};

mod format;

pub fn visit_formatting(mut ctx: Context) -> Context {
  let rcfile = &ctx.config.rcfile;
  for package in ctx.packages.all.iter_mut() {
    if rcfile.sort_packages || !rcfile.sort_first.is_empty() {
      if let Some(expected) = format::get_sorted_first(rcfile, package) {
        package.formatting_mismatches.push(FormatMismatch {
          expected,
          property_path: "/".to_string(),
          variant: PackagePropertiesAreNotSorted,
        });
      }
    }
    if rcfile.format_bugs {
      if let Some(expected) = format::get_formatted_bugs(package) {
        package.formatting_mismatches.push(FormatMismatch {
          expected,
          property_path: "/bugs".to_string(),
          variant: BugsPropertyIsNotFormatted,
        });
      }
    }
    if rcfile.format_repository {
      if let Some(expected) = format::get_formatted_repository(package) {
        package.formatting_mismatches.push(FormatMismatch {
          expected,
          property_path: "/repository".to_string(),
          variant: RepositoryPropertyIsNotFormatted,
        });
      }
    }
    if !rcfile.sort_exports.is_empty() {
      if let Some(expected) = format::get_sorted_exports(rcfile, package) {
        package.formatting_mismatches.push(FormatMismatch {
          expected,
          property_path: "/exports".to_string(),
          variant: ExportsPropertyIsNotSorted,
        });
      }
    }
    if !rcfile.sort_az.is_empty() {
      for key in rcfile.sort_az.iter() {
        if let Some(expected) = format::get_sorted_az(key, package) {
          package.formatting_mismatches.push(FormatMismatch {
            expected,
            property_path: format!("/{key}"),
            variant: PropertyIsNotSortedAz,
          });
        }
      }
    }
  }

  ctx
}
