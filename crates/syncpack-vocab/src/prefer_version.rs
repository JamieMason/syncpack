/// When a version group has `preferVersion` set, this determines the direction
/// used to pick a winner among differing versions.
#[derive(Clone, Debug)]
pub enum PreferVersion {
  HighestSemver,
  LowestSemver,
}
