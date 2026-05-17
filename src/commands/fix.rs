#[cfg(test)]
#[path = "fix_test.rs"]
mod fix_test;

use {
  crate::{
    catalogs::detect_bun_catalogs,
    commands::reporter::FixReporter,
    context::Context,
    disk::{
      DiskIo, PackageManager, copy_expected_specifier_json, empty_yaml_file, ensure_object_path, insert_catalog_definition,
      set_nested_prop, write_json_file, write_yaml_file,
    },
    errors::SyncpackError,
    instance::{FixableInstance, InstanceIdx, InstanceState, InvalidInstance, Severity},
    source::Source,
    specifier::Specifier,
    version_group::{InstanceAction, VersionGroupBehavior},
  },
  serde_json::Value as JsonValue,
  std::rc::Rc,
};

pub fn run<D: DiskIo>(mut ctx: Context, reporter: &dyn FixReporter, io: &D) -> Result<Context, SyncpackError> {
  let mut contains_unfixable_issues = false;
  let mut was_invalid = false;
  let strict = ctx.config.rcfile.strict;
  // Apply-time dispatch reads `instance.descriptor.source_idx`; no SourceIdx
  // is duplicated into the action tuple.
  let mut fix_actions: Vec<(InstanceIdx, bool)> = vec![];

  ctx
    .version_groups
    .iter()
    .filter(|group| !group.dependencies().is_empty() && !group.is_ignored())
    .for_each(|group| {
      let mut has_printed_group = false;
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        let mut has_printed_dependency = false;
        dependency
          .get_sorted_instances(&ctx.instances, &ctx.sources.all)
          .for_each(|(idx, instance)| {
            let action = group.resolve_action(instance, strict);
            // `Render(Error)` from non-fixable states (Unfixable, Conflict,
            // strict-Suspect, user-configured Error on Fixable) cannot be
            // auto-fixed in this pass.
            if matches!(action, InstanceAction::Render(Severity::Error)) {
              contains_unfixable_issues = true;
            }
            // Only `Fix(_)` instances are applied. `Render(Warn)` surfaces but
            // does not change state; `Skip` is silent.
            if !matches!(action, InstanceAction::Fix(_)) {
              return;
            }
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
            fix_actions.push((idx, instance.is_banned()));
          });
      })
    });

  apply_fix_actions(&mut ctx, &fix_actions);

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

/// Apply the recorded fix actions in the order they were collected.
/// `MissingFromCatalog` consumers ALSO trigger an upstream insert into the
/// target catalog source (pnpm yaml on Disk, or Bun root pkg.json via
/// `disk.package_json_root_idx`).
fn apply_fix_actions(ctx: &mut Context, actions: &[(InstanceIdx, bool)]) {
  for &(inst_idx, is_banned) in actions {
    let state = ctx.instances[inst_idx.0].state.borrow().clone();

    // `MissingFromCatalog` insert runs BEFORE the consumer rewrite so an
    // empty pnpm-workspace.yaml can be auto-created on the way through.
    if let InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::MissingFromCatalog {
      catalog_name,
      winning_specifier,
    })) = &state
    {
      apply_missing_from_catalog_insert(ctx, inst_idx, catalog_name, winning_specifier);
    }

    let consumer_source_idx = ctx.instances[inst_idx.0].source_idx();
    // Copy the file_idx (Package) out of the source under an immutable
    // borrow; the borrow ends before we mutate ctx.disk below.
    let consumer_file_idx: Option<usize> = match &ctx.sources.all[consumer_source_idx.0] {
      Source::Package { file_idx, .. } => Some(*file_idx),
      Source::PnpmYaml => None,
    };
    if is_banned {
      if let Some(fi) = consumer_file_idx {
        let file = &mut ctx.disk.package_json_files[fi];
        let instance = &ctx.instances[inst_idx.0];
        remove_instance_from_disk(file, instance);
      } else if let Some(yaml) = ctx.disk.pnpm_workspace.as_mut() {
        let instance = &ctx.instances[inst_idx.0];
        if let Some(catalog_name) = instance.catalog_name() {
          crate::disk::remove_catalog_definition(yaml, catalog_name, &instance.descriptor.name);
        }
      }
    } else if let Some(fi) = consumer_file_idx {
      let file = &mut ctx.disk.package_json_files[fi];
      let instance = &ctx.instances[inst_idx.0];
      copy_expected_specifier_json(file, instance);
    } else {
      // PnpmYaml source — route through disk.pnpm_workspace.
      if let Some(yaml) = ctx.disk.pnpm_workspace.as_mut() {
        let instance = &ctx.instances[inst_idx.0];
        crate::disk::copy_expected_specifier_yaml(yaml, instance);
      }
    }
  }
}

/// Insert `dep_name → winning_specifier` into the right catalog source
/// before the consumer rewrite runs. Auto-creates the implicit default
/// catalog file when none exists and PM=Pnpm/Bun. Routing follows
/// `ctx.disk.package_manager`.
fn apply_missing_from_catalog_insert(ctx: &mut Context, inst_idx: InstanceIdx, catalog_name: &str, winning_specifier: &Rc<Specifier>) {
  let dep_name = ctx.instances[inst_idx.0].descriptor.name.clone();

  match ctx.disk.package_manager {
    Some(PackageManager::Pnpm) => {
      if ctx.disk.pnpm_workspace.is_none() {
        let path = ctx.disk.cwd.join("pnpm-workspace.yaml");
        ctx.disk.pnpm_workspace = Some(empty_yaml_file(path));
      }
      let yaml = ctx.disk.pnpm_workspace.as_mut().unwrap();
      insert_catalog_definition(yaml, catalog_name, &dep_name, winning_specifier);
    }
    Some(PackageManager::Bun) => {
      let Some(idx) = ctx.disk.package_json_root_idx else {
        return;
      };
      // Bun prefix detection: shared with discovery via the same helper.
      // Falls back to "" when no Bun catalogs exist yet (auto-create-default).
      let prefix = detect_bun_catalogs(&ctx.disk.package_json_files[idx].contents)
        .ok()
        .flatten()
        .map(|(p, _)| p)
        .unwrap_or("");
      let pkg = &mut ctx.disk.package_json_files[idx];
      let parent_pointer = if catalog_name == "default" {
        format!("{prefix}/catalog")
      } else {
        format!("{prefix}/catalogs/{catalog_name}")
      };
      ensure_object_path(pkg, &parent_pointer);
      set_nested_prop(
        pkg,
        &parent_pointer,
        &dep_name,
        JsonValue::String(winning_specifier.get_raw().to_string()),
      );
    }
    _ => {}
  }
}

/// Remove an instance's prop from the underlying file.
fn remove_instance_from_disk(file: &mut crate::disk::File<JsonValue>, instance: &crate::instance::Instance) {
  use crate::dependency::Strategy;
  match instance.descriptor.dependency_type.strategy {
    Strategy::NameAndVersionProps | Strategy::NamedVersionString | Strategy::UnnamedVersionString => {
      let path_to_prop = &instance.descriptor.dependency_type.path;
      if let Some(parent_path) = path_to_prop.rfind('/') {
        let parent_pointer = &path_to_prop[..parent_path];
        let prop_name = &path_to_prop[parent_path + 1..];
        crate::disk::remove_prop(file, parent_pointer, prop_name);
      }
    }
    Strategy::VersionsByName => {
      let path_to_obj = &instance.descriptor.dependency_type.path;
      crate::disk::remove_prop(file, path_to_obj, &instance.descriptor.name);
    }
    Strategy::InvalidConfig => unreachable!("unrecognised strategy"),
  }
}
