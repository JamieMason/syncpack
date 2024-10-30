use {super::regexes, log::debug};

/// Convert non-semver specifiers to semver when behaviour is identical
pub fn sanitise(specifier: &str) -> &str {
  if specifier == "latest" || specifier == "x" {
    debug!("Sanitising specifier: {} → *", specifier);
    "*"
  } else {
    specifier
  }
}

pub fn is_simple_semver(str: &str) -> bool {
  is_exact(str) || is_latest(str) || is_major(str) || is_minor(str) || is_range(str) || is_range_major(str) || is_range_minor(str)
}

pub fn is_exact(str: &str) -> bool {
  regexes::EXACT.is_match(str) || regexes::EXACT_TAG.is_match(str)
}

pub fn is_latest(str: &str) -> bool {
  str == "*" || str == "latest" || str == "x"
}

pub fn is_major(str: &str) -> bool {
  regexes::MAJOR.is_match(str)
}

pub fn is_minor(str: &str) -> bool {
  regexes::MINOR.is_match(str)
}

pub fn is_range(specifier: &str) -> bool {
  regexes::CARET.is_match(specifier)
    || regexes::CARET_TAG.is_match(specifier)
    || regexes::TILDE.is_match(specifier)
    || regexes::TILDE_TAG.is_match(specifier)
    || regexes::GT.is_match(specifier)
    || regexes::GT_TAG.is_match(specifier)
    || regexes::GTE.is_match(specifier)
    || regexes::GTE_TAG.is_match(specifier)
    || regexes::LT.is_match(specifier)
    || regexes::LT_TAG.is_match(specifier)
    || regexes::LTE.is_match(specifier)
    || regexes::LTE_TAG.is_match(specifier)
}

pub fn is_range_major(specifier: &str) -> bool {
  regexes::CARET_MAJOR.is_match(specifier)
    || regexes::TILDE_MAJOR.is_match(specifier)
    || regexes::GT_MAJOR.is_match(specifier)
    || regexes::GTE_MAJOR.is_match(specifier)
    || regexes::LT_MAJOR.is_match(specifier)
    || regexes::LTE_MAJOR.is_match(specifier)
}

pub fn is_range_minor(specifier: &str) -> bool {
  regexes::CARET_MINOR.is_match(specifier)
    || regexes::TILDE_MINOR.is_match(specifier)
    || regexes::GT_MINOR.is_match(specifier)
    || regexes::GTE_MINOR.is_match(specifier)
    || regexes::LT_MINOR.is_match(specifier)
    || regexes::LTE_MINOR.is_match(specifier)
}

/// Is this a semver range containing multiple parts?
/// Such as OR (" || ") or AND (" ")
pub fn is_complex_range(specifier: &str) -> bool {
  regexes::OR_OPERATOR
    .split(specifier)
    .map(|str| str.trim())
    .filter(|str| !str.is_empty())
    .all(|or_condition| {
      or_condition
        .split(' ')
        .map(|str| str.trim())
        .filter(|str| !str.is_empty())
        .all(is_simple_semver)
    })
}

pub fn is_tag(str: &str) -> bool {
  regexes::TAG.is_match(str)
}

pub fn is_workspace_protocol(str: &str) -> bool {
  regexes::WORKSPACE_PROTOCOL.is_match(str)
}

pub fn is_alias(str: &str) -> bool {
  regexes::ALIAS.is_match(str)
}

pub fn is_git(str: &str) -> bool {
  regexes::GIT.is_match(str)
}

pub fn is_url(str: &str) -> bool {
  regexes::URL.is_match(str)
}

pub fn is_file(str: &str) -> bool {
  regexes::FILE.is_match(str)
}
