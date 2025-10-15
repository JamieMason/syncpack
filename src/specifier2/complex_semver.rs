use super::Specifier2;

#[cfg(test)]
#[path = "complex_semver_test.rs"]
mod complex_semver_test;

#[derive(Debug, PartialEq)]
pub struct ComplexSemver {
  pub raw: String,
}

impl ComplexSemver {
  pub fn new(raw: &str) -> Specifier2 {
    Specifier2::ComplexSemver(Self { raw: raw.to_string() })
  }
}
