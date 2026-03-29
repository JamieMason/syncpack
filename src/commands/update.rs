use {
  crate::{
    commands::{ui, ui::LINE_ENDING},
    context::{Context, SyncpackError},
    registry::updates::RegistryUpdates,
    version_group::VersionGroup,
  },
  log::{error, warn},
};

pub fn run(ctx: Context, registry_updates: &RegistryUpdates) -> Result<Context, SyncpackError> {
  let mut was_outdated = false;

  ctx
    .version_groups
    .iter()
    .filter(|group| matches!(group, VersionGroup::PreferredSemver(g) if g.prefer_highest))
    .for_each(|group| {
      let mut has_printed_group = false;
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        let mut has_printed_dependency = false;
        dependency
          .get_sorted_instances(&ctx.instances)
          .filter(|instance| instance.is_outdated())
          .for_each(|instance| {
            was_outdated = true;
            if !has_printed_group {
              ui::group::print_header(&ctx, group);
              has_printed_group = true;
            }
            if ctx.config.cli.check {
              if !has_printed_dependency {
                ui::dependency::print_outdated(&ctx, dependency, group.variant_label());
                has_printed_dependency = true;
              }
              ui::instance::print_outdated(&ctx, instance);
            } else {
              if !has_printed_dependency {
                ui::dependency::print_fixed(&ctx, dependency, group.variant_label());
                has_printed_dependency = true;
              }
              ui::instance::print_fixed(&ctx, instance);
              instance.descriptor.package.borrow().copy_expected_specifier(instance);
            }
          });
      })
    });

  if !registry_updates.failed.is_empty() {
    println!(" ");
    registry_updates.failed.iter().for_each(|name| {
      error!("Failed to fetch {name}");
    });
    warn!(
      "Syncpack does not yet support custom npm registries{LINE_ENDING}  Subscribe to https://github.com/JamieMason/syncpack/issues/220"
    );
  } else if !was_outdated {
    ui::util::print_no_issues_found();
  }

  if ctx.config.cli.check {
    return if was_outdated { Err(SyncpackError::IssuesFound) } else { Ok(ctx) };
  }

  if !ctx.config.cli.dry_run {
    ctx.packages.all.iter().for_each(|package| {
      package
        .borrow()
        .write_to_disk(ctx.config.rcfile.indent.as_deref(), &ctx.packages.formatting);
    });
  }

  Ok(ctx)
}
