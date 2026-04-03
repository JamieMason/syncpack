pub mod pattern_matcher;

#[cfg(test)]
mod group_selector_test;

use {
  crate::{dependency::DependencyType, errors::ConfigError, instance::InstanceDescriptor},
  pattern_matcher::PatternMatcher,
};

#[derive(Clone, Debug)]
pub struct GroupSelector {
  /// Patterns to match against the installed dependency name.
  pub include_dependencies: Vec<PatternMatcher>,
  pub exclude_dependencies: Vec<PatternMatcher>,
  /// If true, match instances whose name is a local package name ($LOCAL).
  pub match_local: bool,
  /// If true, exclude instances whose name is a local package name (!$LOCAL).
  pub exclude_local: bool,
  /// Named locations where dependencies should be found.
  ///
  /// Possible values:
  /// - "dev" or "!dev"
  /// - "peer" or "!peer"
  /// - "prod" or "!prod"
  /// - "local" or "!local"
  /// - "overrides" or "!overrides"
  /// - "pnpm_overrides" or "!pnpm_overrides"
  /// - "resolutions" or "!resolutions"
  pub include_dependency_types: Vec<String>,
  pub exclude_dependency_types: Vec<String>,
  /// Optional label to describe the group.
  pub label: String,
  /// Patterns to match against the package name the dependency is located in.
  pub include_packages: Vec<PatternMatcher>,
  pub exclude_packages: Vec<PatternMatcher>,
  /// Types of version specifier the installed dependency should have.
  ///
  /// Possible values:
  /// - "alias" or "!alias"
  /// - "delete" or "!delete"
  /// - "exact" or "!exact"
  /// - "file" or "!file"
  /// - "hosted-git" or -!hosted-git"
  /// - "latest" or "!latest"
  /// - "range" or "!range"
  /// - "tag" or "!tag"
  /// - "unsupported" or "!unsupported"
  /// - "url" or "!url"
  /// - "workspace-protocol" or -!workspace-protocol"
  pub include_specifier_types: Vec<String>,
  pub exclude_specifier_types: Vec<String>,
  // Cache frequently accessed values
  has_dependency_type_filters: bool,
  has_specifier_type_filters: bool,
  has_dependency_filters: bool,
  has_package_filters: bool,
}

impl GroupSelector {
  pub fn new(
    dependencies: Vec<String>,
    dependency_types: Vec<String>,
    label: String,
    packages: Vec<String>,
    specifier_types: Vec<String>,
  ) -> GroupSelector {
    let (match_local, exclude_local, filtered_dependencies) =
      dependencies
        .into_iter()
        .fold((false, false, Vec::new()), |(mut ml, mut el, mut deps), d| {
          match d.as_str() {
            "$LOCAL" => ml = true,
            "!$LOCAL" => el = true,
            _ => deps.push(d),
          }
          (ml, el, deps)
        });

    let include_dependencies = create_patterns(true, &filtered_dependencies);
    let exclude_dependencies = create_patterns(false, &filtered_dependencies);
    let include_dependency_types = create_identifiers(true, &dependency_types);
    let exclude_dependency_types = create_identifiers(false, &dependency_types);
    let include_packages = create_patterns(true, &packages);
    let exclude_packages = create_patterns(false, &packages);
    let include_specifier_types = create_identifiers(true, &specifier_types);
    let exclude_specifier_types = create_identifiers(false, &specifier_types);

    GroupSelector {
      // Pre-compute boolean flags to avoid repeated empty checks
      has_dependency_type_filters: !include_dependency_types.is_empty() || !exclude_dependency_types.is_empty(),
      has_specifier_type_filters: !include_specifier_types.is_empty() || !exclude_specifier_types.is_empty(),
      has_dependency_filters: !include_dependencies.is_empty() || !exclude_dependencies.is_empty() || match_local || exclude_local,
      has_package_filters: !include_packages.is_empty() || !exclude_packages.is_empty(),

      include_dependencies,
      exclude_dependencies,
      match_local,
      exclude_local,
      include_dependency_types,
      exclude_dependency_types,
      label,
      include_packages,
      exclude_packages,
      include_specifier_types,
      exclude_specifier_types,
    }
  }

  /// Validate that all dependency type filters reference known dependency types.
  pub fn validate_dependency_types(&self, all_dependency_types: &[DependencyType]) -> Result<(), ConfigError> {
    for expected in self.include_dependency_types.iter().chain(self.exclude_dependency_types.iter()) {
      if !all_dependency_types.iter().any(|actual| actual.name == *expected) {
        return Err(ConfigError::InvalidDependencyType { name: expected.clone() });
      }
    }
    Ok(())
  }

  pub fn can_add(&self, descriptor: &InstanceDescriptor) -> bool {
    // Order checks from cheapest/most-likely-to-fail to most expensive
    // 1. Specifier types (often empty, cheap string comparison)
    if self.has_specifier_type_filters && !self.matches_specifier_types(descriptor) {
      return false;
    }

    // 2. Dependency types (cheap string comparison)
    if self.has_dependency_type_filters && !self.matches_dependency_types(descriptor) {
      return false;
    }

    // 3. Dependencies (pattern matching, optimised for common cases)
    if self.has_dependency_filters && !self.matches_dependencies(descriptor) {
      return false;
    }

    // 4. Packages (pattern matching + borrow, most expensive)
    if self.has_package_filters && !self.matches_packages(descriptor) {
      return false;
    }

    true
  }

  #[inline]
  fn matches_dependency_types(&self, descriptor: &InstanceDescriptor) -> bool {
    matches_identifiers(
      &descriptor.dependency_type.name,
      &self.include_dependency_types,
      &self.exclude_dependency_types,
    )
  }

  #[inline]
  fn matches_packages(&self, descriptor: &InstanceDescriptor) -> bool {
    let package_name = &descriptor.package.borrow().name;
    let is_included = self.include_packages.is_empty() || matches_any_pattern(package_name, &self.include_packages);
    let is_excluded = !self.exclude_packages.is_empty() && matches_any_pattern(package_name, &self.exclude_packages);
    is_included && !is_excluded
  }

  #[inline]
  fn matches_dependencies(&self, descriptor: &InstanceDescriptor) -> bool {
    let is_local = descriptor.is_local_dependency;
    // excludes: explicit exclude patterns OR exclude_local wins over everything
    let is_excluded = (!self.exclude_dependencies.is_empty() && matches_any_pattern(&descriptor.internal_name, &self.exclude_dependencies))
      || (self.exclude_local && is_local);
    if is_excluded {
      return false;
    }
    // includes: no filters at all → match; matches a pattern → match; match_local + is_local → match

    (self.include_dependencies.is_empty() && !self.match_local)
      || matches_any_pattern(&descriptor.internal_name, &self.include_dependencies)
      || (self.match_local && is_local)
  }

  #[inline]
  fn matches_specifier_types(&self, descriptor: &InstanceDescriptor) -> bool {
    matches_identifiers(
      descriptor.specifier.get_config_identifier(),
      &self.include_specifier_types,
      &self.exclude_specifier_types,
    )
  }
}

fn create_patterns(is_include: bool, patterns: &[String]) -> Vec<PatternMatcher> {
  patterns
    .iter()
    .filter(|pattern| *pattern != "**" && pattern.starts_with('!') != is_include)
    .map(|pattern| {
      let pattern = pattern.replace('!', "");
      PatternMatcher::from_pattern(&pattern)
    })
    .collect()
}

fn matches_any_pattern(value: &str, patterns: &[PatternMatcher]) -> bool {
  patterns.iter().any(|pattern| pattern.is_match(value))
}

fn create_identifiers(is_include: bool, patterns: &[String]) -> Vec<String> {
  patterns
    .iter()
    .filter(|pattern| *pattern != "**" && pattern.starts_with('!') != is_include)
    .map(|pattern| pattern.replace('!', ""))
    .collect()
}

fn matches_identifiers(name: &str, includes: &[String], excludes: &[String]) -> bool {
  let is_included = includes.is_empty() || matches_any_identifier(name, includes);
  let is_excluded = !excludes.is_empty() && matches_any_identifier(name, excludes);
  is_included && !is_excluded
}

fn matches_any_identifier(value: &str, identifiers: &[String]) -> bool {
  identifiers.iter().any(|id| id == value)
}
