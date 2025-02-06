use super::{basic_semver::BasicSemver, semver_range::SemverRange};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct WorkspaceProtocol {
  /// The exact version specifier as it was provided
  pub raw: String,
  /// The "workspace:" protocol is stateful and incomplete on its own, it
  /// depends on knowledge held elsewhere by the package manager in order to be
  /// useful. For this reason we need to know the local version of the package
  pub local_version: BasicSemver,
  /// The version portion of the specifier completed using the local version,
  /// for example:
  ///
  /// - "workspace:*" → "1.2.3"
  /// - "workspace:1.2.3" → "1.2.3"
  /// - "workspace:^" → "^1.2.3"
  /// - "workspace:^1.2.3" → "^1.2.3"
  /// - "workspace:~" → "~1.2.3"
  /// - "workspace:~1.2.3" → "~1.2.3"
  pub semver: BasicSemver,
}

impl WorkspaceProtocol {
  pub fn with_range(self, range: &SemverRange) -> Self {
    let semver = self.semver.with_range(range);
    Self {
      raw: format!("workspace:{}", semver.raw),
      local_version: self.local_version,
      semver,
    }
  }
}
