use {
  crate::{context::Config, disk::Disk},
  log::debug,
};

#[cfg(test)]
#[path = "source_patterns_test.rs"]
mod source_patterns_test;

trait DebugNone {
  fn debug_none(self, msg: &str) -> Self;
}

impl<T> DebugNone for Option<T> {
  fn debug_none(self, msg: &str) -> Self {
    if self.is_none() {
      debug!("{msg}");
    }
    self
  }
}

/// Based on the user's config file and command line `--source` options, return
/// the source glob patterns which should be used to resolve package.json files
pub fn get_source_patterns(config: &Config, disk: &Disk) -> Vec<String> {
  get_cli_patterns(config)
    .debug_none("No --source patterns provided")
    .or_else(|| get_rcfile_patterns(config))
    .debug_none("No .source patterns in rcfile")
    .or_else(|| {
      get_npm_and_yarn_patterns(disk)
        .debug_none("No workspaces patterns in package.json")
        .or_else(|| get_pnpm_patterns(disk))
        .debug_none("No packages in pnpm-workspace.yaml")
        .or_else(|| get_lerna_patterns(disk))
        .debug_none("No packages in lerna.json")
        .map(append_root_package_json)
    })
    .map(normalise_patterns)
    .debug_none("Using default source patterns")
    .unwrap_or_else(get_default_patterns)
}

/// Get source patterns provided via the `--source` CLI option
fn get_cli_patterns(config: &Config) -> Option<Vec<String>> {
  (!config.cli.source_patterns.is_empty()).then(|| config.cli.source_patterns.clone())
}

/// Get source patterns from the syncpack config file
fn get_rcfile_patterns(config: &Config) -> Option<Vec<String>> {
  (!config.rcfile.source.is_empty()).then(|| config.rcfile.source.clone())
}

/// Look for source patterns in the `package.json` file in the locations
/// searched by `npm` and `yarn`
fn get_npm_and_yarn_patterns(disk: &Disk) -> Option<Vec<String>> {
  let contents = &disk.package_json_root()?.contents;
  contents
    .pointer("/workspaces/packages")
    .or_else(|| contents.get("workspaces"))
    .and_then(|v| v.as_array())
    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
}

/// Look for source patterns in the `pnpm-workspace.yaml` file
fn get_pnpm_patterns(disk: &Disk) -> Option<Vec<String>> {
  disk
    .pnpm_workspace
    .as_ref()?
    .contents
    .get("packages")
    .and_then(|v| v.as_sequence())
    .map(|seq| seq.iter().filter_map(|v| v.as_str().map(String::from)).collect())
}

/// Look for source patterns in the `lerna.json` file
fn get_lerna_patterns(disk: &Disk) -> Option<Vec<String>> {
  disk
    .lerna_json
    .as_ref()?
    .contents
    .get("packages")
    .and_then(|v| v.as_array())
    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
}

/// Default source patterns to use if no other source patterns are found
fn get_default_patterns() -> Vec<String> {
  vec![String::from("/package.json"), String::from("packages/*/package.json")]
}

fn append_root_package_json(mut patterns: Vec<String>) -> Vec<String> {
  patterns.push("package.json".to_string());
  patterns
}

fn normalise_patterns(patterns: Vec<String>) -> Vec<String> {
  patterns.into_iter().map(normalise_pattern).collect()
}

/// Normalize a source pattern by:
/// 1. Preserving negation prefix (`!`) through normalization
/// 2. Converting Windows backslashes to forward slashes for glob compatibility
/// 3. Ensuring pattern ends with /package.json
/// 4. Anchoring slashless patterns to the workspace root with a leading `/`
///    so gitignore basename rules don't make them match at any depth
///
/// Examples:
/// - "projects\\apps\\*" -> "projects/apps/*/package.json"
/// - "projects/libs/*" -> "projects/libs/*/package.json"
/// - "package.json" -> "/package.json"
/// - "apps\\*/package.json" -> "apps/*/package.json"
/// - "!apps/test2" -> "!apps/test2/package.json"
pub fn normalise_pattern(mut pattern: String) -> String {
  let negated = pattern.starts_with('!');
  if negated {
    pattern.remove(0);
  }
  let mut normalized = pattern.replace('\\', "/");
  if !normalized.contains("package.json") {
    normalized = format!("{normalized}/package.json");
  }
  if !normalized.contains('/') {
    normalized = format!("/{normalized}");
  }
  if negated {
    format!("!{normalized}")
  } else {
    normalized
  }
}
