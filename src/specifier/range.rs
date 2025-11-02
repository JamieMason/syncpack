use {
  crate::{
    semver_range::SemverRange,
    specifier::{strip_semver_range, Specifier},
  },
  std::rc::Rc,
};

#[derive(Debug, PartialEq)]
pub struct Range {
  /// ">=1.2.3"
  pub raw: String,
  /// Used when checking if specifiers satisfy each other
  ///
  /// - ">=1.2.3"
  pub node_range: Rc<node_semver::Range>,
  /// Used for ordering and comparison, semver range characters are NOT
  /// included
  ///
  /// - ">=1.2.3" → "1.2.3"
  pub node_version: Rc<node_semver::Version>,
  /// The semver range characters used in this specifier
  ///
  /// SemverRange::Gte
  pub semver_range: SemverRange,
  /// The raw semver specifier without range characters
  ///
  /// "1.2.3"
  pub semver_number: String,
}

impl Range {
  pub fn create(raw: &str) -> Specifier {
    let semver_range = SemverRange::parse(raw);
    let semver_number = strip_semver_range(raw).to_string();

    match (Specifier::new_node_range(raw), Specifier::new_node_version(&semver_number)) {
      (Some(node_range), Some(node_version)) => Specifier::Range(Self {
        raw: raw.to_string(),
        node_range,
        node_version,
        semver_range,
        semver_number,
      }),
      _ => Specifier::Unsupported(raw.to_string()),
    }
  }
}
