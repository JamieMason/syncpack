use {
  super::registry_client::MockRegistryClient,
  crate::{
    catalogs::{Catalog, CatalogsByName},
    cli::{Cli, SortBy, Subcommand, UpdateTarget},
    config::Config,
    context::Context,
    package_json::PackageJson,
    packages::Packages,
    rcfile::Rcfile,
    registry_client::RegistryClient,
    specifier::Specifier,
  },
  log::LevelFilter,
  serde_json::Value,
  std::{cell::RefCell, collections::HashMap, env, path::PathBuf, sync::Arc},
};

pub fn cli() -> Cli {
  Cli {
    check: true,
    config_path: None,
    cwd: env::current_dir().unwrap(),
    dependencies: vec![],
    dependency_types: vec![],
    disable_ansi: true,
    dry_run: true,
    log_levels: vec![LevelFilter::Error],
    packages: vec![],
    show_hints: false,
    show_ignored: false,
    show_instances: false,
    show_status_codes: false,
    sort: SortBy::Name,
    source_patterns: vec![],
    specifier_types: vec![],
    subcommand: Subcommand::Lint,
    target: UpdateTarget::Latest,
  }
}

/// Create an empty Config struct
pub fn config() -> Config {
  Config {
    cli: cli(),
    rcfile: rcfile(),
  }
}

/// Create a Config struct from a mocked .syncpackrc
pub fn config_from_mock(value: serde_json::Value) -> Config {
  Config {
    cli: cli(),
    rcfile: rcfile_from_mock(value),
  }
}

/// Create an empty Rcfile struct
pub fn rcfile() -> Rcfile {
  let empty_json = "{}".to_string();
  serde_json::from_str::<Rcfile>(&empty_json).unwrap()
}

/// Create an Rcfile struct from a mocked .syncpackrc
pub fn rcfile_from_mock(value: serde_json::Value) -> Rcfile {
  serde_json::from_value::<Rcfile>(value).unwrap()
}

/// Parse a package.json string
pub fn package_json_from_value(contents: Value) -> PackageJson {
  PackageJson {
    name: contents
      .pointer("/name")
      .and_then(|name| name.as_str())
      .unwrap_or("NAME_IS_MISSING")
      .to_string(),
    file_path: PathBuf::new(),
    formatting_mismatches: RefCell::new(vec![]),
    json: RefCell::new(contents.to_string()),
    contents: RefCell::new(contents),
  }
}

/// Create an collection of package.json files from mocked values
pub fn packages_from_mocks(values: Vec<serde_json::Value>) -> Packages {
  let mut packages = Packages::new();
  for value in values {
    packages.add_package(package_json_from_value(value));
  }
  packages
}

/// Create a MockRegistryClient from mocked package data
fn registry_client_from_mocks(mock_updates: serde_json::Value) -> Option<Arc<dyn RegistryClient>> {
  Some(Arc::new(MockRegistryClient::from_json(mock_updates)))
}

/// Create a CatalogsByName from mocked catalog data
///
/// Examples:
/// - json!({"default": {"react": "^17.0.2"}}) -> one catalog
/// - json!({"default": {...}, "react18": {...}}) -> multiple catalogs
pub fn catalogs_from_mocks(value: serde_json::Value) -> CatalogsByName {
  let mut catalogs = HashMap::new();
  if let Some(obj) = value.as_object() {
    for (catalog_name, catalog_value) in obj {
      let mut catalog = Catalog::new();
      if let Some(deps) = catalog_value.as_object() {
        for (dep_name, version) in deps {
          if let Some(version_str) = version.as_str() {
            catalog.insert(dep_name.clone(), Specifier::new(version_str));
          }
        }
      }
      catalogs.insert(catalog_name.clone(), catalog);
    }
  }
  catalogs
}

/// Create a Context struct with mocked npm registry updates applied to it
pub async fn context_with_registry_updates(
  config: Config,
  packages: Packages,
  mock_updates: serde_json::Value,
  catalogs: Option<CatalogsByName>,
) -> Context {
  let registry_client = registry_client_from_mocks(mock_updates);
  let mut ctx = Context::create(config, packages, registry_client, catalogs);
  ctx.fetch_all_updates().await;
  ctx
}
