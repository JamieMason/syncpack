use crate::specifier::Specifier;

#[derive(Debug, PartialEq)]
pub struct File {
  /// "file:path/to/directory"
  /// "file:path/to/foo.tar.gz"
  pub raw: String,
}

impl File {
  pub fn create(raw: &str) -> Specifier {
    Specifier::File(Self { raw: raw.to_string() })
  }
}
