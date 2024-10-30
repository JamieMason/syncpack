use super::{
  orderable::{IsOrderable, Orderable},
  parser,
  simple_semver::SimpleSemver,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Semver {
  Simple(SimpleSemver),
  Complex(String),
}

impl Semver {
  pub fn new(specifier: &str) -> Result<Self, String> {
    let str = parser::sanitise(specifier);
    let string = str.to_string();
    if let Ok(simple_semver) = SimpleSemver::new(str) {
      Ok(Self::Simple(simple_semver))
    } else if parser::is_complex_range(str) {
      Ok(Self::Complex(string))
    } else {
      Err(format!(
        "'{specifier}' was expected to be a semver specifier but was not recognised"
      ))
    }
  }
}

impl IsOrderable for Semver {
  fn get_orderable(&self) -> Orderable {
    match self {
      Self::Simple(simple_semver) => simple_semver.get_orderable(),
      Self::Complex(_) => Orderable::new(),
    }
  }
}
