use {
  std::{collections::HashMap, rc::Rc},
  syncpack_specifier::Specifier,
};

/// The result of fetching package versions from the npm registry.
#[derive(Debug)]
pub struct RegistryUpdates {
  /// All updates from the npm registry keyed by internal dependency name
  pub updates_by_internal_name: HashMap<String, Vec<Rc<Specifier>>>,
  /// Per-version publish timestamps (ISO 8601) keyed by internal
  /// dependency name. Used by the `update` UI to render a `~Nd` /
  /// `~Nmo` / `~N.Ny` "how stale" hint next to each version.
  pub times_by_internal_name: HashMap<String, HashMap<String, String>>,
  /// The internal names of all failed updates
  pub failed: Vec<String>,
}
