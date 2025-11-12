use crate::specifier::Specifier;

#[derive(Debug, PartialEq)]
pub struct Catalog {
  /// The complete raw string
  ///
  /// Examples:
  /// - "catalog:"
  /// - "catalog:react18"
  pub raw: String,

  /// The catalog name (if specified)
  ///
  /// Examples:
  /// - "catalog:" -> None (uses default catalog)
  /// - "catalog:react18" -> Some("react18")
  pub name: Option<String>,
}

impl Catalog {
  /// Create a new Catalog from a raw string
  pub fn new(raw: String) -> Option<Self> {
    let name_str = raw.strip_prefix("catalog:")?;

    let name = if name_str.is_empty() { None } else { Some(name_str.to_string()) };

    Some(Self { raw, name })
  }

  /// Create a Catalog as Specifier variant (for compatibility)
  pub fn create(raw: &str) -> Specifier {
    match Self::new(raw.to_string()) {
      Some(catalog) => Specifier::Catalog(catalog),
      None => Specifier::Unsupported(raw.to_string()),
    }
  }
}
