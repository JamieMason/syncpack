pub mod dependency_type;

pub use dependency_type::{DependencyType, Strategy};

/// Registry URL for fetching package metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UpdateUrl {
  pub internal_name: String,
  /// e.g. `"https://registry.npmjs.org/react"`.
  pub url: String,
}
