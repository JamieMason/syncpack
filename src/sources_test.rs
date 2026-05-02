use {
  crate::{
    dependency::{DependencyType, Strategy},
    disk::{detect_formatting, parse_yaml_file, Disk, File},
    source::SourceKind,
    sources::{parse_catalog_name, Sources},
  },
  serde_json::json,
  std::path::PathBuf,
};

fn pkg_json_dep_type(name: &str, path: &str, strategy: Strategy) -> DependencyType {
  DependencyType {
    name_path: None,
    name: name.to_string(),
    path: path.to_string(),
    strategy,
    source: SourceKind::PackageJson,
    is_catalog_definition: false,
  }
}

fn pnpm_yaml_user_dep_type(name: &str, path: &str) -> DependencyType {
  DependencyType {
    name_path: None,
    name: name.to_string(),
    path: path.to_string(),
    strategy: Strategy::VersionsByName,
    source: SourceKind::PnpmWorkspace,
    is_catalog_definition: false,
  }
}

fn auto_gen_pnpm_catalog_dep_type(name: &str, path: &str) -> DependencyType {
  DependencyType {
    name_path: None,
    name: name.to_string(),
    path: path.to_string(),
    strategy: Strategy::VersionsByName,
    source: SourceKind::PnpmWorkspace,
    is_catalog_definition: true,
  }
}

fn name_version_props_dep_type(name: &str, name_path: &str, path: &str) -> DependencyType {
  DependencyType {
    name_path: Some(name_path.to_string()),
    name: name.to_string(),
    path: path.to_string(),
    strategy: Strategy::NameAndVersionProps,
    source: SourceKind::PackageJson,
    is_catalog_definition: false,
  }
}

/// Build a `Disk` with N synthetic package.json files at /packages/{name}/package.json
/// plus optional yaml. Returns disk + all paths (used as user-pattern-filtered list).
fn disk_with_packages(values: &[serde_json::Value], yaml: Option<&str>) -> (Disk, Vec<PathBuf>) {
  let mut package_json_files: Vec<File<serde_json::Value>> = Vec::new();
  let mut all_paths: Vec<PathBuf> = Vec::new();
  for value in values {
    let name = value
      .pointer("/name")
      .and_then(|n| n.as_str())
      .unwrap_or("NAME_IS_MISSING")
      .to_string();
    let raw = serde_json::to_string_pretty(value).unwrap_or_default();
    let filepath = PathBuf::from(format!("/packages/{name}/package.json"));
    package_json_files.push(File {
      filepath: filepath.clone(),
      formatting: detect_formatting(&raw),
      contents: value.clone(),
      dirty: false,
    });
    all_paths.push(filepath);
  }
  let pnpm_workspace = yaml.and_then(|raw| parse_yaml_file(raw.to_string(), PathBuf::from("/test/pnpm-workspace.yaml")));
  let disk = Disk {
    cwd: PathBuf::from("/test"),
    lerna_json: None,
    package_json_files,
    package_json_root_idx: None,
    package_manager: None,
    pnpm_workspace,
  };
  (disk, all_paths)
}

#[test]
fn sources_packages_iter_only_yields_packages() {
  let (disk, file_paths) = disk_with_packages(&[json!({"name": "a"}), json!({"name": "b"})], Some("packages: ['*']\n"));
  let sources = Sources::from_disk(&disk, &file_paths);
  let names: Vec<&str> = sources.packages_iter().map(|(_, s)| s.name()).collect();
  assert_eq!(names, vec!["a", "b"], "yaml source must be excluded");
}

#[test]
fn sources_find_package_returns_idx_or_none() {
  let (disk, file_paths) = disk_with_packages(&[json!({"name": "a"}), json!({"name": "b"})], Some("packages: ['*']\n"));
  let sources = Sources::from_disk(&disk, &file_paths);
  let a_idx = sources.find_package("a").expect("found a");
  let b_idx = sources.find_package("b").expect("found b");
  assert_eq!(a_idx.0, 0, "a is at arena slot 0");
  assert_eq!(b_idx.0, 1, "b is at arena slot 1");
  assert!(sources.find_package("missing").is_none());
}

#[test]
fn parse_catalog_name_default_for_bare() {
  assert_eq!(parse_catalog_name("pnpmCatalog"), "default");
  assert_eq!(parse_catalog_name("bunCatalog"), "default");
  assert_eq!(parse_catalog_name("pnpmCatalog:react18"), "react18");
  assert_eq!(parse_catalog_name("bunCatalog:react.18"), "react.18");
}

#[test]
fn iter_skips_dep_type_with_mismatched_source() {
  let (disk, file_paths) = disk_with_packages(
    &[json!({
      "name": "a",
      "dependencies": { "react": "^18.0.0" }
    })],
    None,
  );
  let sources = Sources::from_disk(&disk, &file_paths);
  let dep_types = vec![pnpm_yaml_user_dep_type("foo", "/catalog")];
  let descriptors: Vec<_> = sources.iter_instances(&disk, &dep_types).collect();
  assert!(descriptors.is_empty(), "package source must not be iterated by yaml dep type");
}

#[test]
fn iter_emits_instance_for_matching_source() {
  let (disk, file_paths) = disk_with_packages(&[], Some("catalog:\n  react: ^18.0.0\n"));
  let sources = Sources::from_disk(&disk, &file_paths);
  let dep_types = vec![auto_gen_pnpm_catalog_dep_type("pnpmCatalog", "/catalog")];
  let descriptors: Vec<_> = sources.iter_instances(&disk, &dep_types).collect();
  assert_eq!(descriptors.len(), 1);
  assert_eq!(descriptors[0].name, "react");
  assert!(
    descriptors[0].dependency_type.is_catalog_definition,
    "expected catalog instance source"
  );
  assert_eq!(parse_catalog_name(&descriptors[0].dependency_type.name), "default");
}

#[test]
fn iter_emits_catalog_variant_for_is_catalog_definition_dep_type() {
  let (disk, file_paths) = disk_with_packages(&[], Some("catalogs:\n  react18:\n    react: ^18.0.0\n"));
  let sources = Sources::from_disk(&disk, &file_paths);
  let dep_types = vec![auto_gen_pnpm_catalog_dep_type("pnpmCatalog:react18", "/catalogs/react18")];
  let descriptors: Vec<_> = sources.iter_instances(&disk, &dep_types).collect();
  assert_eq!(descriptors.len(), 1);
  assert!(
    descriptors[0].dependency_type.is_catalog_definition,
    "expected catalog source for is_catalog_definition dep type"
  );
  assert_eq!(parse_catalog_name(&descriptors[0].dependency_type.name), "react18");
}

#[test]
fn iter_emits_package_variant_for_user_customtype_pnpm_workspace_source() {
  let (disk, file_paths) = disk_with_packages(&[], Some("foo:\n  react: ^18.0.0\n"));
  let sources = Sources::from_disk(&disk, &file_paths);
  // User customType reading yaml — is_catalog_definition is FALSE.
  let dep_types = vec![pnpm_yaml_user_dep_type("foo", "/foo")];
  let descriptors: Vec<_> = sources.iter_instances(&disk, &dep_types).collect();
  assert_eq!(descriptors.len(), 1);
  assert!(
    !descriptors[0].dependency_type.is_catalog_definition,
    "user customType yaml source must NOT produce a catalog instance"
  );
}

#[test]
fn iter_handles_versions_by_name_strategy() {
  let (disk, file_paths) = disk_with_packages(&[], Some("catalog:\n  react: ^18.0.0\n  vue: ^3.0.0\n"));
  let sources = Sources::from_disk(&disk, &file_paths);
  let dep_types = vec![auto_gen_pnpm_catalog_dep_type("pnpmCatalog", "/catalog")];
  let mut names: Vec<String> = sources.iter_instances(&disk, &dep_types).map(|d| d.name).collect();
  names.sort();
  assert_eq!(names, vec!["react".to_string(), "vue".to_string()]);
}

#[test]
fn iter_handles_name_and_version_props_strategy() {
  // Pick a non-`local` name to avoid the empty-version fallback baked into
  // the iteration.
  let dep_types = vec![name_version_props_dep_type("custom", "/name", "/version")];

  let (disk, file_paths) = disk_with_packages(
    &[json!({
      "name": "a",
      "dependencies": { "react": "^18.0.0" }
    })],
    None,
  );
  let sources = Sources::from_disk(&disk, &file_paths);
  let descriptors: Vec<_> = sources.iter_instances(&disk, &dep_types).collect();
  assert!(descriptors.is_empty(), "no /version → silent skip");

  let (disk, file_paths) = disk_with_packages(
    &[json!({
      "name": "a",
      "version": "1.2.3"
    })],
    None,
  );
  let sources = Sources::from_disk(&disk, &file_paths);
  let names: Vec<String> = sources.iter_instances(&disk, &dep_types).map(|d| d.name).collect();
  assert_eq!(names, vec!["a".to_string()]);
}

#[test]
fn iter_silent_skip_on_no_match() {
  let (disk, file_paths) = disk_with_packages(&[], Some("# empty\n"));
  let sources = Sources::from_disk(&disk, &file_paths);
  let dep_types = vec![DependencyType {
    name_path: Some("/catalog/name".to_string()),
    name: "weirdo".to_string(),
    path: "/catalog/version".to_string(),
    strategy: Strategy::NameAndVersionProps,
    source: SourceKind::PnpmWorkspace,
    is_catalog_definition: false,
  }];
  let descriptors: Vec<_> = sources.iter_instances(&disk, &dep_types).collect();
  assert!(descriptors.is_empty());
}

#[test]
fn iter_local_package_names_excludes_yaml_sources() {
  // yaml's "name" is the literal "pnpm-workspace.yaml" — must NOT be treated
  // as a local package when computing is_local_dependency for catalog defs.
  let (disk, file_paths) = disk_with_packages(
    &[json!({"name": "real-package"})],
    Some("catalog:\n  pnpm-workspace.yaml: ^1.0.0\n  real-package: ^2.0.0\n"),
  );
  let sources = Sources::from_disk(&disk, &file_paths);
  let dep_types = vec![auto_gen_pnpm_catalog_dep_type("pnpmCatalog", "/catalog")];
  let mut got: Vec<(String, bool)> = sources
    .iter_instances(&disk, &dep_types)
    .map(|d| (d.name, d.is_local_dependency))
    .collect();
  got.sort();
  assert_eq!(
    got,
    vec![("pnpm-workspace.yaml".to_string(), false), ("real-package".to_string(), true),]
  );
}

#[test]
fn iter_pkg_json_dep_type_emits_package_variant() {
  let (disk, file_paths) = disk_with_packages(
    &[json!({
      "name": "a",
      "dependencies": { "react": "^18.0.0" }
    })],
    None,
  );
  let sources = Sources::from_disk(&disk, &file_paths);
  let dep_types = vec![pkg_json_dep_type("dev", "/dependencies", Strategy::VersionsByName)];
  let descriptors: Vec<_> = sources.iter_instances(&disk, &dep_types).collect();
  assert_eq!(descriptors.len(), 1);
  assert_eq!(descriptors[0].name, "react");
  assert!(!descriptors[0].dependency_type.is_catalog_definition);
}

#[test]
fn sources_from_disk_mirrors_package_json_files_one_to_one() {
  let (disk, all_paths) = disk_with_packages(&[json!({"name": "a"}), json!({"name": "b"})], None);
  let sources = Sources::from_disk(&disk, &all_paths);
  assert_eq!(sources.all.len(), 2, "no yaml: 1:1 with disk.package_json_files");

  let (disk_y, all_paths_y) = disk_with_packages(&[json!({"name": "a"}), json!({"name": "b"})], Some("packages: ['*']\n"));
  let sources_y = Sources::from_disk(&disk_y, &all_paths_y);
  assert_eq!(sources_y.all.len(), 3, "with yaml: +1 for PnpmYaml unit variant");
  assert_eq!(sources_y.pnpm_yaml_source_idx, Some(2), "yaml at tail slot");
}

mod via_full_pipeline {
  use {
    crate::{
      instance::{InstanceState, ValidInstance::*},
      test::{
        builder::TestBuilder,
        expect::{expect, ExpectedInstance},
      },
    },
    serde_json::json,
  };

  #[tokio::test]
  async fn iter_instances_pass1_bun_catalog_against_root_regardless_of_user_pattern() {
    // Bun catalog discovery against root pkg.json runs even when the user
    // `source` pattern excludes the root. The catalog instance still appears;
    // the root's non-catalog deps do not.
    let ctx = TestBuilder::new()
      .with_bun_catalogs(json!({
        "name": "root",
        "version": "0.0.0",
        "catalog": {"react": "^18.0.0"}
      }))
      .with_packages(vec![json!({
        "name": "a",
        "version": "0.0.0",
        "dependencies": {"foo": "1.0.0"}
      })])
      .with_config(json!({"source": ["packages/*"]}))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "a",
        id: "a in /version of a",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalog of root",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
      },
    ]);
  }

  #[tokio::test]
  async fn iter_instances_pass2_non_catalog_skips_non_user_sources() {
    // Bun root's regular `/dependencies` is NOT iterated when user `source`
    // pattern excludes root. Only the workspace package's dep surfaces.
    let ctx = TestBuilder::new()
      .with_bun_catalogs(json!({
        "name": "root",
        "version": "0.0.0",
        "dependencies": {"root-only": "1.0.0"}
      }))
      .with_packages(vec![json!({
        "name": "a",
        "version": "0.0.0",
        "dependencies": {"react": "^18.0.0"}
      })])
      .with_config(json!({"source": ["packages/*"]}))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "a",
        id: "a in /version of a",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "react",
        id: "react in /dependencies of a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
      },
    ]);
  }

  #[tokio::test]
  async fn sources_user_source_indices_excludes_root_when_user_pattern_excludes_root() {
    // User pattern excludes root → root's /version + /dependencies are not
    // emitted; only the workspace package's instances appear.
    let ctx = TestBuilder::new()
      .with_bun_catalogs(json!({
        "name": "root",
        "version": "0.0.0",
        "dependencies": {"root-only": "1.0.0"}
      }))
      .with_packages(vec![
        json!({"name": "a", "version": "0.0.0"}),
        json!({"name": "b", "version": "0.0.0"}),
      ])
      .with_config(json!({"source": ["packages/*"]}))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "a",
        id: "a in /version of a",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "b",
        id: "b in /version of b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
      },
    ]);
  }

  #[tokio::test]
  async fn sources_user_source_indices_includes_root_when_user_pattern_includes_root() {
    // User pattern includes root → root's /version + /dependencies surface
    // alongside the workspace package's instances.
    let ctx = TestBuilder::new()
      .with_bun_catalogs(json!({
        "name": "root",
        "version": "0.0.0",
        "dependencies": {"root-only": "1.0.0"}
      }))
      .with_packages(vec![json!({"name": "a", "version": "0.0.0"})])
      .with_config(json!({"source": ["package.json", "packages/*"]}))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "root",
        id: "root in /version of root",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "a",
        id: "a in /version of a",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "root-only",
        id: "root-only in /dependencies of root",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }
}
