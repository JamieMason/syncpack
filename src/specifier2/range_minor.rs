use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct RangeMinor {
  pub raw: String,
}

impl RangeMinor {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::RangeMinor(Self { raw: raw.to_string() })
  }
}
