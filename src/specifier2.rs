use {
  crate::{
    cli::UpdateTarget,
    specifier::{parser, semver_range::SemverRange},
  },
  node_semver::{Range, Version},
  std::{cell::RefCell, collections::HashMap, rc::Rc},
};

#[cfg(test)]
#[path = "specifier2_test.rs"]
mod specifier2_test;

thread_local! {
  static SPECIFIER_CACHE: RefCell<HashMap<String, Rc<Specifier2>>> = RefCell::new(HashMap::new());
  static RANGE_CACHE: RefCell<HashMap<String, Rc<Range>>> = RefCell::new(HashMap::new());
  static VERSION_CACHE: RefCell<HashMap<String, Rc<Version>>> = RefCell::new(HashMap::new());
}

const ALIAS: &str = "alias";
const RANGE_COMPLEX: &str = "range-complex";
const EXACT: &str = "exact";
const FILE: &str = "file";
const GIT: &str = "git";
const LATEST: &str = "latest";
const MAJOR: &str = "major";
const MINOR: &str = "minor";
const MISSING: &str = "missing";
const RANGE: &str = "range";
const RANGE_MAJOR: &str = "range-major";
const RANGE_MINOR: &str = "range-minor";
const TAG: &str = "tag";
const UNSUPPORTED: &str = "unsupported";
const URL: &str = "url";
const WORKSPACE_PROTOCOL: &str = "workspace-protocol";

#[derive(Debug, PartialEq)]
pub enum Specifier2 {
  Alias(String),             // "npm:foo@1.2.3"
  ComplexSemver(String),     // ">=1.2.3 <2.0.0"
  Exact(String),             // "1.2.3"
  File(String),              // "file:../path"
  Git(String),               // "github:user/repo#v1.2.3"
  Latest(String),            // "latest", "*"
  Major(String),             // "1"
  Minor(String),             // "1.2"
  None,                      // Missing .version property
  Range(String),             // "~1.2.3"
  RangeMajor(String),        // "^1"
  RangeMinor(String),        // "~1.2"
  Tag(String),               // "alpha", "beta"
  Unsupported(String),       // "}wat{"
  Url(String),               // "https://example.com/package.tgz"
  WorkspaceProtocol(String), // "workspace:^"
}

// Creation Methods
impl Specifier2 {
  /// Get or create a reference to a single Specifier which represents the given
  /// version string
  pub fn new(value: &str) -> Rc<Self> {
    SPECIFIER_CACHE.with(|cache| {
      let mut cache = cache.borrow_mut();
      match cache.get(value) {
        Some(rc) => rc.clone(),
        None => {
          let rc = Rc::new(Self::create(value));
          cache.insert(value.to_string(), rc.clone());
          rc
        }
      }
    })
  }

  /// Create a new Specifier for the given version string
  fn create(value: &str) -> Self {
    if value.is_empty() {
      return Self::None;
    }
    if parser::is_exact(value) {
      return Self::Exact(value.to_string());
    }
    if parser::is_range(value) {
      return Self::Range(value.to_string());
    }
    if parser::is_latest(value) {
      return Self::Latest(value.to_string());
    }
    if parser::is_major(value) {
      return Self::Major(value.to_string());
    }
    if parser::is_minor(value) {
      return Self::Minor(value.to_string());
    }
    if parser::is_range_major(value) {
      return Self::RangeMajor(value.to_string());
    }
    if parser::is_range_minor(value) {
      return Self::RangeMinor(value.to_string());
    }
    if parser::is_complex_range(value) {
      return Self::ComplexSemver(value.to_string());
    }
    let first_char = value.chars().next().unwrap_or('\0');
    if first_char == 'w' && value.starts_with("workspace:") {
      return Self::WorkspaceProtocol(value.to_string());
    }
    if parser::is_tag(value) {
      return Self::Tag(value.to_string());
    }
    if first_char == 'n' && value.starts_with("npm:") {
      return Self::Alias(value.to_string());
    }
    if parser::is_git(value) {
      return Self::Git(value.to_string());
    }
    if first_char == 'f' && value.starts_with("file:") {
      return Self::File(value.to_string());
    }
    if first_char == 'h' && (value.starts_with("http://") || value.starts_with("https://")) {
      return Self::Url(value.to_string());
    }
    Self::Unsupported(value.to_string())
  }
}

// Getters
impl Specifier2 {
  /// Get the "specifier type" name as used in config files.
  ///
  /// Examples:
  /// - "npm:lodash@^4.17.21" -> "alias"
  /// - "^16.11.10" -> "range"
  /// - ""1.2.3" -> "exact"
  pub fn get_config_identifier(&self) -> &'static str {
    match self {
      Self::Alias(_) => ALIAS,
      Self::ComplexSemver(_) => RANGE_COMPLEX,
      Self::Exact(_) => EXACT,
      Self::File(_) => FILE,
      Self::Git(_) => GIT,
      Self::Latest(_) => LATEST,
      Self::Major(_) => MAJOR,
      Self::Minor(_) => MINOR,
      Self::None => MISSING,
      Self::Range(_) => RANGE,
      Self::RangeMajor(_) => RANGE_MAJOR,
      Self::RangeMinor(_) => RANGE_MINOR,
      Self::Tag(_) => TAG,
      Self::Unsupported(_) => UNSUPPORTED,
      Self::Url(_) => URL,
      Self::WorkspaceProtocol(_) => WORKSPACE_PROTOCOL,
    }
  }

  /// If the current variant is a Specifier::Alias, returns the name of the npm
  /// dependency which is being aliased.
  ///
  /// Examples:
  /// - "npm:lodash@^4.17.21" -> Some("lodash")
  /// - "^16.11.10" -> None
  /// - "npm:express" -> None
  pub fn get_alias_name(&self) -> Option<&str> {
    match self {
      Self::Alias(raw) => {
        if let Some(after_prefix) = raw.strip_prefix("npm:") {
          if let Some(at_pos) = after_prefix.rfind('@') {
            // Check if this @ is actually a version separator (not part of scoped name)
            if at_pos > 0 && !after_prefix[..at_pos].is_empty() {
              // There's a version specifier, extract name before @
              Some(&after_prefix[..at_pos])
            } else {
              // The @ is part of a scoped package name, no version
              Some(after_prefix)
            }
          } else {
            // No @ found, entire string after npm: is the name
            Some(after_prefix)
          }
        } else {
          None
        }
      }
      _ => None,
    }
  }

  /// Returns the semver version number of the npm dependency in the format
  /// MAJOR.MINOR.PATCH, if it exists. Only the version number is returned,
  /// excluding any semver ranges if present.
  ///
  /// Examples:
  /// - "npm:lodash@^4.17.21" -> Some("4.17.21")
  /// - ">=16.11.10" -> Some("16.11.10")
  /// - "npm:express" -> None
  pub fn get_semver_number(&self) -> Option<&str> {
    match self {
      Self::Alias(raw) => {
        if let Some(after_prefix) = raw.strip_prefix("npm:") {
          if let Some(at_pos) = after_prefix.rfind('@') {
            // Check if this @ is actually a version separator (not part of scoped name)
            if at_pos > 0 && !after_prefix[..at_pos].is_empty() {
              // There's a version specifier after @
              let version_part = &after_prefix[at_pos + 1..];
              if !version_part.is_empty() {
                Some(version_part)
              } else {
                None
              }
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      }
      Self::Exact(version) => Some(version),
      Self::Range(raw) => {
        // Extract version from range patterns like "^1.2.3", "~1.2.3", ">=1.2.3"
        if let Some(stripped) = raw.strip_prefix('^') {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix('~') {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix(">=") {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix("<=") {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix('>') {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix('<') {
          Some(stripped)
        } else {
          None
        }
      }
      Self::Major(version) => Some(version),
      Self::Minor(version) => Some(version),
      Self::RangeMajor(raw) => {
        // Extract version from range patterns like "^1", "~1"
        if let Some(stripped) = raw.strip_prefix('^') {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix('~') {
          Some(stripped)
        } else {
          None
        }
      }
      Self::RangeMinor(raw) => {
        // Extract version from range patterns like "^1.2", "~1.2", ">=1.2"
        if let Some(stripped) = raw.strip_prefix('^') {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix('~') {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix(">=") {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix("<=") {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix('>') {
          Some(stripped)
        } else if let Some(stripped) = raw.strip_prefix('<') {
          Some(stripped)
        } else {
          None
        }
      }
      _ => None,
    }
  }

  pub fn get_node_version(&self) -> Option<Version> {
    todo!()
  }

  pub fn get_node_range(&self) -> Option<Range> {
    todo!()
  }

  pub fn get_semver_range(&self) -> Option<SemverRange> {
    todo!()
  }
}

// Mapping Methods
impl Specifier2 {
  pub fn with_range(&self, range: &SemverRange) -> Option<String> {
    let range_str = range.unwrap();

    match self {
      Self::Alias(raw) => {
        if let Some(after_prefix) = raw.strip_prefix("npm:") {
          if let Some(at_pos) = after_prefix.rfind('@') {
            // Check if this @ is actually a version separator (not part of scoped name)
            if at_pos > 0 && !after_prefix[..at_pos].is_empty() {
              // There's a version specifier, extract and apply range
              let package_name = &after_prefix[..at_pos];
              let version_part = &after_prefix[at_pos + 1..];
              if !version_part.is_empty() {
                // Extract the base version number from the current version
                let base_version = if let Some(stripped) = version_part.strip_prefix('^') {
                  stripped
                } else if let Some(stripped) = version_part.strip_prefix('~') {
                  stripped
                } else if let Some(stripped) = version_part.strip_prefix(">=") {
                  stripped
                } else if let Some(stripped) = version_part.strip_prefix("<=") {
                  stripped
                } else if let Some(stripped) = version_part.strip_prefix('>') {
                  stripped
                } else if let Some(stripped) = version_part.strip_prefix('<') {
                  stripped
                } else {
                  version_part
                };
                Some(format!("npm:{package_name}@{range_str}{base_version}"))
              } else {
                None
              }
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      }
      Self::Exact(version) => Some(format!("{range_str}{version}")),
      Self::Range(raw) => {
        // Extract the base version and apply new range
        let base_version = if let Some(stripped) = raw.strip_prefix('^') {
          stripped
        } else if let Some(stripped) = raw.strip_prefix('~') {
          stripped
        } else if let Some(stripped) = raw.strip_prefix(">=") {
          stripped
        } else if let Some(stripped) = raw.strip_prefix("<=") {
          stripped
        } else if let Some(stripped) = raw.strip_prefix('>') {
          stripped
        } else if let Some(stripped) = raw.strip_prefix('<') {
          stripped
        } else {
          raw
        };
        Some(format!("{range_str}{base_version}"))
      }
      Self::Major(version) => Some(format!("{range_str}{version}")),
      Self::Minor(version) => Some(format!("{range_str}{version}")),
      Self::RangeMajor(raw) => {
        // Extract the base version and apply new range
        let base_version = if let Some(stripped) = raw.strip_prefix('^') {
          stripped
        } else if let Some(stripped) = raw.strip_prefix('~') {
          stripped
        } else {
          raw
        };
        Some(format!("{range_str}{base_version}"))
      }
      Self::RangeMinor(raw) => {
        // Extract the base version and apply new range
        let base_version = if let Some(stripped) = raw.strip_prefix('^') {
          stripped
        } else if let Some(stripped) = raw.strip_prefix('~') {
          stripped
        } else if let Some(stripped) = raw.strip_prefix(">=") {
          stripped
        } else if let Some(stripped) = raw.strip_prefix("<=") {
          stripped
        } else if let Some(stripped) = raw.strip_prefix('>') {
          stripped
        } else if let Some(stripped) = raw.strip_prefix('<') {
          stripped
        } else {
          raw
        };
        Some(format!("{range_str}{base_version}"))
      }
      Self::WorkspaceProtocol(raw) => {
        if let Some(after_prefix) = raw.strip_prefix("workspace:") {
          if after_prefix == "*" {
            Some(format!("workspace:{range_str}"))
          } else {
            // Apply range to existing version after workspace:
            let base_version = if let Some(stripped) = after_prefix.strip_prefix('^') {
              stripped
            } else if let Some(stripped) = after_prefix.strip_prefix('~') {
              stripped
            } else {
              after_prefix
            };
            if base_version.is_empty() {
              Some(format!("workspace:{range_str}"))
            } else {
              Some(format!("workspace:{range_str}{base_version}"))
            }
          }
        } else {
          None
        }
      }
      _ => None,
    }
  }

  /// Return a new `Specifier` with the given semver version number applied to
  /// it when it is possible to do so, otherwise the same `Specifier` is
  /// returned. The range is also changed.
  pub fn with_node_version(self, node_version: &Version) -> Self {
    todo!()
  }
}

// Comparison Methods
impl Specifier2 {
  pub fn has_same_release_channel_as(&self, other: &Self) -> bool {
    todo! {}
  }

  pub fn has_same_version_number_as(&self, other: &Self) -> bool {
    todo! {}
  }

  pub fn has_semver_range_of(&self, range: &SemverRange) -> bool {
    todo!()
  }

  pub fn is_eligible_update_for(&self, other: &Self, target: &UpdateTarget) -> bool {
    todo! {}
  }

  pub fn is_older_than(&self, other: &Self) -> bool {
    todo! {}
  }

  pub fn is_older_than_by_minor(&self, other: &Self) -> bool {
    todo! {}
  }

  pub fn is_older_than_by_patch(&self, other: &Self) -> bool {
    todo! {}
  }

  pub fn is_workspace_protocol(&self) -> bool {
    matches!(self, Self::WorkspaceProtocol(_))
  }

  pub fn satisfies(&self, range: &Range) -> bool {
    todo! {}
  }

  pub fn satisfies_all(&self, ranges: &[Range]) -> bool {
    todo! {}
  }
}
