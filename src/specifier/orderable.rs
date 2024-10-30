use {super::semver_range::SemverRange, node_semver::Version, std::cmp::Ordering};

pub trait IsOrderable: std::fmt::Debug {
  fn get_orderable(&self) -> Orderable;
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub(crate) struct Orderable {
  pub range: SemverRange,
  pub version: Version,
}

impl Orderable {
  pub fn new() -> Self {
    Self {
      range: SemverRange::Lt,
      version: Version {
        major: 0,
        minor: 0,
        patch: 0,
        // "Build metadata MUST be ignored when determining version precedence"
        // https://semver.org/spec/v2.0.0.html#spec-item-10
        build: vec![],
        pre_release: vec![],
      },
    }
  }
}

impl Ord for Orderable {
  fn cmp(&self, other: &Self) -> Ordering {
    // major
    match self.version.major.cmp(&other.version.major) {
      Ordering::Greater => Ordering::Greater,
      Ordering::Less => Ordering::Less,
      // minor
      Ordering::Equal => match self.version.minor.cmp(&other.version.minor) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        // patch
        Ordering::Equal => match self.version.patch.cmp(&other.version.patch) {
          Ordering::Greater => Ordering::Greater,
          Ordering::Less => Ordering::Less,
          // pre_release
          Ordering::Equal => match self.version.pre_release.cmp(&other.version.pre_release) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.range.cmp(&other.range),
          },
        },
      },
    }
  }
}

impl PartialOrd for Orderable {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Eq for Orderable {}
