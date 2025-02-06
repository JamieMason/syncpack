use {
  crate::{
    config::Config,
    instance::Instance,
    instance_state::InstanceState,
    package_json::{FormatMismatch, FormatMismatchVariant, PackageJson},
    packages::Packages,
    semver_group::SemverGroup,
    specifier::basic_semver::BasicSemver,
    version_group::VersionGroup,
  },
  std::{cell::RefCell, collections::HashMap, rc::Rc},
};

#[derive(Debug)]
pub struct Context {
  /// All default configuration with user config applied
  pub config: Config,
  /// Every instance in the project
  pub instances: Vec<Rc<Instance>>,
  /// Index of every local package with a valid name and version
  pub local_versions: HashMap<String, BasicSemver>,
  /// Every package.json in the project
  pub packages: Packages,
  /// All semver groups
  pub semver_groups: Vec<SemverGroup>,
  /// All version groups, their dependencies, and their instances
  pub version_groups: Vec<VersionGroup>,
}

impl Context {
  pub fn create(config: Config, packages: Packages) -> Self {
    let mut instances = vec![];
    let dependency_groups = config.rcfile.get_dependency_groups(&packages);
    let semver_groups = config.rcfile.get_semver_groups(&packages);
    let mut version_groups = config.rcfile.get_version_groups(&packages);
    let local_versions = packages.get_local_versions();

    packages.get_all_instances(&config, |mut descriptor| {
      let dependency_group = dependency_groups.iter().find(|alias| alias.can_add(&descriptor));

      if let Some(group) = dependency_group {
        descriptor.internal_name = group.label.clone();
      }

      if let Some(cli_group) = &config.cli.filter {
        descriptor.matches_cli_filter = cli_group.can_add(&descriptor);
      }

      let semver_group = semver_groups.iter().find(|group| group.selector.can_add(&descriptor));
      let version_group = version_groups.iter_mut().find(|group| group.selector.can_add(&descriptor));
      let instance = Rc::new(Instance::new(descriptor));

      instances.push(Rc::clone(&instance));

      if let Some(group) = semver_group {
        instance.set_semver_group(group);
      }

      if let Some(group) = version_group {
        group.add_instance(instance);
      }
    });

    Self {
      config,
      instances,
      local_versions,
      packages,
      semver_groups,
      version_groups,
    }
  }

  /// Get all packages with valid formatting
  pub fn get_formatted_packages(&self) -> Vec<Rc<RefCell<PackageJson>>> {
    self
      .packages
      .all
      .iter()
      .filter(|package| package.borrow().formatting_mismatches.borrow().is_empty())
      .map(Rc::clone)
      .collect()
  }

  /// Get all formatting issues in package.json files, grouped by issue type
  pub fn get_formatting_mismatches_by_variant(&self) -> HashMap<FormatMismatchVariant, Vec<Rc<FormatMismatch>>> {
    let mut mismatches_by_variant = HashMap::new();
    self.packages.all.iter().for_each(|package| {
      package.borrow().formatting_mismatches.borrow().iter().for_each(|mismatch| {
        let variant = mismatch.variant.clone();
        let mismatches = mismatches_by_variant.entry(variant).or_insert_with(Vec::new);
        mismatches.push(Rc::clone(mismatch));
      });
    });
    mismatches_by_variant
  }

  /// Quit with the correct exit code based on the validity of each instance
  pub fn exit_program(&self) -> ! {
    if self.config.cli.inspect_mismatches {
      for instance in self.instances.iter() {
        match *instance.state.borrow() {
          InstanceState::Valid(_) => continue,
          InstanceState::Suspect(_) => {
            if self.config.rcfile.strict {
              std::process::exit(1);
            } else {
              continue;
            }
          }
          _ => std::process::exit(1),
        }
      }
    }
    if self.config.cli.inspect_formatting {
      for package in self.packages.all.iter() {
        if !package.borrow().formatting_mismatches.borrow().is_empty() {
          std::process::exit(1);
        }
      }
    }
    std::process::exit(0);
  }
}
