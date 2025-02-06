use {
  super::{basic_semver::BasicSemver, semver_range::SemverRange},
  log::debug,
};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Alias {
  /// The exact version specifier as it was provided
  pub raw: String,
  /// The name of the dependency being aliased, for example:
  ///
  /// - "npm:foo" → "foo"
  /// - "npm:foo@1.2.3" → "foo"
  /// - "npm:@foo/bar@1.2.3" → "@foo/bar"
  pub name: String,
  /// The version of the dependency being aliased if set, for example:
  ///
  /// - "npm:foo" → None
  /// - "npm:foo@1.2.3" → Some("1.2.3")
  /// - "npm:@foo/bar@1.2.3" → Some("1.2.3")
  pub semver: Option<BasicSemver>,
}

impl Alias {
  pub fn with_range(self, range: &SemverRange) -> Self {
    if let Some(semver) = self.semver {
      let semver = semver.with_range(range);
      Self {
        raw: format!("npm:{}@{}", self.name, semver.raw),
        name: self.name,
        semver: Some(semver),
      }
    } else {
      debug!("Cannot apply semver range '{:?}' to specifier '{}'", range, self.raw);
      self
    }
  }
}
