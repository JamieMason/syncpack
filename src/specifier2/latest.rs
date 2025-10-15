use super::Specifier2;

#[cfg(test)]
#[path = "latest_test.rs"]
mod latest_test;

#[derive(Debug, PartialEq)]
pub struct Latest {
  pub raw: String,
}

impl Latest {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Latest(Self { raw: raw.to_string() })
  }
}
