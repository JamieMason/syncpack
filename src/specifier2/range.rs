use crate::specifier::semver_range::SemverRange;

use super::Specifier2;

#[cfg(test)]
#[path = "range_test.rs"]
mod range_test;

#[derive(Debug, PartialEq)]
pub struct Range {
  /// ">=1.2.3"
  pub raw: String,
  /// SemverRange::Gte
  pub semver_range: Option<SemverRange>,
  /// "1.2.3"
  pub semver_number: Option<String>,
}

impl Range {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Range(Self { raw: raw.to_string() })
  }
}
