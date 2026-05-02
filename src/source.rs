use {
  crate::errors::UnsupportedConfigError,
  serde::{Deserialize, Serialize},
  serde_json::Value,
};

#[cfg(test)]
#[path = "source_test.rs"]
mod source_test;

/// Which kind of file a `DependencyType` reads from.
///
/// Embedded on `DependencyType` so iteration can pair sources × dep types.
/// Default for user `customTypes` is `PackageJson`. Auto-generated catalog
/// dep types set this explicitly (`pnpmCatalog*` → `PnpmWorkspace`,
/// `bunCatalog*` → `PackageJson`).
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum SourceKind {
  PackageJson,
  PnpmWorkspace,
}

impl SourceKind {
  /// Parse the user-facing string form of a source.
  ///
  /// Accepts the PascalCase form used in `customTypes.<name>.source`.
  pub fn parse(raw: &str) -> Result<Self, UnsupportedConfigError> {
    match raw {
      "PackageJson" => Ok(SourceKind::PackageJson),
      "PnpmWorkspace" => Ok(SourceKind::PnpmWorkspace),
      other => Err(UnsupportedConfigError::InvalidSource { value: other.to_string() }),
    }
  }
}

/// A formatting mismatch detected against a `package.json`.
#[derive(Debug)]
pub struct FormatMismatch {
  /// The formatted value
  pub expected: Value,
  /// The path to the property that was linted
  pub property_path: String,
  /// The broken linting rule
  pub variant: FormatMismatchVariant,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum FormatMismatchVariant {
  /// - ✓ `rcFile.formatBugs` is enabled
  /// - ✘ The `bugs` property is not formatted
  BugsPropertyIsNotFormatted,
  /// - ✓ `rcFile.formatRepository` is enabled
  /// - ✘ The `repository` property is not formatted
  RepositoryPropertyIsNotFormatted,
  /// - ✓ `rcFile.sortAz` is enabled
  /// - ✘ This property is not sorted alphabetically
  PropertyIsNotSortedAz,
  /// - ✓ `rcFile.sortPackages` is enabled
  /// - ✘ This package.json's properties are not sorted
  PackagePropertiesAreNotSorted,
  /// - ✓ `rcFile.sortExports` is enabled
  /// - ✘ The `exports` property is not sorted
  ExportsPropertyIsNotSorted,
}

/// A file containing dependency declarations. Either a `package.json`
/// (`Package`) or the workspace's `pnpm-workspace.yaml` (`PnpmYaml`, unit
/// variant — yaml lives on `Disk.pnpm_workspace`). `Package` is a struct
/// variant carrying an index into `disk.package_json_files`, the cached
/// package name, and any formatting mismatches detected by
/// `visit_formatting`.
#[derive(Debug)]
pub enum Source {
  Package {
    /// Index into `disk.package_json_files` for this package.
    file_idx: usize,
    /// Cached `name` property value.
    name: String,
    /// Mutated by `visit_formatting`.
    formatting_mismatches: Vec<FormatMismatch>,
  },
  PnpmYaml,
}

impl Source {
  /// A short label for this source. Returns the package's name for
  /// `Package`, or the literal string `"pnpm-workspace.yaml"` for `PnpmYaml`.
  pub fn name(&self) -> &str {
    match self {
      Source::Package { name, .. } => name.as_str(),
      Source::PnpmYaml => "pnpm-workspace.yaml",
    }
  }

  pub fn kind(&self) -> SourceKind {
    match self {
      Source::Package { .. } => SourceKind::PackageJson,
      Source::PnpmYaml => SourceKind::PnpmWorkspace,
    }
  }
}
