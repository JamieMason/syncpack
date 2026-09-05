use crate::{
  context::Discovered,
  inspection::Inspection,
  source::{FormatMismatch, FormatMismatchVariant::*, Source},
  sources::SourceIdx,
};

mod format;

/// Formatting inspection: read each package.json's loaded contents and record
/// any formatting mismatches into `out.formatting`, keyed by `SourceIdx`. Pure
/// — reads already-loaded file contents, mutates only `out`.
pub fn visit_formatting(discovered: &Discovered, out: &mut Inspection) {
  let rcfile = &discovered.config.rcfile;
  for (source_idx, source) in discovered.sources.all.iter().enumerate() {
    let Source::Package { file_idx, .. } = source else { continue };
    let source_idx = SourceIdx(source_idx);
    let file = &discovered.disk.package_json_files[*file_idx];
    if (rcfile.sort_packages || !rcfile.sort_first.is_empty())
      && let Some(expected) = format::get_sorted_first(rcfile, &file.contents)
    {
      out.add_format_mismatch(
        source_idx,
        FormatMismatch {
          expected,
          property_path: "/".to_string(),
          variant: PackagePropertiesAreNotSorted,
        },
      );
    }
    if rcfile.format_bugs
      && let Some(expected) = format::get_formatted_bugs(&file.contents)
    {
      out.add_format_mismatch(
        source_idx,
        FormatMismatch {
          expected,
          property_path: "/bugs".to_string(),
          variant: BugsPropertyIsNotFormatted,
        },
      );
    }
    if rcfile.format_repository
      && let Some(expected) = format::get_formatted_repository(&file.contents)
    {
      out.add_format_mismatch(
        source_idx,
        FormatMismatch {
          expected,
          property_path: "/repository".to_string(),
          variant: RepositoryPropertyIsNotFormatted,
        },
      );
    }
    if !rcfile.sort_exports.is_empty()
      && let Some(expected) = format::get_sorted_exports(rcfile, &file.contents)
    {
      out.add_format_mismatch(
        source_idx,
        FormatMismatch {
          expected,
          property_path: "/exports".to_string(),
          variant: ExportsPropertyIsNotSorted,
        },
      );
    }
    if !rcfile.sort_az.is_empty() {
      for key in rcfile.sort_az.iter() {
        if let Some(expected) = format::get_sorted_az(key, &file.contents) {
          out.add_format_mismatch(
            source_idx,
            FormatMismatch {
              expected,
              property_path: format!("/{key}"),
              variant: PropertyIsNotSortedAz,
            },
          );
        }
      }
    }
  }
}
