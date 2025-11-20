use {
  crate::{commands::ui, context::Context},
  log::warn,
};

pub fn run(ctx: Context) -> i32 {
  let mut contains_unfixable_issues = false;
  let mut was_invalid = false;

  ctx
    .version_groups
    .iter()
    .filter(|group| !group.dependencies.is_empty() && !group.has_ignored_variant())
    .for_each(|group| {
      let mut has_printed_group = false;
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        let mut has_printed_dependency = false;
        dependency
          .get_sorted_instances()
          .inspect(|instance| {
            if instance.is_unfixable() || instance.is_suspect() && ctx.config.rcfile.strict {
              contains_unfixable_issues = true
            }
          })
          .filter(|instance| instance.is_fixable())
          .for_each(|instance| {
            was_invalid = true;
            if !has_printed_group {
              ui::group::print_header(&ctx, group);
              has_printed_group = true;
            }
            if !has_printed_dependency {
              ui::dependency::print_fixed(&ctx, dependency, &group.variant);
              has_printed_dependency = true;
            }
            if ctx.config.cli.show_instances {
              ui::instance::print_fixed(&ctx, instance, &group.variant);
            }
            if instance.is_banned() {
              instance.remove()
            } else {
              instance.descriptor.package.borrow().copy_expected_specifier(instance);
            }
          });
      })
    });

  if !ctx.config.cli.dry_run {
    ctx.packages.all.iter().for_each(|package| {
      package.borrow().write_to_disk(&ctx.config);
    });
  }

  if contains_unfixable_issues {
    println!(" ");
    warn!("Some issues remain which cannot be fixed automatically, run syncpack lint to view them");
  }

  if !contains_unfixable_issues && !was_invalid {
    ui::util::print_no_issues_found();
  }

  if contains_unfixable_issues {
    1
  } else {
    0
  }
}
