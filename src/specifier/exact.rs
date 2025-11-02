use {crate::specifier::Specifier, std::rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Exact {
  /// "1.2.3"
  pub raw: String,
  /// Used for ordering and comparison, semver range characters are NOT
  /// included, for example:
  ///
  /// - "1.2.3" → "1.2.3"
  /// - "^1.2.3" → "1.2.3"
  pub node_version: Rc<node_semver::Version>,
  /// Range representation of exact version
  pub node_range: Rc<node_semver::Range>,
}

impl Exact {
  pub fn create(raw: &str) -> Specifier {
    match Specifier::new_node_version(raw) {
      Some(node_version) => {
        let node_range = Specifier::new_node_range(raw).unwrap_or_else(|| {
          // Fallback: should never happen if node_version parses
          Rc::new(node_semver::Range::parse(raw).unwrap())
        });
        Specifier::Exact(Self {
          raw: raw.to_string(),
          node_version,
          node_range,
        })
      }
      None => Specifier::Unsupported(raw.to_string()),
    }
  }
}
