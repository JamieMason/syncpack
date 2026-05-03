use std::{path::PathBuf, process::Command};

fn fixture_dir() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/js-config-loading")
}

fn run_syncpack(args: &[&str]) -> (String, String, i32) {
  let output = Command::new(env!("CARGO_BIN_EXE_syncpack"))
    .args(args)
    .current_dir(fixture_dir())
    .output()
    .expect("failed to run syncpack");
  let stdout = String::from_utf8(output.stdout).unwrap();
  let stderr = String::from_utf8(output.stderr).unwrap();
  let code = output.status.code().unwrap_or(1);
  (stdout, stderr, code)
}

#[test]
fn loads_cjs_config_without_error() {
  let (_stdout, stderr, code) = run_syncpack(&["list"]);
  assert_eq!(code, 0, "syncpack list exited {code}\nstderr:\n{stderr}");
  assert!(
    !stderr.contains("ERR_UNSUPPORTED_ESM_URL_SCHEME"),
    "regression of issue #327:\n{stderr}"
  );
}
