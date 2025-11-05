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
    // Strip leading = if present (npm allows =1.2.3 as equivalent to 1.2.3)
    let version_without_equals = raw.strip_prefix('=').unwrap_or(raw);

    match Specifier::new_node_version(version_without_equals) {
      Some(node_version) => {
        let node_range = Specifier::new_node_range(version_without_equals).unwrap_or_else(|| {
          // Fallback: should never happen if node_version parses
          Rc::new(node_semver::Range::parse(version_without_equals).unwrap())
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
