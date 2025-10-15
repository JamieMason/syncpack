use super::Specifier2;

#[cfg(test)]
#[path = "url_test.rs"]
mod url_test;

#[derive(Debug, PartialEq)]
pub struct Url {
  pub raw: String,
}

impl Url {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Url(Self { raw: raw.to_string() })
  }
}
