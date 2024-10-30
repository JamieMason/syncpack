#[cfg(test)]
#[path = "specifier_test.rs"]
mod specifier_test;

use {
  crate::specifier::{
    non_semver::NonSemver,
    orderable::{IsOrderable, Orderable},
    semver::Semver,
    simple_semver::SimpleSemver,
  },
  semver_range::SemverRange,
};

pub mod non_semver;
pub mod orderable;
pub mod parser;
pub mod regexes;
pub mod semver;
pub mod semver_range;
pub mod simple_semver;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Specifier {
  None,
  Semver(Semver),
  NonSemver(NonSemver),
}

impl Specifier {
  pub fn new(specifier: &str) -> Self {
    let str = parser::sanitise(specifier);
    if specifier.is_empty() {
      Self::None
    } else if let Ok(semver) = Semver::new(str) {
      Self::Semver(semver)
    } else {
      Self::NonSemver(NonSemver::new(str))
    }
  }

  /// Get the `specifier_type` name as used in config files.
  pub fn get_config_identifier(&self) -> String {
    match self {
      Self::Semver(simple_semver) => match simple_semver {
        Semver::Simple(variant) => match variant {
          SimpleSemver::Exact(_) => "exact",
          SimpleSemver::Latest(_) => "latest",
          SimpleSemver::Major(_) => "major",
          SimpleSemver::Minor(_) => "minor",
          SimpleSemver::Range(_) => "range",
          SimpleSemver::RangeMajor(_) => "range-major",
          SimpleSemver::RangeMinor(_) => "range-minor",
        },
        Semver::Complex(_) => "range-complex",
      },
      Self::NonSemver(non_semver) => match non_semver {
        NonSemver::Alias(_) => "alias",
        NonSemver::File(_) => "file",
        NonSemver::Git(_) => "git",
        NonSemver::Tag(_) => "tag",
        NonSemver::Url(_) => "url",
        NonSemver::WorkspaceProtocol(_) => "workspace-protocol",
        NonSemver::Unsupported(_) => "unsupported",
      },
      Self::None => "missing",
    }
    .to_string()
  }

  /// Try to parse this specifier into one from the `node_semver` crate
  pub fn parse_with_node_semver(&self) -> Result<node_semver::Range, node_semver::SemverError> {
    self.unwrap().parse::<node_semver::Range>()
  }

  /// Get the raw string value of the specifier, eg "^1.4.1"
  pub fn unwrap(&self) -> String {
    match self {
      Self::Semver(simple_semver) => match simple_semver {
        Semver::Simple(variant) => match variant {
          SimpleSemver::Exact(string) => string.clone(),
          SimpleSemver::Latest(string) => string.clone(),
          SimpleSemver::Major(string) => string.clone(),
          SimpleSemver::Minor(string) => string.clone(),
          SimpleSemver::Range(string) => string.clone(),
          SimpleSemver::RangeMajor(string) => string.clone(),
          SimpleSemver::RangeMinor(string) => string.clone(),
        },
        Semver::Complex(string) => string.clone(),
      },
      Self::NonSemver(non_semver) => match non_semver {
        NonSemver::Alias(string) => string.clone(),
        NonSemver::File(string) => string.clone(),
        NonSemver::Git(string) => string.clone(),
        NonSemver::Tag(string) => string.clone(),
        NonSemver::Url(string) => string.clone(),
        NonSemver::WorkspaceProtocol(string) => string.clone(),
        NonSemver::Unsupported(string) => string.clone(),
      },
      Self::None => "VERSION_IS_MISSING".to_string(),
    }
  }

  /// Is this specifier semver, without &&s or ||s?
  pub fn is_simple_semver(&self) -> bool {
    matches!(self, Specifier::Semver(Semver::Simple(_)))
  }

  /// If this specifier is a simple semver, return it
  pub fn get_simple_semver(&self) -> Option<SimpleSemver> {
    if let Specifier::Semver(Semver::Simple(simple_semver)) = self {
      Some(simple_semver.clone())
    } else {
      None
    }
  }

  /// Get the semver range for this specifier, if it has one
  pub fn get_semver_range(&self) -> Option<SemverRange> {
    if let Specifier::Semver(Semver::Simple(simple_semver)) = self {
      Some(simple_semver.get_range())
    } else {
      None
    }
  }

  /// Does this specifier have the given semver range?
  pub fn has_semver_range_of(&self, range: &SemverRange) -> bool {
    match self {
      Self::Semver(Semver::Simple(simple_semver)) => simple_semver.has_semver_range_of(range),
      _ => false,
    }
  }

  /// Regardless of the range, does this specifier and the other both have eg.
  /// "1.4.1" as their version?
  pub fn has_same_version_number_as(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Semver(Semver::Simple(simple_semver)), Self::Semver(Semver::Simple(other_simple_semver))) => {
        simple_semver.has_same_version_number_as(other_simple_semver)
      }
      _ => false,
    }
  }

  /// Does this specifier match every one of the given specifiers?
  pub fn satisfies_all(&self, others: Vec<&Self>) -> bool {
    if !matches!(self, Specifier::None) {
      if let Ok(node_range) = self.parse_with_node_semver() {
        return others
          .iter()
          .flat_map(|other| other.parse_with_node_semver())
          .all(|other_range| node_range.allows_any(&other_range));
      }
    }
    false
  }

  /// Does this specifier match the given specifier?
  pub fn satisfies(&self, other: &Self) -> bool {
    if !matches!(self, Specifier::None) {
      if let Ok(node_range) = self.parse_with_node_semver() {
        if let Ok(other_node_range) = other.parse_with_node_semver() {
          return node_range.allows_any(&other_node_range);
        }
      }
    }
    false
  }
}

impl IsOrderable for Specifier {
  /// Return a struct which can be used to check equality or sort specifiers
  fn get_orderable(&self) -> Orderable {
    match self {
      Self::Semver(semver) => semver.get_orderable(),
      Self::NonSemver(non_semver) => non_semver.get_orderable(),
      Self::None => Orderable::new(),
    }
  }
}
