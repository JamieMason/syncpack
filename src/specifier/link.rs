use crate::specifier::Specifier;

#[derive(Debug, PartialEq)]
pub struct Link {
  /// "link:../path/to/workspace"
  /// "link:./relative/path"
  pub raw: String,
}

impl Link {
  pub fn create(raw: &str) -> Specifier {
    Specifier::Link(Self { raw: raw.to_string() })
  }
}
