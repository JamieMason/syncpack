use {crate::source_kind::SourceKind, serde::Serialize};

#[derive(Clone, Debug, Serialize)]
pub enum Strategy {
  /// "name~version"
  NameAndVersionProps,
  /// "name@version"
  NamedVersionString,
  /// "version"
  UnnamedVersionString,
  /// "versionsByName"
  VersionsByName,
  /// Not recognised
  InvalidConfig,
}

impl Strategy {
  pub fn new(strategy: &str) -> Strategy {
    match strategy {
      "name~version" => Strategy::NameAndVersionProps,
      "name@version" => Strategy::NamedVersionString,
      "version" => Strategy::UnnamedVersionString,
      "versionsByName" => Strategy::VersionsByName,
      _ => Strategy::InvalidConfig,
    }
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct DependencyType {
  /// JSON pointer to the property holding the dependency name.
  pub name_path: Option<String>,
  pub name: String,
  /// JSON pointer to the property holding the version string.
  pub path: String,
  pub strategy: Strategy,
  /// Defaults to `PackageJson` for user `customTypes`. Auto-generated
  /// catalog dep types set explicitly.
  pub source: SourceKind,
  /// `true` only for built-in catalog dep types (`pnpmCatalog*`,
  /// `bunCatalog*`). User `customTypes` cannot set this — the config-side
  /// `DependencyType::new` hard-wires it to `false`. Drives
  /// `Instance::is_catalog_instance` / `Instance::catalog_name` and
  /// `Rcfile::catalog_dep_type_names` selection.
  pub is_catalog_definition: bool,
}
