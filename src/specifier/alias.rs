use {
  super::{basic_semver::BasicSemver, semver_range::SemverRange},
  crate::specifier::regexes::NAME_WITHIN_NPM_ALIAS,
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

  pub fn with_semver(self, semver: &BasicSemver) -> Self {
    Self {
      raw: format!("npm:{}@{}", self.name, semver.raw),
      name: self.name,
      semver: Some(semver.clone()),
    }
  }

  /// Match a package name inside an npm alias specifier
  /// "npm:@lit-labs/ssr@3.3.0" -> "@lit-labs/ssr"
  /// "npm:@jsr/luca__cases@1" -> "@jsr/luca__cases"
  /// "npm:@jsr/std__fmt@^1.0.3" -> "@jsr/std__fmt"
  /// "npm:@jsr/std__yaml" -> "@jsr/std__yaml"
  /// "npm:lit@3.2.1" -> "lit"
  pub fn extract_package_name(&self) -> Option<String> {
    NAME_WITHIN_NPM_ALIAS
      .captures(&self.raw)
      .and_then(|caps| caps.get(1))
      .map(|m| m.as_str().to_string())
  }
}
