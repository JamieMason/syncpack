use {
  crate::{
    cli::Cli,
    dependency_type::DependencyType,
    effects::ui::LINE_ENDING,
    group_selector::GroupSelector,
    packages::Packages,
    semver_group::{AnySemverGroup, SemverGroup},
    version_group::{AnyVersionGroup, VersionGroup},
  },
  log::{debug, error},
  serde::Deserialize,
  serde_json::Value,
  std::{
    collections::HashMap,
    fs,
    path::Path,
    process::{exit, Command},
    time::Instant,
  },
};

fn empty_custom_types() -> HashMap<String, CustomType> {
  HashMap::new()
}

fn default_max_concurrent_requests() -> usize {
  12
}

fn default_true() -> bool {
  true
}

fn default_false() -> bool {
  false
}

fn default_indent() -> String {
  "  ".to_string()
}

fn default_sort_az() -> Vec<String> {
  vec![
    "bin".to_string(),
    "contributors".to_string(),
    "dependencies".to_string(),
    "devDependencies".to_string(),
    "keywords".to_string(),
    "peerDependencies".to_string(),
    "resolutions".to_string(),
    "scripts".to_string(),
  ]
}

fn default_sort_exports() -> Vec<String> {
  vec![
    "types".to_string(),
    "node-addons".to_string(),
    "node".to_string(),
    "browser".to_string(),
    "module".to_string(),
    "import".to_string(),
    "require".to_string(),
    "svelte".to_string(),
    "development".to_string(),
    "production".to_string(),
    "script".to_string(),
    "default".to_string(),
  ]
}

fn sort_first() -> Vec<String> {
  vec![
    "name".to_string(),
    "description".to_string(),
    "version".to_string(),
    "author".to_string(),
  ]
}

fn default_source() -> Vec<String> {
  vec![]
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomType {
  pub strategy: String,
  pub name_path: Option<String>,
  pub path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyGroup {
  pub alias_name: String,
  #[serde(default)]
  pub dependencies: Vec<String>,
  #[serde(default)]
  pub dependency_types: Vec<String>,
  #[serde(default)]
  pub packages: Vec<String>,
  #[serde(default)]
  pub specifier_types: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rcfile {
  #[serde(default = "empty_custom_types")]
  pub custom_types: HashMap<String, CustomType>,
  #[serde(default)]
  pub dependency_groups: Vec<DependencyGroup>,
  #[serde(default = "default_true")]
  pub format_bugs: bool,
  #[serde(default = "default_true")]
  pub format_repository: bool,
  #[serde(default = "default_indent")]
  pub indent: String,
  #[serde(default = "default_max_concurrent_requests")]
  pub max_concurrent_requests: usize,
  #[serde(default)]
  pub semver_groups: Vec<AnySemverGroup>,
  #[serde(default = "default_sort_az")]
  pub sort_az: Vec<String>,
  #[serde(default = "default_sort_exports")]
  pub sort_exports: Vec<String>,
  #[serde(default = "sort_first")]
  pub sort_first: Vec<String>,
  #[serde(default = "default_true")]
  pub sort_packages: bool,
  #[serde(default = "default_source")]
  pub source: Vec<String>,
  #[serde(default = "default_false")]
  pub strict: bool,
  #[serde(default)]
  pub version_groups: Vec<AnyVersionGroup>,
}

impl Default for Rcfile {
  fn default() -> Self {
    Rcfile::from_json("{}".to_string())
  }
}

impl Rcfile {
  fn from_json_path(file_path: &Path) -> Option<Rcfile> {
    fs::read_to_string(file_path)
      .map_err(|err| error!("Failed to read config file {:?}: {}", file_path, err))
      .ok()
      .map(Rcfile::from_json)
  }

  fn from_yaml_path(file_path: &Path) -> Option<Rcfile> {
    fs::read_to_string(file_path)
      .map_err(|err| error!("Failed to read config file {:?}: {}", file_path, err))
      .ok()
      .and_then(|contents| {
        serde_yaml::from_str::<Rcfile>(&contents)
          .map_err(|err| error!("Failed to parse YAML config file {:?}: {}", file_path, err))
          .ok()
      })
  }

  fn from_javascript_path(file_path: &Path) -> Option<Rcfile> {
    let nodejs_script = format!(
      r#"
        (async () => {{
          try {{
            // Use tsx to import the config file (handles both JS and TS)
            const {{ default: config }} = await import('{}');
            console.log(JSON.stringify(config || {{}}));
          }} catch (error) {{
            // Fallback to require for CommonJS modules
            try {{
              const config = require('{}');
              console.log(JSON.stringify(config.default || config || {{}}));
            }} catch (requireError) {{
              console.error('Failed to load config:', error.message);
              console.log('{{}}');
            }}
          }}
        }})();
        "#,
      file_path.to_string_lossy().replace('\\', "\\\\").replace('"', "\\\""),
      file_path.to_string_lossy().replace('\\', "\\\\").replace('"', "\\\"")
    );

    Command::new("npx")
      .args(["tsx", "-e", &nodejs_script])
      .current_dir(file_path.parent().unwrap_or_else(|| Path::new(".")))
      .output()
      .map_err(|err| error!("Failed to execute tsx: {}", err))
      .ok()
      .and_then(|output| {
        if output.status.success() {
          String::from_utf8(output.stdout)
            .map_err(|err| error!("Invalid UTF-8 in config output: {}", err))
            .ok()
        } else {
          error!(
            "Failed to load JavaScript/TypeScript config {:?}: {}",
            file_path,
            String::from_utf8_lossy(&output.stderr)
          );
          None
        }
      })
      .map(|json_str| {
        debug!("Loaded config from {:?}: {}", file_path, json_str.trim());
        Rcfile::from_json(json_str)
      })
  }

  fn try_from_package_json_config_property(cli: &Cli) -> Option<Rcfile> {
    let package_json_path = cli.cwd.join("package.json");
    package_json_path
      .exists()
      .then(|| {
        fs::read_to_string(&package_json_path)
          .map_err(|err| error!("Failed to read package.json: {}", err))
          .ok()
      })
      .flatten()
      .and_then(|contents| {
        serde_json::from_str::<Value>(&contents)
          .map_err(|err| error!("Failed to parse package.json: {}", err))
          .ok()
      })
      .and_then(|package_json| {
        package_json
          .get("syncpack")
          .inspect(|_| debug!("Found syncpack config in package.json"))
          .or_else(|| {
            package_json
              .pointer("/config/syncpack")
              .inspect(|_| debug!("Found config.syncpack in package.json"))
          })
          .and_then(|syncpack_config| serde_json::to_string(syncpack_config).ok().map(Rcfile::from_json))
      })
  }

  fn try_from_json_candidates(cli: &Cli) -> Option<Rcfile> {
    let candidates = vec![".syncpackrc", ".syncpackrc.json"];
    for candidate in candidates {
      let config_path = cli.cwd.join(candidate);
      if config_path.exists() {
        debug!("Found JSON config file: {:?}", config_path);
        return Rcfile::from_json_path(&config_path);
      }
    }
    None
  }

  fn try_from_yaml_candidates(cli: &Cli) -> Option<Rcfile> {
    let candidates = vec![".syncpackrc.yaml", ".syncpackrc.yml"];
    for candidate in candidates {
      let config_path = cli.cwd.join(candidate);
      if config_path.exists() {
        debug!("Found YAML config file: {:?}", config_path);
        return Rcfile::from_yaml_path(&config_path);
      }
    }
    None
  }

  fn try_from_js_candidates(cli: &Cli) -> Option<Rcfile> {
    let candidates = vec![
      ".syncpackrc.js",
      ".syncpackrc.ts",
      ".syncpackrc.mjs",
      ".syncpackrc.cjs",
      "syncpack.config.js",
      "syncpack.config.ts",
      "syncpack.config.mjs",
      "syncpack.config.cjs",
    ];
    for candidate in candidates {
      let config_path = cli.cwd.join(candidate);
      if config_path.exists() {
        debug!("Found JavaScript/TypeScript config file: {:?}", config_path);
        return Rcfile::from_javascript_path(&config_path);
      }
    }
    None
  }

  pub fn from_disk(cli: &Cli) -> Rcfile {
    let start = Instant::now();
    let rcfile = Rcfile::try_from_json_candidates(cli)
      .or_else(|| Rcfile::try_from_yaml_candidates(cli))
      .or_else(|| Rcfile::try_from_package_json_config_property(cli))
      .or_else(|| Rcfile::try_from_js_candidates(cli))
      .unwrap_or_else(|| {
        debug!("No config file found, using defaults");
        Rcfile::default()
      });
    debug!("Config discovery completed in {:?}", start.elapsed());
    rcfile
  }

  /// Create a new rcfile from a JSON string or revert to defaults
  pub fn from_json(json: String) -> Rcfile {
    match serde_json::from_str(&json) {
      Ok(rcfile) => rcfile,
      Err(err) => {
        error!("Your syncpack config file failed validation{LINE_ENDING}  {err}");
        exit(1);
      }
    }
  }

  /// Create every alias defined in the rcfile.
  pub fn get_dependency_groups(&self, packages: &Packages) -> Vec<GroupSelector> {
    self
      .dependency_groups
      .iter()
      .map(|dependency_group_config| {
        GroupSelector::new(
          /* all_packages: */ packages,
          /* include_dependencies: */ dependency_group_config.dependencies.clone(),
          /* include_dependency_types: */ dependency_group_config.dependency_types.clone(),
          /* alias_name: */ dependency_group_config.alias_name.clone(),
          /* include_packages: */ dependency_group_config.packages.clone(),
          /* include_specifier_types: */ dependency_group_config.specifier_types.clone(),
        )
      })
      .collect()
  }

  /// Create every semver group defined in the rcfile.
  pub fn get_semver_groups(&self, packages: &Packages) -> Vec<SemverGroup> {
    let mut all_groups: Vec<SemverGroup> = vec![];
    all_groups.push(SemverGroup::get_exact_local_specifiers());
    self.semver_groups.iter().for_each(|group_config| {
      all_groups.push(SemverGroup::from_config(group_config, packages));
    });
    all_groups.push(SemverGroup::get_catch_all());
    all_groups
  }

  /// Create every version group defined in the rcfile.
  pub fn get_version_groups(&self, packages: &Packages) -> Vec<VersionGroup> {
    let mut all_groups: Vec<VersionGroup> = self
      .version_groups
      .iter()
      .map(|group_config| VersionGroup::from_config(group_config, packages))
      .collect();
    all_groups.push(VersionGroup::get_catch_all());
    all_groups
  }

  /// Get all custom types defined in the rcfile, combined with the default
  /// types.
  pub fn get_all_dependency_types(&self) -> Vec<DependencyType> {
    // Custom dependency types defined in the rcfile
    let custom_types = &self.custom_types;
    // Internal dependency types are also defined as custom types
    let default_types = HashMap::from([
      (
        String::from("dev"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("devDependencies"),
        },
      ),
      (
        String::from("local"),
        CustomType {
          strategy: String::from("name~version"),
          name_path: Some(String::from("name")),
          path: String::from("version"),
        },
      ),
      (
        String::from("overrides"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("overrides"),
        },
      ),
      (
        String::from("peer"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("peerDependencies"),
        },
      ),
      (
        String::from("pnpmOverrides"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("pnpm.overrides"),
        },
      ),
      (
        String::from("prod"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("dependencies"),
        },
      ),
      (
        String::from("resolutions"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("resolutions"),
        },
      ),
    ]);
    // Collect which dependency types are enabled
    let mut dependency_types: Vec<DependencyType> = vec![];
    default_types.iter().for_each(|(name, custom_type)| {
      dependency_types.push(DependencyType::new(name, custom_type));
    });
    custom_types.iter().for_each(|(name, custom_type)| {
      dependency_types.push(DependencyType::new(name, custom_type));
    });
    dependency_types
  }
}
