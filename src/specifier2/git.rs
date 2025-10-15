use crate::specifier::sanitise_value;

use super::Specifier2;

#[derive(Debug, PartialEq)]
pub struct Git {
  /// The exact version specifier as it was provided.
  ///
  /// Examples:
  /// "git+ssh://git@github.com/npm/cli"
  /// "git+ssh://git@github.com/npm/cli#1.2.3"
  /// "git+ssh://git@github.com/npm/cli#HEAD"
  /// "git@github.com:npm/cli.git"
  /// "git@github.com:npm/cli.git#1.2.3"
  /// "git@github.com:npm/cli.git#HEAD"
  /// "github:uNetworking/uWebSockets.js"
  /// "github:uNetworking/uWebSockets.js#1.2.3"
  /// "github:uNetworking/uWebSockets.js#HEAD"
  pub raw: String,
  /// The name of the dependency being aliased.
  ///
  /// Examples:
  /// "git+ssh://git@github.com/npm/cli"
  /// "git@github.com:npm/cli.git"
  /// "github:uNetworking/uWebSockets.js"
  pub origin: String,
  /// The tagged version if set.
  ///
  /// Examples:
  /// "git+ssh://git@github.com/npm/cli" → None
  /// "git+ssh://git@github.com/npm/cli#1.2.3" → Some("1.2.3")
  /// "git+ssh://git@github.com/npm/cli#HEAD" → None
  /// "git@github.com:npm/cli.git" → None
  /// "git@github.com:npm/cli.git#1.2.3" → Some("1.2.3")
  /// "git@github.com:npm/cli.git#HEAD" → None
  /// "github:uNetworking/uWebSockets.js" → None
  /// "github:uNetworking/uWebSockets.js#1.2.3" → Some("1.2.3")
  /// "github:uNetworking/uWebSockets.js#HEAD" → None
  pub semver_number: Option<String>,
}

impl Git {
  pub fn new(raw: &str) -> Specifier2 {
    if let Some(hash_pos) = raw.find('#') {
      let origin = &raw[..hash_pos];
      let git_tag_str = &raw[hash_pos + 1..];

      if origin.is_empty() {
        return Specifier2::Unsupported(raw.to_string());
      }

      if git_tag_str.is_empty() {
        Specifier2::Git(Self {
          raw: raw.to_string(),
          origin: origin.to_string(),
          semver_number: None,
        })
      } else {
        Specifier2::Git(Self {
          raw: raw.to_string(),
          origin: origin.to_string(),
          semver_number: sanitise_value(git_tag_str)
            .as_deref()
            .or(Some(git_tag_str))
            .filter(|tag| Specifier2::is_valid_semver(tag))
            .map(str::to_string),
        })
      }
    } else {
      // No hash, just the origin
      Specifier2::Git(Self {
        raw: raw.to_string(),
        origin: raw.to_string(),
        semver_number: None,
      })
    }
  }
}
