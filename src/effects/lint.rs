use crate::{context::Context, effects::ui::Ui, version_group::VersionGroupVariant};

/// Run the lint command side effects
pub fn run(ctx: Context) -> Context {
  let ui = Ui { ctx: &ctx };
  let has_cli_filter = ctx.config.cli.filter.is_some();
  let running_multiple_commands = ctx.config.cli.inspect_mismatches && ctx.config.cli.inspect_formatting;

  if ctx.config.cli.inspect_mismatches {
    if running_multiple_commands {
      ui.print_command_header("SEMVER RANGES AND VERSION MISMATCHES");
    }
    ctx.version_groups.iter().for_each(|group| {
      if has_cli_filter && !*group.matches_cli_filter.borrow() {
        return;
      }
      ui.print_group_header(group);
      if group.dependencies.borrow().len() == 0 {
        let label = &group.selector.label;
        ui.print_empty_group();
        return;
      }
      if !ctx.config.cli.show_ignored && matches!(group.variant, VersionGroupVariant::Ignored) {
        ui.print_ignored_group(group);
        return;
      }
      group.for_each_dependency(&ctx.config.cli.sort, |dependency| {
        if has_cli_filter && !*dependency.matches_cli_filter.borrow() {
          return;
        }
        ui.print_dependency(dependency, &group.variant);
        dependency.for_each_instance(|instance| {
          if ctx.config.cli.show_instances {
            if has_cli_filter && !*instance.matches_cli_filter.borrow() {
              return;
            }
            ui.print_instance(instance, &group.variant);
          }
        });
      });
    });
  }
  if ctx.config.cli.inspect_formatting {
    if running_multiple_commands {
      ui.print_command_header("PACKAGE FORMATTING");
    }
    ui.print_formatted_packages(&ctx.get_formatted_packages());
    ctx.get_formatting_mismatches_by_variant().iter().for_each(|(variant, mismatches)| {
      ui.print_formatting_mismatches(variant, mismatches);
    });
  }
  ctx
}
