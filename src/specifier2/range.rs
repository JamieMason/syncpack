use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct Range {
  pub raw: String,
}

impl Range {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Range(Self { raw: raw.to_string() })
  }
}
