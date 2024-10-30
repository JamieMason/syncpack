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
  pub static ref ALIAS: Regex = Regex::new(r"^npm:").unwrap();
  /// "file:"
  pub static ref FILE: Regex = Regex::new(r"^file:").unwrap();
  /// "workspace:"
  pub static ref WORKSPACE_PROTOCOL: Regex = Regex::new(r"^workspace:").unwrap();
  /// "https://"
  pub static ref URL: Regex = Regex::new(r"^https?://").unwrap();
  /// "git://"
  pub static ref GIT: Regex = Regex::new(r"^git(\+(ssh|https?))?://").unwrap();
  /// "alpha"
  pub static ref TAG: Regex = Regex::new(r"^[a-zA-Z0-9-]+$").unwrap();
  /// a logical OR in a semver range
  pub static ref OR_OPERATOR:Regex = Regex::new(r" ?\|\| ?").unwrap();
}

/// Check if a string matches any of the regexes
pub fn matches_any(regexes: Vec<&Regex>, string: &str) -> bool {
  regexes.iter().any(|re| re.is_match(string))
}
