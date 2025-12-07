use crate::pattern_matcher::PatternMatcher;

#[test]
fn from_pattern_exact() {
  let matcher = PatternMatcher::from_pattern("react");
  assert!(matches!(matcher, PatternMatcher::Exact(_)));
  assert!(matcher.is_match("react"));
  assert!(!matcher.is_match("react-dom"));
  assert!(!matcher.is_match("preact"));
}

#[test]
fn from_pattern_prefix_with_slash_star_star() {
  let matcher = PatternMatcher::from_pattern("@aws-sdk/**");
  assert!(matches!(matcher, PatternMatcher::Prefix(_)));
  assert!(matcher.is_match("@aws-sdk/client-s3"));
  assert!(matcher.is_match("@aws-sdk/client-dynamodb"));
  assert!(!matcher.is_match("@aws-sdk"));
  assert!(!matcher.is_match("@aws"));
  assert!(!matcher.is_match("aws-sdk"));
}

#[test]
fn from_pattern_prefix_scoped_package() {
  let matcher = PatternMatcher::from_pattern("@types/**");
  assert!(matches!(matcher, PatternMatcher::Prefix(_)));
  assert!(matcher.is_match("@types/node"));
  assert!(matcher.is_match("@types/react"));
  assert!(!matcher.is_match("@types"));
  assert!(!matcher.is_match("types/node"));
}

#[test]
fn from_pattern_suffix_with_star_star_slash() {
  let matcher = PatternMatcher::from_pattern("**-loader");
  assert!(matches!(matcher, PatternMatcher::Suffix(_)));
  assert!(matcher.is_match("css-loader"));
  assert!(matcher.is_match("style-loader"));
  assert!(matcher.is_match("webpack-dev-loader"));
  assert!(!matcher.is_match("loader"));
  assert!(!matcher.is_match("css"));
}

#[test]
fn from_pattern_suffix_with_star_star() {
  let matcher = PatternMatcher::from_pattern("**-test");
  assert!(matches!(matcher, PatternMatcher::Suffix(_)));
  assert!(matcher.is_match("my-test"));
  assert!(matcher.is_match("another-test"));
  assert!(!matcher.is_match("test"));
}

#[test]
fn from_pattern_glob_complex() {
  let matcher = PatternMatcher::from_pattern("**/test/**");
  assert!(matches!(matcher, PatternMatcher::Glob(_)));
  assert!(matcher.is_match("foo/test/bar"));
  assert!(matcher.is_match("test/bar"));
  assert!(!matcher.is_match("test"));
}

#[test]
fn from_pattern_glob_with_question_mark() {
  let matcher = PatternMatcher::from_pattern("react?");
  assert!(matches!(matcher, PatternMatcher::Glob(_)));
  assert!(matcher.is_match("reacta"));
  assert!(matcher.is_match("react1"));
  assert!(!matcher.is_match("react"));
  assert!(!matcher.is_match("reactab"));
}

#[test]
fn from_pattern_glob_with_brackets() {
  let matcher = PatternMatcher::from_pattern("react[0-9]");
  assert!(matches!(matcher, PatternMatcher::Glob(_)));
  assert!(matcher.is_match("react1"));
  assert!(matcher.is_match("react9"));
  assert!(!matcher.is_match("react"));
  assert!(!matcher.is_match("reacta"));
}

#[test]
fn from_pattern_prefix_with_wildcard_in_prefix_falls_back_to_glob() {
  let matcher = PatternMatcher::from_pattern("@*/client/**");
  assert!(matches!(matcher, PatternMatcher::Glob(_)));
}

#[test]
fn from_pattern_suffix_with_wildcard_in_suffix_falls_back_to_glob() {
  let matcher = PatternMatcher::from_pattern("**/*-loader");
  assert!(matches!(matcher, PatternMatcher::Glob(_)));
}

#[test]
fn is_match_handles_empty_strings() {
  let exact = PatternMatcher::from_pattern("react");
  assert!(!exact.is_match(""));

  let prefix = PatternMatcher::from_pattern("@aws/**");
  assert!(!prefix.is_match(""));

  let suffix = PatternMatcher::from_pattern("**-loader");
  assert!(!suffix.is_match(""));
}
