use crate::{context::Context, effects::ui};

/// Run the lint command side effects
pub fn run(ctx: Context) -> ! {
  let mut is_invalid = false;

  ctx.get_version_groups().for_each(|group| {
    ui::group::print_header(&ctx, group);
    if group.dependencies.is_empty() {
      ui::group::print_empty();
      return;
    }
    if !ctx.config.cli.show_ignored && group.has_ignored_variant() {
      ui::group::print_ignored(group);
      return;
    }
    group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
      ui::dependency::print(&ctx, dependency, &group.variant);
      dependency.get_sorted_instances().for_each(|instance| {
        if !instance.is_valid() || ctx.config.cli.show_instances {
          ui::instance::print(&ctx, instance, &group.variant);
        }
        if instance.is_invalid() || (instance.is_suspect() && ctx.config.rcfile.strict) {
          is_invalid = true;
        }
      });
    });
  });

  std::process::exit(if is_invalid { 1 } else { 0 });
}
