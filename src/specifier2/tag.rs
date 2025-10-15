use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct Tag {
  pub raw: String,
}

impl Tag {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Tag(Self { raw: raw.to_string() })
  }
}
