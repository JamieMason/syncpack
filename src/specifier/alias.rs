use {crate::specifier::Specifier, std::rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Alias {
  /// "npm:@fluidframework/build-tools@~0.44.0"
  pub raw: String,
  /// "@fluidframework/build-tools"
  pub name: String,
  /// The version part of the alias (e.g., "^1.2.3" or "*" if none specified)
  pub version_str: String,
  /// Cached inner specifier for delegation
  pub inner_specifier: Rc<Specifier>,
}

impl Alias {
  pub fn create(raw: &str) -> Specifier {
    let name = raw.strip_prefix("npm:").map(|after_prefix| {
      after_prefix
        .rfind('@')
        .filter(|&at_pos| at_pos > 0 && !after_prefix[..at_pos].is_empty())
        .map(|at_pos| &after_prefix[..at_pos])
        .unwrap_or(after_prefix)
    });

    let version_part = raw
      .strip_prefix("npm:")
      .and_then(|after_prefix| after_prefix.rfind('@').map(|at_pos| (after_prefix, at_pos)))
      .and_then(|(after_prefix, at_pos)| (at_pos > 0 && !after_prefix[..at_pos].is_empty()).then(|| &after_prefix[at_pos + 1..]))
      .filter(|version| !version.is_empty());

    name
      .map(|name| {
        // Default to "*" if no version specified
        let version_str = version_part.unwrap_or("*");

        // Create inner specifier by parsing the version string directly
        // This bypasses the cache to avoid re-borrowing issues
        let inner = Specifier::create(version_str);
        let inner_specifier = Rc::new(inner);

        Specifier::Alias(Self {
          raw: raw.to_string(),
          name: name.to_string(),
          version_str: version_str.to_string(),
          inner_specifier,
        })
      })
      .unwrap_or(Specifier::Unsupported(raw.to_string()))
  }
}
