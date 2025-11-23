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

  /// Get the underlying Specifier if resolved
  pub fn as_resolved(&self) -> Option<&Rc<Specifier>> {
    match self {
      Self::Resolved(spec) => Some(spec),
      Self::RangeOnly(_) => None,
    }
  }
}
