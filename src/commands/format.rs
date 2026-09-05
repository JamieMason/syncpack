use {
  crate::{commands::reporter::FormatReporter, context::Inspected, errors::SyncpackError},
  syncpack_io::DiskIo,
  syncpack_model::source::Source,
};

pub fn run<D: DiskIo>(inspected: Inspected, reporter: &dyn FormatReporter, io: &D) -> Result<Inspected, SyncpackError> {
  if inspected.data.config.cli.check {
    check_formatting(inspected, reporter)
  } else {
    fix_formatting(inspected, reporter, io)
  }
}

fn check_formatting(inspected: Inspected, reporter: &dyn FormatReporter) -> Result<Inspected, SyncpackError> {
  let mut is_invalid = false;
  {
    let data = &inspected.data;
    let inspection = &inspected.inspection;
    for (source_idx, source) in data.sources.all.iter().enumerate() {
      let Source::Package { file_idx, name } = source else {
        continue;
      };
      let mismatches = &inspection.formatting[source_idx];
      if mismatches.is_empty() {
        continue;
      }
      is_invalid = true;
      let file = &data.disk.package_json_files[*file_idx];
      reporter.on_package_header(data, name, &file.filepath, mismatches.len());
      mismatches.iter().for_each(|mismatch| {
        reporter.on_mismatch_unfixed(data, name, &file.filepath, mismatch);
      });
    }
  }
  if !is_invalid {
    reporter.on_no_issues();
  }
  if is_invalid {
    Err(SyncpackError::IssuesFound)
  } else {
    Ok(inspected)
  }
}

fn fix_formatting<D: DiskIo>(mut inspected: Inspected, reporter: &dyn FormatReporter, io: &D) -> Result<Inspected, SyncpackError> {
  let mut was_invalid = false;
  let mut fix_targets: Vec<(usize, usize)> = vec![];

  {
    let data = &inspected.data;
    let inspection = &inspected.inspection;
    for (source_idx, source) in data.sources.all.iter().enumerate() {
      let Source::Package { file_idx, name } = source else {
        continue;
      };
      let mismatches = &inspection.formatting[source_idx];
      if mismatches.is_empty() {
        continue;
      }
      was_invalid = true;
      let file = &data.disk.package_json_files[*file_idx];
      reporter.on_package_header(data, name, &file.filepath, mismatches.len());
      for mismatch in mismatches.iter() {
        reporter.on_mismatch_fixed(data, name, &file.filepath, mismatch);
      }
      fix_targets.push((source_idx, *file_idx));
    }
  }

  for (source_idx, file_idx) in fix_targets {
    let mismatches = &inspected.inspection.formatting[source_idx];
    let file = &mut inspected.data.disk.package_json_files[file_idx];
    for mismatch in mismatches {
      syncpack_io::set_prop(file, &mismatch.property_path, mismatch.expected.clone());
    }
  }

  if !inspected.data.config.cli.dry_run {
    let indent = inspected.data.config.rcfile.indent.as_deref();
    let fallback = inspected.data.disk.formatting_fallback();
    for file in inspected.data.disk.package_json_files.iter_mut() {
      syncpack_io::write_json_file(file, io, indent, &fallback)?;
    }
  }
  if !was_invalid {
    reporter.on_no_issues();
  }
  Ok(inspected)
}
