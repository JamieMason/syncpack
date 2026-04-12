use crate::{
  commands::reporter::FixReporter, context::Context, disk::DiskIo, errors::SyncpackError, instance::InstanceIdx,
  version_group::VersionGroupBehavior,
};

pub fn run<D: DiskIo>(mut ctx: Context, reporter: &dyn FixReporter, io: &D) -> Result<Context, SyncpackError> {
  let mut contains_unfixable_issues = false;
  let mut was_invalid = false;
  let mut fix_actions: Vec<(usize, InstanceIdx, bool)> = vec![];

  ctx
    .version_groups
    .iter()
    .filter(|group| !group.dependencies().is_empty() && !group.is_ignored())
    .for_each(|group| {
      let mut has_printed_group = false;
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        let mut has_printed_dependency = false;
        dependency
          .get_sorted_instances(&ctx.instances, &ctx.packages.all)
          .inspect(|(_, instance)| {
            if instance.is_unfixable() || instance.is_suspect() && ctx.config.rcfile.strict {
              contains_unfixable_issues = true
            }
          })
          .filter(|(_, instance)| instance.is_fixable())
          .for_each(|(idx, instance)| {
            was_invalid = true;
            if !has_printed_group {
              reporter.on_group_header(&ctx, group);
              has_printed_group = true;
            }
            if !has_printed_dependency {
              reporter.on_dependency(&ctx, dependency, group.variant_label());
              has_printed_dependency = true;
            }
            reporter.on_instance(&ctx, instance, group.variant_label());
            fix_actions.push((instance.descriptor.package_idx.0, idx, instance.is_banned()));
          });
      })
    });

  for (pkg_idx, inst_idx, is_banned) in fix_actions {
    let instance = &ctx.instances[inst_idx.0];
    let package = &mut ctx.packages.all[pkg_idx];
    if is_banned {
      instance.remove(package);
    } else {
      package.copy_expected_specifier(instance);
    }
  }

  if !ctx.config.cli.dry_run {
    let indent = ctx.config.rcfile.indent.as_deref();
    let formatting = &ctx.packages.formatting;
    for package in ctx.packages.all.iter_mut() {
      package.write_to_disk(io, indent, formatting)?;
    }
  }

  if contains_unfixable_issues {
    reporter.on_unfixable_warning();
  }

  if !contains_unfixable_issues && !was_invalid {
    reporter.on_no_issues();
  }

  if contains_unfixable_issues {
    Err(SyncpackError::IssuesFound)
  } else {
    Ok(ctx)
  }
}
