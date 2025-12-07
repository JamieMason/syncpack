//! Fast pattern matching for dependency and package names.
//!
//! Optimizes common glob patterns into faster string operations:
//! - "react" → exact match (==)
//! - "@aws-sdk/**" → prefix match (starts_with)
//! - "**-loader" → suffix match (ends_with)
//! - Complex patterns → full glob matching (fallback)

use {
  globset::{Glob, GlobMatcher},
  std::fmt,
};

/// Pattern matcher optimized for common npm package name patterns.
#[derive(Clone)]
pub enum PatternMatcher {
  /// Exact string match: "react" → value == "react"
  Exact(String),

  /// Prefix match: "@aws-sdk/**" → value.starts_with("@aws-sdk/")
  Prefix(String),

  /// Suffix match: "**-loader" → value.ends_with("-loader")
  Suffix(String),

  /// Full glob matching for complex patterns
  Glob(GlobMatcher),
}

impl PatternMatcher {
  /// Create a pattern matcher from a glob pattern string.
  ///
  /// Examples:
  /// - "react" → Exact("react")
  /// - "@aws-sdk/**" → Prefix("@aws-sdk/")
  /// - "**-loader" → Suffix("-loader")
  /// - "**/test/**" → Glob(...)
  pub fn from_pattern(pattern: &str) -> Self {
    // Exact match (no wildcards)
    if !pattern.contains('*') && !pattern.contains('?') && !pattern.contains('[') {
      return Self::Exact(pattern.to_string());
    }

    // Prefix: "@aws-sdk/**", "foo/**"
    // Must end with /** and have no wildcards before that
    if let Some(prefix) = pattern.strip_suffix("/**") {
      if !prefix.contains('*') && !prefix.contains('?') && !prefix.contains('[') {
        return Self::Prefix(format!("{prefix}/"));
      }
    }

    // Suffix: "**-loader", "**/test"
    // Must start with **/ or ** and have no wildcards after
    if let Some(suffix) = pattern.strip_prefix("**/") {
      if !suffix.contains('*') && !suffix.contains('?') && !suffix.contains('[') {
        return Self::Suffix(suffix.to_string());
      }
    }
    if let Some(suffix) = pattern.strip_prefix("**") {
      if !suffix.is_empty() && !suffix.contains('*') && !suffix.contains('?') && !suffix.contains('[') {
        return Self::Suffix(suffix.to_string());
      }
    }

    // Complex glob fallback
    Self::Glob(Glob::new(pattern).expect("invalid glob pattern").compile_matcher())
  }

  /// Check if a value matches this pattern.
  #[inline]
  pub fn is_match(&self, value: &str) -> bool {
    match self {
      Self::Exact(s) => value == s,
      Self::Prefix(p) => value.starts_with(p),
      Self::Suffix(s) => value.ends_with(s),
      Self::Glob(g) => g.is_match(value),
    }
  }
}

impl fmt::Debug for PatternMatcher {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Exact(s) => write!(f, "Exact({s:?})"),
      Self::Prefix(p) => write!(f, "Prefix({p:?})"),
      Self::Suffix(s) => write!(f, "Suffix({s:?})"),
      Self::Glob(_) => write!(f, "Glob(...)"),
    }
  }
}

#[cfg(test)]
#[path = "pattern_matcher_test.rs"]
mod pattern_matcher_test;
