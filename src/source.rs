use {serde_json::Value, syncpack_vocab::source_kind::SourceKind};

#[cfg(test)]
#[path = "source_test.rs"]
mod source_test;

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
/// variant carrying an index into `disk.package_json_files` and the cached
/// package name. Immutable — formatting mismatches are held in
/// `Inspection.formatting`, keyed by `SourceIdx`.
#[derive(Debug)]
pub enum Source {
  Package {
    /// Index into `disk.package_json_files` for this package.
    file_idx: usize,
    /// Cached `name` property value.
    name: String,
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
