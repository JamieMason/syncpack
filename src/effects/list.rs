use crate::{context::Context, effects::ui};

pub fn run(ctx: Context) -> ! {
  let mut is_invalid = false;

  ctx
    .get_version_groups()
    .filter(|group| !group.has_ignored_variant() || ctx.config.cli.show_ignored)
    .for_each(|group| {
      ui::group::print_header(&ctx, group);
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        ui::dependency::print(&ctx, dependency, &group.variant);
        dependency.get_sorted_instances().for_each(|instance| {
          if ctx.config.cli.show_instances {
            ui::instance::print(&ctx, instance, &group.variant);
          }
          if instance.is_invalid() || (instance.is_suspect() && ctx.config.rcfile.strict) {
            is_invalid = true;
          }
        });
      });
    });

  if !is_invalid {
    ui::util::print_no_issues_found();
  }

  std::process::exit(if is_invalid { 1 } else { 0 });
}
