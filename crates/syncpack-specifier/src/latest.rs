use {
  crate::{HUGE, Specifier, semver_range::SemverRange},
  std::{cell::OnceCell, rc::Rc},
};

#[derive(Debug)]
pub struct Latest {
  /// "*", "latest", "x"
  pub raw: String,
  /// Lazily parsed HUGE version for ordering (999999.999999.999999)
  node_version: OnceCell<Rc<node_semver::Version>>,
  /// The semver range is always Any for Latest
  pub semver_range: SemverRange,
}

impl PartialEq for Latest {
  fn eq(&self, other: &Self) -> bool {
    self.raw == other.raw && self.semver_range == other.semver_range
  }
}

impl Latest {
  pub fn create(raw: &str) -> Specifier {
    Specifier::Latest(Self {
      raw: raw.to_string(),
      node_version: OnceCell::new(),
      semver_range: SemverRange::Any,
    })
  }

  #[cfg(test)]
  pub fn create_test(raw: &str) -> Self {
    Self {
      raw: raw.to_string(),
      node_version: OnceCell::new(),
      semver_range: SemverRange::Any,
    }
  }

  pub fn get_node_version(&self) -> Rc<node_semver::Version> {
    self
      .node_version
      .get_or_init(|| {
        let huge = HUGE.to_string();
        let huge_version = format!("{huge}.{huge}.{huge}");
        Specifier::new_node_version(&huge_version).expect("HUGE version is always valid")
      })
      .clone()
  }
}
