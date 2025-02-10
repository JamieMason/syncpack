use {
  super::*,
  crate::test::{self, mock},
  serde_json::json,
};

#[test]
fn formats_bugs_into_github_shorthand() {
  assert_eq!(
    get_formatted_bugs(&mock::package_json_from_value(json!({
      "name": "a",
      "bugs": {
        "url": "https://github.com/User/repo/issues"
      }
    }))),
    Some(json!("https://github.com/User/repo/issues"))
  );
}

#[test]
fn formats_repository_into_gitlab_shorthand() {
  assert_eq!(
    get_formatted_repository(&mock::package_json_from_value(json!({
      "name": "a",
      "repository": {
        "url": "git://gitlab.com/User/repo",
        "type": "git",
      },
    }))),
    Some(json!("git://gitlab.com/User/repo"))
  );
}

#[test]
fn formats_repository_into_github_shorthand() {
  assert_eq!(
    get_formatted_repository(&mock::package_json_from_value(json!({
      "name": "a",
      "repository": {
        "url": "git://github.com/User/repo",
        "type": "git",
      },
    }))),
    Some(json!("User/repo"))
  );
}

#[test]
fn retains_long_format_when_directory_property_used() {
  assert_eq!(
    get_formatted_repository(&mock::package_json_from_value(json!({
      "name": "a",
      "repository": {
        "url": "git://gitlab.com/User/repo",
        "type": "git",
        "directory": "packages/foo",
      },
    }))),
    None
  );
}

#[test]
fn sorts_conditional_exports() {
  assert_eq!(
    get_sorted_exports(
      &mock::rcfile(),
      &mock::package_json_from_value(json!({
        "name": "a",
        "exports": {
            "default": "./default.cjs",
            "svelte": "./index.svelte",
            "require": "./index-require.cjs",
            "import": "./index-module.js",
        },
      }))
    ),
    Some(json!({
      "import": "./index-module.js",
      "require": "./index-require.cjs",
      "svelte": "./index.svelte",
      "default": "./default.cjs",
    })),
  )
}

#[test]
fn returns_none_when_conditional_exports_already_sorted() {
  assert_eq!(
    get_sorted_exports(
      &mock::rcfile(),
      &mock::package_json_from_value(json!({
        "name": "a",
        "exports": {
            "import": "./index-module.js",
            "require": "./index-require.cjs",
        },
      }))
    ),
    None
  )
}

#[test]
fn sorts_conditional_exports_sub_paths() {
  assert_eq!(
    get_sorted_exports(
      &mock::rcfile(),
      &mock::package_json_from_value(json!({
        "name": "a",
        "exports": {
          ".": "./index.js",
          "./feature.js": {
            "default": "./feature.js",
            "node": "./feature-node.js",
          },
        },
      }))
    ),
    Some(json!({
      ".": "./index.js",
      "./feature.js": {
        "node": "./feature-node.js",
        "default": "./feature.js",
      },
    })),
  )
}

#[test]
fn returns_none_when_conditional_exports_sub_paths_already_sorted() {
  assert_eq!(
    get_sorted_exports(
      &mock::rcfile(),
      &mock::package_json_from_value(json!({
        "name": "a",
        "exports": {
            ".": "./index.js",
            "./feature.js": {
              "node": "./feature-node.js",
              "default": "./feature.js",
            },
        },
      }))
    ),
    None
  )
}

#[test]
fn sorts_object_properties_alphabetically_by_key() {
  assert_eq!(
    get_sorted_az(
      "dependencies",
      &mock::package_json_from_value(json!({
          "dependencies": {
              "B": "",
              "@B": "",
              "1B": "",
              "A": "",
              "@A": "",
              "1A": "",
          },
      }))
    ),
    Some(json!({
        "@A": "",
        "@B": "",
        "1A": "",
        "1B": "",
        "A": "",
        "B": "",
    }))
  );
}
#[test]
fn sorts_array_members_alphabetically_by_value() {
  assert_eq!(
    get_sorted_az(
      "keywords",
      &mock::package_json_from_value(json!({
          "keywords": ["B", "@B", "1B", "A", "@A", "1A"],
      }))
    ),
    Some(json!(["@A", "@B", "1A", "1B", "A", "B"]))
  );
}

#[test]
fn sorts_named_root_properties_first_leaving_the_rest_alone() {
  assert_eq!(
    get_sorted_first(
      &test::mock::rcfile_from_mock(json!({
          "sortFirst": ["name", "F", "E", "D"],
          "sortPackages": false,
      })),
      &mock::package_json_from_value(json!({
          "D": "",
          "B": "",
          "name": "a",
          "F": "",
          "A": "",
          "E": "",
      }))
    ),
    Some(json!({
        "name": "a",
        "F": "",
        "E": "",
        "D": "",
        "B": "",
        "A": "",
    }))
  );
}

#[test]
fn sorts_all_root_properties_alphabetically() {
  assert_eq!(
    get_sorted_first(
      &test::mock::rcfile_from_mock(json!({
          "sortFirst": [],
          "sortPackages": true,
      })),
      &mock::package_json_from_value(json!({
          "D": "",
          "B": "",
          "name": "a",
          "F": "",
          "A": "",
          "E": "",
      }))
    ),
    Some(json!({
        "A": "",
        "B": "",
        "D": "",
        "E": "",
        "F": "",
        "name": "a",
    }))
  );
}

#[test]
fn sorts_named_properties_first_then_the_rest_alphabetically() {
  assert_eq!(
    get_sorted_first(
      &test::mock::rcfile_from_mock(json!({
          "sortFirst": ["name", "F", "E", "D"],
          "sortPackages": true,
      })),
      &mock::package_json_from_value(json!({
          "name": "a",
          "A": "",
          "F": "",
          "B": "",
          "D": "",
          "E": "",
      }))
    ),
    Some(json!({
        "name": "a",
        "F": "",
        "E": "",
        "D": "",
        "A": "",
        "B": "",
    }))
  );
}
