use crate::specifier::semver_range::SemverRange;

use super::Specifier2;

#[cfg(test)]
#[path = "range_minor_test.rs"]
mod range_minor_test;

#[derive(Debug, PartialEq)]
pub struct RangeMinor {
  /// "^1.2"
  pub raw: String,
  /// SemverRange::Minor
  pub semver_range: Option<SemverRange>,
  /// "1.2"
  pub semver_number: Option<String>,
}

impl RangeMinor {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::RangeMinor(Self { raw: raw.to_string() })
  }
}
