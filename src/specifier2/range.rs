use super::Specifier2;

#[cfg(test)]
#[path = "range_test.rs"]
mod range_test;

#[derive(Debug, PartialEq)]
pub struct Range {
  pub raw: String,
}

impl Range {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Range(Self { raw: raw.to_string() })
  }
}
