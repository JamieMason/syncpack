use {
  crate::CustomType,
  syncpack_vocab::{
    config_error::UnsupportedConfigError,
    dependency_type::{DependencyType, Strategy},
    source_kind::SourceKind,
  },
};

/// Build a `DependencyType` from a user `customTypes` entry. Normalises
/// `.`-separated selector paths to JSON pointers and defaults the source to
/// `PackageJson`.
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

/// Converts a "some.nested.prop.name" selector to "/some/nested/prop/name"
fn normalize_path(path: String) -> String {
  let mut normalized_path = String::from("/");
  normalized_path.push_str(&path.replace('.', "/"));
  normalized_path
}

#[cfg(test)]
mod dependency_type_test {
  use {crate::compute_all_dependency_types, std::collections::HashMap, syncpack_vocab::source_kind::SourceKind};

  #[test]
  fn dependency_type_default_sources_are_correct() {
    let dep_types = compute_all_dependency_types(&HashMap::new()).expect("default dep types compute");
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
