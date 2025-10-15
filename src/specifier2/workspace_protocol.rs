use crate::{specifier::semver_range::SemverRange, specifier2::strip_semver_range};

use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct WorkspaceProtocol {
  /// Examples:
  /// "workspace:^"
  /// "workspace:*"
  /// "workspace:~"
  /// "workspace:^1.2.3"
  pub raw: String,
  /// Examples:
  /// "^" -> Some(SemverRange::Minor)
  /// "*" -> Some(SemverRange::Any)
  /// "~" -> Some(SemverRange::Patch)
  /// "^1.2.3" -> Some(SemverRange::Minor)
  pub semver_range: Option<SemverRange>,
  /// Examples:
  /// "^" -> None
  /// "*" -> None
  /// "~" -> None
  /// "^1.2.3" -> Some("1.2.3")
  pub semver_number: Option<String>,
}

impl WorkspaceProtocol {
  pub fn new(raw: &str) -> Specifier2 {
    let semver_string = raw.strip_prefix("workspace:");
    let semver_number = match semver_string {
      Some("*" | "^" | "~") => None,
      Some(semver_number) => Some(strip_semver_range(semver_number).to_string()),
      None => None,
    };
    Specifier2::WorkspaceProtocol(Self {
      raw: raw.to_string(),
      semver_range: semver_string.map(SemverRange::parse),
      semver_number,
    })
  }
}
