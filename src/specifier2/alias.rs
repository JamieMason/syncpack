use crate::specifier::semver_range::SemverRange;

use super::Specifier2;

#[cfg(test)]
#[path = "alias_test.rs"]
mod alias_test;

#[derive(Debug, PartialEq)]
pub struct Alias {
  /// "npm:@fluidframework/build-tools@~0.44.0"
  pub raw: String,
  /// "@fluidframework/build-tools"
  pub name: String,
  /// SemverRange::Patch
  pub semver_range: Option<SemverRange>,
  /// "0.44.0"
  pub semver_number: Option<String>,
}

impl Alias {
  pub fn new(raw: &str) -> Specifier2 {
    let name = raw.strip_prefix("npm:").map(|after_prefix| {
      after_prefix
        .rfind('@')
        .filter(|&at_pos| at_pos > 0 && !after_prefix[..at_pos].is_empty())
        .map(|at_pos| &after_prefix[..at_pos])
        .unwrap_or(after_prefix)
    });
    let semver_string = raw
      .strip_prefix("npm:")
      .and_then(|after_prefix| after_prefix.rfind('@').map(|at_pos| (after_prefix, at_pos)))
      .and_then(|(after_prefix, at_pos)| (at_pos > 0 && !after_prefix[..at_pos].is_empty()).then(|| &after_prefix[at_pos + 1..]))
      .filter(|version| !version.is_empty())
      .map(|version| version.to_string());
    name
      .map(|name| {
        Specifier2::Alias(Self {
          raw: raw.to_string(),
          name: name.to_string(),
          semver_string,
        })
      })
      .unwrap_or(Specifier2::Unsupported(raw.to_string()))
  }
}
