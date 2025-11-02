use {
  crate::{specifier, specifier::Specifier},
  std::rc::Rc,
};

#[derive(Debug, PartialEq)]
pub struct Minor {
  /// "1.2"
  pub raw: String,
  /// Used for ordering and comparison
  ///
  /// "1.2" -> "1.2.999999"
  pub node_version: Rc<node_semver::Version>,
}

impl Minor {
  pub fn create(raw: &str) -> Specifier {
    let padded = format!("{}.{}", raw, specifier::HUGE);
    match Specifier::new_node_version(&padded) {
      Some(node_version) => Specifier::Minor(Self {
        raw: raw.to_string(),
        node_version,
      }),
      None => Specifier::Unsupported(raw.to_string()),
    }
  }
}
