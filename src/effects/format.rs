use crate::{context::Context, effects::ui};

/// Run the fix command side effects
pub fn run(ctx: Context) -> ! {
  if ctx.config.cli.check {
    check_formatting(ctx)
  } else {
    fix_formatting(ctx)
  }
}

fn check_formatting(ctx: Context) -> ! {
  ui::package::print_formatted(&ctx, &ctx.get_formatted_packages());

  ctx.get_formatting_mismatches_by_variant().iter().for_each(|(variant, mismatches)| {
    ui::package::print_formatting_mismatches(&ctx, variant, mismatches);
  });

  for package in ctx.packages.all.iter() {
    if !package.borrow().formatting_mismatches.borrow().is_empty() {
      std::process::exit(1);
    }
  }

  std::process::exit(0);
}

fn fix_formatting(ctx: Context) -> ! {
  ctx.packages.all.iter().for_each(|package| {
    let package = package.borrow();
    let mut formatting_mismatches = package.formatting_mismatches.borrow_mut();
    formatting_mismatches.iter().for_each(|mismatch| {
      if mismatch.property_path == "/" {
        *package.contents.borrow_mut() = mismatch.expected.clone();
      } else if let Some(value) = package.contents.borrow_mut().pointer_mut(&mismatch.property_path) {
        *value = mismatch.expected.clone();
      }
    });
    *formatting_mismatches = vec![];
  });

  ui::package::print_formatted(&ctx, &ctx.packages.all);

  if !ctx.config.cli.dry_run {
    ctx.packages.all.iter().for_each(|package| {
      package.borrow().write_to_disk(&ctx.config);
    });
  }

  std::process::exit(0);
}
