use super::Specifier2;

#[cfg(test)]
#[path = "range_minor_test.rs"]
mod range_minor_test;

#[derive(Debug, PartialEq)]
pub struct RangeMinor {
  pub raw: String,
}

impl RangeMinor {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::RangeMinor(Self { raw: raw.to_string() })
  }
}
