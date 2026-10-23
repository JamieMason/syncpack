use {
  crate::registry::client::{PackageMeta, RegistryError},
  serde_json::json,
  std::collections::BTreeMap,
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
    time: BTreeMap::new(),
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
    time: BTreeMap::new(),
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

#[derive(Debug, thiserror::Error)]
#[error("operation timed out")]
struct TimedOut;

#[derive(Debug, thiserror::Error)]
#[error("error decoding response body")]
struct DecodeFailed(#[source] TimedOut);

#[test]
fn reason_joins_every_nested_cause() {
  let err = RegistryError::FetchError {
    url: "https://registry.npmjs.org/typescript".to_string(),
    source: Box::new(DecodeFailed(TimedOut)),
  };
  assert_eq!(err.reason(), "error decoding response body: operation timed out");
}

#[test]
fn reason_for_an_http_error_is_the_status() {
  let err = RegistryError::HttpError {
    url: "https://registry.npmjs.org/typescript".to_string(),
    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
  };
  assert_eq!(err.reason(), "HTTP 500 Internal Server Error");
}
