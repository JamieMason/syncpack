use {lazy_static::lazy_static, regex::Regex};

lazy_static! {
  /// Any character used in a semver range
  pub static ref RANGE_CHARS: Regex = Regex::new(r"^(~|\^|\*|>=?|<=?)").unwrap();
  /// "1.2.3"
  pub static ref EXACT: Regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
  /// "1.2.3-alpha" || "1.2.3-rc.1"
  pub static ref EXACT_TAG: Regex = Regex::new(r"^\d+\.\d+\.\d+\-[a-z0-9.-_]+$").unwrap();
  /// "^1.2.3"
  pub static ref CARET: Regex = Regex::new(r"^\^(\d+\.\d+\.\d+)$").unwrap();
  /// "^1.2.3-alpha" || "^1.2.3-rc.1"
  pub static ref CARET_TAG: Regex = Regex::new(r"^\^(\d+\.\d+\.\d+)\-[a-z0-9.-_]+$").unwrap();
  /// "~1.2.3"
  pub static ref TILDE: Regex = Regex::new(r"^~(\d+\.\d+\.\d+)$").unwrap();
  /// "~1.2.3-alpha" || "~1.2.3-rc.1"
  pub static ref TILDE_TAG: Regex = Regex::new(r"^~(\d+\.\d+\.\d+)\-[a-z0-9.-_]+$").unwrap();
  /// ">1.2.3"
  pub static ref GT: Regex = Regex::new(r"^>(\d+\.\d+\.\d+)$").unwrap();
  /// ">1.2.3-alpha" || ">1.2.3-rc.1"
  pub static ref GT_TAG: Regex = Regex::new(r"^>(\d+\.\d+\.\d+)\-[a-z0-9.-_]+$").unwrap();
  /// ">=1.2.3"
  pub static ref GTE: Regex = Regex::new(r"^>=(\d+\.\d+\.\d+)$").unwrap();
  /// ">=1.2.3-alpha" || ">=1.2.3-rc.1"
  pub static ref GTE_TAG: Regex = Regex::new(r"^>=(\d+\.\d+\.\d+)\-[a-z0-9.-_]+$").unwrap();
  /// "<1.2.3"
  pub static ref LT: Regex = Regex::new(r"^<(\d+\.\d+\.\d+)$").unwrap();
  /// "<1.2.3-alpha" || "<1.2.3-rc.1"
  pub static ref LT_TAG: Regex = Regex::new(r"^<(\d+\.\d+\.\d+)\-[a-z0-9.-_]+$").unwrap();
  /// "<=1.2.3"
  pub static ref LTE: Regex = Regex::new(r"^<=(\d+\.\d+\.\d+)$").unwrap();
  /// "<=1.2.3-alpha" || "<=1.2.3-rc.1"
  pub static ref LTE_TAG: Regex = Regex::new(r"^<=(\d+\.\d+\.\d+)\-[a-z0-9.-_]+$").unwrap();
  /// "^1.2"
  pub static ref CARET_MINOR: Regex = Regex::new(r"^\^(\d+\.\d+)$").unwrap();
  /// "~1.2"
  pub static ref TILDE_MINOR: Regex = Regex::new(r"^~(\d+\.\d+)$").unwrap();
  /// ">1.2"
  pub static ref GT_MINOR: Regex = Regex::new(r"^>(\d+\.\d+)$").unwrap();
  /// ">=1.2"
  pub static ref GTE_MINOR: Regex = Regex::new(r"^>=(\d+\.\d+)$").unwrap();
  /// "<1.2"
  pub static ref LT_MINOR: Regex = Regex::new(r"^<(\d+\.\d+)$").unwrap();
  /// "<=1.2"
  pub static ref LTE_MINOR: Regex = Regex::new(r"^<=(\d+\.\d+)$").unwrap();
  /// "^1"
  pub static ref CARET_MAJOR: Regex = Regex::new(r"^\^(\d+)$").unwrap();
  /// "~1"
  pub static ref TILDE_MAJOR: Regex = Regex::new(r"^~(\d+)$").unwrap();
  /// ">1"
  pub static ref GT_MAJOR: Regex = Regex::new(r"^>(\d+)$").unwrap();
  /// ">=1"
  pub static ref GTE_MAJOR: Regex = Regex::new(r"^>=(\d+)$").unwrap();
  /// "<1"
  pub static ref LT_MAJOR: Regex = Regex::new(r"^<(\d+)$").unwrap();
  /// "<=1"
  pub static ref LTE_MAJOR: Regex = Regex::new(r"^<=(\d+)$").unwrap();
  /// "1"
  pub static ref MAJOR: Regex = Regex::new(r"^(\d+)$").unwrap();
  /// "1.2"
  pub static ref MINOR: Regex = Regex::new(r"^(\d+\.\d+)$").unwrap();
  /// "npm:"
  pub static ref ALIAS: Regex = Regex::new(r"^npm:@?.+@.+").unwrap();
  /// "file:"
  pub static ref FILE: Regex = Regex::new(r"^file:").unwrap();
  /// "workspace:"
  pub static ref WORKSPACE_PROTOCOL: Regex = Regex::new(r"^workspace:").unwrap();
  /// "https://"
  pub static ref URL: Regex = Regex::new(r"^https?://").unwrap();
  /// "git+ssh://git@github.com/npm/cli"
  /// "git@github.com:npm/cli.git"
  /// "github:uNetworking/uWebSockets.js"
  /// "git://github.com/user/foo"
  pub static ref GIT: Regex = Regex::new(r"^git(://|hub:|@|\+(https|ssh)://)").unwrap();
  /// "alpha"
  pub static ref TAG: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
  /// infix operators in complex semver ranges
  /// "1.0.0 || 2.0.0"
  /// "1.0.0 - 2.0.0"
  /// "<1.0.0 >2.0.0"
  pub static ref INFIX_OPERATORS:Regex = Regex::new(r" ?(-|\|\|) ?| ").unwrap();
}
