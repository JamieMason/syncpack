use super::Specifier2;

#[cfg(test)]
#[path = "range_major_test.rs"]
mod range_major_test;

#[derive(Debug, PartialEq)]
pub struct RangeMajor {
  pub raw: String,
}

impl RangeMajor {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::RangeMajor(Self { raw: raw.to_string() })
  }
}
