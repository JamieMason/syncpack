use crate::specifier::Specifier;

#[derive(Debug, PartialEq)]
pub struct Url {
  /// - "http://insecure.com/foo.tgz"
  /// - "https://server.com/foo.tgz"
  /// - "https://server.com/foo.tgz"
  pub raw: String,
}

impl Url {
  pub fn create(raw: &str) -> Specifier {
    Specifier::Url(Self { raw: raw.to_string() })
  }
}
