use {
  crate::cli::Cli,
  std::{env, fs, path::PathBuf},
};

fn write_temp_config(name: &str) -> PathBuf {
  let path = env::temp_dir().join(format!("syncpack-cli-test-{}-{name}", std::process::id()));
  fs::write(&path, "{}").expect("write temp config");
  path
}

#[test]
fn config_flag_accepts_existing_file() {
  let temp = write_temp_config("ok.json");
  let args = ["syncpack", "lint", "--config", temp.to_str().unwrap()]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

  let cli = Cli::parse(&args).expect("--config should parse");
  let config_path = cli.config_path.expect("config_path should be Some");
  assert!(config_path.is_absolute());
  assert_eq!(config_path, temp);

  let _ = fs::remove_file(&temp);
}

#[test]
fn config_flag_rejects_missing_file_with_clap_error() {
  let args = ["syncpack", "lint", "--config", "definitely-not-a-real-file.ts"]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

  let err = Cli::parse(&args).expect_err("missing file must be rejected");
  let msg = format!("{err:?}");
  assert!(msg.contains("file not found"), "expected clap-style error, got: {msg}");
}

#[test]
fn config_flag_rejects_directory_with_clap_error() {
  let dir = env::temp_dir();
  let args = ["syncpack", "lint", "--config", dir.to_str().unwrap()]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

  let err = Cli::parse(&args).expect_err("directory must be rejected");
  let msg = format!("{err:?}");
  assert!(msg.contains("not a file"), "expected clap-style error, got: {msg}");
}

mod source_mode {
  use crate::{cli::Cli, rcfile::SourceMode};

  fn args(extra: &[&str]) -> Vec<String> {
    let mut v = vec!["syncpack".to_string(), "lint".to_string()];
    v.extend(extra.iter().map(|s| s.to_string()));
    v
  }

  #[test]
  fn defaults_to_none_when_omitted() {
    let cli = Cli::parse(&args(&[])).expect("default lint should parse");
    assert_eq!(cli.source_mode, None);
  }

  #[test]
  fn parses_extend() {
    let cli = Cli::parse(&args(&["--source-mode", "extend"])).expect("--source-mode extend should parse");
    assert_eq!(cli.source_mode, Some(SourceMode::Extend));
  }

  #[test]
  fn parses_replace() {
    let cli = Cli::parse(&args(&["--source-mode", "replace"])).expect("--source-mode replace should parse");
    assert_eq!(cli.source_mode, Some(SourceMode::Replace));
  }

  #[test]
  fn rejects_invalid_value() {
    let err = Cli::parse(&args(&["--source-mode", "merge"])).expect_err("invalid value must fail");
    let msg = format!("{err:?}");
    assert!(
      msg.contains("merge") || msg.contains("invalid value"),
      "expected clap-style invalid-value error, got: {msg}"
    );
  }
}
