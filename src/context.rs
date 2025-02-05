use {
  crate::{
    config::Config,
    instance::Instance,
    instance_state::InstanceState,
    package_json::{FormatMismatch, FormatMismatchVariant, PackageJson},
    packages::Packages,
    semver_group::SemverGroup,
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
    let version_groups = config.rcfile.get_version_groups(&packages);

    packages.get_all_instances(&config, |instance| {
      let instance = Rc::new(instance);
      instances.push(Rc::clone(&instance));
      if let Some(dependency_group) = dependency_groups.iter().find(|alias| alias.can_add(&instance)) {
        instance.set_internal_name(&dependency_group.label);
      }
      if let Some(semver_group) = semver_groups.iter().find(|semver_group| semver_group.selector.can_add(&instance)) {
        instance.set_semver_group(semver_group);
      }
      if let Some(version_group) = version_groups
        .iter()
        .find(|version_group| version_group.selector.can_add(&instance))
      {
        version_group.add_instance(instance, &config.cli.filter);
      }
    });

    Self {
      config,
      instances,
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
