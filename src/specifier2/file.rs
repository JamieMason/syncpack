use super::Specifier2;

#[cfg(test)]
#[path = "file_test.rs"]
mod file_test;

#[derive(Debug, PartialEq)]
pub struct File {
  pub raw: String,
}

impl File {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::File(Self { raw: raw.to_string() })
  }
}
