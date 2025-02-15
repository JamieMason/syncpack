use {
  crate::{
    instance_state::{FixableInstance::*, InstanceState, SuspectInstance::*},
    test::{
      self,
      expect::{expect, ExpectedInstance},
    },
    visit_packages::visit_packages,
    Context,
  },
  serde_json::json,
};

#[test]
fn refuses_to_ban_local_version() {
  let config = test::mock::config_from_mock(json!({
    "versionGroups": [{
      "dependencies": ["package-a"],
      "isBanned": true
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
    json!({
      "name": "package-a",
      "version": "1.0.0"
    }),
    json!({
      "name": "package-b",
      "dependencies": {
        "package-a": "1.1.0"
      }
    }),
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(RefuseToBanLocal),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "package-a",
      id: "package-a in /dependencies of package-b",
      actual: "1.1.0",
      expected: Some(""),
      overridden: None,
    },
  ]);
}
