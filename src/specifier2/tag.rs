use super::Specifier2;

#[cfg(test)]
#[path = "tag_test.rs"]
mod tag_test;

#[derive(Debug, PartialEq)]
pub struct Tag {
  pub raw: String,
}

impl Tag {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Tag(Self { raw: raw.to_string() })
  }
}
