use crate::{context::Context, effects::ui};

pub fn run(ctx: Context) -> i32 {
  if ctx.config.cli.check {
    check_formatting(ctx)
  } else {
    fix_formatting(ctx)
  }
}

fn check_formatting(ctx: Context) -> i32 {
  let mut is_invalid = false;
  ctx
    .packages
    .all
    .iter()
    .filter(|package| package.borrow().has_formatting_mismatches())
    .for_each(|package| {
      is_invalid = true;
      ui::package::print_invalid_package(&ctx, &package.borrow());
      package.borrow().formatting_mismatches.borrow().iter().for_each(|mismatch| {
        ui::package::print_invalid(&ctx, mismatch);
      });
    });
  if !is_invalid {
    ui::util::print_no_issues_found();
  }
  if is_invalid {
    1
  } else {
    0
  }
}

fn fix_formatting(ctx: Context) -> i32 {
  let mut was_invalid = false;
  ctx
    .packages
    .all
    .iter()
    .filter(|package| package.borrow().has_formatting_mismatches())
    .for_each(|package| {
      was_invalid = true;
      let package = package.borrow();
      ui::package::print_fixed_package(&ctx, &package);
      package.formatting_mismatches.borrow().iter().for_each(|mismatch| {
        ui::package::print_fixed(&ctx, mismatch);
        if mismatch.property_path == "/" {
          *package.contents.borrow_mut() = mismatch.expected.clone();
        } else if let Some(value) = package.contents.borrow_mut().pointer_mut(&mismatch.property_path) {
          *value = mismatch.expected.clone();
        }
      });
    });
  if !ctx.config.cli.dry_run {
    ctx.packages.all.iter().for_each(|package| {
      package.borrow().write_to_disk(&ctx.config);
    });
  }
  if was_invalid {
    1
  } else {
    ui::util::print_no_issues_found();
    0
  }
}
