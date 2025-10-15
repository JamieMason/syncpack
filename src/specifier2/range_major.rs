use crate::specifier::semver_range::SemverRange;

use super::Specifier2;

#[cfg(test)]
#[path = "range_major_test.rs"]
mod range_major_test;

#[derive(Debug, PartialEq)]
pub struct RangeMajor {
  /// "^1"
  pub raw: String,
  /// SemverRange::Minor
  pub semver_range: Option<SemverRange>,
  /// "1"
  pub semver_number: Option<String>,
}

impl RangeMajor {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::RangeMajor(Self { raw: raw.to_string() })
  }
}
