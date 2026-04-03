pub mod dependency_type;

pub use dependency_type::{DependencyType, Strategy};

/// URL information for fetching package metadata from npm registry.
/// Used by the update command to fetch available versions.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UpdateUrl {
  /// The name of the dependency
  pub internal_name: String,
  /// Registry URL, e.g., "https://registry.npmjs.org/react"
  pub url: String,
}
