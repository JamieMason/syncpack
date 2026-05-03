use {super::get_javascript_contents, std::path::Path};

#[test]
fn import_uses_path_to_file_url() {
  let script = get_javascript_contents(Path::new("/home/jamie/syncpack.config.cjs"));
  assert!(script.contains("import(require('node:url').pathToFileURL("));
}

#[test]
fn require_uses_raw_path() {
  let script = get_javascript_contents(Path::new("/home/jamie/syncpack.config.cjs"));
  assert!(script.contains("require('/home/jamie/syncpack.config.cjs')"));
}

#[test]
fn escapes_windows_backslashes() {
  let script = get_javascript_contents(Path::new(r"C:\Users\jamie\syncpack.config.cjs"));
  assert!(script.contains(r"C:\\Users\\jamie\\syncpack.config.cjs"));
  assert!(!script.contains(r"'C:\Users"));
}

#[test]
fn escapes_single_quotes() {
  let script = get_javascript_contents(Path::new("/home/jamie's projects/syncpack.config.cjs"));
  assert!(script.contains(r"/home/jamie\'s projects/syncpack.config.cjs"));
}
