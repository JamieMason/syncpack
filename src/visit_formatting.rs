use crate::{
  context::Context,
  source::{FormatMismatch, FormatMismatchVariant::*, Source},
};

mod format;

pub fn visit_formatting(mut ctx: Context) -> Context {
  let rcfile = &ctx.config.rcfile;
  // Compute mismatches by reading from disk first, then push into the
  // sources arena. Two-pass split avoids overlapping borrows.
  let mut new_mismatches: Vec<(usize, FormatMismatch)> = Vec::new();
  for (source_idx, source) in ctx.sources.all.iter().enumerate() {
    let Source::Package { file_idx, .. } = source else { continue };
    let file = &ctx.disk.package_json_files[*file_idx];
    if rcfile.sort_packages || !rcfile.sort_first.is_empty() {
      if let Some(expected) = format::get_sorted_first(rcfile, &file.contents) {
        new_mismatches.push((
          source_idx,
          FormatMismatch {
            expected,
            property_path: "/".to_string(),
            variant: PackagePropertiesAreNotSorted,
          },
        ));
      }
    }
    if rcfile.format_bugs {
      if let Some(expected) = format::get_formatted_bugs(&file.contents) {
        new_mismatches.push((
          source_idx,
          FormatMismatch {
            expected,
            property_path: "/bugs".to_string(),
            variant: BugsPropertyIsNotFormatted,
          },
        ));
      }
    }
    if rcfile.format_repository {
      if let Some(expected) = format::get_formatted_repository(&file.contents) {
        new_mismatches.push((
          source_idx,
          FormatMismatch {
            expected,
            property_path: "/repository".to_string(),
            variant: RepositoryPropertyIsNotFormatted,
          },
        ));
      }
    }
    if !rcfile.sort_exports.is_empty() {
      if let Some(expected) = format::get_sorted_exports(rcfile, &file.contents) {
        new_mismatches.push((
          source_idx,
          FormatMismatch {
            expected,
            property_path: "/exports".to_string(),
            variant: ExportsPropertyIsNotSorted,
          },
        ));
      }
    }
    if !rcfile.sort_az.is_empty() {
      for key in rcfile.sort_az.iter() {
        if let Some(expected) = format::get_sorted_az(key, &file.contents) {
          new_mismatches.push((
            source_idx,
            FormatMismatch {
              expected,
              property_path: format!("/{key}"),
              variant: PropertyIsNotSortedAz,
            },
          ));
        }
      }
    }
  }
  for (source_idx, mismatch) in new_mismatches {
    if let Source::Package { formatting_mismatches, .. } = &mut ctx.sources.all[source_idx] {
      formatting_mismatches.push(mismatch);
    }
  }

  ctx
}
