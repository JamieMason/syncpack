use {
  crate::{semver_range::SemverRange, specifier::Specifier},
  std::rc::Rc,
};

#[derive(Debug, PartialEq)]
pub struct Git {
  /// The exact version specifier as it was provided.
  ///
  /// Examples:
  /// - "git+ssh://git@github.com/npm/cli"
  /// - "git+ssh://git@github.com/npm/cli#1.2.3"
  /// - "git+ssh://git@github.com/npm/cli#HEAD"
  /// - "git@github.com:npm/cli.git"
  /// - "git@github.com:npm/cli.git#1.2.3"
  /// - "git@github.com:npm/cli.git#HEAD"
  /// - "github:uNetworking/uWebSockets.js"
  /// - "github:uNetworking/uWebSockets.js#1.2.3"
  /// - "github:uNetworking/uWebSockets.js#HEAD"
  /// - "git://github.com/user/foo#semver:^1.2.3"
  pub raw: String,
  /// Used when checking if specifiers satisfy each other
  ///
  /// Examples:
  /// - "git+ssh://git@github.com/npm/cli" → None
  /// - "git+ssh://git@github.com/npm/cli#1.2.3" → Some("1.2.3")
  /// - "git+ssh://git@github.com/npm/cli#HEAD" → None
  /// - "git@github.com:npm/cli.git#1.2.3" → Some("1.2.3")
  /// - "github:uNetworking/uWebSockets.js#1.2.3" → Some("1.2.3")
  /// - "git://github.com/user/foo#semver:^1.2.3" → Some("^1.2.3")
  pub node_range: Option<Rc<node_semver::Range>>,
  /// Used for ordering and comparison, semver range characters are NOT
  /// included
  ///
  /// Examples:
  /// - "git+ssh://git@github.com/npm/cli" → None
  /// - "git+ssh://git@github.com/npm/cli#1.2.3" → Some("1.2.3")
  /// - "git+ssh://git@github.com/npm/cli#HEAD" → None
  /// - "git@github.com:npm/cli.git#1.2.3" → Some("1.2.3")
  /// - "github:uNetworking/uWebSockets.js#1.2.3" → Some("1.2.3")
  /// - "git://github.com/user/foo#semver:^1.2.3" → Some("1.2.3")
  pub node_version: Option<Rc<node_semver::Version>>,
  /// The name of the dependency being aliased.
  ///
  /// Examples:
  /// - "git+ssh://git@github.com/npm/cli"
  /// - "git@github.com:npm/cli.git"
  /// - "github:uNetworking/uWebSockets.js"
  pub origin: String,
  /// The tagged version if set.
  ///
  /// Examples:
  /// - "git+ssh://git@github.com/npm/cli" → None
  /// - "git+ssh://git@github.com/npm/cli#1.2.3" → Some("1.2.3")
  /// - "git+ssh://git@github.com/npm/cli#HEAD" → None
  /// - "git@github.com:npm/cli.git" → None
  /// - "git@github.com:npm/cli.git#1.2.3" → Some("1.2.3")
  /// - "git@github.com:npm/cli.git#HEAD" → None
  /// - "github:uNetworking/uWebSockets.js" → None
  /// - "github:uNetworking/uWebSockets.js#1.2.3" → Some("1.2.3")
  /// - "github:uNetworking/uWebSockets.js#HEAD" → None
  pub semver_number: Option<String>,
  /// The semver range characters used in this specifier, if set
  ///
  /// Examples:
  /// - "git+ssh://git@github.com/npm/cli" → None
  /// - "git+ssh://git@github.com/npm/cli#1.2.3" → Some(SemverRange::Exact)
  /// - "git://github.com/user/foo#semver:^1.2.3" → Some(SemverRange::Minor)
  pub semver_range: Option<SemverRange>,
}

impl Git {
  pub fn create(raw: &str) -> Specifier {
    raw
      .find('#')
      .map(|hash_pos| {
        let origin = &raw[..hash_pos];
        let git_tag = &raw[hash_pos + 1..];
        (origin, git_tag)
      })
      .map(|(origin, git_tag)| {
        if origin.is_empty() {
          Specifier::Unsupported(raw.to_string())
        } else {
          // Strip "semver:" prefix if present
          let tag = if git_tag.starts_with("semver:") {
            git_tag.strip_prefix("semver:").unwrap_or(git_tag)
          } else {
            git_tag
          };

          if tag.is_empty() {
            // Empty tag
            return Specifier::Git(Self {
              raw: raw.to_string(),
              node_range: None,
              node_version: None,
              origin: origin.to_string(),
              semver_number: None,
              semver_range: None,
            });
          }

          // Try to parse as node_range (includes range characters)
          let node_range = Specifier::new_node_range(tag);

          if node_range.is_none() {
            // Not a valid semver range (e.g. HEAD, branch name)
            return Specifier::Git(Self {
              raw: raw.to_string(),
              node_range: None,
              node_version: None,
              origin: origin.to_string(),
              semver_number: None,
              semver_range: None,
            });
          }

          // Parse semver_range to strip range chars for bare version
          let semver_range = SemverRange::parse(tag);
          let range_str = semver_range.unwrap();
          let bare_version = tag.strip_prefix(&range_str).unwrap_or(tag);

          // Parse the bare version - note that node_range can parse shorthand like "^1"
          // but node_version requires complete versions like "1.0.0"
          let node_version = Specifier::new_node_version(bare_version);

          // Valid semver tag - store semver_number and semver_range even for shorthand
          // versions where node_version parsing fails
          Specifier::Git(Self {
            raw: raw.to_string(),
            node_range: Some(node_range.unwrap()),
            node_version,
            origin: origin.to_string(),
            semver_number: Some(bare_version.to_string()),
            semver_range: Some(semver_range),
          })
        }
      })
      // There is no hash, just the origin
      .unwrap_or_else(|| {
        Specifier::Git(Self {
          raw: raw.to_string(),
          node_range: None,
          node_version: None,
          origin: raw.to_string(),
          semver_number: None,
          semver_range: None,
        })
      })
  }
}
