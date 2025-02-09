use {
  super::{basic_semver::BasicSemver, semver_range::SemverRange},
  log::debug,
};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Git {
  /// The exact version specifier as it was provided, for example:
  ///
  /// - "git+ssh://git@github.com/npm/cli"
  /// - "git+ssh://git@github.com/npm/cli#1.2.3"
  /// - "git+ssh://git@github.com/npm/cli#HEAD"
  /// - "git@github.com:npm/cli.git"
  /// - "git@github.com:npm/cli.git#1.2.3"
  /// - "git@github.com:npm/cli.git#HEAD"
  /// - "github:uNetworking/uWebSockets.js"
  /// - "github:uNetworking/uWebSockets.js#1.2.3"
  /// - "github:uNetworking/uWebSockets.js#HEAD"
  pub raw: String,
  /// The name of the dependency being aliased, for example:
  ///
  /// - "git+ssh://git@github.com/npm/cli"
  /// - "git@github.com:npm/cli.git"
  /// - "github:uNetworking/uWebSockets.js"
  pub origin: String,
  /// The tagged version if set, for example:
  ///
  /// "git+ssh://git@github.com/npm/cli" → None
  /// "git+ssh://git@github.com/npm/cli#1.2.3" → Some("1.2.3")
  /// "git+ssh://git@github.com/npm/cli#HEAD" → None
  /// "git@github.com:npm/cli.git" → None
  /// "git@github.com:npm/cli.git#1.2.3" → Some("1.2.3")
  /// "git@github.com:npm/cli.git#HEAD" → None
  /// "github:uNetworking/uWebSockets.js" → None
  /// "github:uNetworking/uWebSockets.js#1.2.3" → Some("1.2.3")
  /// "github:uNetworking/uWebSockets.js#HEAD" → None
  pub semver: Option<BasicSemver>,
}

impl Git {
  pub fn with_range(self, range: &SemverRange) -> Self {
    if let Some(semver) = self.semver {
      let semver = semver.with_range(range);
      Self {
        raw: format!("{}#{}", self.origin, semver.raw),
        origin: self.origin,
        semver: Some(semver),
      }
    } else {
      debug!("Cannot apply semver range '{:?}' to specifier '{}'", range, self.raw);
      self
    }
  }

  pub fn with_semver(self, semver: &BasicSemver) -> Self {
    if let Some(current_semver) = self.semver {
      Self {
        raw: self.raw.replace(&current_semver.node_version.to_string(), &semver.raw),
        origin: self.origin,
        semver: Some(semver.clone()),
      }
    } else {
      Self {
        raw: format!("{}#{}", self.origin, semver.raw),
        origin: self.origin,
        semver: Some(semver.clone()),
      }
    }
  }
}
