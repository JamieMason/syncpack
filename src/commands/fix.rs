#[cfg(test)]
#[path = "fix_test.rs"]
mod fix_test;

use {
  crate::{
    catalogs::detect_bun_catalogs,
    commands::{InstanceAction, action_of, reporter::FixReporter},
    context::{Discovered, Inspected},
    disk::{copy_expected_specifier_json, insert_catalog_definition},
    errors::SyncpackError,
    inspection::Inspection,
    instance::{FixableInstance, InstanceIdx, InstanceState, InvalidInstance},
    source::Source,
    specifier::Specifier,
    version_group::VersionGroupBehavior,
  },
  serde_json::Value as JsonValue,
  std::rc::Rc,
  syncpack_io::{DiskIo, empty_yaml_file, ensure_object_path, set_nested_prop, write_json_file, write_yaml_file},
  syncpack_vocab::{package_manager::PackageManager, severity::Severity},
};

pub fn run<D: DiskIo>(mut inspected: Inspected, reporter: &dyn FixReporter, io: &D) -> Result<Inspected, SyncpackError> {
  let mut contains_unfixable_issues = false;
  let mut was_invalid = false;
  // Apply-time dispatch reads `instance.descriptor.source_idx`; no SourceIdx
  // is duplicated into the action tuple.
  let mut fix_actions: Vec<(InstanceIdx, bool)> = vec![];

  {
    let data = &inspected.data;
    let inspection = &inspected.inspection;
    data
      .version_groups
      .iter()
      .filter(|group| !group.dependency_indices().is_empty() && !group.is_ignored())
      .for_each(|group| {
        let mut has_printed_group = false;
        for dep_idx in group.get_sorted_dependencies(&data.dependencies, &data.config.cli.sort) {
          let dependency = &data.dependencies[dep_idx.0];
          let mut has_printed_dependency = false;
          for (idx, instance) in crate::version_group::get_sorted_instances(dependency, inspection, &data.instances, &data.sources.all) {
            let action = action_of(inspection.severity(idx).unwrap_or(Severity::None));
            // `Render(Error)` from non-fixable states (Unfixable, Conflict,
            // strict-Suspect, user-configured Error on Fixable) cannot be
            // auto-fixed in this pass.
            if matches!(action, InstanceAction::Render(Severity::Error)) {
              contains_unfixable_issues = true;
            }
            // Only `Fix(_)` instances are applied. `Render(Warn)` surfaces but
            // does not change state; `Skip` is silent.
            if !matches!(action, InstanceAction::Fix(_)) {
              continue;
            }
            was_invalid = true;
            if !has_printed_group {
              reporter.on_group_header(data, group);
              has_printed_group = true;
            }
            if !has_printed_dependency {
              reporter.on_dependency(data, inspection, dep_idx, dependency, group.variant_label());
              has_printed_dependency = true;
            }
            reporter.on_instance(data, inspection, idx, instance, group.variant_label());
            fix_actions.push((idx, inspection.state(idx).is_banned()));
          }
        }
      });
  }

  apply_fix_actions(&mut inspected.data, &inspected.inspection, &fix_actions);

  if !inspected.data.config.cli.dry_run {
    let indent = inspected.data.config.rcfile.indent.as_deref();
    let fallback = inspected.data.disk.formatting_fallback();
    for file in inspected.data.disk.package_json_files.iter_mut() {
      write_json_file(file, io, indent, &fallback)?;
    }
    if let Some(yaml) = inspected.data.disk.pnpm_workspace.as_mut() {
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
    Ok(inspected)
  }
}

/// Apply the recorded fix actions in the order they were collected.
/// `MissingFromCatalog` consumers ALSO trigger an upstream insert into the
/// target catalog source (pnpm yaml on Disk, or Bun root pkg.json via
/// `disk.package_json_root_idx`).
fn apply_fix_actions(data: &mut Discovered, inspection: &Inspection, actions: &[(InstanceIdx, bool)]) {
  for &(inst_idx, is_banned) in actions {
    let state = inspection.state(inst_idx).clone();

    // `MissingFromCatalog` insert runs BEFORE the consumer rewrite so an
    // empty pnpm-workspace.yaml can be auto-created on the way through.
    if let InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::MissingFromCatalog {
      catalog_name,
      winning_specifier,
    })) = &state
    {
      apply_missing_from_catalog_insert(data, inst_idx, catalog_name, winning_specifier);
    }

    let consumer_source_idx = data.instances[inst_idx.0].source_idx();
    // Copy the file_idx (Package) out of the source under an immutable
    // borrow; the borrow ends before we mutate data.disk below.
    let consumer_file_idx: Option<usize> = match &data.sources.all[consumer_source_idx.0] {
      Source::Package { file_idx, .. } => Some(*file_idx),
      Source::PnpmYaml => None,
    };
    if is_banned {
      if let Some(fi) = consumer_file_idx {
        let file = &mut data.disk.package_json_files[fi];
        let instance = &data.instances[inst_idx.0];
        remove_instance_from_disk(file, instance);
      } else if let Some(yaml) = data.disk.pnpm_workspace.as_mut() {
        let instance = &data.instances[inst_idx.0];
        if let Some(catalog_name) = instance.catalog_name() {
          crate::disk::remove_catalog_definition(yaml, catalog_name, &instance.descriptor.name);
        }
      }
    } else if let Some(expected) = inspection.expected(inst_idx) {
      if let Some(fi) = consumer_file_idx {
        let file = &mut data.disk.package_json_files[fi];
        let instance = &data.instances[inst_idx.0];
        copy_expected_specifier_json(file, &instance.descriptor.dependency_type, &instance.descriptor.name, expected);
      } else if let Some(yaml) = data.disk.pnpm_workspace.as_mut() {
        let instance = &data.instances[inst_idx.0];
        crate::disk::copy_expected_specifier_yaml(
          yaml,
          &instance.descriptor.dependency_type,
          &instance.descriptor.name,
          instance.catalog_name(),
          expected,
        );
      }
    }
  }
}

/// Insert `dep_name → winning_specifier` into the right catalog source
/// before the consumer rewrite runs. Auto-creates the implicit default
/// catalog file when none exists and PM=Pnpm/Bun. Routing follows
/// `data.disk.package_manager`.
fn apply_missing_from_catalog_insert(data: &mut Discovered, inst_idx: InstanceIdx, catalog_name: &str, winning_specifier: &Rc<Specifier>) {
  let dep_name = data.instances[inst_idx.0].descriptor.name.clone();

  match data.disk.package_manager {
    Some(PackageManager::Pnpm) => {
      if data.disk.pnpm_workspace.is_none() {
        let path = data.disk.cwd.join("pnpm-workspace.yaml");
        data.disk.pnpm_workspace = Some(empty_yaml_file(path));
      }
      let yaml = data.disk.pnpm_workspace.as_mut().unwrap();
      insert_catalog_definition(yaml, catalog_name, &dep_name, winning_specifier);
    }
    Some(PackageManager::Bun) => {
      let Some(idx) = data.disk.package_json_root_idx else {
        return;
      };
      // Bun prefix detection: shared with discovery via the same helper.
      // Falls back to "" when no Bun catalogs exist yet (auto-create-default).
      let prefix = detect_bun_catalogs(&data.disk.package_json_files[idx].contents)
        .ok()
        .flatten()
        .map(|(p, _)| p)
        .unwrap_or("");
      let pkg = &mut data.disk.package_json_files[idx];
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
fn remove_instance_from_disk(file: &mut syncpack_io::File<JsonValue>, instance: &crate::instance::Instance) {
  use syncpack_vocab::dependency_type::Strategy;
  match instance.descriptor.dependency_type.strategy {
    Strategy::NameAndVersionProps | Strategy::NamedVersionString | Strategy::UnnamedVersionString => {
      let path_to_prop = &instance.descriptor.dependency_type.path;
      if let Some(parent_path) = path_to_prop.rfind('/') {
        let parent_pointer = &path_to_prop[..parent_path];
        let prop_name = &path_to_prop[parent_path + 1..];
        syncpack_io::remove_prop(file, parent_pointer, prop_name);
      }
    }
    Strategy::VersionsByName => {
      let path_to_obj = &instance.descriptor.dependency_type.path;
      syncpack_io::remove_prop(file, path_to_obj, &instance.descriptor.name);
    }
    Strategy::InvalidConfig => unreachable!("unrecognised strategy"),
  }
}
