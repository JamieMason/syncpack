use super::Specifier2;

#[cfg(test)]
#[path = "major_test.rs"]
mod major_test;

#[derive(Debug, PartialEq)]
pub struct Major {
  pub raw: String,
}

impl Major {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Major(Self { raw: raw.to_string() })
  }
}
