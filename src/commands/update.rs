use {
  crate::{commands::ui, context::Context, version_group::VersionGroupVariant},
  log::error,
};

pub fn run(ctx: Context) -> i32 {
  let mut was_outdated = false;

  ctx
    .version_groups
    .iter()
    .filter(|group| matches!(group.variant, VersionGroupVariant::HighestSemver))
    .for_each(|group| {
      let mut has_printed_group = false;
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        let mut has_printed_dependency = false;
        dependency
          .get_sorted_instances()
          .filter(|instance| instance.is_outdated())
          .for_each(|instance| {
            was_outdated = true;
            if !has_printed_group {
              ui::group::print_header(&ctx, group);
              has_printed_group = true;
            }
            if ctx.config.cli.check {
              if !has_printed_dependency {
                ui::dependency::print_outdated(&ctx, dependency, &group.variant);
                has_printed_dependency = true;
              }
              ui::instance::print_outdated(&ctx, instance, &group.variant);
            } else {
              if !has_printed_dependency {
                ui::dependency::print_fixed(&ctx, dependency, &group.variant);
                has_printed_dependency = true;
              }
              ui::instance::print_fixed(&ctx, instance, &group.variant);
              instance.descriptor.package.borrow().copy_expected_specifier(instance);
            }
          });
      })
    });

  if !ctx.failed_updates.is_empty() {
    println!(" ");
    ctx.failed_updates.iter().for_each(|name| {
      error!("Failed to fetch {name}");
    });
  } else if !was_outdated {
    ui::util::print_no_issues_found();
  }

  if ctx.config.cli.check {
    return if was_outdated { 1 } else { 0 };
  }

  if !ctx.config.cli.dry_run {
    ctx.packages.all.iter().for_each(|package| {
      package.borrow().write_to_disk(&ctx.config);
    });
  }

  0
}
