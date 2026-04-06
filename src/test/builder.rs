#![allow(dead_code)]

use {
  super::mock,
  crate::{
    catalogs::CatalogsByName, cli::UpdateTarget, context::Context, visit_formatting::visit_formatting, visit_packages::visit_packages,
  },
  serde_json::{json, Value},
};

#[cfg(test)]
#[path = "builder_test.rs"]
mod builder_test;

/// Builder pattern for creating test contexts with reduced boilerplate
pub struct TestBuilder {
  catalogs: Option<Value>,
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
      catalogs: None,
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

  pub fn with_catalogs(mut self, catalogs: Value) -> Self {
    self.catalogs = Some(catalogs);
    self
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
    let catalogs = self.create_catalogs();
    let packages = mock::packages_from_mocks(self.packages);
    if let Some(target) = self.update_target {
      match target {
        UpdateTarget::Latest => config.cli.target = UpdateTarget::Latest,
        UpdateTarget::Minor => config.cli.target = UpdateTarget::Minor,
        UpdateTarget::Patch => config.cli.target = UpdateTarget::Patch,
      }
    }
    Context::create(config, packages, catalogs).unwrap()
  }

  pub fn build_and_visit_packages(self) -> Context {
    let ctx = self.build();
    visit_packages(ctx, &None)
  }

  pub fn build_and_visit_formatting(self) -> Context {
    let ctx = self.build();
    visit_formatting(ctx)
  }

  pub async fn build_with_registry_and_visit(self) -> Context {
    let mut config = mock::config_from_mock(self.build_config());
    let catalogs = self.create_catalogs();
    let packages = mock::packages_from_mocks(self.packages);

    if let Some(target) = self.update_target {
      match target {
        UpdateTarget::Latest => config.cli.target = UpdateTarget::Latest,
        UpdateTarget::Minor => config.cli.target = UpdateTarget::Minor,
        UpdateTarget::Patch => config.cli.target = UpdateTarget::Patch,
      }
    }

    if let Some(mock_updates) = self.registry_updates {
      let (ctx, updates) = mock::context_with_registry_updates(config, packages, mock_updates, catalogs).await;
      visit_packages(ctx, &Some(updates))
    } else {
      let ctx = Context::create(config, packages, catalogs).unwrap();
      visit_packages(ctx, &None)
    }
  }

  /// Create catalogs if provided
  fn create_catalogs(&self) -> Option<CatalogsByName> {
    self.catalogs.as_ref().map(|catalogs| mock::catalogs_from_mocks(catalogs.clone()))
  }
}

impl Default for TestBuilder {
  fn default() -> Self {
    Self::new()
  }
}
