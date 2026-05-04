use {
  crate::{
    commands::{ui, ui::LINE_ENDING},
    context::Context,
    disk::{DiskIo, copy_expected_specifier_json, copy_expected_specifier_yaml, write_json_file, write_yaml_file},
    errors::SyncpackError,
    instance::InstanceIdx,
    registry::updates::RegistryUpdates,
    source::Source,
    version_group::VersionGroup,
  },
  log::{error, warn},
};

pub fn run<D: DiskIo>(mut ctx: Context, registry_updates: RegistryUpdates, io: &D) -> Result<Context, SyncpackError> {
  let mut was_outdated = false;
  // Apply-time dispatch reads `instance.source_idx()`; no SourceIdx is
  // duplicated into the action tuple. Catalog defs (pnpm yaml or Bun root
  // pkg.json) and regular package.json instances flow through the same
  // `ctx.sources.all[idx]` write path.
  let mut copy_actions: Vec<InstanceIdx> = vec![];

  ctx
    .version_groups
    .iter()
    .filter(|group| matches!(group, VersionGroup::PreferredSemver(g) if g.prefer_highest) || matches!(group, VersionGroup::CatalogDefs(_)))
    .for_each(|group| {
      let mut has_printed_group = false;
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        let mut has_printed_dependency = false;
        dependency
          .get_sorted_instances(&ctx.instances, &ctx.sources.all)
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
              copy_actions.push(idx);
            }
          });
      })
    });

  for inst_idx in copy_actions {
    let source_idx = ctx.instances[inst_idx.0].source_idx();
    let consumer_file_idx: Option<usize> = match &ctx.sources.all[source_idx.0] {
      Source::Package { file_idx, .. } => Some(*file_idx),
      Source::PnpmYaml => None,
    };
    let instance = &ctx.instances[inst_idx.0];
    if let Some(fi) = consumer_file_idx {
      copy_expected_specifier_json(&mut ctx.disk.package_json_files[fi], instance);
    } else if let Some(yaml) = ctx.disk.pnpm_workspace.as_mut() {
      copy_expected_specifier_yaml(yaml, instance);
    }
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
    let fallback = ctx.disk.formatting_fallback();
    for file in ctx.disk.package_json_files.iter_mut() {
      write_json_file(file, io, indent, &fallback)?;
    }
    if let Some(yaml) = ctx.disk.pnpm_workspace.as_mut() {
      write_yaml_file(yaml, io, indent, &fallback)?;
    }
  }

  Ok(ctx)
}
