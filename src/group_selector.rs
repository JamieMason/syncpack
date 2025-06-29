use {
  crate::{dependency_type::DependencyType, instance::InstanceDescriptor, packages::Packages},
  globset::{Glob, GlobMatcher},
  log::error,
  std::process,
};

#[derive(Clone, Debug)]
pub struct GroupSelector {
  /// Glob patterns to match against the installed dependency name.
  ///
  /// The keyword "$LOCAL" can also be used to match every locally-developed
  /// package used as a dependency.
  pub include_dependencies: Vec<GlobMatcher>,
  pub exclude_dependencies: Vec<GlobMatcher>,
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
  /// Glob patterns to match against the package name the dependency is located
  /// in.
  pub include_packages: Vec<GlobMatcher>,
  pub exclude_packages: Vec<GlobMatcher>,
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
    all_packages: &Packages,
    dependencies: Vec<String>,
    dependency_types: Vec<String>,
    label: String,
    packages: Vec<String>,
    specifier_types: Vec<String>,
    all_dependency_types: &[DependencyType],
  ) -> GroupSelector {
    let dependencies = with_resolved_keywords(&dependencies, all_packages);

    let include_dependencies = create_globs(true, &dependencies);
    let exclude_dependencies = create_globs(false, &dependencies);
    let include_dependency_types = create_identifiers(true, &dependency_types);
    let exclude_dependency_types = create_identifiers(false, &dependency_types);
    let include_packages = create_globs(true, &packages);
    let exclude_packages = create_globs(false, &packages);
    let include_specifier_types = create_identifiers(true, &specifier_types);
    let exclude_specifier_types = create_identifiers(false, &specifier_types);

    // Validate dependency types during construction
    for expected in include_dependency_types.iter().chain(exclude_dependency_types.iter()) {
      if !all_dependency_types.iter().any(|actual| actual.name == *expected) {
        error!("dependencyType '{expected}' does not match any of syncpack or your customTypes");
        error!("check your syncpack config file");
        process::exit(1);
      }
    }

    GroupSelector {
      // Pre-compute boolean flags to avoid repeated empty checks
      has_dependency_type_filters: !include_dependency_types.is_empty() || !exclude_dependency_types.is_empty(),
      has_specifier_type_filters: !include_specifier_types.is_empty() || !exclude_specifier_types.is_empty(),
      has_dependency_filters: !include_dependencies.is_empty() || !exclude_dependencies.is_empty(),
      has_package_filters: !include_packages.is_empty() || !exclude_packages.is_empty(),

      include_dependencies,
      exclude_dependencies,
      include_dependency_types,
      exclude_dependency_types,
      label,
      include_packages,
      exclude_packages,
      include_specifier_types,
      exclude_specifier_types,
    }
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

    // 3. Dependencies (glob matching, more expensive)
    if self.has_dependency_filters && !self.matches_dependencies(descriptor) {
      return false;
    }

    // 4. Packages (glob matching + borrow, most expensive)
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
    // Cache the borrow result to avoid repeated borrow checks
    let package_name = &descriptor.package.borrow().name;
    matches_globs(package_name, &self.include_packages, &self.exclude_packages)
  }

  #[inline]
  fn matches_dependencies(&self, descriptor: &InstanceDescriptor) -> bool {
    matches_globs(&descriptor.internal_name, &self.include_dependencies, &self.exclude_dependencies)
  }

  #[inline]
  fn matches_specifier_types(&self, descriptor: &InstanceDescriptor) -> bool {
    matches_identifiers(
      &descriptor.specifier.get_config_identifier(),
      &self.include_specifier_types,
      &self.exclude_specifier_types,
    )
  }
}

fn create_globs(is_include: bool, patterns: &[String]) -> Vec<GlobMatcher> {
  patterns
    .iter()
    .filter(|pattern| *pattern != "**" && pattern.starts_with('!') != is_include)
    .map(|pattern| {
      Glob::new(&pattern.replace('!', ""))
        .expect("invalid glob pattern")
        .compile_matcher()
    })
    .collect()
}

fn matches_globs(value: &str, includes: &[GlobMatcher], excludes: &[GlobMatcher]) -> bool {
  let is_included = includes.is_empty() || matches_any_glob(value, includes);
  let is_excluded = !excludes.is_empty() && matches_any_glob(value, excludes);
  is_included && !is_excluded
}

fn matches_any_glob(value: &str, globs: &[GlobMatcher]) -> bool {
  globs.iter().any(|glob| glob.is_match(value))
}

fn create_identifiers(is_include: bool, patterns: &[String]) -> Vec<String> {
  patterns
    .iter()
    .filter(|pattern| *pattern != "**" && *pattern != "$LOCAL" && pattern.starts_with('!') != is_include)
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

/// Resolve keywords such as `$LOCAL` and `!$LOCAL` to their actual values.
fn with_resolved_keywords(dependency_names: &[String], packages: &Packages) -> Vec<String> {
  let mut resolved_dependencies: Vec<String> = vec![];
  for dependency_name in dependency_names.iter() {
    match dependency_name.as_str() {
      "$LOCAL" => {
        for package in packages.all.iter() {
          let package_name = package.borrow().name.clone();
          resolved_dependencies.push(package_name);
        }
      }
      "!$LOCAL" => {
        for package in packages.all.iter() {
          let package_name = package.borrow().name.clone();
          resolved_dependencies.push(format!("!{package_name}"));
        }
      }
      _ => {
        resolved_dependencies.push(dependency_name.clone());
      }
    }
  }
  resolved_dependencies
}
