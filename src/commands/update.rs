use {
  crate::{
    commands::{ui, ui::LINE_ENDING},
    context::Context,
    errors::SyncpackError,
    instance::InstanceIdx,
    registry::updates::RegistryUpdates,
    version_group::VersionGroup,
  },
  log::{error, warn},
};

pub fn run(mut ctx: Context, registry_updates: RegistryUpdates) -> Result<Context, SyncpackError> {
  let mut was_outdated = false;
  let mut copy_actions: Vec<(usize, InstanceIdx)> = vec![];

  ctx
    .version_groups
    .iter()
    .filter(|group| matches!(group, VersionGroup::PreferredSemver(g) if g.prefer_highest))
    .for_each(|group| {
      let mut has_printed_group = false;
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        let mut has_printed_dependency = false;
        dependency
          .get_sorted_instances(&ctx.instances, &ctx.packages.all)
          .filter(|(_, instance)| instance.is_outdated())
          .for_each(|(idx, instance)| {
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
              copy_actions.push((instance.descriptor.package_idx.0, idx));
            }
          });
      })
    });

  for (pkg_idx, inst_idx) in copy_actions {
    let instance = &ctx.instances[inst_idx.0];
    let package = &mut ctx.packages.all[pkg_idx];
    package.copy_expected_specifier(instance);
  }

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
    let indent = ctx.config.rcfile.indent.as_deref();
    let formatting = &ctx.packages.formatting;
    for package in ctx.packages.all.iter_mut() {
      package.write_to_disk(indent, formatting);
    }
  }

  Ok(ctx)
}
