use {
  itertools::Itertools,
  std::cmp::Ordering,
  syncpack_model::dependency::{Dependency, DependencyIdx},
  syncpack_rules::version_group::{VersionGroup, VersionGroupBehavior},
  syncpack_vocab::severity::Severity,
};

/// Order a version group's dependencies for display. Command-time sorting keyed
/// off the CLI's `--sort`, so it lives in the bin rather than the pure rules
/// engine.
pub fn get_sorted_dependencies(group: &VersionGroup, arena: &[Dependency], sort: &crate::cli::SortBy) -> Vec<DependencyIdx> {
  group
    .dependency_indices()
    .iter()
    .copied()
    .sorted_by(|a, b| match sort {
      crate::cli::SortBy::Count => arena[b.0].instances.len().cmp(&arena[a.0].instances.len()),
      crate::cli::SortBy::Name => Ordering::Equal,
    })
    .collect()
}

/// Map a resolved severity to the action a command should take. Pure function
/// of `severity` — commands branch on this at report/apply time.
pub fn action_of(severity: Severity) -> InstanceAction {
  match severity {
    Severity::None => InstanceAction::Valid,
    Severity::Fix => InstanceAction::Fix(Severity::Fix),
    Severity::Warn | Severity::Error => InstanceAction::Render(severity),
  }
}

/// Resolver output consumed by each command's iteration loop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstanceAction {
  /// Do nothing — instance is Valid or Unknown.
  Valid,
  /// Render as warning or error; never apply the fix.
  Render(Severity),
  /// Apply the fix; render under `fix`/`lint`.
  Fix(Severity),
}

/// Write fixes to disk
pub mod fix;

/// DEPRECATED: Use fix instead
pub mod fix_mismatches;
/// Lint and fix package.json formatting
pub mod format;
/// Output all dependencies as flattened JSON objects
pub mod json;
/// Write lint messages to the UI
pub mod lint;
/// DEPRECATED: Use lint instead
pub mod lint_semver_ranges;
/// Query and list all instances in the project
pub mod list;
/// DEPRECATED: Use lint instead
pub mod list_mismatches;
/// DEPRECATED: Not yet implemented in v14
pub mod prompt;
/// Reporter traits for fix and format output
pub mod reporter;
/// DEPRECATED: Use fix instead
pub mod set_semver_ranges;
/// A shared module with methods for printing messages to the console
pub mod ui;
/// Find and apply updates from the npm registry
pub mod update;
