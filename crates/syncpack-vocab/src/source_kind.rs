use {
  crate::config_error::UnsupportedConfigError,
  serde::{Deserialize, Serialize},
};

/// Which kind of file a `DependencyType` reads from.
///
/// Embedded on `DependencyType` so iteration can pair sources × dep types.
/// Default for user `customTypes` is `PackageJson`. Auto-generated catalog
/// dep types set this explicitly (`pnpmCatalog*` → `PnpmWorkspace`,
/// `bunCatalog*` → `PackageJson`).
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum SourceKind {
  PackageJson,
  PnpmWorkspace,
}

impl SourceKind {
  /// Parse the user-facing string form of a source.
  ///
  /// Accepts the PascalCase form used in `customTypes.<name>.source`.
  pub fn parse(raw: &str) -> Result<Self, UnsupportedConfigError> {
    match raw {
      "PackageJson" => Ok(SourceKind::PackageJson),
      "PnpmWorkspace" => Ok(SourceKind::PnpmWorkspace),
      other => Err(UnsupportedConfigError::InvalidSource { value: other.to_string() }),
    }
  }
}
