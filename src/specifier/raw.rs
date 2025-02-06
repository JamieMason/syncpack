use {super::semver_range::SemverRange, log::debug};

/// A specifier not containing semver so not much can be done with it
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Raw {
  /// The exact version specifier as it was provided
  pub raw: String,
}

impl Raw {
  pub fn with_range(self, range: &SemverRange) -> Self {
    debug!("Cannot apply semver range '{:?}' to specifier '{}'", range, self.raw);
    self
  }
}
