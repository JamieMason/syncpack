use crate::specifier2::{
  modname::File, modname::Url, Alias, ComplexSemver, Exact, Git, Latest, Major, Minor, Range, RangeMajor, RangeMinor, Specifier2, Tag,
  WorkspaceProtocol,
};

pub fn get_latest() -> Vec<&'static str> {
  vec!["latest", "*", "x"]
}

pub fn get_tag() -> Vec<&'static str> {
  vec!["alpha", "beta"]
}

pub fn get_major() -> Vec<&'static str> {
  vec!["1"]
}

pub fn get_minor() -> Vec<&'static str> {
  vec!["1.2"]
}

pub fn get_exact() -> Vec<&'static str> {
  vec!["1.2.3", "1.2.3-alpha", "1.2.3-rc.1", "1.2.3-alpha", "1.2.3-rc.0"]
}

pub fn get_complex_semver() -> Vec<&'static str> {
  vec![
    "1.3.0 || <1.0.0 >2.0.0",
    "<1.0.0 >2.0.0",
    "<1.0.0 >=2.0.0",
    "<1.5.0 || >=1.6.0",
    "<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2",
    "<=1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2",
    ">1.0.0 <1.0.0",
    ">1.0.0 <=2.0.0",
    ">=2.3.4 || <=1.2.3",
  ]
}

pub fn get_workspace_protocol() -> Vec<&'static str> {
  vec!["workspace:*", "workspace:^", "workspace:~"]
}

pub fn get_range() -> Vec<&'static str> {
  vec![
    "<1.2.3-alpha",
    "<1.2.3-rc.0",
    "<=1.2.3-alpha",
    "<=1.2.3-rc.0",
    ">1.2.3-alpha",
    ">1.2.3-rc.0",
    ">=1.2.3-alpha",
    ">=1.2.3-rc.0",
    "^1.2.3",
    "^1.2.3-alpha",
    "^1.2.3-rc.0",
    "~1.2.3-alpha",
    "~1.2.3-rc.0",
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
  ]
}

pub fn get_range_major() -> Vec<&'static str> {
  vec!["~1"]
}

pub fn get_range_minor() -> Vec<&'static str> {
  vec!["<5.0", "<=5.0", ">5.0", ">=5.0", "^4.1", "~1.2", "~1.2"]
}

pub fn get_alias() -> Vec<(&'static str, Alias)> {
  vec![
    (
      "npm:@minh.nguyen/plugin-transform-destructuring@^7.5.2",
      Alias {
        raw: "npm:@minh.nguyen/plugin-transform-destructuring@^7.5.2",
        name: "@minh.nguyen/plugin-transform-destructuring",
        semver_string: Some("^7.5.2"),
      },
    ),
    (
      "npm:@types/selenium-webdriver@4.1.18",
      Alias {
        raw: "npm:@types/selenium-webdriver@4.1.18",
        name: "@types/selenium-webdriver",
        semver_string: Some("4.1.18"),
      },
    ),
    (
      "npm:foo@1.2.3",
      Alias {
        raw: "npm:foo@1.2.3",
        name: "foo",
        semver_string: Some("1.2.3"),
      },
    ),
  ]
}

pub fn get_file() -> Vec<&'static str> {
  vec![
    "file:../path/to/foo",
    "file:./path/to/foo",
    "file:/../path/to/foo",
    "file:/./path/to/foo",
    "file:/.path/to/foo",
    "file://.",
    "file://../path/to/foo",
    "file://./path/to/foo",
    "file:////path/to/foo",
    "file:///path/to/foo",
    "file://path/to/foo",
    "file:/path/to/foo",
    "file:/~path/to/foo",
    "file:path/to/directory",
    "file:path/to/foo",
    "file:path/to/foo.tar.gz",
    "file:path/to/foo.tgz",
  ]
}

pub fn get_git() -> Vec<&'static str> {
  vec![
    "git+https://github.com/user/foo",
    "git+ssh://git@github.com/user/foo#1.2.3",
    "git+ssh://git@github.com/user/foo#semver:^1.2.3",
    "git+ssh://git@github.com:user/foo#semver:^1.2.3",
    "git+ssh://git@notgithub.com/user/foo",
    "git+ssh://git@notgithub.com/user/foo#1.2.3",
    "git+ssh://git@notgithub.com/user/foo#semver:^1.2.3",
    "git+ssh://git@notgithub.com:user/foo",
    "git+ssh://git@notgithub.com:user/foo#1.2.3",
    "git+ssh://git@notgithub.com:user/foo#semver:^1.2.3",
    "git+ssh://github.com/user/foo",
    "git+ssh://github.com/user/foo#1.2.3",
    "git+ssh://github.com/user/foo#semver:^1.2.3",
    "git+ssh://mydomain.com:1234#1.2.3",
    "git+ssh://mydomain.com:1234/hey",
    "git+ssh://mydomain.com:1234/hey#1.2.3",
    "git+ssh://mydomain.com:foo",
    "git+ssh://mydomain.com:foo#1.2.3",
    "git+ssh://mydomain.com:foo/bar#1.2.3",
    "git+ssh://notgithub.com/user/foo",
    "git+ssh://notgithub.com/user/foo#1.2.3",
    "git+ssh://notgithub.com/user/foo#semver:^1.2.3",
    "git+ssh://username:password@mydomain.com:1234/hey#1.2.3",
    "git://github.com/user/foo",
    "git://github.com/user/foo#1.2.3",
    "git://github.com/user/foo#semver:^1.2.3",
    "git://notgithub.com/user/foo",
    "git://notgithub.com/user/foo#1.2.3",
    "git://notgithub.com/user/foo#semver:^1.2.3",
  ]
}

pub fn get_url() -> Vec<&'static str> {
  vec![
    "http://insecure.com/foo.tgz",
    "https://server.com/foo.tgz",
    "https://server.com/foo.tgz",
  ]
}
