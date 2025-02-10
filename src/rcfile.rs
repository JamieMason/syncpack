use {
  crate::{
    cli::Cli,
    dependency_type::DependencyType,
    group_selector::GroupSelector,
    packages::Packages,
    semver_group::{AnySemverGroup, SemverGroup},
    version_group::{AnyVersionGroup, VersionGroup},
  },
  log::{debug, error},
  serde::Deserialize,
  std::{
    collections::HashMap,
    env, io,
    process::{exit, Command},
  },
};

fn empty_custom_types() -> HashMap<String, CustomType> {
  HashMap::new()
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
  #[serde(default)]
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

impl Rcfile {
  /// Until we can port cosmiconfig to Rust, call out to Node.js to get the
  /// rcfile from the filesystem
  pub fn from_cosmiconfig(cli: &Cli) -> Rcfile {
    let require_path = match env::var("COSMICONFIG_REQUIRE_PATH") {
      Ok(v) => serde_json::to_string(&v).unwrap(),
      Err(_) => "'cosmiconfig'".to_string(),
    };

    let nodejs_script = format!(
      r#"
        require({})
          .cosmiconfig('syncpack')
          .search({})
          .then(res => (res.config ? JSON.stringify(res.config) : '{{}}'))
          .catch(() => '{{}}')
          .then(console.log);
        "#,
      require_path,
      serde_json::to_string(&cli.cwd).unwrap()
    );

    let output = Command::new("node")
      .arg("-e")
      .arg(nodejs_script)
      .current_dir(&cli.cwd)
      .output()
      .and_then(|output| {
        if output.status.success() {
          String::from_utf8(output.stdout).map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        } else {
          Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8(output.stderr).expect("Failed to run cosmiconfig"),
          ))
        }
      })
      .inspect(|json| debug!("raw rcfile contents: '{}'", json.trim()))
      .map(Rcfile::from_json);

    match output {
      Ok(rcfile) => rcfile,
      Err(err) => {
        error!("There was an error when attempting to locate your syncpack rcfile");
        error!("Please raise an issue at https://github.com/JamieMason/syncpack/issues/new?template=bug_report.yaml");
        error!("{err}");
        exit(1);
      }
    }
  }

  /// Create a new rcfile from a JSON string or revert to defaults
  pub fn from_json(json: String) -> Rcfile {
    match serde_json::from_str(&json) {
      Ok(rcfile) => rcfile,
      Err(err) => {
        error!("Your syncpack config file failed validation\n  {err}");
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
        if dependency_group_config.alias_name.is_empty() {
          error!("A unique aliasName is required for each dependency group");
          error!("{:?}", dependency_group_config);
          exit(1);
        }
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
