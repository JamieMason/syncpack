use serde::Deserialize;

/// How user-supplied source patterns combine with workspace discovery.
///
/// - `Replace` (default): user patterns from `--source` or rcfile `source` replace the discovered patterns entirely. Discovery is skipped.
/// - `Extend`: user patterns are appended to discovered patterns so a project can keep its `workspaces` / `pnpm-workspace.yaml` /
///   `lerna.json` packages *and* pull in extra paths.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SourceMode {
  #[default]
  Replace,
  Extend,
}

impl std::str::FromStr for SourceMode {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "replace" => Ok(SourceMode::Replace),
      "extend" => Ok(SourceMode::Extend),
      other => Err(format!("invalid value '{other}' for sourceMode: expected 'replace' or 'extend'")),
    }
  }
}
