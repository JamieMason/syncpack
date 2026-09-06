use {
  serde_json::Value,
  std::{collections::BTreeMap, env, process::Command},
};

/// Permitted intra-workspace dependencies for a `syncpack-*` library crate:
/// `normal` = the production `[dependencies]` graph, `dev` = `[dev-dependencies]`.
struct Allowed {
  normal: &'static [&'static str],
  dev: &'static [&'static str],
}

/// The strict crate DAG, encoded as an allowlist. Cargo already rejects cycles
/// at compile time; this guards the other direction — a *forbidden edge* (e.g.
/// `rules -> disk`) that cargo would happily compile. Dev-dependencies cross
/// only for `io`'s `test-support` mocks (`disk`/`registry` dev-dep `io`), so
/// they get their own tighter list rather than a free pass. The bin crate
/// `syncpack` depends on everything by design and is unrestricted (skipped
/// below).
fn allowlist() -> BTreeMap<&'static str, Allowed> {
  BTreeMap::from([
    ("syncpack-specifier", Allowed { normal: &[], dev: &[] }),
    ("syncpack-io", Allowed { normal: &[], dev: &[] }),
    (
      "syncpack-vocab",
      Allowed {
        normal: &["syncpack-specifier"],
        dev: &[],
      },
    ),
    (
      "syncpack-config",
      Allowed {
        normal: &["syncpack-vocab", "syncpack-specifier"],
        dev: &[],
      },
    ),
    (
      "syncpack-disk",
      Allowed {
        normal: &["syncpack-io", "syncpack-vocab", "syncpack-specifier"],
        dev: &["syncpack-io"],
      },
    ),
    (
      "syncpack-registry",
      Allowed {
        normal: &["syncpack-io", "syncpack-vocab", "syncpack-specifier"],
        dev: &["syncpack-io"],
      },
    ),
    (
      "syncpack-model",
      Allowed {
        normal: &["syncpack-vocab", "syncpack-specifier"],
        dev: &[],
      },
    ),
    (
      "syncpack-rules",
      Allowed {
        normal: &["syncpack-model", "syncpack-vocab", "syncpack-specifier"],
        dev: &[],
      },
    ),
  ])
}

/// True for this workspace's own crates (the bin `syncpack` and every
/// `syncpack-*` library), false for third-party dependencies.
fn is_workspace_crate(name: &str) -> bool {
  name == "syncpack" || name.starts_with("syncpack-")
}

#[test]
fn crate_dependencies_obey_the_allowlisted_dag() {
  let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
  let output = Command::new(cargo)
    .args(["metadata", "--format-version", "1", "--no-deps"])
    .output()
    .expect("cargo metadata should run");
  assert!(
    output.status.success(),
    "cargo metadata failed: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let metadata: Value = serde_json::from_slice(&output.stdout).expect("cargo metadata should emit valid json");
  let packages = metadata["packages"].as_array().expect("metadata.packages array");

  let allowlist = allowlist();
  let mut violations: Vec<String> = vec![];

  for package in packages {
    let name = package["name"].as_str().expect("package name");
    // Skip third-party crates and the unrestricted bin.
    if !is_workspace_crate(name) || name == "syncpack" {
      continue;
    }
    let allowed = allowlist.get(name).unwrap_or_else(|| {
      panic!(
        "crate `{name}` is missing from the DAG allowlist — add it to `allowlist()` in src/dag_guard_test.rs with its permitted dependencies"
      )
    });
    let dependencies = package["dependencies"].as_array().expect("package.dependencies array");
    for dependency in dependencies {
      let dependency_name = dependency["name"].as_str().expect("dependency name");
      if !is_workspace_crate(dependency_name) {
        continue;
      }
      // Normal (kind: null) and build (kind: "build") deps form the production
      // graph; dev-deps have their own tighter allowlist (test-support only).
      let (permitted, kind_label) = match dependency["kind"].as_str() {
        Some("dev") => (allowed.dev, "dev"),
        Some(other) => (allowed.normal, other),
        None => (allowed.normal, "normal"),
      };
      if !permitted.contains(&dependency_name) {
        violations.push(format!("  {name} -[{kind_label}]-> {dependency_name} (allowed: {permitted:?})"));
      }
    }
  }

  assert!(
    violations.is_empty(),
    "forbidden crate dependencies break the DAG encoded in `allowlist()` (src/dag_guard_test.rs):\n{}",
    violations.join("\n")
  );
}
