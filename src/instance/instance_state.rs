use {
  crate::specifier::Specifier,
  std::{cmp::Ordering, rc::Rc},
};

/// Strip the Debug payload off an enum variant's `format!("{:?}", ..)` so
/// `MissingFromCatalog { catalog_name: "default", winning_specifier: .. }`
/// renders as `MissingFromCatalog` and `NotUsingCatalog("react18")` renders as
/// `NotUsingCatalog`. Unit variants pass through unchanged.
fn trim_variant_name(debug: String) -> String {
  let cut = debug.find(['(', ' ', '{']).unwrap_or(debug.len());
  debug[..cut].to_string()
}

/// The state of a dependency instance after validation.
#[derive(Clone, Debug)]
pub enum InstanceState {
  /// Initial state before inspection. Should not appear after visit_packages().
  Unknown,
  /// Instance follows all rules correctly.
  Valid(ValidInstance),
  /// Instance breaks rules in some way.
  Invalid(InvalidInstance),
  /// Instance represents a user misconfiguration.
  Suspect(SuspectInstance),
}

impl InstanceState {
  #[cfg(test)]
  pub fn valid(state: ValidInstance) -> Self {
    InstanceState::Valid(state)
  }

  #[cfg(test)]
  pub fn suspect(state: SuspectInstance) -> Self {
    InstanceState::Suspect(state)
  }

  #[cfg(test)]
  pub fn fixable(state: FixableInstance) -> Self {
    InstanceState::Invalid(InvalidInstance::Fixable(state))
  }

  #[cfg(test)]
  pub fn conflict(state: SemverGroupAndVersionConflict) -> Self {
    InstanceState::Invalid(InvalidInstance::Conflict(state))
  }

  #[cfg(test)]
  pub fn unfixable(state: UnfixableInstance) -> Self {
    InstanceState::Invalid(InvalidInstance::Unfixable(state))
  }

  pub fn get_status_type(&self) -> &'static str {
    match self {
      InstanceState::Unknown => "Unknown",
      InstanceState::Valid(_) => "Valid",
      InstanceState::Invalid(variant) => match variant {
        InvalidInstance::Fixable(_) => "Fixable",
        InvalidInstance::Unfixable(_) => "Unfixable",
        InvalidInstance::Conflict(_) => "Conflict",
      },
      InstanceState::Suspect(_) => "Suspect",
    }
  }

  pub fn get_name(&self) -> String {
    match self {
      InstanceState::Unknown => "Unknown".to_string(),
      InstanceState::Valid(variant) => trim_variant_name(format!("{variant:?}")),
      InstanceState::Invalid(variant) => match variant {
        InvalidInstance::Fixable(variant) => trim_variant_name(format!("{variant:?}")),
        InvalidInstance::Conflict(variant) => trim_variant_name(format!("{variant:?}")),
        InvalidInstance::Unfixable(variant) => trim_variant_name(format!("{variant:?}")),
      },
      InstanceState::Suspect(variant) => trim_variant_name(format!("{variant:?}")),
    }
  }

  pub fn get_severity(&self) -> u8 {
    match self {
      InstanceState::Unknown => 0,
      InstanceState::Valid(_) => 1,
      InstanceState::Invalid(_) => 2,
      InstanceState::Suspect(_) => 3,
    }
  }

  pub fn is_valid(&self) -> bool {
    matches!(self, InstanceState::Valid(_))
  }

  pub fn is_invalid(&self) -> bool {
    matches!(self, InstanceState::Invalid(_))
  }

  pub fn is_suspect(&self) -> bool {
    matches!(self, InstanceState::Suspect(_))
  }

  pub fn is_fixable(&self) -> bool {
    matches!(self, InstanceState::Invalid(InvalidInstance::Fixable(_)))
  }

  pub fn is_banned(&self) -> bool {
    matches!(self, InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::IsBanned)))
  }

  pub fn is_unfixable(&self) -> bool {
    matches!(self, InstanceState::Invalid(InvalidInstance::Unfixable(_)))
  }

  pub fn is_outdated(&self) -> bool {
    matches!(
      self,
      InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToNpmRegistry))
    )
  }
}

impl PartialEq for InstanceState {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (InstanceState::Unknown, InstanceState::Unknown) => true,
      (InstanceState::Valid(a), InstanceState::Valid(b)) => a == b,
      (InstanceState::Invalid(a), InstanceState::Invalid(b)) => a == b,
      (InstanceState::Suspect(a), InstanceState::Suspect(b)) => a == b,
      _ => false,
    }
  }
}

impl Eq for InstanceState {}

impl PartialOrd for InstanceState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for InstanceState {
  fn cmp(&self, other: &Self) -> Ordering {
    self.get_severity().cmp(&other.get_severity())
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ValidInstance {
  /// - ✓ Instance uses the catalog: protocol and wins out
  IsCatalog,
  /// - ✓ Instance is a catalog definition in pnpm-workspace.yaml or root package.json
  /// - ✓ Zero or more siblings use the catalog: protocol for this dependency
  IsCatalogDefinition,
  /// - ✓ Instance is configured to be ignored by Syncpack
  IsIgnored,
  /// - ✓ Instance is a local package and its version is valid
  IsLocalAndValid,
  /// - ✓ Instance is identical to the version of its locally-developed package
  /// - ✓ Instance matches its semver group
  IsIdenticalToLocal,
  /// - ✓ Instance matches the version of its locally-developed package
  /// - ✓ Instance matches its semver group
  /// - ! Considered a loose match we should highlight
  SatisfiesLocal,
  /// - ✓ Instance is identical to highest/lowest semver in its group
  /// - ✓ Instance matches its semver group
  IsHighestOrLowestSemver,
  /// - ✓ Instance has same semver number as highest/lowest semver in its group
  /// - ✓ Instance matches its semver group
  /// - ✓ Range preferred by semver group satisfies the highest/lowest semver
  /// - ! Considered a loose match we should highlight
  SatisfiesHighestOrLowestSemver,
  /// - ! No Instances are simple semver
  /// - ✓ Instance is identical to every other instance in its version group
  IsNonSemverButIdentical,
  /// - ✓ Instance is identical to its pinned version group
  /// - ✓ Instance matches its semver group
  IsIdenticalToPin,
  /// - ✓ Instance's range satisfies all other ranges in its same range group
  /// - ✓ Instance matches its semver group
  SatisfiesSameRangeGroup,
  /// - ✓ Instance matches its same minor group
  /// - ✓ Instance matches its semver group
  SatisfiesSameMinorGroup,
  /// - ✓ Instance is identical to a matching snapTo instance
  /// - ✓ Instance matches its semver group
  IsIdenticalToSnapTarget,
  /// - ✓ Instance has same semver number as matching snapTo instance
  /// - ✓ Instance matches its semver group
  /// - ✓ Range preferred by semver group satisfies the matching snapTo instance
  /// - ! Considered a loose match we should highlight
  SatisfiesSnapTarget,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InvalidInstance {
  Fixable(FixableInstance),
  Unfixable(UnfixableInstance),
  Conflict(SemverGroupAndVersionConflict),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FixableInstance {
  /// - ✘ Instance is in a banned version group
  IsBanned,
  /// - ✓ Instance is in a highest/lowest semver group
  /// - ✓ One or more other instances use the catalog: protocol
  /// - ✘ Instance does not use the catalog: protocol
  /// - ! catalog: protocol wins
  DiffersToCatalog,
  /// - ✓ Instance is in a catalog version group
  /// - ✓ Instance's dependency is defined in exactly one catalog
  /// - ✘ Instance does not use the catalog: protocol
  /// - ! Fix: replace specifier with `catalog:` or `catalog:{name}`
  /// - String carries the target catalog name (`"default"` for the unnamed catalog).
  NotUsingCatalog(String),
  /// - ✓ Instance is in a catalog version group
  /// - ✓ One catalog (or zero — implicit "default") exists in the project
  /// - ✘ Instance's dependency is not defined in any catalog
  /// - ✘ Instance does not use the catalog: protocol
  /// - ! Fix: add dependency to the catalog and replace specifier with catalog:
  /// - `catalog_name` carries the target catalog (`"default"` for the unnamed catalog)
  /// - `winning_specifier` carries the value to enshrine in the catalog (resolved at visit time: unique value when all-identical, highest
  ///   semver otherwise)
  MissingFromCatalog {
    catalog_name: String,
    winning_specifier: Rc<Specifier>,
  },
  /// - ✘ Instance mismatches the version of its locally-developed package
  DiffersToLocal,
  /// - ✘ Instance mismatches highest/lowest semver in its group
  DiffersToHighestOrLowestSemver,
  /// - ✘ Instance is older than highest semver published to the registry
  DiffersToNpmRegistry,
  /// - ✘ Instance mismatches the matching snapTo instance
  DiffersToSnapTarget,
  /// - ✘ Instance mismatches its pinned version group
  DiffersToPin,
  /// - ✓ Instance has same semver number as highest/lowest semver in its group
  /// - ✘ Instance mismatches its semver group
  /// - ✓ Range preferred by semver group satisfies the highest/lowest semver
  /// - ✓ Fixing the semver range satisfy both groups
  SemverRangeMismatch,
  /// - ✓ Instance has same semver number as its pinned version group
  /// - ✓ Instance matches its semver group
  /// - ! The semver group requires a range which is different to the pinned version
  /// - ! Pinned version wins
  PinOverridesSemverRange,
  /// - ✓ Instance has same semver number as its pinned version group
  /// - ✘ Instance mismatches its semver group
  /// - ! The semver group requires a range which is different to the pinned version
  /// - ! Pinned version wins
  PinOverridesSemverRangeMismatch,
  /// - ✓ Instance has same major.minor as all other instances in its group
  /// - ✓ Instance matches its semver group
  /// - ! The semver group requires a range which would break same minor policy
  /// - ! Same minor policy wins
  SameMinorOverridesSemverRange,
  /// - ✓ Instance has same major.minor as all other instances in its group
  /// - ✘ Instance mismatches its semver group
  /// - ! The semver group requires a range which would break same minor policy
  /// - ! Same minor policy wins
  SameMinorOverridesSemverRangeMismatch,
  /// - ✓ Instance is in a sameMinor version group with preferVersion set
  /// - ✓ All instances share the same MAJOR version
  /// - ✘ Instance's MAJOR.MINOR is not the highest (or lowest) MAJOR.MINOR in the group
  /// - ! Fix: update to the preferred MAJOR.MINOR target
  /// - ! Range selection (in priority order):
  ///     1. If instance has a semver group with a safe preferred range → use preferred range
  ///     2. If instance has no semver group and on-disk range is safe → preserve on-disk range
  ///     3. Otherwise → force ~ (sameMinor policy wins over unsafe ranges)
  DiffersToHighestOrLowestSemverMinor,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnfixableInstance {
  /// - ✘ Instance depends on a local package whose package.json version is not exact semver
  /// - ? We can't know what the version should be
  DependsOnInvalidLocalPackage,
  /// - ✘ Instance mismatches others in its group
  /// - ✘ One or more Instances are not simple semver
  /// - ? We can't know what's right or what isn't
  NonSemverMismatch,
  /// - ✘ Instance mismatches its same range group
  /// - ✘ Instance's range doesn't satisfy all other ranges in its same range group
  /// - ? Instance has no semver group
  /// - ? We can't know what range the user wants and have to ask them
  SameRangeMismatch,
  /// - ✘ Instance mismatches its same minor group
  /// - ? Instance has no semver group
  /// - ? We can't know what range the user wants and have to ask them
  SameMinorMismatch,
  /// - ✘ Instance is in a sameMinor version group
  /// - ✘ One or more other instances have a different MAJOR version
  /// - ? Crossing a major version boundary is unsafe
  /// - ? We cannot know which MAJOR the user wants and have to ask them
  SameMinorHasMajorMismatch,
  /// - ✓ Instance is in a catalog version group
  /// - ✓ MissingFromCatalog applies to multiple instances of the same dep
  /// - ✘ Their specifiers differ AND at least one is non-semver
  /// - ? Syncpack cannot pick which specifier to enshrine in the catalog
  /// - String carries the target catalog name (`"default"` for the unnamed catalog).
  MissingFromCatalogAndNonSemverMismatch(String),
  /// - ✓ Instance is in a catalog version group
  /// - ✓ Two or more catalogs exist in the project
  /// - ✓ Instance's dependency is defined in zero OR two-or-more of those catalogs
  /// - ✘ Instance does not use the catalog: protocol
  /// - ? Syncpack cannot determine which catalog the dependency belongs to
  NotUsingCatalogAndCatalogUnknown,
  /// - ✓ Instance is in a catalog version group
  /// - ✓ Project has 0 catalogs
  /// - ✘ `Disk.package_manager` is npm/yarn/Unknown — no recognized lock file
  /// - ? Cannot infer whether to create pnpm-workspace.yaml or root package.json /catalog
  CannotInferCatalogFile,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SemverGroupAndVersionConflict {
  /// - ✓ Instance has same semver number as highest/lowest semver in its group
  /// - ✓ Instance matches its semver group
  /// - ✘ Range preferred by semver group will not satisfy the highest/lowest semver
  /// - ? We can't know whether the incompatible range matters or not and have to ask
  MatchConflictsWithHighestOrLowestSemver,
  /// - ✓ Instance has same semver number as highest/lowest semver in its group
  /// - ✘ Instance mismatches its semver group
  /// - ✘ Range preferred by semver group will not satisfy the highest/lowest semver
  /// - ? We can't know whether the incompatible range matters or not and have to ask
  MismatchConflictsWithHighestOrLowestSemver,
  /// - ✓ Instance has same semver number as the matching snapTo instance
  /// - ✓ Instance matches its semver group
  /// - ✘ Range preferred by semver group will not satisfy the matching snapTo instance
  /// - ? We can't know whether the incompatible range matters or not and have to ask
  MatchConflictsWithSnapTarget,
  /// - ✓ Instance has same semver number as the matching snapTo instance
  /// - ✘ Instance mismatches its semver group
  /// - ✘ Range preferred by semver group will not satisfy the matching snapTo instance
  /// - ? We can't know whether the incompatible range matters or not and have to ask
  MismatchConflictsWithSnapTarget,
  /// - ✓ Instance has same semver number as local instance in its group
  /// - ✓ Instance matches its semver group
  /// - ✘ Range preferred by semver group will not satisfy the local instance
  /// - ? We can't know whether the incompatible range matters or not and have to ask
  MatchConflictsWithLocal,
  /// - ✓ Instance has same semver number as local instance
  /// - ✘ Instance mismatches its semver group
  /// - ✘ Range preferred by semver group will not satisfy the local instance
  /// - ? We can't know whether the incompatible range matters or not and have to ask
  MismatchConflictsWithLocal,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SuspectInstance {
  /// - ✘ Local Instance is in a banned version group
  /// - ✘ Misconfiguration: Syncpack refuses to change local dependency specifiers
  RefuseToBanLocal,
  /// - ✘ Local Instance mismatches its pinned version group
  /// - ✘ Misconfiguration: Syncpack refuses to change local dependency specifiers
  RefuseToPinLocal,
  /// - ✘ Local Instance is in a snapped to version group
  /// - ✘ An Instance of this dependency was found in the snapped to package
  /// - ✘ Misconfiguration: Syncpack refuses to change local dependency specifiers
  RefuseToSnapLocal,
  /// - ! Local Instance has no version property
  /// - ! Not an error on its own unless an instance of it mismatches
  InvalidLocalVersion,
  /// - ✓ Instance is in a snapped to version group
  /// - ✘ An instance of the same dependency was not found in any of the snapped to packages
  /// - ! This is a misconfiguration resulting in this instance being orphaned
  DependsOnMissingSnapTarget,
  /// - ✓ Instance uses the catalog: protocol (bare `catalog:` or `catalog:name`)
  /// - ✘ The referenced catalog does not exist OR the dep name is not defined in it
  DependsOnMissingCatalogDefinition,
  /// - ✓ Instance is in a catalog version group
  /// - ✓ Instance is a local instance (its own package.json `/version` property)
  /// - ✘ Local instances cannot use the catalog: protocol
  /// - ! Reconfigure version groups to exclude local instances from the catalog group
  RefuseToCatalogLocal,
}
