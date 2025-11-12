use {
  crate::{
    semver_range::SemverRange,
    specifier::{
      alias::Alias, catalog::Catalog, complex_semver::ComplexSemver, exact::Exact, file::File, git::Git, latest::Latest, major::Major,
      minor::Minor, range::Range, range_major::RangeMajor, range_minor::RangeMinor, tag::Tag, url::Url,
      workspace_protocol::WorkspaceProtocol, workspace_specifier::WorkspaceSpecifier, Specifier,
    },
  },
  std::rc::Rc,
};

pub fn get_latest() -> Vec<(&'static str, Latest)> {
  let huge = crate::specifier::HUGE.to_string();
  let huge_version = format!("{huge}.{huge}.{huge}");
  let node_version = crate::specifier::Specifier::new_node_version(&huge_version).unwrap();

  vec![
    (
      "latest",
      Latest {
        raw: "latest".to_string(),
        node_version: node_version.clone(),
        semver_range: SemverRange::Any,
      },
    ),
    (
      "*",
      Latest {
        raw: "*".to_string(),
        node_version: node_version.clone(),
        semver_range: SemverRange::Any,
      },
    ),
    (
      "x",
      Latest {
        raw: "x".to_string(),
        node_version,
        semver_range: SemverRange::Any,
      },
    ),
  ]
}

pub fn get_tag() -> Vec<(&'static str, Tag)> {
  vec![
    ("alpha", Tag { raw: "alpha".to_string() }),
    ("beta", Tag { raw: "beta".to_string() }),
  ]
}

pub fn get_catalog() -> Vec<(&'static str, Catalog)> {
  vec![
    (
      "catalog:",
      Catalog {
        raw: "catalog:".to_string(),
        name: None,
      },
    ),
    (
      "catalog:react18",
      Catalog {
        raw: "catalog:react18".to_string(),
        name: Some("react18".to_string()),
      },
    ),
    (
      "catalog:testing",
      Catalog {
        raw: "catalog:testing".to_string(),
        name: Some("testing".to_string()),
      },
    ),
  ]
}

pub fn get_major() -> Vec<(&'static str, Major)> {
  vec![(
    "1",
    Major {
      raw: "1".to_string(),
      node_version: Rc::new(node_semver::Version::parse("1.999999.999999").unwrap()),
    },
  )]
}

pub fn get_minor() -> Vec<(&'static str, Minor)> {
  vec![(
    "1.2",
    Minor {
      raw: "1.2".to_string(),
      node_version: Rc::new(node_semver::Version::parse("1.2.999999").unwrap()),
    },
  )]
}

pub fn get_exact() -> Vec<(&'static str, Exact)> {
  vec![
    (
      "1.2.3",
      Exact {
        raw: "1.2.3".to_string(),
        node_version: Rc::new(node_semver::Version::parse("1.2.3").unwrap()),
        node_range: Rc::new(node_semver::Range::parse("1.2.3").unwrap()),
      },
    ),
    (
      "1.2.3-alpha",
      Exact {
        raw: "1.2.3-alpha".to_string(),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-alpha").unwrap()),
        node_range: Rc::new(node_semver::Range::parse("1.2.3-alpha").unwrap()),
      },
    ),
    (
      "1.2.3-rc.1",
      Exact {
        raw: "1.2.3-rc.1".to_string(),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-rc.1").unwrap()),
        node_range: Rc::new(node_semver::Range::parse("1.2.3-rc.1").unwrap()),
      },
    ),
    (
      "1.2.3-rc.0",
      Exact {
        raw: "1.2.3-rc.0".to_string(),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-rc.0").unwrap()),
        node_range: Rc::new(node_semver::Range::parse("1.2.3-rc.0").unwrap()),
      },
    ),
  ]
}

pub fn get_complex_semver() -> Vec<(&'static str, ComplexSemver)> {
  vec![
    (
      "1.3.0 || <1.0.0 >2.0.0",
      ComplexSemver {
        raw: "1.3.0 || <1.0.0 >2.0.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse("1.3.0 || <1.0.0 >2.0.0").unwrap()),
      },
    ),
    (
      "<1.0.0 >2.0.0",
      ComplexSemver {
        raw: "<1.0.0 >2.0.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<1.0.0 >2.0.0").unwrap()),
      },
    ),
    (
      "<1.0.0 >=2.0.0",
      ComplexSemver {
        raw: "<1.0.0 >=2.0.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<1.0.0 >=2.0.0").unwrap()),
      },
    ),
    (
      "<1.5.0 || >=1.6.0",
      ComplexSemver {
        raw: "<1.5.0 || >=1.6.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<1.5.0 || >=1.6.0").unwrap()),
      },
    ),
    (
      "<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2",
      ComplexSemver {
        raw: "<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2").unwrap()),
      },
    ),
    (
      "<=1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2",
      ComplexSemver {
        raw: "<=1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<=1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2").unwrap()),
      },
    ),
    (
      ">1.0.0 <1.0.0",
      ComplexSemver {
        raw: ">1.0.0 <1.0.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse(">1.0.0 <1.0.0").unwrap()),
      },
    ),
    (
      ">1.0.0 <=2.0.0",
      ComplexSemver {
        raw: ">1.0.0 <=2.0.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse(">1.0.0 <=2.0.0").unwrap()),
      },
    ),
    (
      ">=2.3.4 || <=1.2.3",
      ComplexSemver {
        raw: ">=2.3.4 || <=1.2.3".to_string(),
        node_range: Rc::new(node_semver::Range::parse(">=2.3.4 || <=1.2.3").unwrap()),
      },
    ),
  ]
}

pub fn get_workspace_protocol() -> Vec<(&'static str, WorkspaceProtocol)> {
  vec![
    (
      "workspace:*",
      WorkspaceProtocol {
        raw: "workspace:*".to_string(),
        version_str: "*".to_string(),
        inner_specifier: WorkspaceSpecifier::RangeOnly(SemverRange::Any),
      },
    ),
    (
      "workspace:^",
      WorkspaceProtocol {
        raw: "workspace:^".to_string(),
        version_str: "^".to_string(),
        inner_specifier: WorkspaceSpecifier::RangeOnly(SemverRange::Minor),
      },
    ),
    (
      "workspace:~",
      WorkspaceProtocol {
        raw: "workspace:~".to_string(),
        version_str: "~".to_string(),
        inner_specifier: WorkspaceSpecifier::RangeOnly(SemverRange::Patch),
      },
    ),
  ]
}

pub fn get_range() -> Vec<(&'static str, Range)> {
  vec![
    (
      "<1.2.3-alpha",
      Range {
        raw: "<1.2.3-alpha".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<1.2.3-alpha").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-alpha").unwrap()),
        semver_range: SemverRange::Lt,
        semver_number: "1.2.3-alpha".to_string(),
      },
    ),
    (
      "<1.2.3-rc.0",
      Range {
        raw: "<1.2.3-rc.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<1.2.3-rc.0").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-rc.0").unwrap()),
        semver_range: SemverRange::Lt,
        semver_number: "1.2.3-rc.0".to_string(),
      },
    ),
    (
      "<=1.2.3-alpha",
      Range {
        raw: "<=1.2.3-alpha".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<=1.2.3-alpha").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-alpha").unwrap()),
        semver_range: SemverRange::Lte,
        semver_number: "1.2.3-alpha".to_string(),
      },
    ),
    (
      "<=1.2.3-rc.0",
      Range {
        raw: "<=1.2.3-rc.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<=1.2.3-rc.0").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-rc.0").unwrap()),
        semver_range: SemverRange::Lte,
        semver_number: "1.2.3-rc.0".to_string(),
      },
    ),
    (
      ">1.2.3-alpha",
      Range {
        raw: ">1.2.3-alpha".to_string(),
        node_range: Rc::new(node_semver::Range::parse(">1.2.3-alpha").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-alpha").unwrap()),
        semver_range: SemverRange::Gt,
        semver_number: "1.2.3-alpha".to_string(),
      },
    ),
    (
      ">1.2.3-rc.0",
      Range {
        raw: ">1.2.3-rc.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse(">1.2.3-rc.0").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-rc.0").unwrap()),
        semver_range: SemverRange::Gt,
        semver_number: "1.2.3-rc.0".to_string(),
      },
    ),
    (
      ">=1.2.3-alpha",
      Range {
        raw: ">=1.2.3-alpha".to_string(),
        node_range: Rc::new(node_semver::Range::parse(">=1.2.3-alpha").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-alpha").unwrap()),
        semver_range: SemverRange::Gte,
        semver_number: "1.2.3-alpha".to_string(),
      },
    ),
    (
      ">=1.2.3-rc.0",
      Range {
        raw: ">=1.2.3-rc.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse(">=1.2.3-rc.0").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-rc.0").unwrap()),
        semver_range: SemverRange::Gte,
        semver_number: "1.2.3-rc.0".to_string(),
      },
    ),
    (
      "^1.2.3",
      Range {
        raw: "^1.2.3".to_string(),
        node_range: Rc::new(node_semver::Range::parse("^1.2.3").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3").unwrap()),
        semver_range: SemverRange::Minor,
        semver_number: "1.2.3".to_string(),
      },
    ),
    (
      "^1.2.3-alpha",
      Range {
        raw: "^1.2.3-alpha".to_string(),
        node_range: Rc::new(node_semver::Range::parse("^1.2.3-alpha").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-alpha").unwrap()),
        semver_range: SemverRange::Minor,
        semver_number: "1.2.3-alpha".to_string(),
      },
    ),
    (
      "^1.2.3-rc.0",
      Range {
        raw: "^1.2.3-rc.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse("^1.2.3-rc.0").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-rc.0").unwrap()),
        semver_range: SemverRange::Minor,
        semver_number: "1.2.3-rc.0".to_string(),
      },
    ),
    (
      "~1.2.3-alpha",
      Range {
        raw: "~1.2.3-alpha".to_string(),
        node_range: Rc::new(node_semver::Range::parse("~1.2.3-alpha").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-alpha").unwrap()),
        semver_range: SemverRange::Patch,
        semver_number: "1.2.3-alpha".to_string(),
      },
    ),
    (
      "~1.2.3-rc.0",
      Range {
        raw: "~1.2.3-rc.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse("~1.2.3-rc.0").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.3-rc.0").unwrap()),
        semver_range: SemverRange::Patch,
        semver_number: "1.2.3-rc.0".to_string(),
      },
    ),
  ]
}

pub fn get_unsupported() -> Vec<&'static str> {
  vec![
    "$typescript",
    "/path/to/foo",
    "/path/to/foo.tar",
    "/path/to/foo.tgz",
    "1.typo.wat",
    "=v1.2.3",
    "@f fo o al/ a d s ;f",
    "@foo/bar",
    "@foo/bar@",
    "git+file://path/to/repo#1.2.3",
    "not-git@hostname.com:some/repo",
    "user/foo#1234::path:dist",
    "user/foo#notimplemented:value",
    "user/foo#path:dist",
    "user/foo#semver:^1.2.3",
    "#1.2.3",
  ]
}

pub fn get_range_major() -> Vec<(&'static str, RangeMajor)> {
  vec![(
    "~1",
    RangeMajor {
      raw: "~1".to_string(),
      node_range: Rc::new(node_semver::Range::parse("~1.999999.999999").unwrap()),
      node_version: Rc::new(node_semver::Version::parse("1.999999.999999").unwrap()),
      semver_number: "1".to_string(),
      semver_range: SemverRange::Patch,
    },
  )]
}

pub fn get_range_minor() -> Vec<(&'static str, RangeMinor)> {
  vec![
    (
      "<5.0",
      RangeMinor {
        raw: "<5.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<5.0").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("5.0.999999").unwrap()),
        semver_number: "5.0".to_string(),
        semver_range: SemverRange::Lt,
      },
    ),
    (
      "<=5.0",
      RangeMinor {
        raw: "<=5.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse("<=5.0").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("5.0.999999").unwrap()),
        semver_number: "5.0".to_string(),
        semver_range: SemverRange::Lte,
      },
    ),
    (
      ">5.0",
      RangeMinor {
        raw: ">5.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse(">5.0").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("5.0.999999").unwrap()),
        semver_number: "5.0".to_string(),
        semver_range: SemverRange::Gt,
      },
    ),
    (
      ">=5.0",
      RangeMinor {
        raw: ">=5.0".to_string(),
        node_range: Rc::new(node_semver::Range::parse(">=5.0").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("5.0.999999").unwrap()),
        semver_number: "5.0".to_string(),
        semver_range: SemverRange::Gte,
      },
    ),
    (
      "^4.1",
      RangeMinor {
        raw: "^4.1".to_string(),
        node_range: Rc::new(node_semver::Range::parse("^4.1").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("4.1.999999").unwrap()),
        semver_number: "4.1".to_string(),
        semver_range: SemverRange::Minor,
      },
    ),
    (
      "~1.2",
      RangeMinor {
        raw: "~1.2".to_string(),
        node_range: Rc::new(node_semver::Range::parse("~1.2").unwrap()),
        node_version: Rc::new(node_semver::Version::parse("1.2.999999").unwrap()),
        semver_number: "1.2".to_string(),
        semver_range: SemverRange::Patch,
      },
    ),
  ]
}

pub fn get_alias() -> Vec<(&'static str, Alias)> {
  vec![
    (
      "npm:@minh.nguyen/plugin-transform-destructuring@^7.5.2",
      Alias {
        raw: "npm:@minh.nguyen/plugin-transform-destructuring@^7.5.2".to_string(),
        name: "@minh.nguyen/plugin-transform-destructuring".to_string(),
        version_str: "^7.5.2".to_string(),
        inner_specifier: Rc::new(Specifier::create("^7.5.2")),
      },
    ),
    (
      "npm:@types/selenium-webdriver@4.1.18",
      Alias {
        raw: "npm:@types/selenium-webdriver@4.1.18".to_string(),
        name: "@types/selenium-webdriver".to_string(),
        version_str: "4.1.18".to_string(),
        inner_specifier: Rc::new(Specifier::create("4.1.18")),
      },
    ),
    (
      "npm:foo@1.2.3",
      Alias {
        raw: "npm:foo@1.2.3".to_string(),
        name: "foo".to_string(),
        version_str: "1.2.3".to_string(),
        inner_specifier: Rc::new(Specifier::create("1.2.3")),
      },
    ),
  ]
}

pub fn get_file() -> Vec<(&'static str, File)> {
  vec![
    (
      "file:../path/to/foo",
      File {
        raw: "file:../path/to/foo".to_string(),
      },
    ),
    (
      "file:./path/to/foo",
      File {
        raw: "file:./path/to/foo".to_string(),
      },
    ),
    (
      "file:/../path/to/foo",
      File {
        raw: "file:/../path/to/foo".to_string(),
      },
    ),
    (
      "file:/./path/to/foo",
      File {
        raw: "file:/./path/to/foo".to_string(),
      },
    ),
    (
      "file:/.path/to/foo",
      File {
        raw: "file:/.path/to/foo".to_string(),
      },
    ),
    (
      "file://.",
      File {
        raw: "file://.".to_string(),
      },
    ),
    (
      "file://../path/to/foo",
      File {
        raw: "file://../path/to/foo".to_string(),
      },
    ),
    (
      "file://./path/to/foo",
      File {
        raw: "file://./path/to/foo".to_string(),
      },
    ),
    (
      "file:////path/to/foo",
      File {
        raw: "file:////path/to/foo".to_string(),
      },
    ),
    (
      "file:///path/to/foo",
      File {
        raw: "file:///path/to/foo".to_string(),
      },
    ),
    (
      "file://path/to/foo",
      File {
        raw: "file://path/to/foo".to_string(),
      },
    ),
    (
      "file:/path/to/foo",
      File {
        raw: "file:/path/to/foo".to_string(),
      },
    ),
    (
      "file:/~path/to/foo",
      File {
        raw: "file:/~path/to/foo".to_string(),
      },
    ),
    (
      "file:path/to/directory",
      File {
        raw: "file:path/to/directory".to_string(),
      },
    ),
    (
      "file:path/to/foo",
      File {
        raw: "file:path/to/foo".to_string(),
      },
    ),
    (
      "file:path/to/foo.tar.gz",
      File {
        raw: "file:path/to/foo.tar.gz".to_string(),
      },
    ),
    (
      "file:path/to/foo.tgz",
      File {
        raw: "file:path/to/foo.tgz".to_string(),
      },
    ),
  ]
}

pub fn get_git() -> Vec<(&'static str, Git)> {
  vec![
    (
      "git+https://github.com/user/foo",
      Git {
        raw: "git+https://github.com/user/foo".to_string(),
        node_range: None,
        node_version: None,
        origin: "git+https://github.com/user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git+ssh://git@github.com/user/foo#1.2.3",
      Git {
        raw: "git+ssh://git@github.com/user/foo#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://git@github.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git+ssh://git@github.com/user/foo#semver:^1.2.3",
      Git {
        raw: "git+ssh://git@github.com/user/foo#semver:^1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("^1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://git@github.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Minor),
      },
    ),
    (
      "git+ssh://git@github.com:user/foo#semver:^1.2.3",
      Git {
        raw: "git+ssh://git@github.com:user/foo#semver:^1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("^1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://git@github.com:user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Minor),
      },
    ),
    (
      "git+ssh://git@notgithub.com/user/foo",
      Git {
        raw: "git+ssh://git@notgithub.com/user/foo".to_string(),
        node_range: None,
        node_version: None,
        origin: "git+ssh://git@notgithub.com/user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git+ssh://git@notgithub.com/user/foo#1.2.3",
      Git {
        raw: "git+ssh://git@notgithub.com/user/foo#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://git@notgithub.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git+ssh://git@notgithub.com/user/foo#semver:^1.2.3",
      Git {
        raw: "git+ssh://git@notgithub.com/user/foo#semver:^1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("^1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://git@notgithub.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Minor),
      },
    ),
    (
      "git+ssh://git@notgithub.com:user/foo",
      Git {
        raw: "git+ssh://git@notgithub.com:user/foo".to_string(),
        node_range: None,
        node_version: None,
        origin: "git+ssh://git@notgithub.com:user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git+ssh://git@notgithub.com:user/foo#1.2.3",
      Git {
        raw: "git+ssh://git@notgithub.com:user/foo#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://git@notgithub.com:user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git+ssh://git@notgithub.com:user/foo#semver:^1.2.3",
      Git {
        raw: "git+ssh://git@notgithub.com:user/foo#semver:^1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("^1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://git@notgithub.com:user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Minor),
      },
    ),
    (
      "git+ssh://github.com/user/foo",
      Git {
        raw: "git+ssh://github.com/user/foo".to_string(),
        node_range: None,
        node_version: None,
        origin: "git+ssh://github.com/user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git+ssh://github.com/user/foo#1.2.3",
      Git {
        raw: "git+ssh://github.com/user/foo#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://github.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git+ssh://github.com/user/foo#semver:^1.2.3",
      Git {
        raw: "git+ssh://github.com/user/foo#semver:^1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("^1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://github.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Minor),
      },
    ),
    (
      "git+ssh://mydomain.com:1234#1.2.3",
      Git {
        raw: "git+ssh://mydomain.com:1234#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://mydomain.com:1234".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git+ssh://mydomain.com:1234/hey",
      Git {
        raw: "git+ssh://mydomain.com:1234/hey".to_string(),
        node_range: None,
        node_version: None,
        origin: "git+ssh://mydomain.com:1234/hey".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git+ssh://mydomain.com:1234/hey#1.2.3",
      Git {
        raw: "git+ssh://mydomain.com:1234/hey#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://mydomain.com:1234/hey".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git+ssh://mydomain.com:foo",
      Git {
        raw: "git+ssh://mydomain.com:foo".to_string(),
        node_range: None,
        node_version: None,
        origin: "git+ssh://mydomain.com:foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git+ssh://mydomain.com:foo#1.2.3",
      Git {
        raw: "git+ssh://mydomain.com:foo#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://mydomain.com:foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git+ssh://mydomain.com:foo/bar#1.2.3",
      Git {
        raw: "git+ssh://mydomain.com:foo/bar#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://mydomain.com:foo/bar".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git+ssh://notgithub.com/user/foo",
      Git {
        raw: "git+ssh://notgithub.com/user/foo".to_string(),
        node_range: None,
        node_version: None,
        origin: "git+ssh://notgithub.com/user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git+ssh://notgithub.com/user/foo#1.2.3",
      Git {
        raw: "git+ssh://notgithub.com/user/foo#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://notgithub.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git+ssh://notgithub.com/user/foo#semver:^1.2.3",
      Git {
        raw: "git+ssh://notgithub.com/user/foo#semver:^1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("^1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://notgithub.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Minor),
      },
    ),
    (
      "git+ssh://username:password@mydomain.com:1234/hey#1.2.3",
      Git {
        raw: "git+ssh://username:password@mydomain.com:1234/hey#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git+ssh://username:password@mydomain.com:1234/hey".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git://github.com/user/foo",
      Git {
        raw: "git://github.com/user/foo".to_string(),
        node_range: None,
        node_version: None,
        origin: "git://github.com/user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git://github.com/user/foo#1.2.3",
      Git {
        raw: "git://github.com/user/foo#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git://github.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git://github.com/user/foo#semver:^1.2.3",
      Git {
        raw: "git://github.com/user/foo#semver:^1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("^1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git://github.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Minor),
      },
    ),
    (
      "git://notgithub.com/user/foo",
      Git {
        raw: "git://notgithub.com/user/foo".to_string(),
        node_range: None,
        node_version: None,
        origin: "git://notgithub.com/user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git://notgithub.com/user/foo#1.2.3",
      Git {
        raw: "git://notgithub.com/user/foo#1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git://notgithub.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Exact),
      },
    ),
    (
      "git://notgithub.com/user/foo#semver:^1.2.3",
      Git {
        raw: "git://notgithub.com/user/foo#semver:^1.2.3".to_string(),
        node_range: Some(Rc::new(node_semver::Range::parse("^1.2.3").unwrap())),
        node_version: Some(Rc::new(node_semver::Version::parse("1.2.3").unwrap())),
        origin: "git://notgithub.com/user/foo".to_string(),
        semver_number: Some("1.2.3".to_string()),
        semver_range: Some(SemverRange::Minor),
      },
    ),
    // Empty tag after hash
    (
      "git://github.com/user/foo#",
      Git {
        raw: "git://github.com/user/foo#".to_string(),
        node_range: None,
        node_version: None,
        origin: "git://github.com/user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    // Non-semver tags (branches, HEAD, etc.)
    (
      "git://github.com/user/foo#HEAD",
      Git {
        raw: "git://github.com/user/foo#HEAD".to_string(),
        node_range: None,
        node_version: None,
        origin: "git://github.com/user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git://github.com/user/foo#main",
      Git {
        raw: "git://github.com/user/foo#main".to_string(),
        node_range: None,
        node_version: None,
        origin: "git://github.com/user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "git://github.com/user/foo#develop",
      Git {
        raw: "git://github.com/user/foo#develop".to_string(),
        node_range: None,
        node_version: None,
        origin: "git://github.com/user/foo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
    (
      "github:user/repo#feature-branch",
      Git {
        raw: "github:user/repo#feature-branch".to_string(),
        node_range: None,
        node_version: None,
        origin: "github:user/repo".to_string(),
        semver_number: None,
        semver_range: None,
      },
    ),
  ]
}

pub fn get_url() -> Vec<(&'static str, Url)> {
  vec![
    (
      "http://insecure.com/foo.tgz",
      Url {
        raw: "http://insecure.com/foo.tgz".to_string(),
      },
    ),
    (
      "https://server.com/foo.tgz",
      Url {
        raw: "https://server.com/foo.tgz".to_string(),
      },
    ),
    (
      "https://server.com/foo.tgz",
      Url {
        raw: "https://server.com/foo.tgz".to_string(),
      },
    ),
  ]
}

// below from Specifer2

pub fn ranges() -> Vec<(&'static str, SemverRange)> {
  vec![
    // ("*", SemverRange::Any),
    ("", SemverRange::Exact),
    ("^", SemverRange::Minor),
    ("~", SemverRange::Patch),
    (">=", SemverRange::Gte),
    (">", SemverRange::Gt),
    ("<=", SemverRange::Lte),
    ("<", SemverRange::Lt),
  ]
}

pub fn prereleases() -> Vec<&'static str> {
  vec!["", "-alpha", "-alpha.0"]
}

pub fn protocols() -> Vec<&'static str> {
  vec!["", "workspace:"]
}

pub fn npm_names() -> Vec<&'static str> {
  vec!["@jsr/std__fs", "@minh.nguyen/plugin-transform-destructuring", "foo"]
}

pub fn git_urls() -> Vec<&'static str> {
  vec![
    "git+ssh://git@github.com/npm/cli",
    "git@github.com:npm/cli.git",
    "github:uNetworking/uWebSockets.js",
  ]
}
