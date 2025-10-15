use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct ComplexSemver {
  pub raw: String,
}

impl ComplexSemver {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::ComplexSemver(Self { raw: raw.to_string() })
  }
}
