use {
  crate::{semver_range::SemverRange, specifier::Specifier},
  std::rc::Rc,
};

#[derive(Debug, PartialEq)]
pub struct Latest {
  /// "*"
  /// "latest"
  /// "x"
  pub raw: String,
  /// Used for ordering - Latest gets the HUGE version (999999.999999.999999)
  /// so it sorts after all real versions
  pub node_version: Rc<node_semver::Version>,
  /// The semver range is always Any for Latest
  pub semver_range: SemverRange,
}

impl Latest {
  pub fn create(raw: &str) -> Specifier {
    let huge = crate::specifier::HUGE.to_string();
    let huge_version = format!("{huge}.{huge}.{huge}");
    match Specifier::new_node_version(&huge_version) {
      Some(node_version) => Specifier::Latest(Self {
        raw: raw.to_string(),
        node_version,
        semver_range: SemverRange::Any,
      }),
      None => Specifier::Unsupported(raw.to_string()),
    }
  }
}
