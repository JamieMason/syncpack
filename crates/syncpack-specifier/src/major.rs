use {
  crate::{HUGE, Specifier},
  std::{cell::OnceCell, rc::Rc},
};

#[derive(Debug)]
pub struct Major {
  /// "1"
  pub raw: String,
  /// Lazily parsed padded version: "1" -> "1.999999.999999"
  node_version: OnceCell<Rc<node_semver::Version>>,
}

impl PartialEq for Major {
  fn eq(&self, other: &Self) -> bool {
    self.raw == other.raw
  }
}

impl Major {
  pub fn create(raw: &str) -> Specifier {
    Specifier::Major(Self {
      raw: raw.to_string(),
      node_version: OnceCell::new(),
    })
  }

  #[cfg(test)]
  pub fn create_test(raw: &str) -> Self {
    Self {
      raw: raw.to_string(),
      node_version: OnceCell::new(),
    }
  }

  pub fn get_node_version(&self) -> Rc<node_semver::Version> {
    self
      .node_version
      .get_or_init(|| {
        let padded = format!("{}.{}.{}", self.raw, HUGE, HUGE);
        Specifier::new_node_version(&padded).expect("pre-validated major version")
      })
      .clone()
  }
}
