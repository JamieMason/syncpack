use {
  crate::{HUGE, Specifier},
  std::{cell::OnceCell, rc::Rc},
};

#[derive(Debug)]
pub struct Minor {
  /// "1.2"
  pub raw: String,
  /// Lazily parsed padded version: "1.2" -> "1.2.999999"
  node_version: OnceCell<Rc<node_semver::Version>>,
}

impl PartialEq for Minor {
  fn eq(&self, other: &Self) -> bool {
    self.raw == other.raw
  }
}

impl Minor {
  pub fn create(raw: &str) -> Specifier {
    Specifier::Minor(Self {
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
        let padded = format!("{}.{}", self.raw, HUGE);
        Specifier::new_node_version(&padded).expect("pre-validated minor version")
      })
      .clone()
  }
}
