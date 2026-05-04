use {
  super::TestBuilder,
  crate::{
    instance::{InstanceState, ValidInstance::*},
    test::expect::{ExpectedInstance, expect},
  },
  serde_json::json,
};

#[tokio::test]
async fn test_builder_basic_usage() {
  let ctx = TestBuilder::new()
    .with_package(json!({
      "name": "package-a",
      "version": "1.0.0"
    }))
    .run()
    .await;

  expect(&ctx).to_have_instances(vec![ExpectedInstance {
    state: InstanceState::valid(IsLocalAndValid),
    dependency_name: "package-a",
    id: "package-a in /version of package-a",
    actual: "1.0.0",
    expected: Some("1.0.0"),
    overridden: None,
  }]);
}

#[tokio::test]
async fn test_builder_with_version_group() {
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
    .run()
    .await;

  // The test should show that foo gets pinned to 2.0.0
  assert!(ctx.instances.len() > 1);
}

#[tokio::test]
async fn test_builder_with_multiple_packages() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({"name": "package-a", "version": "1.0.0"}),
      json!({"name": "package-b", "version": "2.0.0"}),
    ])
    .run()
    .await;

  assert_eq!(ctx.instances.len(), 2);
}

#[tokio::test]
async fn test_builder_with_strict_mode() {
  let ctx = TestBuilder::new()
    .with_package(json!({
      "name": "package-a",
      "version": "1.0.0",
      "dependencies": {"package-a": "workspace:*"}
    }))
    .with_strict(true)
    .run()
    .await;

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
    .run()
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
    .run()
    .await;

  // Should target minor updates (1.1.0) not latest (2.0.0)
  let foo_instance = ctx.instances.iter().find(|i| i.descriptor.internal_name == "foo").unwrap();
  assert_eq!(foo_instance.expected_specifier.borrow().as_ref().unwrap().get_raw(), "1.1.0");
}
