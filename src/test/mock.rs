use {
  super::registry_client::MockRegistryClient,
  crate::{
    catalogs::{Catalog, CatalogsByName},
    cli::{Cli, ReporterKind, SortBy, Subcommand, UpdateTarget},
    context::{Config, Context},
    package_json::PackageJson,
    packages::Packages,
    rcfile::Rcfile,
    registry::{client::RegistryClient, updates::RegistryUpdates},
    specifier::Specifier,
  },
  log::LevelFilter,
  serde_json::Value,
  std::{collections::HashMap, env, path::PathBuf, sync::Arc},
};

pub fn cli() -> Cli {
  Cli {
    check: true,
    config_path: None,
    cwd: env::current_dir().unwrap(),
    disable_ansi: true,
    dry_run: true,
    filters: None,
    log_levels: vec![LevelFilter::Error],
    reporter: ReporterKind::Pretty,
    show_hints: false,
    show_ignored: false,
    show_instances: false,
    show_status_codes: false,
    sort: SortBy::Name,
    source_patterns: vec![],
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
  Rcfile::default()
}

/// Create an Rcfile struct from a mocked .syncpackrc
pub fn rcfile_from_mock(value: serde_json::Value) -> Rcfile {
  serde_json::from_value::<crate::rcfile::RawRcfile>(value)
    .unwrap()
    .try_into()
    .unwrap()
}

/// Parse a package.json string
pub fn package_json_from_value(contents: Value) -> PackageJson {
  let name = contents
    .pointer("/name")
    .and_then(|name| name.as_str())
    .unwrap_or("NAME_IS_MISSING")
    .to_string();

  // Create a realistic file path based on package name
  // e.g., "package-a" -> "/packages/package-a/package.json"
  let file_path = PathBuf::from(format!("/packages/{name}/package.json"));

  let raw = serde_json::to_string_pretty(&contents).unwrap_or_default();
  PackageJson {
    name,
    file_path,
    formatting_mismatches: vec![],
    raw,
    contents,
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

/// Create a Context and RegistryUpdates from mocked npm registry data
pub async fn context_with_registry_updates(
  config: Config,
  packages: Packages,
  mock_updates: serde_json::Value,
  catalogs: Option<CatalogsByName>,
) -> (Context, RegistryUpdates) {
  let client: Arc<dyn RegistryClient> = Arc::new(MockRegistryClient::from_json(mock_updates));
  let ctx = Context::create(config, packages, catalogs).unwrap();
  let updates = RegistryUpdates::fetch(
    &client,
    &ctx.version_groups,
    &ctx.instances,
    ctx.config.rcfile.max_concurrent_requests,
  )
  .await;
  (ctx, updates)
}
