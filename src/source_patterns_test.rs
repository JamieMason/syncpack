use crate::source_patterns::{get_source_patterns, normalise_pattern};

#[test]
fn normalizes_backslashes_to_forward_slashes() {
  let windows_backslashes = [
    ("projects\\apps\\*", "projects/apps/*/package.json"),
    ("projects\\libs\\lib1", "projects/libs/lib1/package.json"),
    ("apps\\*\\src", "apps/*/src/package.json"),
  ];
  let mixed_slashes = [
    ("projects\\mixed/pkg1", "projects/mixed/pkg1/package.json"),
    ("apps/test\\utils", "apps/test/utils/package.json"),
  ];
  let forward_slashes = [
    ("projects/apps/*", "projects/apps/*/package.json"),
    ("packages/*", "packages/*/package.json"),
  ];
  let backslashes_with_package_json = [
    ("apps\\*/package.json", "apps/*/package.json"),
    ("projects\\libs\\*\\package.json", "projects/libs/*/package.json"),
  ];
  let forward_slashes_with_package_json = [
    ("apps/*/package.json", "apps/*/package.json"),
    ("packages/*/package.json", "packages/*/package.json"),
  ];
  let bare_package_json = [("package.json", "/package.json")];
  let glob_patterns = [
    ("**\\*\\package.json", "**/*/package.json"),
    ("src\\**\\tests", "src/**/tests/package.json"),
  ];
  let negated_globs = [
    ("!apps/test2", "!apps/test2/package.json"),
    ("!packages/*", "!packages/*/package.json"),
    ("!apps/test2/package.json", "!apps/test2/package.json"),
    ("!projects\\apps\\*", "!projects/apps/*/package.json"),
  ];
  let non_standard_manifest_names = [
    ("packages/*/package.public.json", "packages/*/package.public.json"),
    ("packages/*/package.private.json", "packages/*/package.private.json"),
    ("packages/foo/manifest.json", "packages/foo/manifest.json"),
    ("package.public.json", "/package.public.json"),
    ("packages\\*\\package.public.json", "packages/*/package.public.json"),
    ("packages/*/*.json", "packages/*/*.json"),
    ("!packages/foo/package.public.json", "!packages/foo/package.public.json"),
    ("!packages/*/*.json", "!packages/*/*.json"),
  ];

  let cases = windows_backslashes
    .iter()
    .chain(mixed_slashes.iter())
    .chain(forward_slashes.iter())
    .chain(backslashes_with_package_json.iter())
    .chain(forward_slashes_with_package_json.iter())
    .chain(bare_package_json.iter())
    .chain(glob_patterns.iter())
    .chain(negated_globs.iter())
    .chain(non_standard_manifest_names.iter());

  for (input, expected) in cases {
    let result = normalise_pattern(input.to_string());
    assert_eq!(result, *expected, "normalize_pattern({input:?}) should return {expected:?}");
  }
}

mod source_mode {
  use {
    super::get_source_patterns,
    crate::{
      context::Config,
      disk::{Disk, File, detect_formatting},
      rcfile::{RawRcfile, Rcfile, SourceMode},
      test::mock,
    },
    serde_json::{Value, json},
    std::path::PathBuf,
  };

  fn empty_disk() -> Disk {
    Disk {
      cwd: PathBuf::from("/test"),
      lerna_json: None,
      package_json_files: vec![],
      package_json_root_idx: None,
      package_manager: None,
      pnpm_workspace: None,
    }
  }

  fn disk_with_root(root: Value) -> Disk {
    let raw = serde_json::to_string_pretty(&root).unwrap();
    let file = File {
      filepath: PathBuf::from("/test/package.json"),
      formatting: detect_formatting(&raw),
      contents: root,
      dirty: false,
    };
    Disk {
      cwd: PathBuf::from("/test"),
      lerna_json: None,
      package_json_files: vec![file],
      package_json_root_idx: Some(0),
      package_manager: None,
      pnpm_workspace: None,
    }
  }

  fn disk_with_npm_workspaces(patterns: &[&str]) -> Disk {
    let ws: Vec<Value> = patterns.iter().map(|s| json!(s)).collect();
    disk_with_root(json!({ "name": "root", "workspaces": ws }))
  }

  fn disk_with_pnpm(patterns: &[&str]) -> Disk {
    let lines: Vec<String> = patterns.iter().map(|p| format!("  - \"{p}\"")).collect();
    let yaml = format!("packages:\n{}\n", lines.join("\n"));
    let mut disk = empty_disk();
    disk.pnpm_workspace = Some(mock::pnpm_yaml_file_from_str(&yaml));
    disk
  }

  fn disk_with_lerna(patterns: &[&str]) -> Disk {
    let packages: Vec<Value> = patterns.iter().map(|s| json!(s)).collect();
    let contents = json!({ "packages": packages });
    let raw = serde_json::to_string_pretty(&contents).unwrap();
    let file = File {
      filepath: PathBuf::from("/test/lerna.json"),
      formatting: detect_formatting(&raw),
      contents,
      dirty: false,
    };
    let mut disk = empty_disk();
    disk.lerna_json = Some(file);
    disk
  }

  fn config_with(rcfile_json: Value, cli_source: &[&str], cli_mode: Option<SourceMode>) -> Config {
    let raw: RawRcfile = serde_json::from_value(rcfile_json).unwrap();
    let rcfile: Rcfile = raw.try_into().unwrap();
    let mut cli = mock::cli();
    cli.source_patterns = cli_source.iter().map(|s| s.to_string()).collect();
    cli.source_mode = cli_mode;
    Config { cli, rcfile }
  }

  // ----- replace mode (default) is unchanged ---------------------------------

  #[test]
  fn replace_with_cli_returns_only_cli() {
    let config = config_with(json!({}), &["custom/*"], None);
    let disk = disk_with_npm_workspaces(&["packages/*"]);
    assert_eq!(get_source_patterns(&config, &disk), vec!["custom/*/package.json"]);
  }

  #[test]
  fn replace_with_rcfile_source_only_returns_rcfile() {
    let config = config_with(json!({ "source": ["custom/*"] }), &[], None);
    let disk = disk_with_npm_workspaces(&["packages/*"]);
    assert_eq!(get_source_patterns(&config, &disk), vec!["custom/*/package.json"]);
  }

  #[test]
  fn replace_with_no_user_input_uses_discovered() {
    let config = config_with(json!({}), &[], None);
    let disk = disk_with_npm_workspaces(&["packages/*"]);
    assert_eq!(
      get_source_patterns(&config, &disk),
      vec!["packages/*/package.json", "/package.json"],
    );
  }

  // ----- extend mode merges user patterns onto discovered --------------------

  #[test]
  fn extend_via_rcfile_appends_to_npm_discovered() {
    let config = config_with(json!({ "sourceMode": "extend", "source": ["custom/extra"] }), &[], None);
    let disk = disk_with_npm_workspaces(&["packages/*"]);
    assert_eq!(
      get_source_patterns(&config, &disk),
      vec!["packages/*/package.json", "/package.json", "custom/extra/package.json"],
    );
  }

  #[test]
  fn extend_via_rcfile_appends_to_pnpm_discovered() {
    let config = config_with(json!({ "sourceMode": "extend", "source": ["custom/extra"] }), &[], None);
    let disk = disk_with_pnpm(&["packages/*"]);
    assert_eq!(
      get_source_patterns(&config, &disk),
      vec!["packages/*/package.json", "/package.json", "custom/extra/package.json"],
    );
  }

  #[test]
  fn extend_via_rcfile_appends_to_lerna_discovered() {
    let config = config_with(json!({ "sourceMode": "extend", "source": ["custom/extra"] }), &[], None);
    let disk = disk_with_lerna(&["legacy/*"]);
    assert_eq!(
      get_source_patterns(&config, &disk),
      vec!["legacy/*/package.json", "/package.json", "custom/extra/package.json"],
    );
  }

  #[test]
  fn extend_cli_overrides_rcfile_inside_user_slice() {
    let config = config_with(json!({ "sourceMode": "extend", "source": ["from-rcfile"] }), &["from-cli"], None);
    let disk = disk_with_npm_workspaces(&["packages/*"]);
    assert_eq!(
      get_source_patterns(&config, &disk),
      vec!["packages/*/package.json", "/package.json", "from-cli/package.json"],
    );
  }

  #[test]
  fn extend_with_no_user_patterns_equals_discovered() {
    let config = config_with(json!({ "sourceMode": "extend" }), &[], None);
    let disk = disk_with_npm_workspaces(&["packages/*"]);
    assert_eq!(
      get_source_patterns(&config, &disk),
      vec!["packages/*/package.json", "/package.json"],
    );
  }

  #[test]
  fn extend_with_user_patterns_but_no_discovery_returns_only_user() {
    let config = config_with(json!({ "sourceMode": "extend", "source": ["custom/extra"] }), &[], None);
    let disk = empty_disk();
    assert_eq!(get_source_patterns(&config, &disk), vec!["custom/extra/package.json"]);
  }

  #[test]
  fn extend_with_nothing_falls_back_to_defaults() {
    let config = config_with(json!({ "sourceMode": "extend" }), &[], None);
    let disk = empty_disk();
    assert_eq!(
      get_source_patterns(&config, &disk),
      vec!["/package.json", "packages/*/package.json"],
    );
  }

  // ----- CLI sourceMode overrides rcfile sourceMode --------------------------

  #[test]
  fn cli_extend_overrides_rcfile_replace() {
    let config = config_with(
      json!({ "sourceMode": "replace", "source": ["from-rcfile"] }),
      &["from-cli"],
      Some(SourceMode::Extend),
    );
    let disk = disk_with_npm_workspaces(&["packages/*"]);
    assert_eq!(
      get_source_patterns(&config, &disk),
      vec!["packages/*/package.json", "/package.json", "from-cli/package.json"],
    );
  }

  #[test]
  fn cli_replace_overrides_rcfile_extend() {
    let config = config_with(
      json!({ "sourceMode": "extend", "source": ["from-rcfile"] }),
      &["from-cli"],
      Some(SourceMode::Replace),
    );
    let disk = disk_with_npm_workspaces(&["packages/*"]);
    assert_eq!(get_source_patterns(&config, &disk), vec!["from-cli/package.json"]);
  }
}
