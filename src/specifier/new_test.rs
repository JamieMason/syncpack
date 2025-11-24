use crate::{
  specifier::{workspace_specifier::WorkspaceSpecifier, Specifier},
  test::faker,
};

#[test]
fn valid_latest() {
  for (value, expected) in faker::get_latest() {
    assert_eq!(*Specifier::new(value), Specifier::Latest(expected));
  }
}

#[test]
fn valid_tag() {
  for (value, expected) in faker::get_tag() {
    assert_eq!(*Specifier::new(value), Specifier::Tag(expected));
  }
}

#[test]
fn valid_catalog() {
  for (value, expected) in faker::get_catalog() {
    assert_eq!(*Specifier::new(value), Specifier::Catalog(expected));
  }
}

#[test]
fn valid_major() {
  for (value, expected) in faker::get_major() {
    assert_eq!(*Specifier::new(value), Specifier::Major(expected));
  }
}

#[test]
fn valid_minor() {
  for (value, expected) in faker::get_minor() {
    assert_eq!(*Specifier::new(value), Specifier::Minor(expected));
  }
}

#[test]
fn valid_exact() {
  for (value, expected) in faker::get_exact() {
    assert_eq!(*Specifier::new(value), Specifier::Exact(expected));
  }
}

#[test]
fn valid_complex_semver() {
  for (value, expected) in faker::get_complex_semver() {
    assert_eq!(*Specifier::new(value), Specifier::ComplexSemver(expected));
  }
}

#[test]
fn valid_workspace_protocol() {
  for (value, expected) in faker::get_workspace_protocol() {
    assert_eq!(*Specifier::new(value), Specifier::WorkspaceProtocol(expected));
  }
}

#[test]
fn valid_range() {
  for (value, expected) in faker::get_range() {
    assert_eq!(*Specifier::new(value), Specifier::Range(expected));
  }
}

#[test]
fn valid_unsupported() {
  for value in faker::get_unsupported() {
    assert_eq!(*Specifier::new(value), Specifier::Unsupported(value.to_string()));
  }
}

#[test]
fn valid_range_major() {
  for (value, expected) in faker::get_range_major() {
    assert_eq!(*Specifier::new(value), Specifier::RangeMajor(expected));
  }
}

#[test]
fn valid_range_minor() {
  for (value, expected) in faker::get_range_minor() {
    assert_eq!(*Specifier::new(value), Specifier::RangeMinor(expected));
  }
}

#[test]
fn valid_alias() {
  for (value, expected) in faker::get_alias() {
    assert_eq!(*Specifier::new(value), Specifier::Alias(expected));
  }
}

#[test]
fn valid_file() {
  for (value, expected) in faker::get_file() {
    assert_eq!(*Specifier::new(value), Specifier::File(expected));
  }
}

#[test]
fn valid_link() {
  for (value, expected) in faker::get_link() {
    assert_eq!(*Specifier::new(value), Specifier::Link(expected));
  }
}

#[test]
fn valid_git() {
  for (value, expected) in faker::get_git() {
    assert_eq!(*Specifier::new(value), Specifier::Git(expected));
  }
}

#[test]
fn valid_url() {
  for (value, expected) in faker::get_url() {
    assert_eq!(*Specifier::new(value), Specifier::Url(expected));
  }
}

#[test]
fn various_basic_semver_formats() {
  for prerelease in faker::prereleases() {
    for (range_str, range_variant) in faker::ranges() {
      // Test Git URLs with semver tags
      for git_url in faker::git_urls() {
        let semver_number = format!("1.2.3{prerelease}");
        let with_range = format!("{range_str}{semver_number}");
        let value = format!("{git_url}#{with_range}");
        let specifier = Specifier::new(&value);

        match &*specifier {
          Specifier::Git(actual) => {
            assert_eq!(actual.raw, value);
            assert_eq!(actual.origin, git_url);

            assert_eq!(actual.semver_number, Some(semver_number.clone()));
            assert_eq!(specifier.get_semver_number(), Some(semver_number.as_str()));

            // Validate semver_range matches expected variant
            assert_eq!(actual.semver_range, Some(range_variant.clone()));

            // Validate node_version is populated when semver_number exists
            let node_version = actual.node_version.as_ref().unwrap();
            assert_eq!(node_version.major, 1);
            assert_eq!(node_version.minor, 2);
            assert_eq!(node_version.patch, 3);
            assert_eq!(node_version.pre_release.is_empty(), prerelease.is_empty());

            // Validate node_range matches the full range string
            let parsed_range = node_semver::Range::parse(&with_range).unwrap();
            assert_eq!(**actual.node_range.as_ref().unwrap(), parsed_range);
          }
          _ => panic!("Expected Git for value: {value}"),
        };
      }

      // Test npm aliases with semver
      for npm_name in faker::npm_names() {
        let semver_number = format!("1.2.3{prerelease}");
        let with_range = format!("{range_str}{semver_number}");
        let value = format!("npm:{npm_name}@{with_range}");
        let specifier = Specifier::new(&value);

        match &*specifier {
          Specifier::Alias(actual) => {
            assert_eq!(actual.raw, value);
            assert_eq!(actual.name, npm_name);
            assert_eq!(specifier.get_semver_number(), Some(semver_number.as_str()));
            assert_eq!(specifier.get_semver_range(), Some(range_variant.clone()));

            // Validate node_version is populated when semver_number exists
            let node_version = specifier.get_node_version().unwrap();
            assert_eq!(node_version.major, 1);
            assert_eq!(node_version.minor, 2);
            assert_eq!(node_version.patch, 3);
            assert_eq!(node_version.pre_release.is_empty(), prerelease.is_empty());

            // Validate node_range matches the full range string
            let parsed_range = node_semver::Range::parse(&with_range).unwrap();
            assert_eq!(*specifier.get_node_range().unwrap(), parsed_range);
          }
          _ => panic!("Expected Alias for value: {value}"),
        };
      }

      // Test basic semver and workspace protocol
      for protocol in faker::protocols() {
        let semver_number = format!("1.2.3{prerelease}");
        let with_range = format!("{range_str}{semver_number}");
        let value = format!("{protocol}{with_range}");
        let specifier = Specifier::new(&value);

        match &*specifier {
          Specifier::Exact(actual) => {
            // Only happens when range_str is "" (Exact)
            assert_eq!(actual.raw, value);
            assert_eq!(specifier.get_semver_number(), Some(semver_number.as_str()));
            assert_eq!(actual.node_version.major, 1);
            assert_eq!(actual.node_version.minor, 2);
            assert_eq!(actual.node_version.patch, 3);
            assert_eq!(actual.node_version.pre_release.is_empty(), prerelease.is_empty());
          }
          Specifier::Range(actual) => {
            // Happens when range_str has a range character and no protocol
            assert_eq!(actual.raw, value);
            assert_eq!(actual.semver_range, range_variant);
            assert_eq!(actual.semver_number, semver_number);
            assert_eq!(specifier.get_semver_number(), Some(semver_number.as_str()));
            assert_eq!(actual.node_version.major, 1);
            assert_eq!(actual.node_version.minor, 2);
            assert_eq!(actual.node_version.patch, 3);
            assert_eq!(actual.node_version.pre_release.is_empty(), prerelease.is_empty());
          }
          Specifier::WorkspaceProtocol(actual) => {
            // Happens when protocol is "workspace:"
            assert_eq!(actual.raw, value);
            assert_eq!(actual.version_str, format!("{}{}", range_variant.unwrap(), semver_number));
            // Check that inner_specifier is resolved
            assert!(matches!(actual.inner_specifier, WorkspaceSpecifier::Resolved(_)));
            assert_eq!(specifier.get_semver_number(), Some(semver_number.as_str()));
          }
          other => panic!("Expected Exact, Range, or WorkspaceProtocol for value: {value}, got {other:?}"),
        };
      }
    }
  }
}
