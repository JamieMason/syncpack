use {
  crate::{
    dependency::{DependencyType, Strategy},
    disk::{Disk, json_view, package_name},
    instance::InstanceDescriptor,
    source::{Source, SourceKind},
    specifier::Specifier,
  },
  serde_json::Value,
  std::{collections::HashSet, path::PathBuf, rc::Rc},
};

#[cfg(test)]
#[path = "sources_test.rs"]
mod sources_test;

/// Index into the `Sources.all` arena.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SourceIdx(pub usize);

/// Owned arena of every `Source` (package.json + pnpm yaml) in the project.
///
/// `Source::Package` carries an index into `disk.package_json_files`;
/// reads/writes route through the Disk. `user_source_indices` lists the slots
/// in `all` whose backing file matches a user `source` pattern; iteration
/// uses this to filter non-catalog dep types in pass 2 (catalog dep types in
/// pass 1 always iterate the canonical source regardless of user pattern
/// membership).
#[derive(Debug)]
pub struct Sources {
  pub all: Vec<Source>,
  /// Slots in `all` whose file matches a user-configured `source` pattern.
  /// Pass 2 of `iter_instances` filters by membership in this list. Yaml is
  /// always iterable when present (pnpm catalogs are not user-pattern
  /// controlled).
  pub user_source_indices: Vec<usize>,
  /// Slot in `all` holding the `Source::PnpmYaml` unit variant for the
  /// parse-time yaml. `None` if no yaml exists at parse time. Auto-created
  /// yaml at fix-time is invisible to sources — it lives only on Disk.
  pub pnpm_yaml_source_idx: Option<usize>,
}

impl Sources {
  pub fn new() -> Self {
    Self {
      all: vec![],
      user_source_indices: vec![],
      pnpm_yaml_source_idx: None,
    }
  }

  /// Build the sources arena by mirroring `disk.package_json_files` 1:1.
  /// Each file becomes one `Source::Package { file_idx, name,
  /// formatting_mismatches: [] }`. `user_source_indices` is computed by
  /// matching each file's `filepath` against the user-pattern-filtered
  /// `file_paths`. Appends `Source::PnpmYaml` at the tail when
  /// `disk.pnpm_workspace` is `Some`.
  pub fn from_disk(disk: &Disk, file_paths: &[PathBuf]) -> Self {
    let mut all: Vec<Source> = Vec::with_capacity(disk.package_json_files.len() + 1);
    let mut user_source_indices: Vec<usize> = Vec::new();
    for (file_idx, file) in disk.package_json_files.iter().enumerate() {
      let name = package_name(file).to_string();
      all.push(Source::Package {
        file_idx,
        name,
        formatting_mismatches: vec![],
      });
      if file_paths.iter().any(|p| p == &file.filepath) {
        user_source_indices.push(file_idx);
      }
    }
    let mut pnpm_yaml_source_idx = None;
    if disk.pnpm_workspace.is_some() {
      pnpm_yaml_source_idx = Some(all.len());
      all.push(Source::PnpmYaml);
    }
    Self {
      all,
      user_source_indices,
      pnpm_yaml_source_idx,
    }
  }

  /// Append a source to the arena and return its index. Test-only helper
  /// that bypasses `user_source_indices` bookkeeping.
  pub fn add_source(&mut self, source: Source) -> SourceIdx {
    let idx = SourceIdx(self.all.len());
    self.all.push(source);
    idx
  }

  /// Iterate the package.json sources only. Pnpm yaml is excluded.
  /// Used by `format`, `find_package`, name lookups.
  pub fn packages_iter(&self) -> impl Iterator<Item = (SourceIdx, &Source)> {
    self.all.iter().enumerate().filter_map(|(i, s)| match s {
      Source::Package { .. } => Some((SourceIdx(i), s)),
      Source::PnpmYaml => None,
    })
  }

  /// Look up a package.json source by its package name.
  pub fn find_package(&self, name: &str) -> Option<SourceIdx> {
    self.packages_iter().find(|(_, s)| s.name() == name).map(|(idx, _)| idx)
  }

  /// Slot in `all` holding the parse-time PnpmYaml entry, if any.
  pub fn find_pnpm_yaml_idx(&self) -> Option<SourceIdx> {
    self.pnpm_yaml_source_idx.map(SourceIdx)
  }

  /// Two-pass iteration: catalog dep types iterate the canonical source
  /// regardless of user pattern membership; non-catalog dep types iterate
  /// only `user_source_indices` (yaml is always included for PnpmWorkspace
  /// dep types when yaml exists).
  ///
  /// Pass 1 (catalog dep types): walk `dep_types` where
  /// `is_catalog_definition`. PackageJson catalogs read from
  /// `disk.package_json_root_idx`'s slot; PnpmWorkspace catalogs read from
  /// the pnpm yaml.
  ///
  /// Pass 2 (non-catalog dep types): walk `user_source_indices` for
  /// PackageJson dep types; walk yaml for PnpmWorkspace dep types.
  pub fn iter_instances<'a>(&'a self, disk: &'a Disk, dep_types: &'a [DependencyType]) -> impl Iterator<Item = InstanceDescriptor> + 'a {
    // Local package names are derived from package.json sources only — the
    // yaml's synthetic "name" must not poison `is_local_dependency`.
    let local_package_names: HashSet<String> = self
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
            && let Some(source_idx) = self
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
          if let (Some(yaml), Some(yaml_idx)) = (yaml_json.as_ref(), self.pnpm_yaml_source_idx) {
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
          for (source_idx, source) in self.all.iter().enumerate() {
            let Source::Package { file_idx, .. } = source else { continue };
            if !self.user_source_indices.contains(file_idx) {
              continue;
            }
            let file = &disk.package_json_files[*file_idx];
            collect_descriptors_for_dep_type(dep_type_rc, &file.contents, SourceIdx(source_idx), &local_package_names, &mut out);
          }
        }
        SourceKind::PnpmWorkspace => {
          if let (Some(yaml), Some(yaml_idx)) = (yaml_json.as_ref(), self.pnpm_yaml_source_idx) {
            collect_descriptors_for_dep_type(dep_type_rc, yaml, SourceIdx(yaml_idx), &local_package_names, &mut out);
          }
        }
      }
    }

    out.into_iter()
  }
}

impl Default for Sources {
  fn default() -> Self {
    Self::new()
  }
}

/// Extract the catalog name suffix from a built-in catalog dep type name.
/// Bare `pnpmCatalog` / `bunCatalog` → `"default"`. Suffixed
/// `pnpmCatalog:react18` → `"react18"`. Dots in the suffix are preserved
/// because dep type names are field-literal constructed. Returns a borrowed
/// `&str` (no allocation).
pub fn parse_catalog_name(dep_type_name: &str) -> &str {
  dep_type_name.split_once(':').map(|(_, suffix)| suffix).unwrap_or("default")
}

/// Build an `InstanceDescriptor`. Stores `source_idx` directly (no enum
/// tagging) and shares the dep type via `Rc`. Catalog vs non-catalog
/// branching is read off `dependency_type.is_catalog_definition` at
/// downstream use sites (`Instance::is_catalog_instance`, `catalog_name`).
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

/// Drive a single dep type against a single source's contents and append
/// every emitted descriptor onto `out`. Reads `contents` via JSON pointers
/// per the dep type's strategy.
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
