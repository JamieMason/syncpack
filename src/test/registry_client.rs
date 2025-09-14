use {
  crate::{
    dependency::UpdateUrl,
    registry_client::{AllPackageVersions, RegistryClient, RegistryError},
  },
  reqwest::StatusCode,
  std::collections::BTreeMap,
};

/// A mock implementation of RegistryClient for testing
#[derive(Debug)]
pub struct MockRegistryClient {
  // Maps package names to a list of versions
  pub package_data: BTreeMap<String, Vec<String>>,
}

#[async_trait::async_trait]
impl RegistryClient for MockRegistryClient {
  /// Return a dynamically constructed PackageMeta based on the package name
  async fn fetch(&self, update_url: &UpdateUrl) -> Result<AllPackageVersions, RegistryError> {
    self
      .package_data
      .get(&update_url.internal_name)
      .map(|versions| AllPackageVersions {
        name: update_url.internal_name.to_string(),
        versions: versions.clone(),
      })
      .ok_or_else(|| RegistryError::HttpError {
        url: update_url.internal_name.to_string(),
        status: StatusCode::NOT_FOUND,
      })
  }
}

impl MockRegistryClient {
  /// Create a new MockRegistryClient from a serde_json::Value
  ///
  /// ```
  /// MockRegistryClient::from_json(json!({
  ///   "foo": ["1.2.3", "4.5.6"],
  ///   "bar": ["4.5.6", "0.1.2"]
  /// }))
  /// ```
  pub fn from_json(json_data: serde_json::Value) -> Self {
    let mut package_data = BTreeMap::new();
    if let serde_json::Value::Object(versions_by_name) = json_data {
      for (package_name, versions) in versions_by_name {
        if let serde_json::Value::Array(version_values) = versions {
          let versions: Vec<String> = version_values.iter().filter_map(|v| v.as_str().map(String::from)).collect();
          package_data.insert(package_name, versions);
        }
      }
    }
    MockRegistryClient { package_data }
  }
}
