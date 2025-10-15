use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct Git {
  pub raw: String,
}

impl Git {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Git(Self { raw: raw.to_string() })
  }
}
