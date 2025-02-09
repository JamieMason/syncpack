#[cfg(test)]
#[path = "specifier_test.rs"]
mod specifier_test;

use {
  alias::Alias,
  basic_semver::{BasicSemver, BasicSemverVariant},
  complex_semver::ComplexSemver,
  git::Git,
  node_semver::{Range, Version},
  raw::Raw,
  semver_range::SemverRange,
  std::cmp::Ordering,
  workspace_protocol::WorkspaceProtocol,
};

mod alias;
pub mod basic_semver;
mod complex_semver;
mod git;
pub mod parser;
mod raw;
pub mod regexes;
pub mod semver_range;
mod workspace_protocol;

fn get_raw_without_range(value: &str) -> String {
  regexes::RANGE_CHARS.replace(value, "").into_owned()
}

fn get_huge() -> u64 {
  999999
}

/// For a variant known to have a semver range, determine the type of range
fn determine_semver_range(value: &str) -> Option<SemverRange> {
  Some(if value.starts_with("*") {
    SemverRange::Any
  } else if value.starts_with("^") {
    SemverRange::Minor
  } else if value.starts_with("~") {
    SemverRange::Patch
  } else if value.starts_with(">=") {
    SemverRange::Gte
  } else if value.starts_with("<=") {
    SemverRange::Lte
  } else if value.starts_with(">") {
    SemverRange::Gt
  } else if value.starts_with("<") {
    SemverRange::Lt
  } else {
    panic!("determine_semver_range called on value that has no semver range");
  })
}

/// Normalise values which are needlessly different
fn sanitise_value(value: &str) -> String {
  if value == "latest" || value == "x" {
    "*".to_string()
  } else {
    let value = value.replace(".x", "").replace(".*", "");
    if value.starts_with("v") {
      value.chars().skip(1).collect()
    } else {
      value
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Specifier {
  Alias(Alias),
  BasicSemver(BasicSemver),
  ComplexSemver(ComplexSemver),
  File(Raw),
  Git(Git),
  None,
  Tag(Raw),
  Unsupported(Raw),
  Url(Raw),
  WorkspaceProtocol(WorkspaceProtocol),
}

impl Specifier {
  /// Create a new instance
  pub fn new(value: &str, local_version: Option<&BasicSemver>) -> Self {
    let raw = value.to_string();

    if parser::is_workspace_protocol(value) {
      return Self::from_workspace_protocol(value, local_version, raw);
    } else if parser::is_alias(value) {
      return Self::from_alias(value, raw);
    } else if parser::is_git(value) {
      return Self::from_git(value, raw);
    } else if parser::is_file(value) {
      return Self::File(raw::Raw { raw });
    } else if parser::is_url(value) {
      return Self::Url(raw::Raw { raw });
    }

    let sanitised = sanitise_value(value);
    let value = sanitised.as_str();

    if parser::is_tag(value) {
      return Self::Tag(raw::Raw { raw });
    }

    match Range::parse(value) {
      Ok(node_range) => {
        if parser::is_complex_range(value) {
          Self::ComplexSemver(ComplexSemver { raw, node_range })
        } else {
          match BasicSemver::new(value) {
            Some(semver) => Self::BasicSemver(semver),
            None => Self::Unsupported(raw::Raw { raw }),
          }
        }
      }
      Err(_) => Self::Unsupported(raw::Raw { raw }),
    }
  }

  /// Create a new instance from a specifier containing "workspace:"
  fn from_workspace_protocol(value: &str, local_version: Option<&BasicSemver>, raw: String) -> Self {
    local_version
      .and_then(|local| {
        let without_protocol = value.replace("workspace:", "");
        let sanitised = sanitise_value(&without_protocol);
        if parser::is_simple_semver(&sanitised) {
          Some(Self::WorkspaceProtocol(WorkspaceProtocol {
            raw: format!("workspace:{sanitised}"),
            local_version: local.clone(),
            semver: BasicSemver::new(&sanitised).unwrap(),
          }))
        } else if sanitised == "~" || sanitised == "^" {
          Some(Self::WorkspaceProtocol(WorkspaceProtocol {
            raw: format!("workspace:{sanitised}"),
            local_version: local.clone(),
            semver: BasicSemver::new(&format!("{}{}", sanitised, local.raw)).unwrap(),
          }))
        } else {
          None
        }
      })
      .unwrap_or_else(|| Self::Unsupported(raw::Raw { raw: raw.clone() }))
  }

  /// Create a new instance from an npm alias specifier
  fn from_alias(value: &str, raw: String) -> Self {
    let aliased_version = {
      let start = value.rfind('@').unwrap() + 1;
      value[start..].to_string()
    };
    let aliased_name = {
      let start = value.find(':').unwrap() + 1;
      let end = value.rfind('@').unwrap();
      value[start..end].to_string()
    };
    if aliased_name.is_empty() {
      Self::Unsupported(raw::Raw { raw })
    } else if aliased_version.is_empty() {
      Self::Alias(alias::Alias {
        raw,
        name: aliased_name,
        semver: None,
      })
    } else if let Self::BasicSemver(inner) = Self::new(&aliased_version, None) {
      Self::Alias(alias::Alias {
        raw,
        name: aliased_name,
        semver: Some(inner),
      })
    } else {
      Self::Unsupported(raw::Raw { raw })
    }
  }

  /// Create a new instance from a git specifier, this can be a git url or some
  /// kind of github shorthand
  fn from_git(value: &str, raw: String) -> Self {
    let parts = value.split('#').collect::<Vec<&str>>();
    let git_tag = parts.get(1).map(|tag| tag.to_string()).unwrap_or_default();
    let git_tag = sanitise_value(&git_tag);
    let origin = parts.first().map(|origin| origin.to_string()).unwrap_or_default();
    if origin.is_empty() {
      Self::Unsupported(raw::Raw { raw })
    } else if git_tag.is_empty() {
      Self::Git(git::Git { raw, origin, semver: None })
    } else if let Some(inner) = BasicSemver::new(&git_tag) {
      Self::Git(git::Git {
        raw,
        origin,
        semver: Some(inner),
      })
    } else {
      Self::Git(git::Git { raw, origin, semver: None })
    }
  }

  fn get_semver(&self) -> Option<&BasicSemver> {
    match self {
      Self::Alias(inner) => inner.semver.as_ref(),
      Self::BasicSemver(inner) => Some(inner),
      Self::Git(inner) => inner.semver.as_ref(),
      Self::WorkspaceProtocol(inner) => Some(&inner.semver),
      _ => None,
    }
  }

  fn get_node_range(&self) -> Option<&Range> {
    match self {
      Self::Alias(inner) => inner.semver.as_ref().map(|semver| &semver.node_range),
      Self::BasicSemver(inner) => Some(&inner.node_range),
      Self::ComplexSemver(inner) => Some(&inner.node_range),
      Self::Git(inner) => inner.semver.as_ref().map(|semver| &semver.node_range),
      Self::WorkspaceProtocol(inner) => Some(&inner.semver.node_range),
      _ => None,
    }
  }

  pub fn get_node_version(&self) -> Option<&Version> {
    match self {
      Self::Alias(inner) => inner.semver.as_ref().map(|semver| &semver.node_version),
      Self::BasicSemver(inner) => Some(&inner.node_version),
      Self::Git(inner) => inner.semver.as_ref().map(|semver| &semver.node_version),
      Self::WorkspaceProtocol(inner) => Some(&inner.semver.node_version),
      _ => None,
    }
  }

  /// Get the semver range for this specifier, if it has one
  pub fn get_semver_range(&self) -> Option<&SemverRange> {
    self.get_semver().map(|semver| &semver.range_variant)
  }

  /// Return a new `Specifier` with the given semver range applied to it when it
  /// is possible to do so, otherwise the same `Specifier` is returned
  pub fn with_range(self, range: &SemverRange) -> Self {
    match self {
      Self::Alias(s) => Self::Alias(s.with_range(range)),
      Self::BasicSemver(s) => Self::BasicSemver(s.with_range(range)),
      Self::ComplexSemver(s) => Self::ComplexSemver(s.with_range(range)),
      Self::File(s) => Self::File(s.with_range(range)),
      Self::Git(s) => Self::Git(s.with_range(range)),
      Self::None => Self::None,
      Self::Tag(s) => Self::Tag(s.with_range(range)),
      Self::Unsupported(s) => Self::Unsupported(s.with_range(range)),
      Self::Url(s) => Self::Url(s.with_range(range)),
      Self::WorkspaceProtocol(s) => Self::WorkspaceProtocol(s.with_range(range)),
    }
  }

  /// Return a new `Specifier` with the given semver version number applied to
  /// it when it is possible to do so, otherwise the same `Specifier` is
  /// returned. The range is also changed.
  pub fn with_semver(self, semver: &BasicSemver) -> Self {
    match self {
      Self::Alias(s) => Self::Alias(s.with_semver(semver)),
      Self::BasicSemver(s) => Self::BasicSemver(s.with_semver(semver)),
      Self::ComplexSemver(s) => Self::ComplexSemver(s.with_semver(semver)),
      Self::File(s) => Self::File(s.with_semver(semver)),
      Self::Git(s) => Self::Git(s.with_semver(semver)),
      Self::None => Self::None,
      Self::Tag(s) => Self::Tag(s.with_semver(semver)),
      Self::Unsupported(s) => Self::Unsupported(s.with_semver(semver)),
      Self::Url(s) => Self::Url(s.with_semver(semver)),
      Self::WorkspaceProtocol(s) => Self::WorkspaceProtocol(s.with_semver(semver)),
    }
  }

  pub fn get_raw(&self) -> String {
    match self {
      Self::Alias(inner) => inner.raw.clone(),
      Self::BasicSemver(inner) => inner.raw.clone(),
      Self::ComplexSemver(inner) => inner.raw.clone(),
      Self::File(inner) => inner.raw.clone(),
      Self::Git(inner) => inner.raw.clone(),
      Self::None => "".to_string(),
      Self::Tag(inner) => inner.raw.clone(),
      Self::Unsupported(inner) => inner.raw.clone(),
      Self::Url(inner) => inner.raw.clone(),
      Self::WorkspaceProtocol(inner) => inner.raw.clone(),
    }
  }

  /// Get the `specifier_type` name as used in config files.
  pub fn get_config_identifier(&self) -> String {
    match self {
      Self::Alias(_) => "alias",
      Self::BasicSemver(semver) => match semver.variant {
        BasicSemverVariant::Latest => "latest",
        BasicSemverVariant::Major => match semver.range_variant {
          SemverRange::Exact => "major",
          _ => "range-major",
        },
        BasicSemverVariant::Minor => match semver.range_variant {
          SemverRange::Exact => "minor",
          _ => "range-minor",
        },
        BasicSemverVariant::Patch => match semver.range_variant {
          SemverRange::Any => "latest",
          SemverRange::Exact => "exact",
          _ => "range",
        },
      },
      Self::ComplexSemver(_) => "range-complex",
      Self::File(_) => "file",
      Self::Git(_) => "git",
      Self::None => "missing",
      Self::Tag(_) => "tag",
      Self::Unsupported(_) => "unsupported",
      Self::Url(_) => "url",
      Self::WorkspaceProtocol(_) => "workspace-protocol",
    }
    .to_string()
  }

  /// Does this specifier have the given semver range?
  pub fn has_semver_range_of(&self, range: &SemverRange) -> bool {
    self.get_semver_range().is_some_and(|a| a == range)
  }

  /// Regardless of the range, does this specifier and the other both have eg.
  /// "1.4.1" as their version?
  pub fn has_same_version_number_as(&self, other: &Self) -> bool {
    match (self.get_node_version(), other.get_node_version()) {
      (Some(left), Some(right)) => left == right,
      _ => false,
    }
  }

  /// Does this specifier match every one of the given specifiers?
  pub fn satisfies_all(&self, others: Vec<&Self>) -> bool {
    others.iter().all(|other| self.satisfies(other))
  }

  /// Does this specifier match the given specifier?
  pub fn satisfies(&self, other: &Self) -> bool {
    self
      .get_node_range()
      .is_some_and(|a| other.get_node_range().is_some_and(|b| a.allows_any(b)))
  }

  pub fn is_alias(&self) -> bool {
    matches!(self, Self::Alias(_))
  }

  pub fn is_basic_semver(&self) -> bool {
    matches!(self, Self::BasicSemver(_))
  }

  pub fn is_complex_semver(&self) -> bool {
    matches!(self, Self::ComplexSemver(_))
  }

  pub fn is_file(&self) -> bool {
    matches!(self, Self::File(_))
  }

  pub fn is_git(&self) -> bool {
    matches!(self, Self::Git(_))
  }

  pub fn is_none(&self) -> bool {
    matches!(self, Self::None)
  }

  pub fn is_tag(&self) -> bool {
    matches!(self, Self::Tag(_))
  }

  pub fn is_unsupported(&self) -> bool {
    matches!(self, Self::Unsupported(_))
  }

  pub fn is_url(&self) -> bool {
    matches!(self, Self::Url(_))
  }

  pub fn is_workspace_protocol(&self) -> bool {
    matches!(self, Self::WorkspaceProtocol(_))
  }
}

impl Ord for Specifier {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self.get_node_version(), other.get_node_version()) {
      (Some(left), Some(right)) => match left.cmp(right) {
        Ordering::Equal => match (self.get_semver_range(), other.get_semver_range()) {
          (Some(left), Some(right)) => left.cmp(right),
          (None, Some(_)) => Ordering::Less,
          (Some(_), None) => Ordering::Greater,
          (None, None) => Ordering::Equal,
        },
        ordering => ordering,
      },
      (None, Some(_)) => Ordering::Less,
      (Some(_), None) => Ordering::Greater,
      (None, None) => Ordering::Equal,
    }
  }
}

impl PartialOrd for Specifier {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Eq for Specifier {}
