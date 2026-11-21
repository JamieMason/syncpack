use std::{
  env, fs,
  path::PathBuf,
  process::Command,
  time::{SystemTime, UNIX_EPOCH},
};

const PACKAGE_JSON_WITH_REACT: &str = r#"{ "name": "a", "version": "1.0.0", "dependencies": { "react": "1.0.0" } }"#;

/// A temp dir used as both the cwd and `XDG_CACHE_HOME`, removed on drop.
struct Workspace(PathBuf);

impl Workspace {
  fn new(name: &str, package_json: &str) -> Self {
    let root = env::temp_dir().join(format!("syncpack-registry-cache-{}-{name}", std::process::id()));
    fs::create_dir_all(&root).expect("create workspace dir");
    fs::write(root.join("package.json"), package_json).expect("write package.json");
    Self(root)
  }

  fn cache_file(&self) -> PathBuf {
    self.0.join("syncpack").join("cache.json")
  }

  fn seed_cache(&self, contents: &str) {
    fs::create_dir_all(self.cache_file().parent().unwrap()).expect("create cache dir");
    fs::write(self.cache_file(), contents).expect("write cache.json");
  }

  fn run(&self, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_syncpack"))
      .args(args)
      .current_dir(&self.0)
      .env("XDG_CACHE_HOME", &self.0)
      .output()
      .expect("failed to run syncpack");
    String::from_utf8_lossy(&output.stderr).into_owned()
  }
}

impl Drop for Workspace {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.0);
  }
}

#[test]
fn corrupt_cache_file_is_left_untouched_by_lint() {
  let workspace = Workspace::new("corrupt", PACKAGE_JSON_WITH_REACT);
  workspace.seed_cache("not valid json");

  workspace.run(&["lint"]);

  assert_eq!(fs::read_to_string(workspace.cache_file()).unwrap(), "not valid json");
}

#[test]
fn update_reads_fresh_cache_entries_and_leaves_the_file_untouched() {
  let workspace = Workspace::new("update-cache-hit", PACKAGE_JSON_WITH_REACT);
  let fresh = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
  let seed = format!(
    r#"{{
  "entries": {{
    "https://registry.npmjs.org/react": {{
      "cached_at": {fresh},
      "data": {{ "name": "react", "versions": ["99.0.0"] }}
    }}
  }}
}}"#
  );
  workspace.seed_cache(&seed);

  let output = workspace.run(&["update", "--dry-run"]);

  assert!(output.contains("99.0.0"), "update ignored the cached entry:\n{output}");
  assert_eq!(
    fs::read_to_string(workspace.cache_file()).unwrap(),
    seed,
    "update rewrote a cache it only read"
  );
}
