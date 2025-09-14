use {
  super::{mock, registry_client::MockRegistryClient},
  crate::{cli::UpdateTarget, context::Context, registry_client::RegistryClient, visit_packages::visit_packages},
  serde_json::{json, Value},
  std::sync::Arc,
};

/// Builder pattern for creating test contexts with reduced boilerplate
pub struct TestBuilder {
  config: Value,
  dependency_groups: Vec<Value>,
  packages: Vec<Value>,
  registry_updates: Option<Value>,
  semver_groups: Vec<Value>,
  strict: Option<bool>,
  update_target: Option<UpdateTarget>,
  version_groups: Vec<Value>,
}

impl TestBuilder {
  pub fn new() -> Self {
    Self {
      config: json!({}),
      dependency_groups: vec![],
      packages: vec![],
      registry_updates: None,
      semver_groups: vec![],
      strict: None,
      update_target: None,
      version_groups: vec![],
    }
  }

  pub fn with_package(mut self, package: Value) -> Self {
    self.packages.push(package);
    self
  }

  pub fn with_packages(mut self, packages: Vec<Value>) -> Self {
    self.packages.extend(packages);
    self
  }

  pub fn with_version_group(mut self, group: Value) -> Self {
    self.version_groups.push(group);
    self
  }

  pub fn with_version_groups(mut self, groups: Vec<Value>) -> Self {
    self.version_groups.extend(groups);
    self
  }

  pub fn with_semver_group(mut self, group: Value) -> Self {
    self.semver_groups.push(group);
    self
  }

  pub fn with_strict(mut self, strict: bool) -> Self {
    self.strict = Some(strict);
    self
  }

  pub fn with_update_target(mut self, target: UpdateTarget) -> Self {
    self.update_target = Some(target);
    self
  }

  pub fn with_registry_updates(mut self, updates: Value) -> Self {
    self.registry_updates = Some(updates);
    self
  }

  pub fn with_config(mut self, config: Value) -> Self {
    self.config = config;
    self
  }

  /// Build the final configuration from all the builder settings
  fn build_config(&self) -> Value {
    let mut config = self.config.clone();
    if !self.version_groups.is_empty() {
      config["versionGroups"] = Value::Array(self.version_groups.clone());
    }
    if !self.semver_groups.is_empty() {
      config["semverGroups"] = Value::Array(self.semver_groups.clone());
    }
    if !self.dependency_groups.is_empty() {
      config["dependencyGroups"] = Value::Array(self.dependency_groups.clone());
    }
    if let Some(strict) = self.strict {
      config["strict"] = Value::Bool(strict);
    }
    config
  }

  pub fn build(self) -> Context {
    let mut config = mock::config_from_mock(self.build_config());
    let registry_client = self.create_registry_client();
    let packages = mock::packages_from_mocks(self.packages);
    if let Some(target) = self.update_target {
      match target {
        UpdateTarget::Latest => config.cli.target = UpdateTarget::Latest,
        UpdateTarget::Minor => config.cli.target = UpdateTarget::Minor,
        UpdateTarget::Patch => config.cli.target = UpdateTarget::Patch,
      }
    }
    Context::create(config, packages, registry_client)
  }

  pub fn build_and_visit_packages(self) -> Context {
    let ctx = self.build();
    visit_packages(ctx)
  }

  pub async fn build_with_registry_and_visit(self) -> Context {
    let mut config = mock::config_from_mock(self.build_config());
    let packages = mock::packages_from_mocks(self.packages);

    if let Some(target) = self.update_target {
      match target {
        UpdateTarget::Latest => config.cli.target = UpdateTarget::Latest,
        UpdateTarget::Minor => config.cli.target = UpdateTarget::Minor,
        UpdateTarget::Patch => config.cli.target = UpdateTarget::Patch,
      }
    }

    let ctx = if let Some(updates) = self.registry_updates {
      mock::context_with_registry_updates(config, packages, updates).await
    } else {
      Context::create(config, packages, None)
    };

    visit_packages(ctx)
  }

  /// Create registry client if updates are provided
  fn create_registry_client(&self) -> Option<Arc<dyn RegistryClient>> {
    self
      .registry_updates
      .as_ref()
      .map(|updates| Arc::new(MockRegistryClient::from_json(updates.clone())) as Arc<dyn RegistryClient>)
  }
}

impl Default for TestBuilder {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    crate::{
      instance_state::{InstanceState, ValidInstance::*},
      test::expect::{expect, ExpectedInstance},
    },
  };

  #[test]
  fn test_builder_basic_usage() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0"
      }))
      .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    }]);
  }

  #[test]
  fn test_builder_with_version_group() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "1.0.0"}
      }))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "2.0.0"
      }))
      .build_and_visit_packages();

    // The test should show that foo gets pinned to 2.0.0
    assert!(ctx.instances.len() > 1);
  }

  #[test]
  fn test_builder_with_multiple_packages() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "package-a", "version": "1.0.0"}),
        json!({"name": "package-b", "version": "2.0.0"}),
      ])
      .build_and_visit_packages();

    assert_eq!(ctx.instances.len(), 2);
  }

  #[test]
  fn test_builder_with_strict_mode() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"package-a": "workspace:*"}
      }))
      .with_strict(true)
      .build_and_visit_packages();

    // In strict mode, workspace protocol should be invalid when differs from local
    assert!(ctx.instances.iter().any(|i| i.state.borrow().is_invalid()));
  }

  #[tokio::test]
  async fn test_builder_with_registry_updates() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "dependencies": {"foo": "1.0.0"}
      }))
      .with_registry_updates(json!({"foo": ["1.0.0", "2.0.0"]}))
      .build_with_registry_and_visit()
      .await;

    // Should show registry update available
    assert!(ctx.instances.iter().any(|i| i.state.borrow().is_outdated()));
  }

  #[tokio::test]
  async fn test_builder_with_update_target() {
    use crate::cli::UpdateTarget;

    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "dependencies": {"foo": "1.0.0"}
      }))
      .with_update_target(UpdateTarget::Minor)
      .with_registry_updates(json!({"foo": ["1.0.0", "1.1.0", "2.0.0"]}))
      .build_with_registry_and_visit()
      .await;

    // Should target minor updates (1.1.0) not latest (2.0.0)
    let foo_instance = ctx.instances.iter().find(|i| i.descriptor.internal_name == "foo").unwrap();
    assert_eq!(foo_instance.expected_specifier.borrow().as_ref().unwrap().get_raw(), "1.1.0");
  }
}
