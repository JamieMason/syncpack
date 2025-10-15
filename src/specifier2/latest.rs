use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct Latest {
  pub raw: String,
}

impl Latest {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Latest(Self { raw: raw.to_string() })
  }
}
