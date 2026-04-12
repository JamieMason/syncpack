use crate::{commands::reporter::FormatReporter, context::Context, disk::DiskIo, errors::SyncpackError};

pub fn run<D: DiskIo>(ctx: Context, reporter: &dyn FormatReporter, io: &D) -> Result<Context, SyncpackError> {
  if ctx.config.cli.check {
    check_formatting(ctx, reporter)
  } else {
    fix_formatting(ctx, reporter, io)
  }
}

fn check_formatting(ctx: Context, reporter: &dyn FormatReporter) -> Result<Context, SyncpackError> {
  let mut is_invalid = false;
  ctx
    .packages
    .all
    .iter()
    .filter(|package| package.has_formatting_mismatches())
    .for_each(|package| {
      is_invalid = true;
      reporter.on_package_header(&ctx, package);
      package.formatting_mismatches.iter().for_each(|mismatch| {
        reporter.on_mismatch_unfixed(&ctx, package, mismatch);
      });
    });
  if !is_invalid {
    reporter.on_no_issues();
  }
  if is_invalid {
    Err(SyncpackError::IssuesFound)
  } else {
    Ok(ctx)
  }
}

fn fix_formatting<D: DiskIo>(mut ctx: Context, reporter: &dyn FormatReporter, io: &D) -> Result<Context, SyncpackError> {
  let mut was_invalid = false;
  let mut fix_indices: Vec<usize> = vec![];

  for (i, package) in ctx.packages.all.iter().enumerate() {
    if !package.has_formatting_mismatches() {
      continue;
    }
    was_invalid = true;
    reporter.on_package_header(&ctx, package);
    for mismatch in package.formatting_mismatches.iter() {
      reporter.on_mismatch_fixed(&ctx, package, mismatch);
    }
    fix_indices.push(i);
  }

  for i in fix_indices {
    let package = &mut ctx.packages.all[i];
    for j in 0..package.formatting_mismatches.len() {
      let property_path = package.formatting_mismatches[j].property_path.clone();
      let expected = package.formatting_mismatches[j].expected.clone();
      package.set_prop(&property_path, expected);
    }
  }

  if !ctx.config.cli.dry_run {
    let indent = ctx.config.rcfile.indent.as_deref();
    let formatting = &ctx.packages.formatting;
    for package in ctx.packages.all.iter_mut() {
      package.write_to_disk(io, indent, formatting)?;
    }
  }
  if !was_invalid {
    reporter.on_no_issues();
  }
  Ok(ctx)
}
