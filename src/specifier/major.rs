use {
  crate::{specifier, specifier::Specifier},
  std::rc::Rc,
};

#[derive(Debug, PartialEq)]
pub struct Major {
  /// "1"
  pub raw: String,
  /// Used for ordering and comparison
  ///
  /// "1" -> "1.999999.999999"
  pub node_version: Rc<node_semver::Version>,
}

impl Major {
  pub fn create(raw: &str) -> Specifier {
    let padded = format!("{}.{}.{}", raw, specifier::HUGE, specifier::HUGE);
    match Specifier::new_node_version(&padded) {
      Some(node_version) => Specifier::Major(Self {
        raw: raw.to_string(),
        node_version,
      }),
      None => Specifier::Unsupported(raw.to_string()),
    }
  }
}
