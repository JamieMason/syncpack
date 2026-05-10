#[cfg(test)]
#[path = "update_target_test.rs"]
mod update_target_test;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateTarget {
  /// "*.*.*"
  Latest,
  /// "1.*.*"
  Minor,
  /// "1.2.*"
  Patch,
}

impl UpdateTarget {
  /// Return the stricter of two `UpdateTarget`s. `Patch` < `Minor` < `Latest`
  /// in strictness, so `Patch` always wins, then `Minor`, then `Latest`.
  pub fn stricter(self, other: Self) -> Self {
    use UpdateTarget::*;
    match (self, other) {
      (Patch, _) | (_, Patch) => Patch,
      (Minor, _) | (_, Minor) => Minor,
      _ => Latest,
    }
  }
}
