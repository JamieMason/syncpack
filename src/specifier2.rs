use {
  crate::{
    cli::UpdateTarget,
    specifier::{parser, semver_range::SemverRange},
    specifier2::{
      alias::Alias, complex_semver::ComplexSemver, exact::Exact, file::File, git::Git, latest::Latest, major::Major, minor::Minor,
      range::Range, range_major::RangeMajor, range_minor::RangeMinor, tag::Tag, url::Url, workspace_protocol::WorkspaceProtocol,
    },
  },
  std::{cell::RefCell, collections::HashMap, rc::Rc},
};

mod alias;
mod complex_semver;
mod exact;
mod file;
mod git;
mod latest;
mod major;
mod minor;
mod range;
mod range_major;
mod range_minor;
#[cfg(test)]
#[path = "specifier2_test.rs"]
mod specifier2_test;
mod tag;
mod url;
mod workspace_protocol;

thread_local! {
  static SPECIFIER_CACHE: RefCell<HashMap<String, Rc<Specifier2>>> = RefCell::new(HashMap::new());
  static RANGE_CACHE: RefCell<HashMap<String, Rc<node_semver::Range>>> = RefCell::new(HashMap::new());
  static VERSION_CACHE: RefCell<HashMap<String, Rc<node_semver::Version>>> = RefCell::new(HashMap::new());
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
  Alias(alias::Alias),                                      // "npm:foo@1.2.3"
  ComplexSemver(complex_semver::ComplexSemver),             // ">=1.2.3 <2.0.0"
  Exact(exact::Exact),                                      // "1.2.3"
  File(file::File),                                         // "file:../path"
  Git(git::Git),                                            // "github:user/repo#v1.2.3"
  Latest(latest::Latest),                                   // "latest", "*"
  Major(major::Major),                                      // "1"
  Minor(minor::Minor),                                      // "1.2"
  None,                                                     // Missing .version property
  Range(range::Range),                                      // "~1.2.3"
  RangeMajor(range_major::RangeMajor),                      // "^1"
  RangeMinor(range_minor::RangeMinor),                      // "~1.2"
  Tag(tag::Tag),                                            // "alpha", "beta"
  Unsupported(String),                                      // "}wat{"
  Url(url::Url),                                            // "https://example.com/package.tgz"
  WorkspaceProtocol(workspace_protocol::WorkspaceProtocol), // "workspace:^", "workspace:*", "workspace:~", "workspace:^1.2.3"
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
      return Exact::new(value);
    }
    if parser::is_range(value) {
      return Range::new(value);
    }
    if parser::is_latest(value) {
      return Latest::new(value);
    }
    if parser::is_major(value) {
      return Major::new(value);
    }
    if parser::is_minor(value) {
      return Minor::new(value);
    }
    if parser::is_range_major(value) {
      return RangeMajor::new(value);
    }
    if parser::is_range_minor(value) {
      return RangeMinor::new(value);
    }
    if parser::is_complex_range(value) {
      return ComplexSemver::new(value);
    }
    let first_char = value.chars().next().unwrap_or('\0');
    if first_char == 'w' && value.starts_with("workspace:") {
      return WorkspaceProtocol::new(value);
    }
    if parser::is_tag(value) {
      return Tag::new(value);
    }
    if first_char == 'n' && value.starts_with("npm:") {
      return Alias::new(value);
    }
    if parser::is_git(value) {
      return Git::new(value);
    }
    if first_char == 'f' && value.starts_with("file:") {
      return file::File::new(value);
    }
    if first_char == 'h' && (value.starts_with("http://") || value.starts_with("https://")) {
      return url::Url::new(value);
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
      Self::Alias(alias) => Some(&alias.name),
      _ => None,
    }
  }

  /// Returns the semver version number of the specifier, if it exists. Only the
  /// version number is returned, WITHOUT semver range characters.
  ///
  /// Examples:
  /// - "npm:lodash@^4.17.21" -> Some("4.17.21")
  /// - ">=16.11.10" -> Some("16.11.10")
  /// - "npm:express" -> None
  pub fn get_semver_number(&self) -> Option<&str> {
    match self {
      Self::Alias(s) => s.semver_string.as_ref().map(|s| strip_semver_range(s)),
      Self::Exact(s) => Some(&s.raw),
      Self::Major(s) => Some(&s.raw),
      Self::Minor(s) => Some(&s.raw),
      Self::Range(s) => Some(strip_semver_range(&s.raw)),
      Self::RangeMajor(s) => Some(strip_semver_range(&s.raw)),
      Self::RangeMinor(s) => Some(strip_semver_range(&s.raw)),
      Self::WorkspaceProtocol(ws) => ws.semver_number.as_deref(),
      Self::ComplexSemver(_)
      | Self::File(_)
      | Self::Git(_)
      | Self::Latest(_)
      | Self::None
      | Self::Tag(_)
      | Self::Unsupported(_)
      | Self::Url(_) => None,
    }
  }

  /// Get or create a reference to a single `node_semver::Version` created from
  /// the semver version number of the specifier, WITHOUT semver range
  /// characters.
  ///
  /// Examples:
  /// - "1.2.3" → Rc(Version("1.2.3"))
  /// - "^1.2.3" → Rc(Version("1.2.3"))
  pub fn get_node_version(&self) -> Option<Rc<node_semver::Version>> {
    todo!()
  }

  /// Get or create a reference to a single `node_semver::Range` created from
  /// the semver version number of the specifier, WITH semver range
  /// characters.
  ///
  /// Examples:
  /// - "1.2.3" → Rc(Range("1.2.3"))
  /// - "^1.2.3" → Rc(Range("^1.2.3"))
  pub fn get_node_range(&self) -> Option<Rc<node_semver::Range>> {
    todo!()
  }

  /// Returns the type of semver range used in the specifier, if one exists.
  pub fn get_semver_range(&self) -> Option<SemverRange> {
    todo!()
  }
}

/// Remove semver range characters from the start of a semver version number
pub fn strip_semver_range(value: &str) -> &str {
  ["^", "~", ">=", "<=", ">", "<"]
    .into_iter()
    .find_map(|prefix| value.strip_prefix(prefix))
    .unwrap_or(value)
}

/// Remove workspace: from the start of a specifier
fn strip_workspace_protocol(value: &str) -> &str {
  value.strip_prefix("workspace:").unwrap_or(value)
}

// Mapping Methods
impl Specifier2 {
  /// Get or create a reference to a single Specifier which represents the
  /// semver version number of this Specifier with the given semver range
  /// applied to it, if a valid semver version number is present and the given
  /// range is compatible with the current specifier type.
  ///
  /// Examples:
  /// - "^1.2.3" + "" → Some("1.2.3")
  /// - "1.2.3" + "^" → Some("^1.2.3")
  /// - "npm:@scope/package@1.2.3" + "^" → Some("npm:@scope/package@^1.2.3")
  /// - "*" + "^" → None
  /// - "npm:@scope/package@1.2.3" + "*" → Some("npm:@scope/package")
  pub fn with_range(&self, range: &SemverRange) -> Option<Rc<Self>> {
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
    .map(|value| Self::new(&value))
  }

  /// Get or create a reference to a single Specifier which represents the
  /// semver range and specifier type of this Specifier with the given semver
  /// version number applied to it, if such a combination is valid and
  /// compatible.
  ///
  /// Examples:
  /// - "workspace:^1.2.3" + "2.3.4" -> Some("workspace:^2.3.4")
  /// - "^1.2.3" + "2.3.4" -> Some("^2.3.4")
  /// - "*" + "1.2.3" -> None
  /// - "npm:@scope/package@1.2.3" + "2.3.4" → Some("npm:@scope/package@2.3.4")
  pub fn with_node_version(self, _node_version: &node_semver::Version) -> Option<Rc<Self>> {
    todo!()
  }
}

// Comparison Methods
impl Specifier2 {
  /// Check if this specifier and another have the same pre-release channel.
  ///
  /// Examples:
  /// - "1.2.3-alpha.1" and "1.2.4-alpha.2" → true
  /// - "1.2.3-alpha.1" and "1.2.4-beta.1" → false
  /// - "1.2.3" and "1.2.4" → true (both stable)
  pub fn has_same_release_channel_as(&self, _other: &Self) -> bool {
    todo! {}
  }

  /// Regardless of the range, does this specifier and the other both have the
  /// same version number (e.g. "1.4.1")?
  ///
  /// Examples:
  /// - "^1.4.1" and "~1.4.1" → true
  /// - "1.4.1" and "^1.4.1" → true
  /// - "1.4.1" and "1.4.2" → false
  pub fn has_same_version_number_as(&self, _other: &Self) -> bool {
    todo! {}
  }

  /// Check if this specifier uses the given semver range type.
  ///
  /// Examples:
  /// - "^1.2.3" with SemverRange::Minor → true
  /// - "~1.2.3" with SemverRange::Minor → false
  /// - "1.2.3" with SemverRange::Exact → true
  pub fn has_semver_range_of(&self, _range: &SemverRange) -> bool {
    todo!()
  }

  /// Is this specifier eligible to update the given specifier based on the
  /// given target constraint?
  ///
  /// Examples:
  /// - "2.0.0" can update "1.0.0" with UpdateTarget::Latest → true
  /// - "1.1.0" can update "1.0.0" with UpdateTarget::Minor → true
  /// - "2.0.0" can update "1.0.0" with UpdateTarget::Minor → false
  /// - "1.2.3" can update "1.2.2" with UpdateTarget::Patch → true
  pub fn is_eligible_update_for(&self, _other: &Self, _target: &UpdateTarget) -> bool {
    todo! {}
  }

  /// Check if this specifier represents an older version than the other.
  ///
  /// Examples:
  /// - "1.0.0" compared to "2.0.0" → true
  /// - "2.0.0" compared to "1.0.0" → false
  /// - "1.0.0" compared to "1.0.0" → false
  pub fn is_older_than(&self, _other: &Self) -> bool {
    todo! {}
  }

  /// Is this specifier on the same major version, but otherwise older?
  ///
  /// Examples:
  /// - "1.0.0" compared to "1.1.0" → true
  /// - "1.0.1" compared to "1.1.0" → true
  /// - "1.0.0" compared to "2.0.0" → false
  /// - "1.1.0" compared to "1.0.0" → false
  pub fn is_older_than_by_minor(&self, _other: &Self) -> bool {
    todo! {}
  }

  /// Is this specifier on the same major and minor version, but otherwise
  /// older?
  ///
  /// Examples:
  /// - "1.0.0" compared to "1.0.1" → true
  /// - "1.0.0" compared to "1.1.0" → false
  /// - "1.0.1" compared to "1.0.0" → false
  pub fn is_older_than_by_patch(&self, _other: &Self) -> bool {
    todo! {}
  }

  /// Check if this specifier uses the workspace protocol.
  ///
  /// Examples:
  /// - "workspace:^1.0.0" → true
  /// - "workspace:*" → true
  /// - "^1.0.0" → false
  pub fn is_workspace_protocol(&self) -> bool {
    matches!(self, Self::WorkspaceProtocol(_))
  }

  /// Does this specifier match the given range?
  ///
  /// Examples:
  /// - "1.2.3" satisfies Range("^1.0.0") → true
  /// - "2.0.0" satisfies Range("^1.0.0") → false
  /// - "0.9.0" satisfies Range("^1.0.0") → false
  pub fn satisfies(&self, _range: &node_semver::Range) -> bool {
    todo! {}
  }

  /// Does this specifier match every one of the given ranges?
  ///
  /// Examples:
  /// - "1.2.3" satisfies [Range("^1.0.0"), Range("~1.2.0")] → true
  /// - "1.3.0" satisfies [Range("^1.0.0"), Range("~1.2.0")] → false
  /// - "2.0.0" satisfies [Range("^1.0.0"), Range("~1.2.0")] → false
  pub fn satisfies_all(&self, _ranges: &[node_semver::Range]) -> bool {
    todo! {}
  }
}
