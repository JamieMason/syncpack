use crate::rcfile::Rcfile;

#[test]
fn default_format_bugs_is_false() {
  let rcfile = Rcfile::default();
  assert!(!rcfile.format_bugs);
}

#[test]
fn default_format_repository_is_false() {
  let rcfile = Rcfile::default();
  assert!(!rcfile.format_repository);
}
