use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct Exact {
  pub raw: String,
}

impl Exact {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Exact(Self { raw: raw.to_string() })
  }
}
