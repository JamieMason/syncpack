use {
  crate::{semver_range::SemverRange, specifier::Specifier},
  std::rc::Rc,
};

/// Specifier type for workspace protocol dependencies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceSpecifier {
  /// Resolved to a complete specifier
  ///
  /// Examples:
  /// - workspace:1.2.3 -> Resolved(Exact("1.2.3"))
  /// - workspace:^1.2.3 -> Resolved(Major(...))
  Resolved(Rc<Specifier>),

  /// Unresolved range prefix (requires local package version)
  ///
  /// Examples:
  /// - workspace:^ -> RangeOnly(SemverRange::Minor)
  /// - workspace:~ -> RangeOnly(SemverRange::Patch)
  /// - workspace:* -> RangeOnly(SemverRange::Any)
  RangeOnly(SemverRange),
}

impl WorkspaceSpecifier {
  /// Check if this requires resolution
  pub fn needs_resolution(&self) -> bool {
    matches!(self, Self::RangeOnly(_))
  }

  /// Resolve RangeOnly with local package version
  /// Returns None if already resolved or resolution fails
  pub fn resolve_with(&self, local_version: &str) -> Option<Rc<Specifier>> {
    match self {
      Self::RangeOnly(range) => {
        let resolved = match range {
          SemverRange::Any => local_version.to_string(),
          SemverRange::Minor => format!("^{local_version}"),
          SemverRange::Patch => format!("~{local_version}"),
          _ => return None, // Only *, ^, and ~ are supported
        };
        Some(Specifier::new(&resolved))
      }
      Self::Resolved(_) => None, // Already resolved
    }
  }

  /// Get the underlying Specifier if resolved
  pub fn as_resolved(&self) -> Option<&Rc<Specifier>> {
    match self {
      Self::Resolved(spec) => Some(spec),
      Self::RangeOnly(_) => None,
    }
  }
}
