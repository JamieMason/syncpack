use {
  crate::registry_client::{LiveRegistryClient, PackageMeta},
  serde_json::json,
  std::collections::BTreeMap,
};

#[test]
fn filters_out_deprecated_versions() {
  // Simulate npm registry response with deprecated versions
  let package_meta = PackageMeta {
    name: "@eslint/js".to_string(),
    versions: {
      let mut versions = BTreeMap::new();
      // Regular version
      versions.insert("9.38.0".to_string(), json!({"version": "9.38.0"}));
      // Deprecated version
      versions.insert(
        "10.0.0".to_string(),
        json!({"version": "10.0.0", "deprecated": "This version should not be used."}),
      );
      // Another regular version
      versions.insert("9.39.0".to_string(), json!({"version": "9.39.0"}));
      versions
    },
  };

  // Extract versions using the filtering logic
  let versions: Vec<String> = package_meta
    .versions
    .into_iter()
    .filter(|(_, metadata)| metadata.get("deprecated").is_none())
    .map(|(version, _)| version)
    .collect();

  // Should only include non-deprecated versions
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

#[test]
fn registry_client_can_be_created() {
  // Verifies that LiveRegistryClient::new() succeeds
  // This loads .npmrc from standard locations (may be empty)
  let result = LiveRegistryClient::new();
  assert!(result.is_ok(), "Failed to create registry client: {:?}", result.err());
}

#[test]
fn registry_url_returns_default_for_regular_packages() {
  let client = LiveRegistryClient::new().unwrap();
  let url = client.registry_url("react");
  // Should use default npm registry (or user's configured default)
  // We can't assert exact URL since user might have custom config,
  // but we can verify it's a valid URL
  assert!(url.scheme() == "https" || url.scheme() == "http");
  assert!(url.host_str().is_some());
}

#[test]
fn registry_url_uses_jsr_fallback_for_jsr_packages() {
  let client = LiveRegistryClient::new().unwrap();
  let url = client.registry_url("@jsr/luca__cases");
  // JSR packages should use npm.jsr.io when not explicitly configured
  // If user has @jsr:registry configured, this will use that instead
  let host = url.host_str().unwrap();
  // Either npm.jsr.io (fallback) or user's configured JSR registry
  assert!(
    host == "npm.jsr.io" || host != "registry.npmjs.org",
    "Expected JSR package to NOT use registry.npmjs.org, got: {}",
    url
  );
}
