use syncpack_vocab::source_kind::SourceKind;

#[cfg(test)]
#[path = "source_test.rs"]
mod source_test;

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
