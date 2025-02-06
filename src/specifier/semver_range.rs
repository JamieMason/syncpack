#[cfg(test)]
#[path = "semver_range_test.rs"]
mod semver_range_test;

use std::{
  cmp::Ordering,
  hash::{Hash, Hasher},
};

#[derive(Clone, Debug)]
pub enum SemverRange {
  /// *
  Any,
  /// ^1.4.2
  Minor,
  /// 1.4.2
  Exact,
  /// >1.4.2
  Gt,
  /// >=1.4.2
  Gte,
  /// <1.4.2
  Lt,
  /// <=1.4.2
  Lte,
  /// ~1.4.2
  Patch,
}

impl SemverRange {
  /// Create a SemverRange if the given string is a valid range
  pub fn new(range: &str) -> Option<SemverRange> {
    match range {
      "*" => Some(SemverRange::Any),
      "^" => Some(SemverRange::Minor),
      "" => Some(SemverRange::Exact),
      ">" => Some(SemverRange::Gt),
      ">=" => Some(SemverRange::Gte),
      "<" => Some(SemverRange::Lt),
      "<=" => Some(SemverRange::Lte),
      "~" => Some(SemverRange::Patch),
      _ => None,
    }
  }

  /// Get the string representation of the range
  pub fn unwrap(&self) -> String {
    match self {
      SemverRange::Any => "*",
      SemverRange::Minor => "^",
      SemverRange::Exact => "",
      SemverRange::Gt => ">",
      SemverRange::Gte => ">=",
      SemverRange::Lt => "<",
      SemverRange::Lte => "<=",
      SemverRange::Patch => "~",
    }
    .to_string()
  }

  /// Get a numeric rank according to its greediness, for use in sorting
  pub fn get_greediness_ranking(&self) -> u8 {
    match self {
      SemverRange::Any => 7,
      SemverRange::Gt => 6,
      SemverRange::Gte => 5,
      SemverRange::Minor => 4,
      SemverRange::Patch => 3,
      SemverRange::Exact => 2,
      SemverRange::Lte => 1,
      SemverRange::Lt => 0,
    }
  }
}

impl Ord for SemverRange {
  fn cmp(&self, other: &Self) -> Ordering {
    self.get_greediness_ranking().cmp(&other.get_greediness_ranking())
  }
}

impl PartialOrd for SemverRange {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for SemverRange {
  fn eq(&self, other: &Self) -> bool {
    self.get_greediness_ranking() == other.get_greediness_ranking()
  }
}

impl Eq for SemverRange {}

impl Hash for SemverRange {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.get_greediness_ranking().hash(state);
  }
}
