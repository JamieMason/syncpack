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
