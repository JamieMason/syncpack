use {
  crate::{
    cli::UpdateTarget,
    semver_range::SemverRange,
    specifier::{
      alias::Alias, complex_semver::ComplexSemver, exact::Exact, git::Git, latest::Latest, major::Major, minor::Minor, range::Range,
      range_major::RangeMajor, range_minor::RangeMinor, tag::Tag, workspace_protocol::WorkspaceProtocol,
    },
  },
  std::{cell::RefCell, collections::HashMap, rc::Rc},
};

pub mod alias;
pub mod complex_semver;
pub mod exact;
pub mod file;
#[cfg(test)]
#[path = "specifier/get_alias_name_test.rs"]
mod get_alias_name_test;
#[cfg(test)]
#[path = "specifier/get_node_range_test.rs"]
mod get_node_range_test;
#[cfg(test)]
#[path = "specifier/get_semver_number_test.rs"]
mod get_semver_number_test;
pub mod git;
#[cfg(test)]
#[path = "specifier/has_same_release_channel_as_test.rs"]
mod has_same_release_channel_as_test;
#[cfg(test)]
#[path = "specifier/has_same_version_number_as_test.rs"]
mod has_same_version_number_as_test;
#[cfg(test)]
#[path = "specifier/has_semver_range_of_test.rs"]
mod has_semver_range_of_test;
#[cfg(test)]
#[path = "specifier/is_eligible_update_for_test.rs"]
mod is_eligible_update_for_test;
#[cfg(test)]
#[path = "specifier/is_older_than_by_minor_test.rs"]
mod is_older_than_by_minor_test;
#[cfg(test)]
#[path = "specifier/is_older_than_by_patch_test.rs"]
mod is_older_than_by_patch_test;
#[cfg(test)]
#[path = "specifier/is_older_than_test.rs"]
mod is_older_than_test;
pub mod latest;
pub mod major;
pub mod minor;
#[cfg(test)]
#[path = "specifier/new_test.rs"]
mod new_test;
#[cfg(test)]
#[path = "specifier/ordering_test.rs"]
mod ordering_test;
pub mod parser;
pub mod range;
pub mod range_major;
pub mod range_minor;
pub mod regexes;
#[cfg(test)]
#[path = "specifier/resolve_workspace_protocol_test.rs"]
mod resolve_workspace_protocol_test;
#[cfg(test)]
#[path = "specifier/satisfies_all_test.rs"]
mod satisfies_all_test;
#[cfg(test)]
#[path = "specifier/satisfies_test.rs"]
mod satisfies_test;
pub mod tag;
pub mod url;
#[cfg(test)]
#[path = "specifier/with_node_version_test.rs"]
mod with_node_version_test;
#[cfg(test)]
#[path = "specifier/with_range_test.rs"]
mod with_range_test;
pub mod workspace_protocol;
#[cfg(test)]
#[path = "specifier/workspace_protocol_test.rs"]
mod workspace_protocol_test;
pub mod workspace_specifier;
#[cfg(test)]
#[path = "specifier/workspace_specifier_test.rs"]
mod workspace_specifier_test;

thread_local! {
  static SPECIFIER_CACHE: RefCell<HashMap<String, Rc<Specifier>>> = RefCell::new(HashMap::new());
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

/// A huge number used to complete shorthand semver versions during ordering and
/// comparison, such as:
///
/// Examples:
/// - "1" -> "1.999999.999999"
/// - "1.2" -> "1.2.999999"
pub const HUGE: u64 = 999999;

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

#[derive(Debug, PartialEq)]
pub enum Specifier {
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
impl Specifier {
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

  /// Get or create a reference to a single Version which represents the given
  /// version string
  pub fn new_node_version(value: &str) -> Option<Rc<node_semver::Version>> {
    VERSION_CACHE.with(|cache| {
      let mut cache = cache.borrow_mut();
      match cache.get(value) {
        Some(rc) => Some(rc.clone()),
        None => node_semver::Version::parse(value).ok().map(|range| {
          let rc = Rc::new(range);
          cache.insert(value.to_string(), rc.clone());
          rc
        }),
      }
    })
  }

  /// Get or create a reference to a single Range which represents the given
  /// version string
  pub fn new_node_range(value: &str) -> Option<Rc<node_semver::Range>> {
    RANGE_CACHE.with(|cache| {
      let mut cache = cache.borrow_mut();
      match cache.get(value) {
        Some(rc) => Some(rc.clone()),
        None => node_semver::Range::parse(value).ok().map(|range| {
          let rc = Rc::new(range);
          cache.insert(value.to_string(), rc.clone());
          rc
        }),
      }
    })
  }

  /// Check if the given version string is a valid semver version
  pub fn is_valid_semver(value: &str) -> bool {
    Self::new_node_range(value).is_some()
  }

  /// Create a new Specifier for the given version string
  pub(crate) fn create(value: &str) -> Self {
    if value.is_empty() {
      return Self::None;
    }
    if parser::is_exact(value) {
      return Exact::create(value);
    }
    if parser::is_range(value) {
      return Range::create(value);
    }
    if parser::is_latest(value) {
      return Latest::create(value);
    }
    if parser::is_major(value) {
      return Major::create(value);
    }
    if parser::is_minor(value) {
      return Minor::create(value);
    }
    if parser::is_range_major(value) {
      return RangeMajor::create(value);
    }
    if parser::is_range_minor(value) {
      return RangeMinor::create(value);
    }
    if parser::is_complex_range(value) {
      return ComplexSemver::create(value);
    }
    let first_char = value.chars().next().unwrap_or('\0');
    if first_char == 'w' && value.starts_with("workspace:") {
      return WorkspaceProtocol::create(value);
    }
    if parser::is_tag(value) {
      return Tag::create(value);
    }
    if first_char == 'n' && value.starts_with("npm:") {
      return Alias::create(value);
    }
    if parser::is_git(value) {
      return Git::create(value);
    }
    if first_char == 'f' && value.starts_with("file:") {
      return file::File::create(value);
    }
    if first_char == 'h' && (value.starts_with("http://") || value.starts_with("https://")) {
      return url::Url::create(value);
    }
    Self::Unsupported(value.to_string())
  }
}

// Getters
impl Specifier {
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
      Self::Alias(s) => s.inner_specifier.get_semver_number(),
      Self::Exact(s) => Some(&s.raw),
      Self::Major(s) => Some(&s.raw),
      Self::Minor(s) => Some(&s.raw),
      Self::Range(s) => Some(strip_semver_range(&s.raw)),
      Self::RangeMajor(s) => Some(strip_semver_range(&s.raw)),
      Self::RangeMinor(s) => Some(strip_semver_range(&s.raw)),
      Self::WorkspaceProtocol(s) => s.as_resolved().and_then(|spec| spec.get_semver_number()),
      Self::Git(s) => s.semver_number.as_deref(),
      Self::ComplexSemver(_) | Self::File(_) | Self::Latest(_) | Self::None | Self::Tag(_) | Self::Unsupported(_) | Self::Url(_) => None,
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
    match self {
      Self::Alias(s) => s.inner_specifier.get_node_version(),
      Self::Git(s) => s.node_version.clone(),
      Self::Exact(s) => Some(s.node_version.clone()),
      Self::Latest(s) => Some(s.node_version.clone()),
      Self::Major(s) => Some(s.node_version.clone()),
      Self::Minor(s) => Some(s.node_version.clone()),
      Self::Range(s) => Some(s.node_version.clone()),
      Self::RangeMajor(s) => Some(s.node_version.clone()),
      Self::RangeMinor(s) => Some(s.node_version.clone()),
      Self::WorkspaceProtocol(s) => s.as_resolved().and_then(|spec| spec.get_node_version()),
      Self::ComplexSemver(_) | Self::File(_) | Self::None | Self::Tag(_) | Self::Unsupported(_) | Self::Url(_) => None,
    }
  }

  /// Get or create a reference to a single `node_semver::Range` created from
  /// the semver version number of the specifier, WITH semver range
  /// characters.
  ///
  /// Examples:
  /// - "1.2.3" → Rc(Range("1.2.3"))
  /// - "^1.2.3" → Rc(Range("^1.2.3"))
  pub fn get_node_range(&self) -> Option<Rc<node_semver::Range>> {
    match self {
      Self::Alias(s) => s.inner_specifier.get_node_range(),
      Self::ComplexSemver(s) => Some(s.node_range.clone()),
      Self::Git(s) => s.node_range.clone(),
      Self::Range(s) => Some(s.node_range.clone()),
      Self::RangeMajor(s) => Some(s.node_range.clone()),
      Self::RangeMinor(s) => Some(s.node_range.clone()),
      Self::WorkspaceProtocol(s) => {
        // Try resolved specifier first, otherwise check for "*" (RangeOnly(Any))
        s.as_resolved().and_then(|spec| spec.get_node_range()).or_else(|| {
          use crate::{semver_range::SemverRange, specifier::workspace_specifier::WorkspaceSpecifier};
          match &s.inner_specifier {
            WorkspaceSpecifier::RangeOnly(SemverRange::Any) => {
              // "workspace:*" → ">=0.0.0 <=999999.999999.999999"
              let huge = HUGE.to_string();
              let range_str = format!(">=0.0.0 <={huge}.{huge}.{huge}");
              Self::new_node_range(&range_str)
            }
            _ => None,
          }
        })
      }
      Self::Exact(s) => Some(s.node_range.clone()),
      Self::Latest(_) => {
        // "*", "latest", "x" → ">=0.0.0 <=999999.999999.999999"
        let huge = HUGE.to_string();
        let range_str = format!(">=0.0.0 <={huge}.{huge}.{huge}");
        Self::new_node_range(&range_str)
      }
      Self::Major(m) => {
        // "1" → ">=1.0.0 <2.0.0"
        let range_str = format!(">={}.0.0 <{}.0.0", m.raw, m.raw.parse::<u64>().ok()? + 1);
        Self::new_node_range(&range_str)
      }
      Self::Minor(m) => {
        // "1.2" → ">=1.2.0 <1.3.0"
        let parts: Vec<&str> = m.raw.split('.').collect();
        if parts.len() != 2 {
          return None;
        }
        let major = parts[0];
        let minor = parts[1].parse::<u64>().ok()?;
        let range_str = format!(">={}.0 <{}.{}.0", m.raw, major, minor + 1);
        Self::new_node_range(&range_str)
      }
      Self::File(_) | Self::None | Self::Tag(_) | Self::Unsupported(_) | Self::Url(_) => None,
    }
  }

  /// Returns the type of semver range used in the specifier, if one exists.
  pub fn get_semver_range(&self) -> Option<SemverRange> {
    match self {
      Self::Alias(s) => s.inner_specifier.get_semver_range(),
      Self::Git(s) => s.semver_range.clone(),
      Self::Latest(s) => Some(s.semver_range.clone()),
      Self::Range(s) => Some(s.semver_range.clone()),
      Self::RangeMajor(s) => Some(s.semver_range.clone()),
      Self::RangeMinor(s) => Some(s.semver_range.clone()),
      Self::WorkspaceProtocol(s) => match &s.inner_specifier {
        workspace_specifier::WorkspaceSpecifier::RangeOnly(r) => Some(r.clone()),
        workspace_specifier::WorkspaceSpecifier::Resolved(spec) => spec.get_semver_range(),
      },
      Self::Exact(_) => Some(SemverRange::Exact),
      Self::Major(_) => Some(SemverRange::Exact),
      Self::Minor(_) => Some(SemverRange::Exact),
      Self::ComplexSemver(_) | Self::File(_) | Self::None | Self::Tag(_) | Self::Unsupported(_) | Self::Url(_) => None,
    }
  }

  /// Get the raw specifier string that was used to create this Specifier.
  pub fn get_raw(&self) -> &str {
    match self {
      Self::Alias(s) => &s.raw,
      Self::ComplexSemver(s) => &s.raw,
      Self::Exact(s) => &s.raw,
      Self::File(s) => &s.raw,
      Self::Git(s) => &s.raw,
      Self::Latest(s) => &s.raw,
      Self::Major(s) => &s.raw,
      Self::Minor(s) => &s.raw,
      Self::Range(s) => &s.raw,
      Self::RangeMajor(s) => &s.raw,
      Self::RangeMinor(s) => &s.raw,
      Self::Tag(s) => &s.raw,
      Self::Url(s) => &s.raw,
      Self::WorkspaceProtocol(s) => &s.raw,
      Self::Unsupported(s) => s,
      Self::None => "",
    }
  }
}

// Mapping Methods
impl Specifier {
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
  /// - "^1" + "*" → "*"
  /// - "npm:foo@^1" + "*" → "npm:foo"
  /// - "workspace:^1" + "*" → "workspace:*"
  /// - "git@github.com:npm/cli.git#^1" + "*" → "git@github.com:npm/cli.git"
  pub fn with_range(&self, range: &SemverRange) -> Option<Rc<Self>> {
    if matches!(range, SemverRange::Any) {
      return match self {
        Self::Exact(_) | Self::Major(_) | Self::Minor(_) | Self::Range(_) | Self::RangeMajor(_) | Self::RangeMinor(_) => {
          Some(Self::new("*"))
        }
        Self::WorkspaceProtocol(_) => Some(Self::new("workspace:*")),
        Self::Alias(s) => {
          if s.inner_specifier.get_semver_number().is_some() {
            Some(Self::new(&format!("npm:{}", s.name)))
          } else {
            None
          }
        }
        Self::Git(s) => s.semver_number.as_ref().map(|_| Self::new(&s.origin)),
        Self::ComplexSemver(_) | Self::File(_) | Self::Latest(_) | Self::None | Self::Tag(_) | Self::Unsupported(_) | Self::Url(_) => None,
      };
    }

    let range_str = range.unwrap();

    match self {
      Self::Exact(s) => {
        let new_specifier = format!("{}{}", range_str, s.raw);
        Some(Self::new(&new_specifier))
      }
      Self::Major(s) => {
        let new_specifier = format!("{}{}", range_str, s.raw);
        Some(Self::new(&new_specifier))
      }
      Self::Minor(s) => {
        let new_specifier = format!("{}{}", range_str, s.raw);
        Some(Self::new(&new_specifier))
      }
      Self::Range(s) => {
        let new_specifier = format!("{}{}", range_str, s.semver_number);
        Some(Self::new(&new_specifier))
      }
      Self::RangeMajor(s) => {
        let new_specifier = format!("{}{}", range_str, s.semver_number);
        Some(Self::new(&new_specifier))
      }
      Self::RangeMinor(s) => {
        let new_specifier = format!("{}{}", range_str, s.semver_number);
        Some(Self::new(&new_specifier))
      }
      Self::Alias(s) => s
        .inner_specifier
        .with_range(range)
        .map(|new_inner| Self::new(&format!("npm:{}@{}", s.name, new_inner.get_raw()))),
      Self::WorkspaceProtocol(s) => {
        // If resolved, use the resolved specifier's version
        if let Some(resolved) = s.as_resolved() {
          if let Some(semver_number) = resolved.get_semver_number() {
            Some(Self::new(&format!("workspace:{range_str}{semver_number}")))
          } else {
            // Resolved but no version number (e.g., Latest)
            Some(Self::new(&format!("workspace:{range_str}")))
          }
        } else {
          // RangeOnly - just use the range prefix
          Some(Self::new(&format!("workspace:{range_str}")))
        }
      }
      Self::Git(s) => s.semver_number.as_ref().map(|semver_number| {
        if range_str.is_empty() {
          // Exact version: use #version
          Self::new(&format!("{}#{}", s.origin, semver_number))
        } else {
          // Range: use #semver:range+version
          Self::new(&format!("{}#semver:{}{}", s.origin, range_str, semver_number))
        }
      }),
      Self::ComplexSemver(_) | Self::File(_) | Self::Latest(_) | Self::None | Self::Tag(_) | Self::Unsupported(_) | Self::Url(_) => None,
    }
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
  /// - "npm:@scope/package@~1.2.3" + "2.3.4" → Some("npm:@scope/package@~2.3.4")
  pub fn with_node_version(&self, node_version: &node_semver::Version) -> Option<Rc<Self>> {
    let version_str = node_version.to_string();

    // Check if this is a padded HUGE value (999999) and omit it from output
    let is_huge_minor = node_version.minor == HUGE;
    let is_huge_patch = node_version.patch == HUGE;

    match self {
      Self::Exact(_) => Some(Self::new(&version_str)),

      Self::Major(_) => {
        // Extract just major, omit .999999.999999
        Some(Self::new(&format!("{}", node_version.major)))
      }

      Self::Minor(_) => {
        // Extract major.minor, omit .999999
        Some(Self::new(&format!("{}.{}", node_version.major, node_version.minor)))
      }

      Self::Range(r) => Some(Self::new(&format!("{}{}", r.semver_range.unwrap(), version_str))),

      Self::RangeMajor(r) => {
        // For ^1 or ~1, extract major only, omit .999999.999999
        Some(Self::new(&format!("{}{}", r.semver_range.unwrap(), node_version.major)))
      }

      Self::RangeMinor(r) => {
        // For ^1.2 or ~1.2, extract major.minor, omit .999999
        Some(Self::new(&format!(
          "{}{}.{}",
          r.semver_range.unwrap(),
          node_version.major,
          node_version.minor
        )))
      }

      Self::Alias(a) => a
        .inner_specifier
        .with_node_version(node_version)
        .map(|new_inner| Self::new(&format!("npm:{}@{}", a.name, new_inner.get_raw()))),

      Self::WorkspaceProtocol(wp) => {
        // Only works if there's a resolved specifier with version
        wp.as_resolved().and_then(|resolved| {
          resolved.get_semver_number().map(|semver_number| {
            let range_str = resolved.get_semver_range().map(|r| r.unwrap()).unwrap_or_default();

            // Check if original was shorthand
            let original_parts: Vec<&str> = semver_number.split('.').collect();
            let formatted_version = match original_parts.len() {
              1 if is_huge_minor && is_huge_patch => node_version.major.to_string(),
              2 if is_huge_patch => {
                format!("{}.{}", node_version.major, node_version.minor)
              }
              _ => version_str.clone(),
            };

            Self::new(&format!("workspace:{range_str}{formatted_version}"))
          })
        })
      }

      Self::Git(g) => {
        // Only works if there's a semver tag
        g.semver_number.as_ref().map(|_| {
          let range_str = g.semver_range.as_ref().map(|r| r.unwrap()).unwrap_or_else(String::new);

          // Check if original was shorthand
          let original_parts: Vec<&str> = g.semver_number.as_ref().unwrap().split('.').collect();
          let formatted_version = match original_parts.len() {
            1 if is_huge_minor && is_huge_patch => node_version.major.to_string(),
            2 if is_huge_patch => {
              format!("{}.{}", node_version.major, node_version.minor)
            }
            _ => version_str.clone(),
          };

          // Rebuild git URL with new version
          // Note: Output format is always origin#range+version, not
          // origin#semver:range+version
          Self::new(&format!("{}#{}{}", g.origin, range_str, formatted_version))
        })
      }

      // These types don't have semver versions, return None
      Self::ComplexSemver(_) | Self::File(_) | Self::Latest(_) | Self::None | Self::Tag(_) | Self::Unsupported(_) | Self::Url(_) => None,
    }
  }
}

// Comparison Methods
impl Specifier {
  /// Check if this specifier has a version that should be used for comparison.
  /// Returns false for Latest (which has HUGE version only for sorting purposes).
  fn has_comparable_version(&self) -> bool {
    match self {
      Self::Latest(_) => false,
      Self::Alias(a) => !matches!(&*a.inner_specifier, Self::Latest(_)) && a.inner_specifier.get_node_version().is_some(),
      _ => self.get_node_version().is_some(),
    }
  }

  /// Check if this specifier and another have the same pre-release channel.
  ///
  /// Examples:
  /// - "1.2.3-alpha.1" and "1.2.4-alpha.2" → true
  /// - "1.2.3-alpha.1" and "1.2.4-beta.1" → false
  /// - "1.2.3" and "1.2.4" → true (both stable)
  pub fn has_same_release_channel_as(&self, other: &Self) -> bool {
    if !self.has_comparable_version() || !other.has_comparable_version() {
      return false;
    }
    let (self_version, other_version) = match (self.get_node_version(), other.get_node_version()) {
      (Some(self_v), Some(other_v)) => (self_v, other_v),
      _ => return false,
    };

    // Get the prerelease identifiers from both versions
    let self_prerelease = &self_version.pre_release;
    let other_prerelease = &other_version.pre_release;

    // Both stable (no prerelease) → same channel
    if self_prerelease.is_empty() && other_prerelease.is_empty() {
      return true;
    }

    // One stable, one prerelease → different channels
    if self_prerelease.is_empty() || other_prerelease.is_empty() {
      return false;
    }

    // Both have prerelease - compare the first identifier segment
    // Prerelease identifiers are stored as a Vec of Identifiers
    // We need to compare the first element as a string
    match (self_prerelease.first(), other_prerelease.first()) {
      (Some(self_first), Some(other_first)) => {
        // Convert to string and compare
        self_first.to_string() == other_first.to_string()
      }
      _ => false,
    }
  }

  /// Regardless of the range, does this specifier and the other both have the
  /// same version number (e.g. "1.4.1")?
  ///
  /// Examples:
  /// - "^1.4.1" and "~1.4.1" → true
  /// - "1.4.1" and "^1.4.1" → true
  /// - "1.4.1" and "1.4.2" → false
  pub fn has_same_version_number_as(&self, other: &Self) -> bool {
    if !self.has_comparable_version() || !other.has_comparable_version() {
      return false;
    }
    match (self.get_node_version(), other.get_node_version()) {
      (Some(left), Some(right)) => left == right,
      _ => false,
    }
  }

  /// Check if this specifier uses the given semver range type.
  ///
  /// Examples:
  /// - "^1.2.3" with SemverRange::Minor → true
  /// - "~1.2.3" with SemverRange::Minor → false
  /// - "1.2.3" with SemverRange::Exact → true
  pub fn has_semver_range_of(&self, range: &SemverRange) -> bool {
    match self {
      // Exact variants (no range prefix) → SemverRange::Exact
      Self::Exact(_) | Self::Major(_) | Self::Minor(_) => range == &SemverRange::Exact,

      // Range variants with explicit semver_range field
      Self::Range(s) => &s.semver_range == range,
      Self::RangeMajor(s) => &s.semver_range == range,
      Self::RangeMinor(s) => &s.semver_range == range,

      // Variants that may contain a semver_range (Option)
      Self::Alias(s) => s.inner_specifier.get_semver_range().as_ref() == Some(range),
      Self::Git(s) => s.semver_range.as_ref() == Some(range),
      Self::WorkspaceProtocol(s) => match &s.inner_specifier {
        workspace_specifier::WorkspaceSpecifier::RangeOnly(r) => r == range,
        workspace_specifier::WorkspaceSpecifier::Resolved(spec) => spec.get_semver_range().as_ref() == Some(range),
      },

      // Latest("*") maps to SemverRange::Any
      Self::Latest(s) => s.raw == "*" && range == &SemverRange::Any,

      // All other variants have no semver range
      Self::ComplexSemver(_) | Self::File(_) | Self::None | Self::Tag(_) | Self::Unsupported(_) | Self::Url(_) => false,
    }
  }

  /// Is this specifier eligible to update the given specifier based on the
  /// given target constraint?
  ///
  /// Examples:
  /// - "2.0.0" can update "1.0.0" with UpdateTarget::Latest → true
  /// - "1.1.0" can update "1.0.0" with UpdateTarget::Minor → true
  /// - "2.0.0" can update "1.0.0" with UpdateTarget::Minor → false
  /// - "1.2.3" can update "1.2.2" with UpdateTarget::Patch → true
  pub fn is_eligible_update_for(&self, other: &Self, target: &UpdateTarget) -> bool {
    // Both must have comparable versions (excludes Latest)
    if !self.has_comparable_version() || !other.has_comparable_version() {
      return false;
    }
    let (self_version, other_version) = match (self.get_node_version(), other.get_node_version()) {
      (Some(self_v), Some(other_v)) => (self_v, other_v),
      _ => return false,
    };

    // Must be newer
    if self_version <= other_version {
      return false;
    }

    match target {
      UpdateTarget::Latest => true,
      UpdateTarget::Minor => self_version.major == other_version.major,
      UpdateTarget::Patch => {
        // HUGE (999999) is a wildcard placeholder for unspecified version components.
        // Shorthand versions like "1" (1.999999.999999) or "1.4" (1.4.999999) use HUGE
        // to indicate "accept anything" in that position. For example, "1.4" (1.4.HUGE)
        // is eligible as a Patch update for "1.4.0" because it potentially matches
        // 1.4.1 and everything above it in the 1.4.* range.
        self_version.major == other_version.major && self_version.minor == other_version.minor
      }
    }
  }

  /// Check if this specifier represents an older version than the other.
  ///
  /// Examples:
  /// - "1.0.0" compared to "2.0.0" → true
  /// - "2.0.0" compared to "1.0.0" → false
  /// - "1.0.0" compared to "1.0.0" → false
  pub fn is_older_than(&self, other: &Self) -> bool {
    if !self.has_comparable_version() || !other.has_comparable_version() {
      return false;
    }
    match (self.get_node_version(), other.get_node_version()) {
      (Some(self_version), Some(other_version)) => self_version < other_version,
      _ => false,
    }
  }

  /// Is this specifier on the same major version, but otherwise older?
  ///
  /// Examples:
  /// - "1.0.0" compared to "1.1.0" → true
  /// - "1.0.1" compared to "1.1.0" → true
  /// - "1.0.0" compared to "2.0.0" → false
  /// - "1.1.0" compared to "1.0.0" → false
  pub fn is_older_than_by_minor(&self, other: &Self) -> bool {
    if !self.has_comparable_version() || !other.has_comparable_version() {
      return false;
    }
    match (self.get_node_version(), other.get_node_version()) {
      (Some(self_version), Some(other_version)) => self_version.major == other_version.major && self_version < other_version,
      _ => false,
    }
  }

  /// Is this specifier on the same major and minor version, but otherwise
  /// older?
  ///
  /// Examples:
  /// - "1.0.0" compared to "1.0.1" → true
  /// - "1.0.0" compared to "1.1.0" → false
  /// - "1.0.1" compared to "1.0.0" → false
  pub fn is_older_than_by_patch(&self, other: &Self) -> bool {
    if !self.has_comparable_version() || !other.has_comparable_version() {
      return false;
    }
    match (self.get_node_version(), other.get_node_version()) {
      (Some(self_version), Some(other_version)) => {
        self_version.major == other_version.major && self_version.minor == other_version.minor && self_version < other_version
      }
      _ => false,
    }
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
  pub fn satisfies(&self, range: &node_semver::Range) -> bool {
    // Workspace protocols with embedded versions should return false
    // because they need to be resolved first
    if matches!(self, Self::WorkspaceProtocol(_)) {
      return false;
    }

    // Try to get a node_range from self for range-to-range comparison
    // Use allows_any to check if self's range overlaps with target range
    if let Some(self_range) = self.get_node_range() {
      return self_range.allows_any(range);
    }

    // If self has a version (not a range), check if it satisfies the target range
    if let Some(self_version) = self.get_node_version() {
      return range.satisfies(&self_version);
    }

    // No version or range to compare
    false
  }

  /// Check if this specifier satisfies all other specifiers.
  ///
  /// Handles both:
  /// - Range-intersects-ranges: self and others are Ranges (checked first)
  /// - Version-satisfies-ranges: self is Version only, others are Ranges
  ///
  /// Accepts slice of Rc<Specifier> for consistency with codebase patterns.
  ///
  /// Examples:
  /// - "1.2.3" satisfies ["^1.0.0", "~1.2.0"] → true
  /// - "^1.0.0" satisfies ["^1.0.0", "~1.2.0"] → true (range intersection)
  /// - ">1.4.2" satisfies ["1.4.2"] → false (ranges don't intersect)
  /// - ">=1.4.2" satisfies ["1.4.2"] → true (ranges intersect)
  pub fn satisfies_all(&self, others: &[Rc<Specifier>]) -> bool {
    match self {
      Specifier::None => false,
      _ => {
        // Check range intersection FIRST (even if self also has a version)
        // This is critical: ">1.4.2" has both a range AND a version (1.4.2),
        // but we need to check range intersection, not version satisfaction
        if let Some(self_range) = self.get_node_range() {
          return others.iter().all(|other| {
            // Deref Rc to access Specifier
            match other.get_node_range() {
              Some(other_range) => self_range.allows_any(&other_range),
              None => false,
            }
          });
        }

        // Fallback: If self has version but no range, check version satisfies all ranges
        // (This case may not actually occur in practice)
        if let Some(self_version) = self.get_node_version() {
          return others.iter().all(|other| match other.get_node_range() {
            Some(range) => range.satisfies(&self_version),
            None => false,
          });
        }

        false
      }
    }
  }

  /// Resolve a workspace protocol specifier using the given local version.
  ///
  /// If the specifier is a workspace protocol (e.g., "workspace:^"), this
  /// method combines the protocol with the local version to produce a regular
  /// specifier (e.g., "^1.2.3").
  ///
  /// If the specifier is not a workspace protocol, it returns itself unchanged
  /// (the same cached Rc).
  ///
  /// Examples:
  /// - "workspace:^" + Exact("1.2.3") → "^1.2.3"
  /// - "workspace:~" + Exact("1.2.3") → "~1.2.3"
  /// - "workspace:*" + Exact("1.2.3") → "*"
  /// - "workspace:^1.2.3" + Exact("2.0.0") → "^1.2.3" (version already embedded)
  /// - "^1.2.3" + Exact("2.0.0") → "^1.2.3" (not a workspace protocol, returns self)
  pub fn resolve_workspace_protocol(&self, local_version: &Self) -> Option<Rc<Self>> {
    // Validate that local_version is an Exact variant
    if !matches!(local_version, Self::Exact(_)) {
      return None;
    }

    // If not a workspace protocol, return self unchanged by looking it up in cache
    let Self::WorkspaceProtocol(ws) = self else {
      let raw = match self {
        Self::Alias(a) => &a.raw,
        Self::ComplexSemver(c) => &c.raw,
        Self::Exact(e) => &e.raw,
        Self::File(f) => &f.raw,
        Self::Git(g) => &g.raw,
        Self::Latest(l) => &l.raw,
        Self::Major(m) => &m.raw,
        Self::Minor(m) => &m.raw,
        Self::None => "",
        Self::Range(r) => &r.raw,
        Self::RangeMajor(r) => &r.raw,
        Self::RangeMinor(r) => &r.raw,
        Self::Tag(t) => &t.raw,
        Self::Unsupported(s) => s,
        Self::Url(u) => &u.raw,
        Self::WorkspaceProtocol(w) => &w.raw,
      };
      return Some(Self::new(raw));
    };

    // If workspace protocol has resolved specifier, strip "workspace:" and create new specifier
    if ws.as_resolved().is_some() {
      let resolved_str = strip_workspace_protocol(&ws.raw);
      return Some(Self::new(resolved_str));
    }

    // Otherwise, combine the semver range with local_version's number
    let local_number = local_version.get_semver_number().expect("Exact should have semver_number");

    // Get the range/operator from workspace protocol
    let range_or_op = strip_workspace_protocol(&ws.raw);

    // Handle special case: "*" stays as "*"
    if range_or_op == "*" {
      return Some(Self::new("*"));
    }

    // Combine operator/range character with local version number
    // For multi-char operators like >=, >, <=, < this preserves them correctly
    let resolved_str = format!("{range_or_op}{local_number}");
    Some(Self::new(&resolved_str))
  }
}

// =============================================================================
// Ordering Traits
// =============================================================================

impl Ord for Specifier {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    // Extract version and range from both specifiers
    let self_version = self.get_node_version();
    let other_version = other.get_node_version();

    match (self_version, other_version) {
      // Both have versions: compare by version first, then by range greediness
      (Some(self_ver), Some(other_ver)) => {
        match self_ver.cmp(&other_ver) {
          Ordering::Equal => {
            // Same version, use range greediness as tiebreaker
            let self_range = self.get_semver_range();
            let other_range = other.get_semver_range();

            match (self_range, other_range) {
              (Some(self_r), Some(other_r)) => self_r.cmp(&other_r),
              (Some(self_r), None) => {
                // self has range, other doesn't (treat other as Exact)
                // Get the greediness of self and compare with Exact (rank 2)
                self_r.get_greediness_ranking().cmp(&2)
              }
              (None, Some(other_r)) => {
                // self has no range (treat as Exact), other has range
                2.cmp(&other_r.get_greediness_ranking())
              }
              (None, None) => Ordering::Equal,
            }
          }
          ordering => ordering,
        }
      }
      // Only self has version: self is greater
      (Some(_), None) => Ordering::Greater,
      // Only other has version: other is greater
      (None, Some(_)) => Ordering::Less,
      // Neither has version: they are equal
      (None, None) => Ordering::Equal,
    }
  }
}

impl PartialOrd for Specifier {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Eq for Specifier {}
