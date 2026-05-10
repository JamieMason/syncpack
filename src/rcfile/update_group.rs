use {
  crate::{errors::UnsupportedConfigError, group_selector::GroupSelector},
  serde::Deserialize,
  serde_json::Value,
  std::collections::HashMap,
  syncpack_specifier::update_target::UpdateTarget,
};

#[derive(Debug)]
pub struct UpdateGroup {
  pub selector: GroupSelector,
  pub policy: UpdatePolicy,
}

#[derive(Debug, Clone)]
pub enum UpdatePolicy {
  Skip,
  /// Clamp eligible registry updates to no greater than this target.
  UpTo(UpdateTarget),
}

impl UpdateGroup {
  pub fn from_config(group: AnyUpdateGroup) -> Result<UpdateGroup, UnsupportedConfigError> {
    let selector = GroupSelector::new(
      /* include_dependencies: */ group.dependencies,
      /* include_dependency_types: */ group.dependency_types,
      /* label: */ group.label,
      /* include_packages: */ group.packages,
      /* include_specifier_types: */ group.specifier_types,
    );
    let policy = match (group.is_ignored, group.target.as_deref()) {
      (Some(true), None) => UpdatePolicy::Skip,
      (None | Some(false), Some("patch")) => UpdatePolicy::UpTo(UpdateTarget::Patch),
      (None | Some(false), Some("minor")) => UpdatePolicy::UpTo(UpdateTarget::Minor),
      (None | Some(false), Some("latest")) => UpdatePolicy::UpTo(UpdateTarget::Latest),
      (None | Some(false), Some(_)) => return Err(UnsupportedConfigError::InvalidUpdateGroup),
      (Some(true), Some(_)) => return Err(UnsupportedConfigError::InvalidUpdateGroup),
      (None | Some(false), None) => return Err(UnsupportedConfigError::InvalidUpdateGroup),
    };
    Ok(UpdateGroup { selector, policy })
  }
}

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
