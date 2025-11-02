use {
  crate::{
    semver_range::SemverRange,
    specifier::{self, strip_semver_range, Specifier},
  },
  std::rc::Rc,
};

#[derive(Debug, PartialEq)]
pub struct RangeMinor {
  /// "^1.2"
  pub raw: String,
  /// Used when checking if specifiers satisfy each other
  ///
  /// - "^1.2.999999"
  pub node_range: Rc<node_semver::Range>,
  /// Used for ordering and comparison, semver range characters are NOT
  /// included
  ///
  /// - "^1" → "1.2.999999"
  pub node_version: Rc<node_semver::Version>,
  /// The raw semver specifier without range characters
  ///
  /// "1"
  pub semver_number: String,
  /// The semver range characters used in this specifier
  ///
  /// `SemverRange::Minor`
  pub semver_range: SemverRange,
}

impl RangeMinor {
  pub fn create(raw: &str) -> Specifier {
    let semver_range = SemverRange::parse(raw);
    let semver_number = strip_semver_range(raw).to_string();
    let padded = format!("{}.{}", semver_number, specifier::HUGE);

    match (Specifier::new_node_version(&padded), Specifier::new_node_range(raw)) {
      (Some(node_version), Some(node_range)) => Specifier::RangeMinor(Self {
        raw: raw.to_string(),
        node_range,
        node_version,
        semver_number,
        semver_range,
      }),
      _ => Specifier::Unsupported(raw.to_string()),
    }
  }
}
