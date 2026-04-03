use crate::{commands::reporter::FormatReporter, context::Context, errors::SyncpackError};

pub fn run(ctx: Context, reporter: &dyn FormatReporter) -> Result<Context, SyncpackError> {
  if ctx.config.cli.check {
    check_formatting(ctx, reporter)
  } else {
    fix_formatting(ctx, reporter)
  }
}

fn check_formatting(ctx: Context, reporter: &dyn FormatReporter) -> Result<Context, SyncpackError> {
  let mut is_invalid = false;
  ctx
    .packages
    .all
    .iter()
    .filter(|package| package.borrow().has_formatting_mismatches())
    .for_each(|package| {
      is_invalid = true;
      let package = package.borrow();
      reporter.on_package_header(&ctx, &package);
      package.formatting_mismatches.borrow().iter().for_each(|mismatch| {
        reporter.on_mismatch_unfixed(&ctx, &package, mismatch);
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

fn fix_formatting(ctx: Context, reporter: &dyn FormatReporter) -> Result<Context, SyncpackError> {
  let mut was_invalid = false;
  ctx
    .packages
    .all
    .iter()
    .filter(|package| package.borrow().has_formatting_mismatches())
    .for_each(|package| {
      was_invalid = true;
      let package = package.borrow();
      reporter.on_package_header(&ctx, &package);
      package.formatting_mismatches.borrow().iter().for_each(|mismatch| {
        reporter.on_mismatch_fixed(&ctx, &package, mismatch);
        if mismatch.property_path == "/" {
          *package.contents.borrow_mut() = mismatch.expected.clone();
        } else if let Some(value) = package.contents.borrow_mut().pointer_mut(&mismatch.property_path) {
          *value = mismatch.expected.clone();
        }
      });
    });
  if !ctx.config.cli.dry_run {
    ctx.packages.all.iter().for_each(|package| {
      package
        .borrow()
        .write_to_disk(ctx.config.rcfile.indent.as_deref(), &ctx.packages.formatting);
    });
  }
  if !was_invalid {
    reporter.on_no_issues();
  }
  Ok(ctx)
}
