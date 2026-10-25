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
}

impl Strategy {
  /// `None` when the configured strategy string is not recognised; the caller
  /// attaches the custom type name to build the config error.
  pub fn new(strategy: &str) -> Option<Strategy> {
    match strategy {
      "name~version" => Some(Strategy::NameAndVersionProps),
      "name@version" => Some(Strategy::NamedVersionString),
      "version" => Some(Strategy::UnnamedVersionString),
      "versionsByName" => Some(Strategy::VersionsByName),
      _ => None,
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
    let strategy = Strategy::new(&config.strategy).ok_or_else(|| UnsupportedConfigError::InvalidStrategy {
      name: name.to_string(),
      value: config.strategy.clone(),
    })?;
    if matches!(strategy, Strategy::NameAndVersionProps) && config.name_path.is_none() {
      return Err(UnsupportedConfigError::MissingNamePath { name: name.to_string() });
    }
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
      strategy,
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
    std::collections::BTreeMap,
  };

  #[test]
  fn dependency_type_default_sources_are_correct() {
    let dep_types = compute_all_dependency_types(&BTreeMap::new()).expect("default dep types compute");
    assert!(!dep_types.is_empty());
    for dt in &dep_types {
      let expected_source = if dt.name == "pnpmOverrides" {
        SourceKind::PnpmWorkspace
      } else {
        SourceKind::PackageJson
      };
      assert_eq!(dt.source, expected_source, "{}: default dep type source mismatch", dt.name);
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
