use {
  serde_json::Value,
  std::{
    path::{Path, PathBuf},
    process::Command,
  },
};

fn fixtures_dir() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

fn run_json(fixture: &str, extra_args: &[&str]) -> String {
  let mut args = vec!["json"];
  args.extend_from_slice(extra_args);
  let output = Command::new(env!("CARGO_BIN_EXE_syncpack"))
    .args(&args)
    .current_dir(fixtures_dir().join(fixture))
    .output()
    .expect("failed to run syncpack");
  String::from_utf8(output.stdout).expect("syncpack stdout was not valid UTF-8")
}

fn package_paths(stdout: &str) -> Vec<PathBuf> {
  stdout
    .lines()
    .filter(|line| !line.is_empty())
    .filter_map(|line| serde_json::from_str::<Value>(line).ok())
    .filter_map(|v| v.get("package").and_then(Value::as_str).map(PathBuf::from))
    .collect()
}

fn ends_with(paths: &[PathBuf], suffix: &str) -> bool {
  paths.iter().any(|p| p.ends_with(Path::new(suffix)))
}

fn has_component(paths: &[PathBuf], component: &str) -> bool {
  paths.iter().any(|p| p.components().any(|c| c.as_os_str() == component))
}

// — issue-311 —

#[test]
fn issue_311_packages_outside_workspace_globs_are_excluded() {
  let paths = package_paths(&run_json("issue-311", &[]));
  assert!(
    !has_component(&paths, "do-not-include"),
    "expected 'do-not-include' to be excluded by workspace globs, got: {paths:?}",
  );
}

// — issue-319 —

#[test]
fn issue_319_negative_globs_exclude_packages() {
  let paths = package_paths(&run_json("issue-319", &[]));
  assert!(
    !ends_with(&paths, "apps/test2/package.json"),
    "expected 'apps/test2/package.json' to be excluded by negative glob, got: {paths:?}",
  );
}

// — issue-334 —

#[test]
fn issue_334_dot_next_directory_excluded_by_gitignore() {
  let paths = package_paths(&run_json("issue-334", &[]));
  assert!(
    !has_component(&paths, ".next"),
    "expected '.next' directory to be excluded by .gitignore, got: {paths:?}",
  );
}

#[test]
fn issue_334_dist_directory_excluded_by_gitignore() {
  let paths = package_paths(&run_json("issue-334", &[]));
  assert!(
    !has_component(&paths, "dist"),
    "expected 'dist' directory to be excluded by .gitignore, got: {paths:?}",
  );
}

#[test]
fn issue_334_single_star_does_not_match_deeply_nested() {
  let paths = package_paths(&run_json("issue-334", &[]));
  assert!(
    !ends_with(&paths, "apps/test/nested/package.json"),
    "expected 'apps/test/nested/package.json' to NOT match 'apps/*', got: {paths:?}",
  );
}

#[test]
fn issue_334_node_modules_pruned() {
  let paths = package_paths(&run_json("issue-334", &[]));
  assert!(
    !has_component(&paths, "node_modules"),
    "expected 'node_modules' to be pruned, got: {paths:?}",
  );
}

#[test]
fn issue_334_nested_gitignore_excludes_build_directory() {
  let paths = package_paths(&run_json("issue-334", &[]));
  assert!(
    !has_component(&paths, "build"),
    "expected 'apps/test/build' to be excluded by nested .gitignore, got: {paths:?}",
  );
}

#[test]
fn issue_334_double_star_matches_deeply_nested() {
  let paths = package_paths(&run_json("issue-334", &["--source", "apps/**/package.json"]));
  assert!(
    ends_with(&paths, "apps/test/nested/package.json"),
    "expected 'apps/**' to match 'apps/test/nested/package.json', got: {paths:?}",
  );
}

#[test]
fn issue_334_bare_package_json_matches_root_only() {
  let paths = package_paths(&run_json("issue-334", &["--source", "package.json"]));
  assert!(
    ends_with(&paths, "issue-334/package.json"),
    "expected root 'package.json' to be matched, got: {paths:?}",
  );
  for path in &paths {
    assert!(
      !path
        .components()
        .any(|c| matches!(c.as_os_str().to_str(), Some("apps" | "node_modules"))),
      "expected bare 'package.json' to match root only, but got nested path: {path:?}",
    );
  }
}

// — defaults —

#[test]
fn defaults_pick_up_root_and_packages_star_package_json() {
  let paths = package_paths(&run_json("defaults", &[]));
  assert!(
    ends_with(&paths, "defaults/package.json"),
    "expected root 'package.json' to be picked up, got: {paths:?}",
  );
  assert!(
    ends_with(&paths, "defaults/packages/foo/package.json"),
    "expected 'packages/foo/package.json' to be picked up, got: {paths:?}",
  );
  assert!(
    !ends_with(&paths, "defaults/packages/foo/nested/package.json"),
    "expected 'packages/foo/nested/package.json' to NOT match 'packages/*/package.json', got: {paths:?}",
  );
  assert!(
    !ends_with(&paths, "defaults/other/bar/package.json"),
    "expected 'other/bar/package.json' to NOT match defaults, got: {paths:?}",
  );
}
