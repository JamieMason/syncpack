use {
  crate::instance::Instance,
  globset::{Glob, GlobMatcher},
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
}

impl GroupSelector {
  pub fn new(
    dependencies: Vec<String>,
    dependency_types: Vec<String>,
    label: String,
    packages: Vec<String>,
    specifier_types: Vec<String>,
  ) -> GroupSelector {
    GroupSelector {
      include_dependencies: create_globs(true, &dependencies),
      exclude_dependencies: create_globs(false, &dependencies),
      include_dependency_types: create_identifiers(true, &dependency_types),
      exclude_dependency_types: create_identifiers(false, &dependency_types),
      label,
      include_packages: create_globs(true, &packages),
      exclude_packages: create_globs(false, &packages),
      include_specifier_types: create_identifiers(true, &specifier_types),
      exclude_specifier_types: create_identifiers(false, &specifier_types),
    }
  }

  pub fn can_add(&self, instance: &Instance) -> bool {
    self.matches_dependency_types(instance)
      && self.matches_packages(instance)
      && self.matches_dependencies(instance)
      && self.matches_specifier_types(instance)
  }

  pub fn matches_dependency_types(&self, instance: &Instance) -> bool {
    matches_identifiers(
      &instance.dependency_type.name,
      &self.include_dependency_types,
      &self.exclude_dependency_types,
    )
  }

  pub fn matches_packages(&self, instance: &Instance) -> bool {
    matches_globs(
      &instance.package.borrow().get_name_unsafe(),
      &self.include_packages,
      &self.exclude_packages,
    )
  }

  pub fn matches_dependencies(&self, instance: &Instance) -> bool {
    matches_globs(&instance.name, &self.include_dependencies, &self.exclude_dependencies)
  }

  pub fn matches_specifier_types(&self, instance: &Instance) -> bool {
    self.include_specifier_types.is_empty()
      || matches_identifiers(
        &instance.actual_specifier.get_config_identifier(),
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
  identifiers.contains(&value.to_string())
}
