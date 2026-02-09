use {
  crate::registry_client::{LiveRegistryClient, PackageMeta},
  npmrc_config_rs::{LoadOptions, NpmrcConfig},
  serde_json::json,
  std::{collections::BTreeMap, fs},
  tempfile::TempDir,
};

#[test]
fn filters_out_deprecated_versions() {
  let package_meta = PackageMeta {
    name: "@eslint/js".to_string(),
    versions: {
      let mut versions = BTreeMap::new();
      versions.insert("9.38.0".to_string(), json!({"version": "9.38.0"}));
      versions.insert(
        "10.0.0".to_string(),
        json!({"version": "10.0.0", "deprecated": "This version should not be used."}),
      );
      versions.insert("9.39.0".to_string(), json!({"version": "9.39.0"}));
      versions
    },
  };

  let versions: Vec<String> = package_meta
    .versions
    .into_iter()
    .filter(|(_, metadata)| metadata.get("deprecated").is_none())
    .map(|(version, _)| version)
    .collect();

  assert_eq!(versions.len(), 2);
  assert!(versions.contains(&"9.38.0".to_string()));
  assert!(versions.contains(&"9.39.0".to_string()));
  assert!(!versions.contains(&"10.0.0".to_string()));
}

#[test]
fn includes_all_versions_when_none_deprecated() {
  let package_meta = PackageMeta {
    name: "test-package".to_string(),
    versions: {
      let mut versions = BTreeMap::new();
      versions.insert("1.0.0".to_string(), json!({"version": "1.0.0"}));
      versions.insert("2.0.0".to_string(), json!({"version": "2.0.0"}));
      versions.insert("3.0.0".to_string(), json!({"version": "3.0.0"}));
      versions
    },
  };

  let versions: Vec<String> = package_meta
    .versions
    .into_iter()
    .filter(|(_, metadata)| metadata.get("deprecated").is_none())
    .map(|(version, _)| version)
    .collect();

  assert_eq!(versions.len(), 3);
  assert!(versions.contains(&"1.0.0".to_string()));
  assert!(versions.contains(&"2.0.0".to_string()));
  assert!(versions.contains(&"3.0.0".to_string()));
}

fn make_client() -> (TempDir, LiveRegistryClient) {
  let dir = TempDir::new().expect("create temp dir");
  fs::write(dir.path().join("package.json"), "{}").expect("write package.json");
  fs::write(dir.path().join(".npmrc"), "").expect("write .npmrc");
  let npmrc = NpmrcConfig::load_with_options(LoadOptions {
    cwd: Some(dir.path().to_path_buf()),
    skip_user: true,
    skip_global: true,
    ..Default::default()
  })
  .expect("load isolated npmrc config");
  (dir, LiveRegistryClient::new(npmrc))
}

#[test]
fn resolve_url_returns_default_for_regular_packages() {
  let (_dir, client) = make_client();
  let (url, registry_base) = client.resolve_url("react").unwrap();
  assert!(url.as_str().ends_with("/react"), "URL should end with package name, got: {url}");
  assert_eq!(registry_base.host_str().unwrap(), "registry.npmjs.org");
}

#[test]
fn resolve_url_uses_jsr_fallback_for_jsr_packages() {
  let (_dir, client) = make_client();
  let (url, _) = client.resolve_url("@jsr/luca__cases").unwrap();
  assert_ne!(
    url.host_str().unwrap(),
    "registry.npmjs.org",
    "Expected JSR package to NOT use registry.npmjs.org, got: {url}",
  );
}
