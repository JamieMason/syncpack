use crate::source::Source;

#[cfg(test)]
#[path = "sources_test.rs"]
mod sources_test;

/// Index into the `Sources.all` arena.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SourceIdx(pub usize);

/// Owned arena of every `Source` (package.json + pnpm yaml) in the project.
///
/// `Source::Package` carries an index into `disk.package_json_files`;
/// reads/writes route through the Disk. `user_source_indices` lists the slots
/// in `all` whose backing file matches a user `source` pattern; iteration
/// uses this to filter non-catalog dep types in pass 2 (catalog dep types in
/// pass 1 always iterate the canonical source regardless of user pattern
/// membership).
#[derive(Debug)]
pub struct Sources {
  pub all: Vec<Source>,
  /// Slots in `all` whose file matches a user-configured `source` pattern.
  /// Pass 2 of `iter_instances` filters by membership in this list. Yaml is
  /// always iterable when present (pnpm catalogs are not user-pattern
  /// controlled).
  pub user_source_indices: Vec<usize>,
  /// Slot in `all` holding the `Source::PnpmYaml` unit variant for the
  /// parse-time yaml. `None` if no yaml exists at parse time. Auto-created
  /// yaml at fix-time is invisible to sources — it lives only on Disk.
  pub pnpm_yaml_source_idx: Option<usize>,
}

impl Sources {
  pub fn new() -> Self {
    Self {
      all: vec![],
      user_source_indices: vec![],
      pnpm_yaml_source_idx: None,
    }
  }

  /// Append a source to the arena and return its index. Test-only helper
  /// that bypasses `user_source_indices` bookkeeping.
  pub fn add_source(&mut self, source: Source) -> SourceIdx {
    let idx = SourceIdx(self.all.len());
    self.all.push(source);
    idx
  }

  /// Iterate the package.json sources only. Pnpm yaml is excluded.
  /// Used by `format`, `find_package`, name lookups.
  pub fn packages_iter(&self) -> impl Iterator<Item = (SourceIdx, &Source)> {
    self.all.iter().enumerate().filter_map(|(i, s)| match s {
      Source::Package { .. } => Some((SourceIdx(i), s)),
      Source::PnpmYaml => None,
    })
  }

  /// Look up a package.json source by its package name.
  pub fn find_package(&self, name: &str) -> Option<SourceIdx> {
    self.packages_iter().find(|(_, s)| s.name() == name).map(|(idx, _)| idx)
  }

  /// Slot in `all` holding the parse-time PnpmYaml entry, if any.
  pub fn find_pnpm_yaml_idx(&self) -> Option<SourceIdx> {
    self.pnpm_yaml_source_idx.map(SourceIdx)
  }
}

impl Default for Sources {
  fn default() -> Self {
    Self::new()
  }
}

/// Extract the catalog name suffix from a built-in catalog dep type name.
/// Bare `pnpmCatalog` / `bunCatalog` → `"default"`. Suffixed
/// `pnpmCatalog:react18` → `"react18"`. Dots in the suffix are preserved
/// because dep type names are field-literal constructed. Returns a borrowed
/// `&str` (no allocation).
pub fn parse_catalog_name(dep_type_name: &str) -> &str {
  dep_type_name.split_once(':').map(|(_, suffix)| suffix).unwrap_or("default")
}
