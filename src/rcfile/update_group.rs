use {serde::Deserialize, serde_json::Value, std::collections::HashMap};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyUpdateGroup {
  #[serde(default)]
  pub dependencies: Vec<String>,
  #[serde(default)]
  pub dependency_types: Vec<String>,
  #[serde(default)]
  pub label: String,
  #[serde(default)]
  pub packages: Vec<String>,
  #[serde(default)]
  pub specifier_types: Vec<String>,
  //
  pub is_ignored: Option<bool>,
  pub target: Option<String>,
  #[serde(flatten)]
  pub unknown_fields: HashMap<String, Value>,
}
