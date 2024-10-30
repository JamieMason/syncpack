use super::{
  orderable::{IsOrderable, Orderable},
  parser,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum NonSemver {
  /// eg. `npm:1.2.3`
  Alias(String),
  /// eg. `file:./path/to/package`
  File(String),
  /// eg. `git://github.com/user/repo.git`
  Git(String),
  /// eg. `alpha`
  Tag(String),
  /// eg. `{wh}at[the]fuu`
  Unsupported(String),
  /// eg. `https://example.com`
  Url(String),
  /// eg. `workspace:*`
  WorkspaceProtocol(String),
}

impl NonSemver {
  pub fn new(specifier: &str) -> Self {
    let str = parser::sanitise(specifier);
    let string = str.to_string();
    if parser::is_alias(str) {
      Self::Alias(string)
    } else if parser::is_file(str) {
      Self::File(string)
    } else if parser::is_git(str) {
      Self::Git(string)
    } else if parser::is_tag(str) {
      Self::Tag(string)
    } else if parser::is_url(str) {
      Self::Url(string)
    } else if parser::is_workspace_protocol(str) {
      Self::WorkspaceProtocol(string)
    } else {
      Self::Unsupported(string)
    }
  }
}

impl IsOrderable for NonSemver {
  fn get_orderable(&self) -> Orderable {
    // @TODO: look for semver substrings in eg URLs, file paths, etc
    Orderable::new()
  }
}
