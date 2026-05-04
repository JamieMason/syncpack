use crate::{commands::reporter::FormatReporter, context::Context, disk::DiskIo, errors::SyncpackError, source::Source};

pub fn run<D: DiskIo>(ctx: Context, reporter: &dyn FormatReporter, io: &D) -> Result<Context, SyncpackError> {
  if ctx.config.cli.check {
    check_formatting(ctx, reporter)
  } else {
    fix_formatting(ctx, reporter, io)
  }
}

fn check_formatting(ctx: Context, reporter: &dyn FormatReporter) -> Result<Context, SyncpackError> {
  let mut is_invalid = false;
  for source in ctx.sources.all.iter() {
    let Source::Package {
      file_idx,
      name,
      formatting_mismatches,
    } = source
    else {
      continue;
    };
    if formatting_mismatches.is_empty() {
      continue;
    }
    is_invalid = true;
    let file = &ctx.disk.package_json_files[*file_idx];
    reporter.on_package_header(&ctx, name, &file.filepath, formatting_mismatches.len());
    formatting_mismatches.iter().for_each(|mismatch| {
      reporter.on_mismatch_unfixed(&ctx, name, &file.filepath, mismatch);
    });
  }
  if !is_invalid {
    reporter.on_no_issues();
  }
  if is_invalid { Err(SyncpackError::IssuesFound) } else { Ok(ctx) }
}

fn fix_formatting<D: DiskIo>(mut ctx: Context, reporter: &dyn FormatReporter, io: &D) -> Result<Context, SyncpackError> {
  let mut was_invalid = false;
  // Tuples of (sources arena slot, file_idx, name, mismatch_count) so the
  // immutable borrow of sources ends before we re-borrow mutably below.
  let mut fix_targets: Vec<(usize, usize)> = vec![];

  for (i, source) in ctx.sources.all.iter().enumerate() {
    let Source::Package {
      file_idx,
      name,
      formatting_mismatches,
    } = source
    else {
      continue;
    };
    if formatting_mismatches.is_empty() {
      continue;
    }
    was_invalid = true;
    let file = &ctx.disk.package_json_files[*file_idx];
    reporter.on_package_header(&ctx, name, &file.filepath, formatting_mismatches.len());
    for mismatch in formatting_mismatches.iter() {
      reporter.on_mismatch_fixed(&ctx, name, &file.filepath, mismatch);
    }
    fix_targets.push((i, *file_idx));
  }

  // Apply mismatches: drain each source's vec to drop the immutable borrow,
  // then mutate disk.package_json_files[file_idx].
  for (source_idx, file_idx) in fix_targets {
    // Drain mismatches out of the source first so we don't hold a borrow
    // when mutating disk.
    let mismatches = match &mut ctx.sources.all[source_idx] {
      Source::Package { formatting_mismatches, .. } => std::mem::take(formatting_mismatches),
      Source::PnpmYaml => continue,
    };
    let file = &mut ctx.disk.package_json_files[file_idx];
    for mismatch in mismatches {
      crate::disk::set_prop(file, &mismatch.property_path, mismatch.expected);
    }
  }

  if !ctx.config.cli.dry_run {
    let indent = ctx.config.rcfile.indent.as_deref();
    let fallback = ctx.disk.formatting_fallback();
    for file in ctx.disk.package_json_files.iter_mut() {
      crate::disk::write_json_file(file, io, indent, &fallback)?;
    }
  }
  if !was_invalid {
    reporter.on_no_issues();
  }
  Ok(ctx)
}
