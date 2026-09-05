use {
  crate::{
    inspection::{FormatConfig, FormatMismatch, FormatMismatchVariant::*, Inspection},
    source::Source,
    sources::{SourceIdx, Sources},
  },
  serde_json::Value,
};

mod format;

/// Formatting inspection: read each package.json's loaded contents and record
/// any formatting mismatches into `out.formatting`, keyed by `SourceIdx`. Pure
/// — reads already-loaded file contents (threaded in as `contents`, indexed by
/// source position) plus the plain-data `FormatConfig`, mutates only `out`.
pub fn visit_formatting(sources: &Sources, contents: &[Option<&Value>], format: &FormatConfig, out: &mut Inspection) {
  for (i, source) in sources.all.iter().enumerate() {
    let Source::Package { .. } = source else { continue };
    let Some(file_contents) = contents[i] else { continue };
    let source_idx = SourceIdx(i);
    if (format.sort_packages || !format.sort_first.is_empty())
      && let Some(expected) = format::get_sorted_first(format, file_contents)
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
    if format.format_bugs
      && let Some(expected) = format::get_formatted_bugs(file_contents)
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
    if format.format_repository
      && let Some(expected) = format::get_formatted_repository(file_contents)
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
    if !format.sort_exports.is_empty()
      && let Some(expected) = format::get_sorted_exports(format, file_contents)
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
    if !format.sort_az.is_empty() {
      for key in format.sort_az.iter() {
        if let Some(expected) = format::get_sorted_az(key, file_contents) {
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
