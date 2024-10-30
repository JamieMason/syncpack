use {super::*, std::cmp::Ordering};

#[test]
fn parses_node_specifier_strings() {
  let cases: Vec<(&str, &str, bool)> = vec![
    ("*", "latest", true),
    ("1", "major", true),
    ("1.2", "minor", true),
    // exact semver versions
    ("0.0.0", "exact", true),
    ("1.2.3-alpha", "exact", true),
    ("1.2.3-rc.1", "exact", true),
    ("1.2.3-alpha", "exact", true),
    ("1.2.3-rc.0", "exact", true),
    // complex semver queries
    ("1.3.0 || <1.0.0 >2.0.0", "range-complex", false),
    ("<1.0.0 >2.0.0", "range-complex", false),
    ("<1.0.0 >=2.0.0", "range-complex", false),
    ("<1.5.0 || >=1.6.0", "range-complex", false),
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", "range-complex", false),
    ("<=1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", "range-complex", false),
    (">1.0.0 <1.0.0", "range-complex", false),
    (">1.0.0 <=2.0.0", "range-complex", false),
    (">=2.3.4 || <=1.2.3", "range-complex", false),
    // workspace protocol
    ("workspace:*", "workspace-protocol", false),
    ("workspace:^", "workspace-protocol", false),
    ("workspace:~", "workspace-protocol", false),
    // simple semver with a semver range
    ("<1.2.3-alpha", "range", true),
    ("<1.2.3-rc.0", "range", true),
    ("<=1.2.3-alpha", "range", true),
    ("<=1.2.3-rc.0", "range", true),
    (">1.2.3-alpha", "range", true),
    (">1.2.3-rc.0", "range", true),
    (">=1.2.3-alpha", "range", true),
    (">=1.2.3-rc.0", "range", true),
    ("^1.2.3", "range", true),
    ("^1.2.3-alpha", "range", true),
    ("^1.2.3-rc.0", "range", true),
    ("~1.2.3-alpha", "range", true),
    ("~1.2.3-rc.0", "range", true),
    // unsupported
    ("$typescript", "unsupported", false),
    ("/path/to/foo", "unsupported", false),
    ("/path/to/foo.tar", "unsupported", false),
    ("/path/to/foo.tgz", "unsupported", false),
    ("1.typo.wat", "unsupported", false),
    ("=v1.2.3", "unsupported", false),
    ("@f fo o al/ a d s ;f", "unsupported", false),
    ("@foo/bar", "unsupported", false),
    ("@foo/bar@", "unsupported", false),
    ("git+file://path/to/repo#1.2.3", "unsupported", false),
    ("not-git@hostname.com:some/repo", "unsupported", false),
    ("user/foo#1234::path:dist", "unsupported", false),
    ("user/foo#notimplemented:value", "unsupported", false),
    ("user/foo#path:dist", "unsupported", false),
    ("user/foo#semver:^1.2.3", "unsupported", false),
    // tags
    ("alpha", "tag", false),
    ("beta", "tag", false),
    ("canary", "tag", false),
    // range major
    ("~1", "range-major", true),
    // range minor
    ("<5.0", "range-minor", true),
    ("<=5.0", "range-minor", true),
    (">5.0", "range-minor", true),
    (">=5.0", "range-minor", true),
    ("^4.1", "range-minor", true),
    ("~1.2", "range-minor", true),
    ("~1.2", "range-minor", true),
    // npm aliases
    ("npm:@minh.nguyen/plugin-transform-destructuring@^7.5.2", "alias", false),
    ("npm:@types/selenium-webdriver@4.1.18", "alias", false),
    ("npm:foo@1.2.3", "alias", false),
    // file paths
    ("file:../path/to/foo", "file", false),
    ("file:./path/to/foo", "file", false),
    ("file:/../path/to/foo", "file", false),
    ("file:/./path/to/foo", "file", false),
    ("file:/.path/to/foo", "file", false),
    ("file://.", "file", false),
    ("file://../path/to/foo", "file", false),
    ("file://./path/to/foo", "file", false),
    ("file:////path/to/foo", "file", false),
    ("file:///path/to/foo", "file", false),
    ("file://path/to/foo", "file", false),
    ("file:/path/to/foo", "file", false),
    ("file:/~path/to/foo", "file", false),
    ("file:path/to/directory", "file", false),
    ("file:path/to/foo", "file", false),
    ("file:path/to/foo.tar.gz", "file", false),
    ("file:path/to/foo.tgz", "file", false),
    // git urls
    ("git+https://github.com/user/foo", "git", false),
    ("git+ssh://git@github.com/user/foo#1.2.3", "git", false),
    ("git+ssh://git@github.com/user/foo#semver:^1.2.3", "git", false),
    ("git+ssh://git@github.com:user/foo#semver:^1.2.3", "git", false),
    ("git+ssh://git@notgithub.com/user/foo", "git", false),
    ("git+ssh://git@notgithub.com/user/foo#1.2.3", "git", false),
    ("git+ssh://git@notgithub.com/user/foo#semver:^1.2.3", "git", false),
    ("git+ssh://git@notgithub.com:user/foo", "git", false),
    ("git+ssh://git@notgithub.com:user/foo#1.2.3", "git", false),
    ("git+ssh://git@notgithub.com:user/foo#semver:^1.2.3", "git", false),
    ("git+ssh://github.com/user/foo", "git", false),
    ("git+ssh://github.com/user/foo#1.2.3", "git", false),
    ("git+ssh://github.com/user/foo#semver:^1.2.3", "git", false),
    ("git+ssh://mydomain.com:1234#1.2.3", "git", false),
    ("git+ssh://mydomain.com:1234/hey", "git", false),
    ("git+ssh://mydomain.com:1234/hey#1.2.3", "git", false),
    ("git+ssh://mydomain.com:foo", "git", false),
    ("git+ssh://mydomain.com:foo#1.2.3", "git", false),
    ("git+ssh://mydomain.com:foo/bar#1.2.3", "git", false),
    ("git+ssh://notgithub.com/user/foo", "git", false),
    ("git+ssh://notgithub.com/user/foo#1.2.3", "git", false),
    ("git+ssh://notgithub.com/user/foo#semver:^1.2.3", "git", false),
    ("git+ssh://username:password@mydomain.com:1234/hey#1.2.3", "git", false),
    ("git://github.com/user/foo", "git", false),
    ("git://github.com/user/foo#1.2.3", "git", false),
    ("git://github.com/user/foo#semver:^1.2.3", "git", false),
    ("git://notgithub.com/user/foo", "git", false),
    ("git://notgithub.com/user/foo#1.2.3", "git", false),
    ("git://notgithub.com/user/foo#semver:^1.2.3", "git", false),
    // urls
    ("http://insecure.com/foo.tgz", "url", false),
    ("https://server.com/foo.tgz", "url", false),
    ("https://server.com/foo.tgz", "url", false),
  ];
  for (value, expected_id, expected_is_simple_semver) in cases {
    let spec = Specifier::new(value);
    assert_eq!(spec.get_config_identifier(), expected_id, "{value} should have ID of {expected_id}");
    assert_eq!(spec.unwrap(), value, "{value} should unwrap to {value}");
    assert_eq!(
      spec.is_simple_semver(),
      expected_is_simple_semver,
      "{value} is_simple_semver should be {expected_is_simple_semver}"
    );
    assert_eq!(spec.get_simple_semver().is_some(), expected_is_simple_semver);
  }
}

#[test]
fn normalises_some_node_specifier_strings() {
  let cases: Vec<(&str, &str, bool, &str)> = vec![
    ("latest", "latest", true, "*"),
    ("x", "latest", true, "*"),
    ("", "missing", false, "VERSION_IS_MISSING"),
  ];
  for (value, expected_id, expected_is_simple_semver, expected_normalisation) in cases {
    let spec = Specifier::new(value);
    assert_eq!(spec.get_config_identifier(), expected_id);
    assert_eq!(spec.unwrap(), expected_normalisation);
    assert_eq!(spec.is_simple_semver(), expected_is_simple_semver);
    assert_eq!(spec.get_simple_semver().is_some(), expected_is_simple_semver);
  }
}

#[test]
fn compares_simple_semver_specifiers_according_to_highest_version_and_greediest_range() {
  let cases: Vec<(&str, &str, Ordering)> = vec![
    /* normal versions */
    ("0.0.0", "0.0.1", Ordering::Less),
    ("0.0.0", "0.1.0", Ordering::Less),
    ("0.0.0", "1.0.0", Ordering::Less),
    ("0.0.0", "0.0.0", Ordering::Equal),
    ("0.0.1", "0.0.0", Ordering::Greater),
    ("0.1.0", "0.0.0", Ordering::Greater),
    ("1.0.0", "0.0.0", Ordering::Greater),
    /* range versions where versions differ */
    ("0.0.0", "~0.0.1", Ordering::Less),
    ("0.0.0", "~0.1.0", Ordering::Less),
    ("0.0.0", "~1.0.0", Ordering::Less),
    ("0.0.1", "~0.0.0", Ordering::Greater),
    ("0.1.0", "~0.0.0", Ordering::Greater),
    ("1.0.0", "~0.0.0", Ordering::Greater),
    ("0.0.0", "^0.0", Ordering::Less),
    ("0", "~0.0", Ordering::Greater),
    ("0", "^0.0", Ordering::Greater),
    /* range greediness applies only when versions are equal */
    ("0.0.0", "~0.0.0", Ordering::Less),
    ("0.0.0", "~0.0", Ordering::Less),
    ("0.0.0", "^0.0.0", Ordering::Less),
    ("0.0", "~0.0", Ordering::Less),
    ("0.0", "^0.0", Ordering::Less),
    ("~0", "^0", Ordering::Less),
    ("0", "~0", Ordering::Less),
    ("0", "^0", Ordering::Less),
    ("0.0.0", ">0.0.0", Ordering::Less),
    ("0.0.0", ">=0.0.0", Ordering::Less),
    ("0.0.0", "<=0.0.0", Ordering::Greater),
    ("0.0.0", "<0.0.0", Ordering::Greater),
    ("0.0.0", "*", Ordering::Less),
    ("^0.0.0", "*", Ordering::Less),
    ("~0.0.0", "*", Ordering::Less),
    (">0.0.0", "*", Ordering::Less),
    (">=0.0.0", "*", Ordering::Less),
    ("<=0.0.0", "*", Ordering::Less),
    ("<0.0.0", "*", Ordering::Less),
    /* an empty or missing specifier is always bottom rank */
    ("", "0.0.0", Ordering::Less),
    ("", "<0.0.0", Ordering::Equal),
    /* stable should be older than tagged */
    ("0.0.0", "0.0.0-alpha", Ordering::Less),
    /* equal tags should not affect comparison */
    ("0.0.0-alpha", "0.0.0-alpha", Ordering::Equal),
    ("0.0.0-alpha", "0.1.0-alpha", Ordering::Less),
    ("0.0.0-alpha", "1.0.0-alpha", Ordering::Less),
    ("0.0.0-alpha", "0.0.0-alpha", Ordering::Equal),
    ("0.0.1-alpha", "0.0.0-alpha", Ordering::Greater),
    ("0.1.0-alpha", "0.0.0-alpha", Ordering::Greater),
    ("1.0.0-alpha", "0.0.0-alpha", Ordering::Greater),
    /* preleases should matter when version is equal */
    ("0.0.0-rc.0.0.0", "0.0.0-rc.0.0.0", Ordering::Equal),
    ("0.0.0-rc.0.0.0", "0.0.0-rc.0.1.0", Ordering::Less),
    ("0.0.0-rc.0.0.0", "0.0.0-rc.1.0.0", Ordering::Less),
    ("0.0.0-rc.0.0.0", "0.0.0-rc.0.0.0", Ordering::Equal),
    ("0.0.0-rc.0.0.1", "0.0.0-rc.0.0.0", Ordering::Greater),
    ("0.0.0-rc.0.1.0", "0.0.0-rc.0.0.0", Ordering::Greater),
    ("0.0.0-rc.1.0.0", "0.0.0-rc.0.0.0", Ordering::Greater),
    /* preleases should not matter when version is greater */
    ("0.1.0-rc.0.0.0", "0.0.0-rc.0.1.0", Ordering::Greater),
    /* compares tags a-z */
    ("0.0.0-alpha", "0.0.0-alpha", Ordering::Equal),
    ("0.0.0-alpha", "0.0.0-beta", Ordering::Less),
    ("0.0.0-beta", "0.0.0-alpha", Ordering::Greater),
    /* range greediness is the same on prereleases */
    ("0.0.0-rc.0", "~0.0.1-rc.0", Ordering::Less),
    ("0.0.0-rc.0", "~0.1.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", "~1.0.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", "~0.0.0-rc.0", Ordering::Less),
    ("0.0.1-rc.0", "~0.0.0-rc.0", Ordering::Greater),
    ("0.1.0-rc.0", "~0.0.0-rc.0", Ordering::Greater),
    ("1.0.0-rc.0", "~0.0.0-rc.0", Ordering::Greater),
    ("0.0.0-rc.0", "~0.0.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", "^0.0.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", ">0.0.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", ">=0.0.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", "<=0.0.0-rc.0", Ordering::Greater),
    ("0.0.0-rc.0", "<0.0.0-rc.0", Ordering::Greater),
    /* workspace: protocol is treated as <0.0.0 for now */
    ("workspace:*", "<0.0.0", Ordering::Equal),
    ("workspace:0.0.0", "<0.0.0", Ordering::Equal),
    ("workspace:>0.0.0", "<0.0.0", Ordering::Equal),
    ("workspace:<=0.0.0", "<0.0.0", Ordering::Equal),
    ("workspace:<=0.0.0", "<0.0.0", Ordering::Equal),
    ("workspace:~", "<0.0.0", Ordering::Equal),
  ];
  for (str_a, str_b, expected) in cases {
    let a = Specifier::new(str_a);
    let b = Specifier::new(str_b);
    let orderable_a = a.get_orderable();
    let orderable_b = b.get_orderable();
    let ordering = orderable_a.cmp(&orderable_b);
    assert_eq!(
      ordering, expected,
      "{a:?} should be {expected:?} {b:?} ({orderable_a:#?} {orderable_b:#?})"
    );
  }
}

#[test]
fn sorts_simple_semver_specifiers_according_to_highest_version_and_greediest_range() {
  fn to_specifiers(specifiers: Vec<&str>) -> Vec<Specifier> {
    specifiers.iter().map(|r| Specifier::new(r)).collect()
  }
  let mut specifiers = to_specifiers(vec!["0.0.0", "<0.0.0", "*", ">0.0.0", ">=0.0.0", "<=0.0.0", "^0.0.0", "~0.0.0"]);
  let expected = to_specifiers(vec!["<0.0.0", "<=0.0.0", "0.0.0", "~0.0.0", "^0.0.0", ">=0.0.0", ">0.0.0", "*"]);

  specifiers.sort_by_key(|s| s.get_orderable());
  assert_eq!(specifiers, expected, "{specifiers:?}, {expected:?}");
}

#[test]
fn states_whether_specifier_satisfies_other_specifiers() {
  let cases: Vec<(&str, Vec<&str>, bool)> = vec![
    ("*", vec!["1.4.2"], true),
    ("^1.4.2", vec!["1.4.2"], true),
    ("1.4.2", vec!["1.4.2"], true),
    (">1.4.2", vec!["1.4.2"], false),
    (">=1.4.2", vec!["1.4.2"], true),
    ("<1.4.2", vec!["1.4.2"], false),
    ("<=1.4.2", vec!["1.4.2"], true),
    ("~1.4.2", vec!["1.4.2"], true),
    ("^1.0.0", vec!["1.4.2"], true),
    ("~1.0.0", vec!["1.4.2"], false),
    ("", vec!["1.4.2"], false),
    ("~1.4.2 || ^1.4.2", vec!["1.4.2"], true),
    ("~1.0.0 || ^1.0.0", vec!["1.4.2"], true),
  ];
  for (value, others, expected) in cases {
    let spec = Specifier::new(value);
    let other_specs: Vec<Specifier> = others.iter().map(|r| Specifier::new(r)).collect();
    let refs_to_other_specs: Vec<&Specifier> = other_specs.iter().collect();
    assert_eq!(
      spec.satisfies_all(refs_to_other_specs),
      expected,
      "'{value}'.satisfies_all({others:?}) should be {expected}"
    );
  }
}
