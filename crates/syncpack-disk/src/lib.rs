#![warn(unreachable_pub)]

use {
  serde_json::Value as JsonValue,
  std::{
    path::{Path, PathBuf},
    rc::Rc,
  },
  syncpack_io::{
    DetectedFormatting, DiskIo, File, PendingYamlOp, YamlFile, ensure_yaml_block, get_or_insert_mapping, parse_json_file, set_nested_prop,
    set_prop,
  },
  syncpack_specifier::Specifier,
  syncpack_vocab::{
    dependency_type::{DependencyType, Strategy},
    package_manager::PackageManager,
  },
  yaml_serde::{Mapping, Value as YamlValue},
};

pub mod catalogs;

#[cfg(test)]
#[path = "disk_test.rs"]
mod disk_test;

/// The owner of all writable files in the workspace
#[derive(Debug)]
pub struct Disk {
  pub cwd: PathBuf,
  pub lerna_json: Option<File<serde_json::Value>>,
  /// Includes the root; `package_json_root_idx` points at it.
  pub package_json_files: Vec<File<serde_json::Value>>,
  pub package_json_root_idx: Option<usize>,
  pub package_manager: Option<PackageManager>,
  pub pnpm_workspace: Option<YamlFile>,
}

impl Disk {
  pub fn from_workspace<T: DiskIo>(io: &T, directory: &Path) -> Self {
    let lerna_json = io.read_json_file(&directory.join("lerna.json"));
    let package_json_root_file = io.read_json_file(&directory.join("package.json"));
    let package_manager = if io.path_exists(&directory.join("pnpm-lock.yaml")) || io.path_exists(&directory.join("pnpm-workspace.yaml")) {
      Some(PackageManager::Pnpm)
    } else if io.path_exists(&directory.join("yarn.lock")) {
      Some(PackageManager::Yarn)
    } else if io.path_exists(&directory.join("package-lock.json")) {
      Some(PackageManager::Npm)
    } else if io.path_exists(&directory.join("bun.lock")) || io.path_exists(&directory.join("bun.lockb")) {
      Some(PackageManager::Bun)
    } else {
      Some(PackageManager::Unknown)
    };
    let pnpm_workspace = if let Some(PackageManager::Pnpm) = package_manager {
      io.read_yaml_file(&directory.join("pnpm-workspace.yaml"))
    } else {
      None
    };

    let mut package_json_files = Vec::new();
    let mut package_json_root_idx = None;
    if let Some(Ok(root_file)) = package_json_root_file {
      package_json_root_idx = Some(package_json_files.len());
      package_json_files.push(root_file);
    }

    Self {
      cwd: directory.to_path_buf(),
      lerna_json: lerna_json.and_then(Result::ok),
      package_json_files,
      package_json_root_idx,
      package_manager,
      pnpm_workspace: pnpm_workspace.and_then(Result::ok),
    }
  }

  /// Borrow the root package.json if one was loaded.
  pub fn package_json_root(&self) -> Option<&File<serde_json::Value>> {
    self.package_json_root_idx.and_then(|i| self.package_json_files.get(i))
  }

  /// Default formatting fallback used when a file's own formatting is empty.
  /// Prefers the root pkg.json's formatting; falls back to defaults.
  pub fn formatting_fallback(&self) -> DetectedFormatting {
    self.package_json_root().map(|f| f.formatting.clone()).unwrap_or_default()
  }

  /// Read every package.json named in `paths`. Skips a path that equals the
  /// root's filepath so the root is not parsed twice. Successfully parsed
  /// files are appended to `package_json_files`. Failures are logged and
  /// skipped.
  pub fn load_package_files<T: DiskIo>(&mut self, io: &T, paths: &[PathBuf]) {
    let root_filepath = self.package_json_root().map(|f| f.filepath.clone());
    for path in paths {
      if root_filepath.as_ref() == Some(path) {
        continue;
      }
      match io.read_textfile(path) {
        Some(Ok(file)) => {
          if let Some(parsed) = parse_json_file(file.contents, file.filepath) {
            self.package_json_files.push(parsed);
          }
        }
        Some(Err(err)) => {
          log::error!("{err}");
        }
        None => {
          log::error!("package.json not readable at {}", path.to_str().unwrap_or("unknown"));
        }
      }
    }
  }
}

/// Read the package's `name` property, falling back to `"NAME_IS_MISSING"`.
pub fn package_name(file: &File<JsonValue>) -> &str {
  file
    .contents
    .pointer("/name")
    .and_then(|name| name.as_str())
    .unwrap_or("NAME_IS_MISSING")
}
/// Apply an instance's `expected_specifier` to the package.json. Dispatches
/// over the dependency type's `Strategy`. Marks the file dirty only when the
/// stored value actually changed.
pub fn copy_expected_specifier_json(file: &mut File<JsonValue>, dependency_type: &DependencyType, name: &str, expected: &Rc<Specifier>) {
  let path_to_prop_str = dependency_type.path.as_str();
  let raw_specifier = expected.get_raw().to_string();
  match dependency_type.strategy {
    Strategy::NameAndVersionProps => {
      set_prop(file, path_to_prop_str, JsonValue::String(raw_specifier));
    }
    Strategy::NamedVersionString => {
      let full_value = format!("{name}@{raw_specifier}");
      set_prop(file, path_to_prop_str, JsonValue::String(full_value));
    }
    Strategy::UnnamedVersionString => {
      set_prop(file, path_to_prop_str, JsonValue::String(raw_specifier));
    }
    Strategy::VersionsByName => {
      set_nested_prop(file, path_to_prop_str, name, JsonValue::String(raw_specifier));
    }
    Strategy::InvalidConfig => {
      unreachable!("unrecognised strategy");
    }
  };
}

/// Where in a `pnpm-workspace.yaml` a dependency definition is written: a flat
/// top-level block such as `catalog:` or `overrides:`, or a named catalog
/// nested under `catalogs.<name>:`.
#[derive(Clone, Copy)]
enum YamlTarget<'a> {
  Block(&'a str),
  NamedCatalog(&'a str),
}

impl<'a> YamlTarget<'a> {
  /// `"default"` is the flat `catalog:` block; any other name nests under
  /// `catalogs.<name>:`.
  fn catalog(catalog_name: &'a str) -> Self {
    match catalog_name {
      "default" => YamlTarget::Block("catalog"),
      name => YamlTarget::NamedCatalog(name),
    }
  }

  /// The path of mapping keys from the document root down to the block that
  /// holds the `dep_name: specifier` entries.
  fn segments(self) -> Vec<String> {
    match self {
      YamlTarget::Block(key) => vec![key.to_string()],
      YamlTarget::NamedCatalog(name) => vec!["catalogs".to_string(), name.to_string()],
    }
  }
}

/// Insert `dep_name → specifier` under the named catalog block. Idempotent —
/// if the value already matches, no mutation occurs and `dirty` stays as it
/// was. Returns `true` when in-memory state changed.
///
/// `catalog_name == "default"` writes under the bare `catalog:` block; any
/// other name writes under `catalogs.{name}`. Each non-idempotent call also
/// pushes a `PendingYamlOp` describing the edit so the write path can replay
/// it via `yamlpatch` and preserve formatting.
pub fn insert_catalog_definition(file: &mut YamlFile, catalog_name: &str, dep_name: &str, specifier: &Rc<Specifier>) -> bool {
  insert_into_target(file, YamlTarget::catalog(catalog_name), dep_name, specifier)
}

/// Insert `dep_name → specifier` into `target`, creating any missing blocks.
/// Idempotent — returns `true` only when in-memory state changed — and records
/// a `PendingYamlOp` so the write path replays the edit via `yamlpatch` and
/// preserves formatting.
fn insert_into_target(file: &mut YamlFile, target: YamlTarget, dep_name: &str, specifier: &Rc<Specifier>) -> bool {
  let next_value = YamlValue::String(specifier.get_raw().to_string());
  // Choose patch shape from PRE-mutation state; `ensure_block` mutates
  // `file.contents` below.
  let pending = build_insert_patch(file, &target.segments(), dep_name, &next_value);
  let block = ensure_block(file, target);
  if block.get(dep_name) == Some(&next_value) {
    return false;
  }
  block.insert(YamlValue::String(dep_name.to_string()), next_value);
  if let Some(op) = pending {
    file.patches.push(op);
  }
  file.dirty = true;
  true
}

/// Descend `segments` from `root`, returning the mapping at that path when
/// every segment exists and is itself a mapping. `segments == []` yields
/// `root` unchanged.
fn descend_to_block<'a>(root: Option<&'a Mapping>, segments: &[String]) -> Option<&'a Mapping> {
  segments
    .iter()
    .fold(root, |node, seg| node.and_then(|m| m.get(seg)).and_then(|v| v.as_mapping()))
}

/// Wrap `dep_name → value` in one nested single-key mapping per `suffix`
/// segment, outermost first. `suffix == []` yields just `{ dep_name: value }`.
fn nest(suffix: &[String], dep_name: &str, value: &YamlValue) -> YamlValue {
  let mut leaf = Mapping::new();
  leaf.insert(YamlValue::String(dep_name.to_string()), value.clone());
  suffix.iter().rev().fold(YamlValue::Mapping(leaf), |acc, seg| {
    let mut wrapper = Mapping::new();
    wrapper.insert(YamlValue::String(seg.clone()), acc);
    YamlValue::Mapping(wrapper)
  })
}

/// Determine the `PendingYamlOp` for writing `dep_name → next_value` into the
/// block at `segments` (e.g. `["catalog"]`, `["overrides"]`, or
/// `["catalogs", name]`), based on the pre-mutation state of `file.contents`.
/// Returns `None` when the existing value already equals `next_value`.
fn build_insert_patch(file: &YamlFile, segments: &[String], dep_name: &str, next_value: &YamlValue) -> Option<PendingYamlOp> {
  let root = file.contents.as_mapping();
  let existing_block = descend_to_block(root, segments);
  let existing_value = existing_block.and_then(|b| b.get(dep_name));
  if existing_value == Some(next_value) {
    return None;
  }
  if existing_value.is_some() {
    let mut path = segments.to_vec();
    path.push(dep_name.to_string());
    return Some(PendingYamlOp::Replace {
      segments: path,
      value: next_value.clone(),
    });
  }
  if existing_block.is_some() {
    return Some(PendingYamlOp::Add {
      segments: segments.to_vec(),
      key: dep_name.to_string(),
      value: next_value.clone(),
    });
  }
  // No block at `segments` yet: add the missing suffix below the deepest
  // ancestor mapping that does exist.
  let depth = (0..segments.len())
    .rev()
    .find(|&i| descend_to_block(root, &segments[..i]).is_some())
    .unwrap_or(0);
  Some(PendingYamlOp::Add {
    segments: segments[..depth].to_vec(),
    key: segments[depth].clone(),
    value: nest(&segments[depth + 1..], dep_name, next_value),
  })
}

/// Remove `dep_name` from the named catalog block. If removal empties the
/// block, the block (and `catalogs` parent map when emptied of all named
/// catalogs) is also pruned. Returns `true` when anything was removed.
///
/// Each remove also pushes one or more `PendingYamlOp::Remove` entries:
/// the dep first, then any parent that became empty as a result. This
/// keeps the `yamlpatch` replay aligned with the in-memory state.
pub fn remove_catalog_definition(file: &mut YamlFile, catalog_name: &str, dep_name: &str) -> bool {
  let key_dep = YamlValue::String(dep_name.to_string());
  let key_name = YamlValue::String(catalog_name.to_string());
  let YamlValue::Mapping(ref mut root) = file.contents else {
    return false;
  };
  let mut new_patches: Vec<PendingYamlOp> = Vec::new();
  let removed = if catalog_name == "default" {
    let block = root.get_mut("catalog").and_then(|v| v.as_mapping_mut());
    let Some(block) = block else { return false };
    let removed = block.remove(&key_dep).is_some();
    if removed {
      new_patches.push(PendingYamlOp::Remove {
        segments: vec!["catalog".to_string(), dep_name.to_string()],
      });
    }
    if removed && block.is_empty() {
      root.remove("catalog");
      new_patches.push(PendingYamlOp::Remove {
        segments: vec!["catalog".to_string()],
      });
    }
    removed
  } else {
    let catalogs = root.get_mut("catalogs").and_then(|v| v.as_mapping_mut());
    let Some(catalogs) = catalogs else { return false };
    let block = catalogs.get_mut(&key_name).and_then(|v| v.as_mapping_mut());
    let Some(block) = block else { return false };
    let removed = block.remove(&key_dep).is_some();
    if removed {
      new_patches.push(PendingYamlOp::Remove {
        segments: vec!["catalogs".to_string(), catalog_name.to_string(), dep_name.to_string()],
      });
    }
    if removed && block.is_empty() {
      catalogs.remove(&key_name);
      new_patches.push(PendingYamlOp::Remove {
        segments: vec!["catalogs".to_string(), catalog_name.to_string()],
      });
    }
    if catalogs.is_empty() {
      root.remove("catalogs");
      new_patches.push(PendingYamlOp::Remove {
        segments: vec!["catalogs".to_string()],
      });
    }
    removed
  };
  if removed {
    file.patches.extend(new_patches);
    file.dirty = true;
  }
  removed
}

/// Apply a consumer instance's `expected_specifier` to the yaml. Catalog
/// instances route through the catalog-definition path; non-catalog
/// `PnpmWorkspace` instances (pnpm `overrides`, which use the
/// `versionsByName` strategy) write into the top-level block named by the
/// dep type's `path` (e.g. `overrides`). No-op when the instance has no
/// expected specifier.
pub fn copy_expected_specifier_yaml(
  file: &mut YamlFile,
  dependency_type: &DependencyType,
  name: &str,
  catalog_name: Option<&str>,
  expected: &Rc<Specifier>,
) {
  if let Some(catalog_name) = catalog_name {
    insert_catalog_definition(file, catalog_name, name, expected);
    return;
  }
  // `overrides` is a flat `name: version` block — the same shape as the
  // default `catalog:` block. `path` is its JSON pointer, e.g. `/overrides`.
  if matches!(dependency_type.strategy, Strategy::VersionsByName) {
    let block_key = dependency_type.path.trim_start_matches('/');
    insert_into_target(file, YamlTarget::Block(block_key), name, expected);
  }
}

/// Get-or-create the mapping that holds a target's `dep_name: specifier`
/// entries, creating intermediate blocks (and promoting the document root)
/// as needed.
fn ensure_block<'a>(file: &'a mut YamlFile, target: YamlTarget) -> &'a mut Mapping {
  match target {
    YamlTarget::Block(key) => ensure_yaml_block(file, key),
    YamlTarget::NamedCatalog(name) => get_or_insert_mapping(ensure_yaml_block(file, "catalogs"), name),
  }
}
