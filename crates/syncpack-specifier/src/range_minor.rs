use {
  crate::{HUGE, Specifier, semver_range::SemverRange, strip_semver_range},
  std::{cell::OnceCell, rc::Rc},
};

#[derive(Debug)]
pub struct RangeMinor {
  /// "^1.2"
  pub raw: String,
  /// Lazily parsed range: "^1.2.999999"
  node_range: OnceCell<Rc<node_semver::Range>>,
  /// Lazily parsed padded version: "1.2.999999"
  node_version: OnceCell<Rc<node_semver::Version>>,
  /// The raw semver specifier without range characters: "1.2"
  pub semver_number: String,
  /// The semver range characters used in this specifier
  pub semver_range: SemverRange,
}

impl PartialEq for RangeMinor {
  fn eq(&self, other: &Self) -> bool {
    self.raw == other.raw
  }
}

impl RangeMinor {
  pub fn create(raw: &str) -> Specifier {
    let semver_range = SemverRange::parse(raw);
    let semver_number = strip_semver_range(raw).to_string();
    Specifier::RangeMinor(Self {
      raw: raw.to_string(),
      node_range: OnceCell::new(),
      node_version: OnceCell::new(),
      semver_number,
      semver_range,
    })
  }

  #[cfg(test)]
  pub fn create_test(raw: &str) -> Self {
    let semver_range = SemverRange::parse(raw);
    let semver_number = strip_semver_range(raw).to_string();
    Self {
      raw: raw.to_string(),
      node_range: OnceCell::new(),
      node_version: OnceCell::new(),
      semver_number,
      semver_range,
    }
  }

  pub fn get_node_version(&self) -> Rc<node_semver::Version> {
    self
      .node_version
      .get_or_init(|| {
        let padded = format!("{}.{}", self.semver_number, HUGE);
        Specifier::new_node_version(&padded).expect("pre-validated range minor version")
      })
      .clone()
  }

  pub fn get_node_range(&self) -> Rc<node_semver::Range> {
    self
      .node_range
      .get_or_init(|| Specifier::new_node_range(&self.raw).expect("pre-validated range minor"))
      .clone()
  }
}
