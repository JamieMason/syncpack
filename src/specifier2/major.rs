use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct Major {
  pub raw: String,
}

impl Major {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Major(Self { raw: raw.to_string() })
  }
}
