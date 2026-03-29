use crate::{
  commands::ui,
  context::{Context, SyncpackError},
};

pub fn run(ctx: Context) -> Result<Context, SyncpackError> {
  let mut is_invalid = false;

  ctx
    .version_groups
    .iter()
    .filter(|group| !group.is_ignored() || ctx.config.cli.show_ignored)
    .for_each(|group| {
      ui::group::print_header(&ctx, group);
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        ui::dependency::print(&ctx, dependency, group.variant_label());
        dependency.get_sorted_instances(&ctx.instances).for_each(|instance| {
          if ctx.config.cli.show_instances {
            ui::instance::print(&ctx, instance);
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

  if is_invalid {
    Err(SyncpackError::IssuesFound)
  } else {
    Ok(ctx)
  }
}
