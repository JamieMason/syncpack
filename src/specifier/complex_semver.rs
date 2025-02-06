use {super::semver_range::SemverRange, log::debug, node_semver::Range};

/// A specifier containing multiple ranges and/or versions
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ComplexSemver {
  /// The exact version specifier as it was provided
  pub raw: String,
  /// A `node_semver::Range` created from the semver portion of the specifier,
  /// WITH any semver range characters, for example:
  ///
  /// - "<1.5.0 || >=1.6.0" → "<1.5.0 || >=1.6.0"
  pub node_range: Range,
}

impl ComplexSemver {
  pub fn with_range(self, range: &SemverRange) -> Self {
    debug!("Cannot apply semver range '{:?}' to specifier '{}'", range, self.raw);
    self
  }
}
