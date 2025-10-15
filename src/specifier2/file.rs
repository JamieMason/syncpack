use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct File {
  pub raw: String,
}

impl File {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::File(Self { raw: raw.to_string() })
  }
}
