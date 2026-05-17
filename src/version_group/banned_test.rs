use {
  crate::{
    instance::{FixableInstance::*, InstanceState, SuspectInstance::*, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
    },
  },
  serde_json::json,
};

#[tokio::test]
async fn refuses_to_ban_local_version() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0"
      }),
      json!({
        "name": "package-b",
        "dependencies": {"package-a": "1.1.0"}
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["package-a"],
      "isBanned": true
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(RefuseToBanLocal),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "package-a",
      id: "package-a in /dependencies of package-b",
      actual: "1.1.0",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn removes_instance_with_name_and_version_props_strategy() {
  let ctx = TestBuilder::new()
    .with_config(json!({
      "customTypes": {
        "customPackage": {
          "strategy": "name~version",
          "namePath": "customName",
          "path": "customVersion"
        }
      }
    }))
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "customName": "my-custom-package",
      "customVersion": "2.1.0"
    })])
    .with_version_group(json!({
      "dependencies": ["my-custom-package"],
      "dependencyTypes": ["customPackage"],
      "isBanned": true
    }))
    .run()
    .await;

  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "my-custom-package",
      id: "my-custom-package in /customVersion of package-a",
      actual: "2.1.0",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn removes_instance_with_named_version_string_strategy() {
  let ctx = TestBuilder::new()
    .with_config(json!({
      "customTypes": {
        "packageManager": {
          "strategy": "name@version",
          "path": "packageManager"
        }
      }
    }))
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "packageManager": "pnpm@7.27.0"
    })])
    .with_version_group(json!({
      "dependencies": ["pnpm"],
      "dependencyTypes": ["packageManager"],
      "isBanned": true
    }))
    .run()
    .await;

  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "pnpm",
      id: "pnpm in /packageManager of package-a",
      actual: "7.27.0",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn removes_instance_with_unnamed_version_string_strategy() {
  let ctx = TestBuilder::new()
    .with_config(json!({
      "customTypes": {
        "nodeVersion": {
          "strategy": "version",
          "path": "engines.node"
        }
      }
    }))
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "engines": {
        "node": ">=16.0.0"
      }
    })])
    .with_version_group(json!({
      "dependencies": ["nodeVersion"],
      "dependencyTypes": ["nodeVersion"],
      "isBanned": true
    }))
    .run()
    .await;

  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "nodeVersion",
      id: "nodeVersion in /engines/node of package-a",
      actual: ">=16.0.0",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn removes_instance_with_versions_by_name_strategy() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "dependencies": {
        "react": "18.0.0",
        "lodash": "4.17.21"
      }
    })])
    .with_version_group(json!({
      "dependencies": ["react"],
      "isBanned": true
    }))
    .run()
    .await;

  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "react",
      id: "react in /dependencies of package-a",
      actual: "18.0.0",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "lodash",
      id: "lodash in /dependencies of package-a",
      actual: "4.17.21",
      expected: Some("4.17.21"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn removes_nested_property_with_unnamed_version_string_strategy() {
  let ctx = TestBuilder::new()
    .with_config(json!({
      "customTypes": {
        "customConfig": {
          "strategy": "version",
          "path": "custom.config.version"
        }
      }
    }))
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "custom": {
        "config": {
          "version": "2.5.0",
          "other": "keep-this"
        }
      }
    })])
    .with_version_group(json!({
      "dependencies": ["customConfig"],
      "dependencyTypes": ["customConfig"],
      "isBanned": true
    }))
    .run()
    .await;

  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "customConfig",
      id: "customConfig in /custom/config/version of package-a",
      actual: "2.5.0",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Severity tests — opt out of auto-fix per status (issue #216).
/// IsBanned is the only configurable key on a Banned group.
mod severity {
  use {super::*, crate::instance::Severity};

  /// `IsBanned` defaults to severity `Fix` — the resolver picks `Fix` when
  /// the user has not specified a severity for the produced status. Pins
  /// today's default behaviour so it doesn't regress.
  #[tokio::test]
  async fn is_banned_default_severity_is_fix() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "1.0.0"}
      })])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "isBanned": true
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(IsBanned),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "1.0.0",
        expected: Some(""),
        overridden: None,
        severity: Some(Severity::Fix),
      },
    ]);
  }

  /// `severity: { IsBanned: "warn" }` — the resolved severity on the
  /// banned instance is `Warn` instead of the default `Fix`.
  #[tokio::test]
  async fn is_banned_warn() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "1.0.0"}
      })])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "isBanned": true,
        "severity": {"IsBanned": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(IsBanned),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "1.0.0",
        expected: Some(""),
        overridden: None,
        severity: Some(Severity::Warn),
      },
    ]);
  }

  /// `severity: { IsBanned: "error" }` — resolves to `Error`.
  #[tokio::test]
  async fn is_banned_error() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "1.0.0"}
      })])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "isBanned": true,
        "severity": {"IsBanned": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(IsBanned),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "1.0.0",
        expected: Some(""),
        overridden: None,
        severity: Some(Severity::Error),
      },
    ]);
  }

  /// Banned permits only `IsBanned`. `DiffersToPin` is a Pinned-only key →
  /// `InvalidSeverityKey` with the offending key, the resolved group type,
  /// and the permitted set.
  #[tokio::test]
  #[should_panic(expected = "InvalidSeverityKey")]
  async fn rejects_invalid_severity_key() {
    let _ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "1.0.0"}
      })])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "isBanned": true,
        "severity": {"DiffersToPin": "error"}
      }))
      .run()
      .await;
  }
}
