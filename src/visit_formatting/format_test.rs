use {
  crate::test::{
    builder::TestBuilder,
    expect::{ExpectedFormatMismatch, expect},
  },
  serde_json::{Value, json},
  syncpack_rules::inspection::FormatMismatchVariant::*,
};

/// An rcfile with every formatting rule switched off, so a test enables only
/// the rule it exercises. Rules named in `keep` are left at their rcfile
/// defaults; `overrides` are layered on top.
fn rcfile_with(keep: &[&str], overrides: Value) -> Value {
  let mut config = json!({
    "sortPackages": false,
    "sortFirst": [],
    "sortAz": [],
    "sortExports": [],
    "formatBugs": false,
    "formatRepository": false,
  });
  let rules = config.as_object_mut().unwrap();
  for rule in keep {
    rules.remove(*rule);
  }
  rules.extend(overrides.as_object().unwrap().clone());
  config
}

#[tokio::test]
async fn reports_nothing_when_every_rule_is_off() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&[], json!({})))
    .with_package(json!({
      "z": "",
      "name": "a",
      "bugs": { "url": "https://github.com/User/repo/issues" },
      "repository": { "url": "git://github.com/User/repo", "type": "git" },
      "keywords": ["b", "a"],
      "exports": { "default": "./default.cjs", "import": "./index-module.js" },
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches("a", vec![]);
}

#[tokio::test]
async fn reports_mismatches_against_the_package_that_has_them() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&["sortAz"], json!({})))
    .with_packages(vec![
      json!({ "name": "a", "keywords": ["B", "A"] }),
      json!({ "name": "b", "keywords": ["A", "B"] }),
    ])
    .run()
    .await;
  expect(&ctx)
    .to_have_format_mismatches(
      "a",
      vec![ExpectedFormatMismatch {
        variant: PropertyIsNotSortedAz,
        property_path: "/keywords",
        expected: json!(["A", "B"]),
      }],
    )
    .to_have_format_mismatches("b", vec![]);
}

#[tokio::test]
async fn formats_bugs_into_github_shorthand() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&[], json!({ "formatBugs": true })))
    .with_package(json!({
      "name": "a",
      "bugs": {
        "url": "https://github.com/User/repo/issues"
      }
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: BugsPropertyIsNotFormatted,
      property_path: "/bugs",
      expected: json!("https://github.com/User/repo/issues"),
    }],
  );
}

#[tokio::test]
async fn formats_repository_into_gitlab_shorthand() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&[], json!({ "formatRepository": true })))
    .with_package(json!({
      "name": "a",
      "repository": {
        "url": "git://gitlab.com/User/repo",
        "type": "git",
      },
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: RepositoryPropertyIsNotFormatted,
      property_path: "/repository",
      expected: json!("git://gitlab.com/User/repo"),
    }],
  );
}

#[tokio::test]
async fn formats_repository_into_github_shorthand() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&[], json!({ "formatRepository": true })))
    .with_package(json!({
      "name": "a",
      "repository": {
        "url": "git://github.com/User/repo",
        "type": "git",
      },
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: RepositoryPropertyIsNotFormatted,
      property_path: "/repository",
      expected: json!("User/repo"),
    }],
  );
}

#[tokio::test]
async fn retains_long_format_when_directory_property_used() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&[], json!({ "formatRepository": true })))
    .with_package(json!({
      "name": "a",
      "repository": {
        "url": "git://gitlab.com/User/repo",
        "type": "git",
        "directory": "packages/foo",
      },
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches("a", vec![]);
}

#[tokio::test]
async fn sorts_conditional_exports() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&["sortExports"], json!({})))
    .with_package(json!({
      "name": "a",
      "exports": {
          "default": "./default.cjs",
          "svelte": "./index.svelte",
          "require": "./index-require.cjs",
          "import": "./index-module.js",
      },
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: ExportsPropertyIsNotSorted,
      property_path: "/exports",
      expected: json!({
        "import": "./index-module.js",
        "require": "./index-require.cjs",
        "svelte": "./index.svelte",
        "default": "./default.cjs",
      }),
    }],
  );
}

#[tokio::test]
async fn returns_none_when_conditional_exports_already_sorted() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&["sortExports"], json!({})))
    .with_package(json!({
      "name": "a",
      "exports": {
          "import": "./index-module.js",
          "require": "./index-require.cjs",
      },
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches("a", vec![]);
}

#[tokio::test]
async fn sorts_conditional_exports_sub_paths() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&["sortExports"], json!({})))
    .with_package(json!({
      "name": "a",
      "exports": {
        ".": "./index.js",
        "./feature.js": {
          "default": "./feature.js",
          "node": "./feature-node.js",
        },
      },
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: ExportsPropertyIsNotSorted,
      property_path: "/exports",
      expected: json!({
        ".": "./index.js",
        "./feature.js": {
          "node": "./feature-node.js",
          "default": "./feature.js",
        },
      }),
    }],
  );
}

#[tokio::test]
async fn returns_none_when_conditional_exports_sub_paths_already_sorted() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&["sortExports"], json!({})))
    .with_package(json!({
      "name": "a",
      "exports": {
          ".": "./index.js",
          "./feature.js": {
            "node": "./feature-node.js",
            "default": "./feature.js",
          },
      },
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches("a", vec![]);
}

#[tokio::test]
async fn sorts_object_properties_alphabetically_by_key() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&["sortAz"], json!({})))
    .with_package(json!({
      "name": "a",
      "dependencies": {
          "B": "1.0.0",
          "@B": "1.0.0",
          "1B": "1.0.0",
          "A": "1.0.0",
          "@A": "1.0.0",
          "1A": "1.0.0",
      },
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: PropertyIsNotSortedAz,
      property_path: "/dependencies",
      expected: json!({
          "@A": "1.0.0",
          "@B": "1.0.0",
          "1A": "1.0.0",
          "1B": "1.0.0",
          "A": "1.0.0",
          "B": "1.0.0",
      }),
    }],
  );
}

#[tokio::test]
async fn sorts_array_members_alphabetically_by_value() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&["sortAz"], json!({})))
    .with_package(json!({
      "name": "a",
      "keywords": ["B", "@B", "1B", "A", "@A", "1A"],
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: PropertyIsNotSortedAz,
      property_path: "/keywords",
      expected: json!(["@A", "@B", "1A", "1B", "A", "B"]),
    }],
  );
}

#[tokio::test]
async fn sorts_named_root_properties_first_leaving_the_rest_alone() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(
      &[],
      json!({
          "sortFirst": ["name", "F", "E", "D"],
          "sortPackages": false,
      }),
    ))
    .with_package(json!({
        "D": "",
        "B": "",
        "name": "a",
        "F": "",
        "A": "",
        "E": "",
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: PackagePropertiesAreNotSorted,
      property_path: "/",
      expected: json!({
          "name": "a",
          "F": "",
          "E": "",
          "D": "",
          "B": "",
          "A": "",
      }),
    }],
  );
}

#[tokio::test]
async fn sorts_all_root_properties_alphabetically() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(
      &[],
      json!({
          "sortFirst": [],
          "sortPackages": true,
      }),
    ))
    .with_package(json!({
        "D": "",
        "B": "",
        "name": "a",
        "F": "",
        "A": "",
        "E": "",
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: PackagePropertiesAreNotSorted,
      property_path: "/",
      expected: json!({
          "A": "",
          "B": "",
          "D": "",
          "E": "",
          "F": "",
          "name": "a",
      }),
    }],
  );
}

#[tokio::test]
async fn sorts_named_properties_first_then_the_rest_alphabetically() {
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(
      &[],
      json!({
          "sortFirst": ["name", "F", "E", "D"],
          "sortPackages": true,
      }),
    ))
    .with_package(json!({
        "name": "a",
        "A": "",
        "F": "",
        "B": "",
        "D": "",
        "E": "",
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: PackagePropertiesAreNotSorted,
      property_path: "/",
      expected: json!({
          "name": "a",
          "F": "",
          "E": "",
          "D": "",
          "A": "",
          "B": "",
      }),
    }],
  );
}

#[tokio::test]
async fn sorts_dependencies_like_javascript_locale_compare() {
  // Sort matches npm's `localeCompare()`: scoped packages (@babel/*) come
  // before packages starting with numbers (5to6-codemod), which differs
  // from ASCII sorting.
  let ctx = TestBuilder::new()
    .with_config(rcfile_with(&["sortAz"], json!({})))
    .with_package(json!({
      "name": "a",
      "dependencies": {
        "5to6-codemod": "1.0.0",
        "@babel/register": "7.0.0",
        "zebra": "1.0.0",
        "@types/node": "18.0.0",
        "1package": "1.0.0",
        "awesome-lib": "2.0.0",
        "@scoped/package": "1.0.0"
      },
    }))
    .run()
    .await;
  expect(&ctx).to_have_format_mismatches(
    "a",
    vec![ExpectedFormatMismatch {
      variant: PropertyIsNotSortedAz,
      property_path: "/dependencies",
      expected: json!({
        "@babel/register": "7.0.0",
        "@scoped/package": "1.0.0",
        "@types/node": "18.0.0",
        "1package": "1.0.0",
        "5to6-codemod": "1.0.0",
        "awesome-lib": "2.0.0",
        "zebra": "1.0.0"
      }),
    }],
  );
}
