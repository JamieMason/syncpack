use crate::specifier::Specifier;

#[derive(Debug, PartialEq)]
pub struct Tag {
  /// "alpha"
  pub raw: String,
}

impl Tag {
  pub fn create(raw: &str) -> Specifier {
    Specifier::Tag(Self { raw: raw.to_string() })
  }
}
