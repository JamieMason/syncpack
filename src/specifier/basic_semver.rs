use {
  super::{determine_semver_range, get_huge, get_raw_without_range, parser, regexes, semver_range::SemverRange},
  log::debug,
  node_semver::{Range, Version},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BasicSemverVariant {
  /// "*"
  Latest,
  /// "1"
  Major,
  /// "1.2"
  Minor,
  /// "1.2.3"
  Patch,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BasicSemver {
  /// The sanitised original version specifier
  pub raw: String,
  /// The type of semver version that this is
  pub variant: BasicSemverVariant,
  /// The type of semver range that is used in the specifier
  pub range_variant: SemverRange,
  /// A `node_semver::Range` created from the semver portion of the specifier,
  /// WITH any semver range characters, for example:
  ///
  /// - "1.2.3" → "1.2.3"
  /// - "^1.2.3" → "^1.2.3"
  pub node_range: Range,
  /// A `node_semver::Version` created from the semver portion of the specifier,
  /// WITHOUT semver range characters, for example:
  ///
  /// - "1.2.3" → "1.2.3"
  /// - "^1.2.3" → "1.2.3"
  pub node_version: Version,
}

impl BasicSemver {
  pub fn new(value: &str) -> Option<Self> {
    Range::parse(value).ok().and_then(|node_range| {
      let raw = value.to_string();
      let huge = get_huge().to_string();
      if parser::is_exact(value) {
        let range_variant = SemverRange::Exact;
        Some(BasicSemver {
          raw,
          variant: BasicSemverVariant::Patch,
          range_variant,
          node_range,
          node_version: Version::parse(value).unwrap(),
        })
      } else if parser::is_range(value) {
        let range_variant = determine_semver_range(value).unwrap();
        let exact = get_raw_without_range(value);
        let node_version = Version::parse(exact).unwrap();
        Some(BasicSemver {
          raw,
          variant: BasicSemverVariant::Patch,
          range_variant,
          node_range,
          node_version,
        })
      } else if parser::is_major(value) {
        let range_variant = SemverRange::Exact;
        let major = format!("{value}.{huge}.{huge}");
        let node_version = Version::parse(major).unwrap();
        Some(BasicSemver {
          raw,
          variant: BasicSemverVariant::Major,
          range_variant,
          node_range,
          node_version,
        })
      } else if parser::is_range_major(value) {
        let range_variant = determine_semver_range(value).unwrap();
        let exact = regexes::RANGE_CHARS.replace(value, "");
        let major = format!("{exact}.{huge}.{huge}");
        let node_version = Version::parse(major).unwrap();
        Some(BasicSemver {
          raw,
          variant: BasicSemverVariant::Major,
          range_variant,
          node_range,
          node_version,
        })
      } else if parser::is_minor(value) {
        let range_variant = SemverRange::Exact;
        let minor = format!("{value}.{huge}");
        let node_version = Version::parse(minor).unwrap();
        Some(BasicSemver {
          raw,
          variant: BasicSemverVariant::Minor,
          range_variant,
          node_range,
          node_version,
        })
      } else if parser::is_range_minor(value) {
        let range_variant = determine_semver_range(value).unwrap();
        let exact = regexes::RANGE_CHARS.replace(value, "");
        let minor = format!("{exact}.{huge}");
        let node_version = Version::parse(minor).unwrap();
        Some(BasicSemver {
          raw,
          variant: BasicSemverVariant::Minor,
          range_variant,
          node_range,
          node_version,
        })
      } else if parser::is_latest(value) {
        let range_variant = SemverRange::Any;
        let latest = format!("{huge}.{huge}.{huge}");
        let node_version = Version::parse(latest).unwrap();
        Some(BasicSemver {
          raw,
          variant: BasicSemverVariant::Latest,
          range_variant,
          node_range,
          node_version,
        })
      } else {
        None
      }
    })
  }

  pub fn with_range(self, range: &SemverRange) -> Self {
    let range_str = range.unwrap();
    match self.variant {
      BasicSemverVariant::Latest => {
        debug!("Cannot apply semver range '{:?}' to specifier '{}'", range, self.raw);
        self
      }
      BasicSemverVariant::Major => {
        let major = self.node_version.major;
        Self::new(&format!("{range_str}{major}")).unwrap()
      }
      BasicSemverVariant::Minor => {
        let major = self.node_version.major;
        let minor = self.node_version.minor;
        Self::new(&format!("{range_str}{major}.{minor}")).unwrap()
      }
      BasicSemverVariant::Patch => {
        let patch = self.node_version.to_string();
        Self::new(&format!("{range_str}{patch}")).unwrap()
      }
    }
  }

  pub fn with_semver(self, semver: &BasicSemver) -> Self {
    semver.clone()
  }
}
