use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct Url {
  pub raw: String,
}

impl Url {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Url(Self { raw: raw.to_string() })
  }
}
