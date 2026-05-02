use {
  crate::{errors::UnsupportedConfigError, rcfile::CustomType, source::SourceKind},
  serde::Serialize,
};

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
  /// `bunCatalog*`). User `customTypes` cannot set this — `DependencyType::new`
  /// hard-wires it to `false`. Drives `Instance::is_catalog_instance` /
  /// `Instance::catalog_name` and `Rcfile::catalog_dep_type_names` selection.
  pub is_catalog_definition: bool,
}

impl DependencyType {
  pub fn new(name: &str, config: &CustomType) -> Result<DependencyType, UnsupportedConfigError> {
    let source = config
      .source
      .as_deref()
      .map(SourceKind::parse)
      .transpose()?
      .unwrap_or(SourceKind::PackageJson);
    Ok(DependencyType {
      name_path: config.name_path.clone().map(normalize_path),
      name: name.to_string(),
      path: normalize_path(config.path.clone()),
      strategy: Strategy::new(config.strategy.as_str()),
      source,
      is_catalog_definition: false,
    })
  }
}

/// Converts a "some.nested.prop.name" selector to "/some/nested/prop/name"
fn normalize_path(path: String) -> String {
  let mut normalized_path = String::from("/");
  normalized_path.push_str(&path.replace('.', "/"));
  normalized_path
}

#[cfg(test)]
mod dependency_type_test {
  use {
    crate::{rcfile::compute_all_dependency_types, source::SourceKind},
    std::collections::HashMap,
  };

  #[test]
  fn dependency_type_default_source_is_package_json() {
    let dep_types = compute_all_dependency_types(&HashMap::new()).expect("default dep types compute");
    assert!(!dep_types.is_empty());
    for dt in &dep_types {
      assert_eq!(
        dt.source,
        SourceKind::PackageJson,
        "{}: default dep type source must be PackageJson",
        dt.name
      );
      assert!(
        !dt.is_catalog_definition,
        "{}: default dep type is_catalog_definition must be false",
        dt.name
      );
    }
    let names: Vec<&str> = dep_types.iter().map(|dt| dt.name.as_str()).collect();
    for expected in ["dev", "prod", "peer", "local", "overrides", "pnpmOverrides", "resolutions"] {
      assert!(names.contains(&expected), "missing default dep type {expected}");
    }
  }
}
