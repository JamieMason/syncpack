use crate::{
  commands::ui,
  context::{Context, SyncpackError},
};

/// Run the lint command side effects
pub fn run(ctx: Context) -> Result<Context, SyncpackError> {
  let mut is_invalid = false;

  ctx.version_groups.iter().for_each(|group| {
    let mut has_printed_group = false;
    group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
      let mut has_printed_dependency = false;
      dependency
        .get_sorted_instances(&ctx.instances)
        .filter(|instance| instance.is_invalid() || (instance.is_suspect() && ctx.config.rcfile.strict))
        .for_each(|instance| {
          if !has_printed_group {
            ui::group::print_header(&ctx, group);
            has_printed_group = true;
          }
          if !has_printed_dependency {
            ui::dependency::print(&ctx, dependency, group.variant_label());
            has_printed_dependency = true;
          }
          if ctx.config.cli.show_instances {
            ui::instance::print(&ctx, instance);
          }
          is_invalid = true;
        });
    });
  });

  if is_invalid {
    Err(SyncpackError::IssuesFound)
  } else {
    ui::util::print_no_issues_found();
    Ok(ctx)
  }
}
