#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageManager {
  Bun,
  Npm,
  Pnpm,
  /// We have looked but could not determine the package manager
  Unknown,
  Yarn,
}
