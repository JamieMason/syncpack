use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct Minor {
  pub raw: String,
}

impl Minor {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::Minor(Self { raw: raw.to_string() })
  }
}
