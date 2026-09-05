use {
  serde_json::{Value, json},
  syncpack_rules::{inspection::FormatConfig, visit_formatting::format::*},
};

/// Identity wrapper kept so the fixture call sites read naturally
/// (`pkg(json!({…}))`). The format helpers take the package.json `Value`.
fn pkg(value: Value) -> Value {
  value
}

/// Build the plain-data `FormatConfig` the format helpers take, from a mock
/// rcfile `Value`, applying the same defaults the real `Rcfile` parser uses for
/// the six formatting fields. Keeps this suite self-contained in the rules
/// crate (no dependency on config/bin).
fn format_config(config: Value) -> FormatConfig {
  let strings = |key: &str, default: &[&str]| -> Vec<String> {
    config
      .get(key)
      .and_then(|v| v.as_array())
      .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
      .unwrap_or_else(|| default.iter().map(|s| s.to_string()).collect())
  };
  let boolean = |key: &str, default: bool| config.get(key).and_then(|v| v.as_bool()).unwrap_or(default);
  FormatConfig {
    sort_packages: boolean("sortPackages", true),
    sort_first: strings("sortFirst", &["name", "description", "version", "author"]),
    format_bugs: boolean("formatBugs", false),
    format_repository: boolean("formatRepository", false),
    sort_exports: strings(
      "sortExports",
      &[
        "types",
        "node-addons",
        "node",
        "browser",
        "module",
        "import",
        "require",
        "svelte",
        "development",
        "production",
        "script",
        "default",
      ],
    ),
    sort_az: strings(
      "sortAz",
      &[
        "bin",
        "contributors",
        "dependencies",
        "devDependencies",
        "keywords",
        "peerDependencies",
        "resolutions",
        "scripts",
      ],
    ),
  }
}

#[test]
fn formats_bugs_into_github_shorthand() {
  assert_eq!(
    get_formatted_bugs(&pkg(json!({
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
    get_formatted_repository(&pkg(json!({
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
    get_formatted_repository(&pkg(json!({
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
    get_formatted_repository(&pkg(json!({
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
      &format_config(json!({})),
      &pkg(json!({
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
      &format_config(json!({})),
      &pkg(json!({
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
      &format_config(json!({})),
      &pkg(json!({
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
      &format_config(json!({})),
      &pkg(json!({
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
      &pkg(json!({
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
      &pkg(json!({
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
      &format_config(json!({
          "sortFirst": ["name", "F", "E", "D"],
          "sortPackages": false,
      })),
      &pkg(json!({
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
      &format_config(json!({
          "sortFirst": [],
          "sortPackages": true,
      })),
      &pkg(json!({
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
      &format_config(json!({
          "sortFirst": ["name", "F", "E", "D"],
          "sortPackages": true,
      })),
      &pkg(json!({
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

#[test]
fn sorts_dependencies_like_javascript_locale_compare() {
  // Sort matches npm's `localeCompare()`: scoped packages (@babel/*) come
  // before packages starting with numbers (5to6-codemod), which differs
  // from ASCII sorting.
  assert_eq!(
    get_sorted_az(
      "dependencies",
      &pkg(json!({
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
    ),
    Some(json!({
      "@babel/register": "7.0.0",
      "@scoped/package": "1.0.0",
      "@types/node": "18.0.0",
      "1package": "1.0.0",
      "5to6-codemod": "1.0.0",
      "awesome-lib": "2.0.0",
      "zebra": "1.0.0"
    }))
  );
}
