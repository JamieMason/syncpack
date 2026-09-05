use {
  serde::{Deserialize, Serialize},
  std::collections::HashMap,
};

/// User-tunable (or internally-defaulted) treatment of an `InstanceState`
/// for a particular instance. See `.plans/severity.md` §3.1.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
  /// Apply the fix; instance becomes valid.
  Fix,
  /// Render as warning; does not flip exit code.
  Warn,
  /// Render as error; flips exit code to `IssuesFound`.
  Error,
  /// JSON-only: emitted for `Valid` / `Unknown` instances where the resolver
  /// returns `Valid`. Not user-deserialisable — writing `"none"` in rcfile
  /// severity maps fails serde.
  #[serde(skip_deserializing)]
  None,
}

pub type SeverityMap = HashMap<String, Severity>;
