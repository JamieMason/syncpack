#![allow(dead_code)]

use {
  crate::{config::Config, specifier::Specifier},
  log::debug,
  serde::Deserialize,
  std::{collections::HashMap, fs, rc::Rc, time::Instant},
};

pub type Catalog = HashMap<String, Rc<Specifier>>;
pub type CatalogsByName = HashMap<String, Catalog>;

/// Extract catalogs from the project configuration.
///
/// Attempts to read catalog definitions from:
/// 1. pnpm-workspace.yaml (pnpm catalogs)
/// 2. package.json at project root (bun catalogs)
///
/// See:
/// - https://pnpm.io/catalogs
/// - https://bun.sh/docs/pm/catalogs
pub fn from_config(config: &Config) -> Option<CatalogsByName> {
  let start = Instant::now();
  let catalogs = try_from_pnpm(config).or_else(|| try_from_bun(config));
  debug!("Catalog discovery completed in {:?}", start.elapsed());
  catalogs
}

/// Try to read catalogs from pnpm-workspace.yaml
///
/// pnpm supports both:
/// - `catalog:` (singular) - default catalog
/// - `catalogs:` (plural) - named catalogs
///
/// Example pnpm-workspace.yaml:
/// ```yaml
/// packages:
///   - 'packages/*'
/// catalog:
///   chalk: ^4.1.2
/// catalogs:
///   react16:
///     react: ^16.7.0
///     react-dom: ^16.7.0
/// ```
fn try_from_pnpm(config: &Config) -> Option<CatalogsByName> {
  let file_path = config.cli.cwd.join("pnpm-workspace.yaml");

  if !file_path.exists() {
    return None;
  }

  debug!("Reading catalogs from pnpm-workspace.yaml");

  let contents = fs::read_to_string(&file_path).ok()?;
  let workspace: PnpmWorkspace = serde_yaml::from_str(&contents).ok()?;

  let mut catalogs_by_name = CatalogsByName::new();

  // Add default catalog if present
  if let Some(default_catalog) = workspace.catalog {
    let mut catalog = Catalog::new();
    for (name, version) in default_catalog {
      catalog.insert(name, Specifier::new(&version));
    }
    if !catalog.is_empty() {
      catalogs_by_name.insert("default".to_string(), catalog);
    }
  }

  // Add named catalogs if present
  if let Some(named_catalogs) = workspace.catalogs {
    for (catalog_name, dependencies) in named_catalogs {
      let mut catalog = Catalog::new();
      for (name, version) in dependencies {
        catalog.insert(name, Specifier::new(&version));
      }
      if !catalog.is_empty() {
        catalogs_by_name.insert(catalog_name, catalog);
      }
    }
  }

  if catalogs_by_name.is_empty() {
    debug!("No catalogs found in pnpm-workspace.yaml");
    None
  } else {
    debug!("Found {} catalog(s) in pnpm-workspace.yaml", catalogs_by_name.len());
    Some(catalogs_by_name)
  }
}

/// Try to read catalogs from package.json at project root
///
/// Bun supports catalogs defined in the root package.json:
/// - At top level: `catalog` and `catalogs`
/// - Under workspaces: `workspaces.catalog` and `workspaces.catalogs`
///
/// Example package.json:
/// ```json
/// {
///   "workspaces": {
///     "catalog": {
///       "react": "^19.0.0"
///     },
///     "catalogs": {
///       "testing": {
///         "jest": "30.0.0"
///       }
///     }
///   }
/// }
/// ```
fn try_from_bun(config: &Config) -> Option<CatalogsByName> {
  let file_path = config.cli.cwd.join("package.json");

  if !file_path.exists() {
    return None;
  }

  debug!("Reading catalogs from package.json");

  let contents = fs::read_to_string(&file_path).ok()?;
  let package_json: BunPackageJson = serde_json::from_str(&contents).ok()?;

  let mut catalogs_by_name = CatalogsByName::new();

  // Try workspaces.catalog and workspaces.catalogs first
  if let Some(workspaces) = package_json.workspaces {
    if let Some(default_catalog) = workspaces.catalog {
      let mut catalog = Catalog::new();
      for (name, version) in default_catalog {
        catalog.insert(name, Specifier::new(&version));
      }
      if !catalog.is_empty() {
        catalogs_by_name.insert("default".to_string(), catalog);
      }
    }

    if let Some(named_catalogs) = workspaces.catalogs {
      for (catalog_name, dependencies) in named_catalogs {
        let mut catalog = Catalog::new();
        for (name, version) in dependencies {
          catalog.insert(name, Specifier::new(&version));
        }
        if !catalog.is_empty() {
          catalogs_by_name.insert(catalog_name, catalog);
        }
      }
    }
  }

  // Fall back to top-level catalog and catalogs
  if catalogs_by_name.is_empty() {
    if let Some(default_catalog) = package_json.catalog {
      let mut catalog = Catalog::new();
      for (name, version) in default_catalog {
        catalog.insert(name, Specifier::new(&version));
      }
      if !catalog.is_empty() {
        catalogs_by_name.insert("default".to_string(), catalog);
      }
    }

    if let Some(named_catalogs) = package_json.catalogs {
      for (catalog_name, dependencies) in named_catalogs {
        let mut catalog = Catalog::new();
        for (name, version) in dependencies {
          catalog.insert(name, Specifier::new(&version));
        }
        if !catalog.is_empty() {
          catalogs_by_name.insert(catalog_name, catalog);
        }
      }
    }
  }

  if catalogs_by_name.is_empty() {
    debug!("No catalogs found in package.json");
    None
  } else {
    debug!("Found {} catalog(s) in package.json", catalogs_by_name.len());
    Some(catalogs_by_name)
  }
}

#[derive(Debug, Deserialize)]
struct PnpmWorkspace {
  catalog: Option<HashMap<String, String>>,
  catalogs: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Debug, Deserialize)]
struct BunPackageJson {
  catalog: Option<HashMap<String, String>>,
  catalogs: Option<HashMap<String, HashMap<String, String>>>,
  workspaces: Option<BunWorkspaces>,
}

#[derive(Debug, Deserialize)]
struct BunWorkspaces {
  catalog: Option<HashMap<String, String>>,
  catalogs: Option<HashMap<String, HashMap<String, String>>>,
}
