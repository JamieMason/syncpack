use {
  serde_json::Value,
  std::{collections::HashSet, path::PathBuf, rc::Rc},
  syncpack_disk::{Disk, package_name},
  syncpack_io::json_view,
  syncpack_model::{
    instance::InstanceDescriptor,
    source::Source,
    sources::{SourceIdx, Sources},
  },
  syncpack_specifier::Specifier,
  syncpack_vocab::{
    dependency_type::{DependencyType, Strategy},
    source_kind::SourceKind,
  },
};

/// Build the sources arena by mirroring `disk.package_json_files` 1:1. Each
/// file becomes one `Source::Package { file_idx, name }`. `user_source_indices`
/// is computed by matching each file's `filepath` against the user-pattern-
/// filtered `file_paths`. Appends `Source::PnpmYaml` at the tail when
/// `disk.pnpm_workspace` is `Some`. Discovery step — reads Disk, so it lives in
/// the bin.
pub fn sources_from_disk(disk: &Disk, file_paths: &[PathBuf]) -> Sources {
  let mut all: Vec<Source> = Vec::with_capacity(disk.package_json_files.len() + 1);
  let mut user_source_indices: Vec<usize> = Vec::new();
  for (file_idx, file) in disk.package_json_files.iter().enumerate() {
    let name = package_name(file).to_string();
    all.push(Source::Package { file_idx, name });
    if file_paths.iter().any(|p| p == &file.filepath) {
      user_source_indices.push(file_idx);
    }
  }
  let mut pnpm_yaml_source_idx = None;
  if disk.pnpm_workspace.is_some() {
    pnpm_yaml_source_idx = Some(all.len());
    all.push(Source::PnpmYaml);
  }
  Sources {
    all,
    user_source_indices,
    pnpm_yaml_source_idx,
  }
}

/// Two-pass instance discovery over the sources arena. Catalog dep types iterate
/// the canonical source regardless of user pattern membership; non-catalog dep
/// types iterate only `user_source_indices` (yaml is always included for
/// PnpmWorkspace dep types when yaml exists).
///
/// Pass 1 (catalog dep types): walk `dep_types` where `is_catalog_definition`.
/// PackageJson catalogs read from `disk.package_json_root_idx`'s slot;
/// PnpmWorkspace catalogs read from the pnpm yaml.
///
/// Pass 2 (non-catalog dep types): walk `user_source_indices` for PackageJson
/// dep types; walk yaml for PnpmWorkspace dep types.
///
/// Discovery step — reads Disk, so it lives in the bin.
pub fn iter_instances(sources: &Sources, disk: &Disk, dep_types: &[DependencyType]) -> Vec<InstanceDescriptor> {
  // Local package names are derived from package.json sources only — the
  // yaml's synthetic "name" must not poison `is_local_dependency`.
  let local_package_names: HashSet<String> = sources
    .all
    .iter()
    .filter_map(|s| match s {
      Source::Package { name, .. } => Some(name.clone()),
      Source::PnpmYaml => None,
    })
    .collect();

  // ONE yaml→json conversion per call, owned, borrowed by both passes.
  let yaml_json: Option<Value> = disk.pnpm_workspace.as_ref().map(json_view);

  // Wrap each dep type in an Rc once. Descriptors built within this call
  // share the Rc allocation; the rcfile's `Vec<DependencyType>` is never
  // converted in place.
  let dep_types_rc: Vec<Rc<DependencyType>> = dep_types.iter().map(|dt| Rc::new(dt.clone())).collect();

  let mut out: Vec<InstanceDescriptor> = Vec::new();

  // Pass 1: catalog dep types iterate the canonical source.
  for dep_type_rc in dep_types_rc.iter().filter(|d| d.is_catalog_definition) {
    match dep_type_rc.source {
      SourceKind::PackageJson => {
        // Bun catalogs always live in the root pkg.json.
        if let Some(root_idx) = disk.package_json_root_idx
          && let Some(source_idx) = sources
            .all
            .iter()
            .position(|s| matches!(s, Source::Package { file_idx, .. } if *file_idx == root_idx))
        {
          let root_file = &disk.package_json_files[root_idx];
          collect_descriptors_for_dep_type(
            dep_type_rc,
            &root_file.contents,
            SourceIdx(source_idx),
            &local_package_names,
            &mut out,
          );
        }
      }
      SourceKind::PnpmWorkspace => {
        if let (Some(yaml), Some(yaml_idx)) = (yaml_json.as_ref(), sources.pnpm_yaml_source_idx) {
          collect_descriptors_for_dep_type(dep_type_rc, yaml, SourceIdx(yaml_idx), &local_package_names, &mut out);
        }
      }
    }
  }

  // Pass 2: non-catalog dep types iterate user_source_indices (or yaml
  // when source is PnpmWorkspace).
  for dep_type_rc in dep_types_rc.iter().filter(|d| !d.is_catalog_definition) {
    match dep_type_rc.source {
      SourceKind::PackageJson => {
        for (source_idx, source) in sources.all.iter().enumerate() {
          let Source::Package { file_idx, .. } = source else { continue };
          if !sources.user_source_indices.contains(file_idx) {
            continue;
          }
          let file = &disk.package_json_files[*file_idx];
          collect_descriptors_for_dep_type(dep_type_rc, &file.contents, SourceIdx(source_idx), &local_package_names, &mut out);
        }
      }
      SourceKind::PnpmWorkspace => {
        if let (Some(yaml), Some(yaml_idx)) = (yaml_json.as_ref(), sources.pnpm_yaml_source_idx) {
          collect_descriptors_for_dep_type(dep_type_rc, yaml, SourceIdx(yaml_idx), &local_package_names, &mut out);
        }
      }
    }
  }

  out
}

/// Build an `InstanceDescriptor`. Stores `source_idx` directly (no enum
/// tagging) and shares the dep type via `Rc`. Catalog vs non-catalog branching
/// is read off `dependency_type.is_catalog_definition` at downstream use sites
/// (`Instance::is_catalog_instance`, `catalog_name`).
fn build_descriptor(
  dependency_type: &Rc<DependencyType>,
  name: &str,
  raw_specifier: &str,
  source_idx: SourceIdx,
  local_package_names: &HashSet<String>,
) -> InstanceDescriptor {
  InstanceDescriptor {
    dependency_type: Rc::clone(dependency_type),
    internal_name: name.to_string(),
    is_local_dependency: local_package_names.contains(name),
    name: name.to_string(),
    source_idx,
    specifier: Specifier::new(raw_specifier),
  }
}

/// Drive a single dep type against a single source's contents and append every
/// emitted descriptor onto `out`. Reads `contents` via JSON pointers per the
/// dep type's strategy.
fn collect_descriptors_for_dep_type(
  dep_type: &Rc<DependencyType>,
  contents: &Value,
  source_idx: SourceIdx,
  local_package_names: &HashSet<String>,
  out: &mut Vec<InstanceDescriptor>,
) {
  match dep_type.strategy {
    Strategy::NameAndVersionProps => {
      let name_path = dep_type.name_path.as_ref().expect("NameAndVersionProps requires name_path");
      let name_val = contents.pointer(name_path);
      let version_val = contents.pointer(&dep_type.path);
      let resolved_specifier: Option<&str> = match version_val {
        Some(Value::String(s)) => Some(s.as_str()),
        None if dep_type.name == "local" => Some(""),
        _ => None,
      };
      if let (Some(Value::String(name)), Some(raw_specifier)) = (name_val, resolved_specifier) {
        out.push(build_descriptor(dep_type, name, raw_specifier, source_idx, local_package_names));
      }
    }
    Strategy::NamedVersionString => {
      if let Some(Value::String(specifier)) = contents.pointer(&dep_type.path)
        && let Some((name, raw_specifier)) = specifier.split_once('@')
      {
        out.push(build_descriptor(dep_type, name, raw_specifier, source_idx, local_package_names));
      }
    }
    Strategy::UnnamedVersionString => {
      if let Some(Value::String(raw_specifier)) = contents.pointer(&dep_type.path) {
        out.push(build_descriptor(
          dep_type,
          &dep_type.name,
          raw_specifier,
          source_idx,
          local_package_names,
        ));
      }
    }
    Strategy::VersionsByName => {
      if let Some(Value::Object(versions_by_name)) = contents.pointer(&dep_type.path) {
        for (name, raw_specifier) in versions_by_name {
          if let Value::String(raw_specifier) = raw_specifier {
            out.push(build_descriptor(dep_type, name, raw_specifier, source_idx, local_package_names));
          }
        }
      }
    }
    Strategy::InvalidConfig => {
      unreachable!("unrecognised strategy");
    }
  }
}
