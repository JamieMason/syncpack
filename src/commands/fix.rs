use crate::{
  commands::reporter::FixReporter,
  context::{Context, SyncpackError},
  version_group::VersionGroupBehavior,
};

pub fn run(ctx: Context, reporter: &dyn FixReporter) -> Result<Context, SyncpackError> {
  let mut contains_unfixable_issues = false;
  let mut was_invalid = false;

  ctx
    .version_groups
    .iter()
    .filter(|group| !group.dependencies().is_empty() && !group.is_ignored())
    .for_each(|group| {
      let mut has_printed_group = false;
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        let mut has_printed_dependency = false;
        dependency
          .get_sorted_instances(&ctx.instances)
          .inspect(|instance| {
            if instance.is_unfixable() || instance.is_suspect() && ctx.config.rcfile.strict {
              contains_unfixable_issues = true
            }
          })
          .filter(|instance| instance.is_fixable())
          .for_each(|instance| {
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
            if instance.is_banned() {
              instance.remove()
            } else {
              instance.descriptor.package.borrow().copy_expected_specifier(instance);
            }
          });
      })
    });

  if !ctx.config.cli.dry_run {
    ctx.packages.all.iter().for_each(|package| {
      package
        .borrow()
        .write_to_disk(ctx.config.rcfile.indent.as_deref(), &ctx.packages.formatting);
    });
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
