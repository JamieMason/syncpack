use {
  serde_json::Value,
  std::{
    env, fs,
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

/// A throwaway workspace on the real filesystem, removed on drop.
struct Workspace(PathBuf);

impl Workspace {
  fn new(name: &str, files: &[(&str, &str)]) -> Self {
    let path = env::temp_dir().join(format!("syncpack-load-errors-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("create workspace");
    let workspace = Self(path);
    for (relative_path, contents) in files {
      workspace.write(relative_path, contents);
    }
    workspace
  }

  fn write(&self, relative_path: &str, contents: &str) {
    let path = self.0.join(relative_path);
    fs::create_dir_all(path.parent().unwrap()).expect("create parent dir");
    fs::write(path, contents).expect("write file");
  }

  fn run(&self, args: &[&str]) -> (String, String, i32) {
    let output = Command::new(env!("CARGO_BIN_EXE_syncpack"))
      .args(args)
      .current_dir(&self.0)
      .output()
      .expect("failed to run syncpack");
    (
      String::from_utf8(output.stdout).unwrap(),
      String::from_utf8(output.stderr).unwrap(),
      output.status.code().unwrap_or(1),
    )
  }
}

impl Drop for Workspace {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.0);
  }
}

// — line endings —

fn crlf(lines: &[&str]) -> String {
  lines.iter().map(|line| format!("{line}\r\n")).collect()
}

fn assert_every_break_is_crlf(file: &str, text: &str) {
  let breaks = text.matches('\n').count();
  assert!(breaks > 1, "{file} should span several lines, got {text:?}");
  assert_eq!(text.matches("\r\n").count(), breaks, "{file} has a bare LF: {text:?}");
  assert_eq!(text.matches('\r').count(), breaks, "{file} has a lone CR: {text:?}");
}

#[test]
fn fix_keeps_crlf_when_it_adds_a_catalog_entry_to_pnpm_workspace_yaml() {
  let ws = Workspace::new(
    "fix-crlf-pnpm-workspace",
    &[
      ("package.json", r#"{"name":"root","version":"1.0.0"}"#),
      ("pnpm-workspace.yaml", &crlf(&["packages:", "  - 'packages/*'"])),
      (
        ".syncpackrc.json",
        r#"{"versionGroups":[{"policy":"catalog","dependencies":["react"]}]}"#,
      ),
      (
        "packages/a/package.json",
        &crlf(&[
          "{",
          r#"  "name": "a","#,
          r#"  "version": "1.0.0","#,
          r#"  "dependencies": {"#,
          r#"    "react": "^17.0.0""#,
          "  }",
          "}",
        ]),
      ),
    ],
  );
  let (_stdout, stderr, code) = ws.run(&["fix"]);
  assert_eq!(code, 0, "stderr:\n{stderr}");
  let yaml = fs::read_to_string(ws.0.join("pnpm-workspace.yaml")).unwrap();
  assert!(yaml.contains("react: ^17.0.0"), "react was not added to the catalog: {yaml:?}");
  assert_every_break_is_crlf("pnpm-workspace.yaml", &yaml);
  let package = fs::read_to_string(ws.0.join("packages/a/package.json")).unwrap();
  assert!(
    package.contains(r#""react": "catalog:""#),
    "react was not moved to the catalog: {package:?}"
  );
  assert_every_break_is_crlf("packages/a/package.json", &package);
}

const ROOT_WITH_PACKAGES_STAR: &str = r#"{"name":"root","version":"1.0.0","workspaces":["packages/*"]}"#;

#[cfg(unix)]
mod unix_filesystem {
  use {super::*, std::os::unix::fs::PermissionsExt};

  fn read_json(ws: &Workspace, relative_path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(ws.0.join(relative_path)).unwrap()).unwrap()
  }

  #[test]
  fn fix_writes_every_writable_package_json_and_names_each_one_it_could_not_write() {
    let package = |name: &str| format!(r#"{{"name":"{name}","version":"1.0.0","dependencies":{{"react":"^17.0.0"}}}}"#);
    let ws = Workspace::new(
      "fix-with-unwritable-files",
      &[
        ("package.json", ROOT_WITH_PACKAGES_STAR),
        (
          ".syncpackrc.json",
          r#"{"versionGroups":[{"dependencies":["react"],"pinVersion":"18.0.0"}]}"#,
        ),
        ("packages/a/package.json", &package("a")),
        ("packages/b/package.json", &package("b")),
        ("packages/c/package.json", &package("c")),
        ("packages/d/package.json", &package("d")),
      ],
    );
    for name in ["b", "d"] {
      fs::set_permissions(
        ws.0.join(format!("packages/{name}/package.json")),
        fs::Permissions::from_mode(0o444),
      )
      .unwrap();
    }
    if fs::OpenOptions::new()
      .write(true)
      .open(ws.0.join("packages/b/package.json"))
      .is_ok()
    {
      eprintln!("skipped: permissions are not enforced for this user (running as root?)");
      return;
    }
    let (_stdout, stderr, code) = ws.run(&["fix"]);
    assert_eq!(code, 1, "stderr:\n{stderr}");
    // The reporter prints workspace-relative paths, so only the error can contain the full path.
    let root = ws.0.canonicalize().unwrap();
    for name in ["b", "d"] {
      let path = root.join(format!("packages/{name}/package.json"));
      assert!(stderr.contains(&path.display().to_string()), "stderr:\n{stderr}");
    }
    for name in ["a", "c"] {
      let written = read_json(&ws, &format!("packages/{name}/package.json"));
      assert_eq!(written["dependencies"]["react"], "18.0.0", "{name} after fix:\n{written}");
    }
  }
}
