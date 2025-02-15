use {
  crate::{context::Context, effects::ui, instance_state::InstanceState},
  colored::*,
  log::info,
};

/// Run the fix command side effects
pub fn run(ctx: Context) -> ! {
  let mut is_invalid = false;

  let get_instance_state_icon = |state: &InstanceState| -> String {
    if state.is_valid() || state.is_fixable() {
      ui::icon::ok().to_string()
    } else if state.is_suspect() {
      ui::icon::warn().to_string()
    } else {
      ui::icon::err().to_string()
    }
  };

  ctx
    .get_version_groups()
    .filter(|group| !group.dependencies.is_empty() && !group.has_ignored_variant())
    .for_each(|group| {
      let mut has_shown_group_header = false;
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        let mut has_shown_dependency_header = false;
        dependency.get_sorted_instances().for_each(|instance_obj| {
          let cannot_autofix = instance_obj.is_unfixable() || instance_obj.is_suspect() && ctx.config.rcfile.strict;
          if instance_obj.is_fixable() || cannot_autofix {
            if !has_shown_group_header {
              ui::group::print_header(&ctx, group);
              has_shown_group_header = true;
            }
            if !has_shown_dependency_header {
              let alias_hint = ui::dependency::get_alias_hint(dependency);
              let state = dependency.get_state();
              let icon = get_instance_state_icon(&state);
              let line = ui::util::join_line(vec![&icon, &dependency.internal_name, &alias_hint]);
              info!("{line}");
              has_shown_dependency_header = true;
            }
            if instance_obj.is_fixable() {
              if instance_obj.is_banned() {
                let name = &instance_obj.descriptor.name;
                let location = ui::instance::get_location(&ctx, instance_obj).dimmed();
                let state_link = ui::instance::get_state_link_in_parens(&ctx, instance_obj, &group.variant);
                info!("  {location} {state_link}");
                instance_obj.remove()
              } else {
                let name = &instance_obj.descriptor.name;
                let expected = ui::instance::get_expected(instance_obj).dimmed();
                let location = ui::instance::get_location(&ctx, instance_obj).dimmed();
                info!("  {expected} {location}");
                instance_obj.descriptor.package.borrow().copy_expected_specifier(instance_obj);
              }
            } else if cannot_autofix {
              is_invalid = true;
              let name = &instance_obj.descriptor.name;
              let actual = ui::instance::get_actual(instance_obj);
              let location = ui::instance::get_location(&ctx, instance_obj).dimmed();
              let state_link = ui::instance::get_state_link_in_parens(&ctx, instance_obj, &group.variant);
              info!("  {actual} {location} {state_link}");
            }
          }
        });
      });
    });

  if !ctx.config.cli.dry_run {
    ctx.packages.all.iter().for_each(|package| {
      package.borrow().write_to_disk(&ctx.config);
    });
  }

  std::process::exit(if is_invalid { 1 } else { 0 });
}
