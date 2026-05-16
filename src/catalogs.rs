use {
  crate::{
    dependency::{DependencyType, Strategy},
    disk::{Disk, PackageManager, YamlFile},
    errors::SyncpackError,
    source::SourceKind,
  },
  log::debug,
  serde_json::Value as JsonValue,
  std::time::Instant,
  yaml_serde::Value as YamlValue,
};

/// All catalog dep types implied by what's parsed onto Disk (pnpm + Bun).
///
/// Pnpm path: when `disk.pnpm_workspace` is present, generates one dep type
/// per catalog name found in the yaml.
///
/// Bun path: gated on `disk.package_manager == Some(Bun)`. Uses
/// `detect_bun_catalogs` (the single source of truth shared with fix-time)
/// to scan top + `/workspaces`; picks the location with content; errors when
/// both have content.
pub fn make_catalog_dep_types(disk: &Disk) -> Result<Vec<DependencyType>, SyncpackError> {
  let start = Instant::now();
  let mut dep_types = Vec::new();
  if let Some(yaml) = &disk.pnpm_workspace {
    for catalog_name in pnpm_catalog_names(yaml) {
      dep_types.push(make_pnpm_dep_type(&catalog_name));
    }
  }
  if matches!(disk.package_manager, Some(PackageManager::Bun))
    && let Some(root) = disk.package_json_root()
    && let Some((prefix, names)) = detect_bun_catalogs(&root.contents)?
  {
    for name in names {
      dep_types.push(make_bun_dep_type(&name, prefix));
    }
  }
  debug!("Catalog discovery completed in {:?}", start.elapsed());
  Ok(dep_types)
}

/// Scan top-level (`""`) and `/workspaces` for Bun catalog blocks. Returns
/// the prefix and names of the location with content; errors if both have
/// content. Single source of truth for "where do Bun catalogs live in this
/// root"; fix-time uses the same helper so discovery and writes agree.
pub fn detect_bun_catalogs(root_contents: &JsonValue) -> Result<Option<(&'static str, Vec<String>)>, SyncpackError> {
  let top = collect_bun_catalog_names(root_contents, "");
  let nested = collect_bun_catalog_names(root_contents, "/workspaces");
  match (top.is_empty(), nested.is_empty()) {
    (true, true) => Ok(None),
    (false, true) => Ok(Some(("", top))),
    (true, false) => Ok(Some(("/workspaces", nested))),
    (false, false) => Err(SyncpackError::BunDualCatalogPath),
  }
}

/// Collect catalog names found under `{prefix}/catalog` (default → `"default"`)
/// and `{prefix}/catalogs/{name}` (named) in a JSON value.
pub(crate) fn collect_bun_catalog_names(contents: &JsonValue, prefix: &str) -> Vec<String> {
  let mut names = Vec::new();
  let default_path = format!("{prefix}/catalog");
  if matches!(contents.pointer(&default_path), Some(JsonValue::Object(_))) {
    names.push("default".to_string());
  }
  let named_path = format!("{prefix}/catalogs");
  if let Some(JsonValue::Object(map)) = contents.pointer(&named_path) {
    for key in map.keys() {
      names.push(key.clone());
    }
  }
  names
}

/// Build a pnpm catalog `DependencyType`. Bypasses `DependencyType::new`
/// (which normalises `.` → `/` in paths); catalog names may legitimately
/// contain dots.
fn make_pnpm_dep_type(catalog_name: &str) -> DependencyType {
  let (name, path) = if catalog_name == "default" {
    ("pnpmCatalog".to_string(), "/catalog".to_string())
  } else {
    (format!("pnpmCatalog:{catalog_name}"), format!("/catalogs/{catalog_name}"))
  };
  DependencyType {
    name_path: None,
    name,
    path,
    strategy: Strategy::VersionsByName,
    source: SourceKind::PnpmWorkspace,
    is_catalog_definition: true,
  }
}

/// Build a Bun catalog `DependencyType`. Bypasses `DependencyType::new`
/// for the same reason as `make_pnpm_dep_type`.
fn make_bun_dep_type(catalog_name: &str, path_prefix: &str) -> DependencyType {
  let (name, path) = if catalog_name == "default" {
    ("bunCatalog".to_string(), format!("{path_prefix}/catalog"))
  } else {
    (
      format!("bunCatalog:{catalog_name}"),
      format!("{path_prefix}/catalogs/{catalog_name}"),
    )
  };
  DependencyType {
    name_path: None,
    name,
    path,
    strategy: Strategy::VersionsByName,
    source: SourceKind::PackageJson,
    is_catalog_definition: true,
  }
}

/// Discover the catalog names (`"default"` + named) defined in a
/// `pnpm-workspace.yaml` file.
pub fn pnpm_catalog_names(file: &YamlFile) -> Vec<String> {
  let mut names = Vec::new();
  if let Some(YamlValue::Mapping(map)) = file.contents.get("catalog")
    && !map.is_empty()
  {
    names.push("default".to_string());
  }
  if let Some(YamlValue::Mapping(map)) = file.contents.get("catalogs") {
    for key in map.keys() {
      if let YamlValue::String(name) = key {
        names.push(name.clone());
      }
    }
  }
  names
}
