use {
  crate::{
    context::Context,
    dependency::Dependency,
    instance::Instance,
    instance_state::{
      FixableInstance, InstanceState, InvalidInstance, SemverGroupAndVersionConflict, SuspectInstance, UnfixableInstance, ValidInstance,
    },
    package_json::{FormatMismatch, FormatMismatchVariant, PackageJson},
    version_group::{VersionGroup, VersionGroupVariant},
  },
  colored::*,
  itertools::Itertools,
  log::{info, warn},
  std::{cell::RefCell, rc::Rc},
};

#[derive(Debug)]
pub struct Ui<'a> {
  pub ctx: &'a Context,
}

impl Ui<'_> {
  pub fn print_command_header(&self, msg: &str) {
    info!("{}", format!(" {msg} ").on_blue().black());
  }

  pub fn print_group_header(&self, group: &VersionGroup) {
    let print_width = 80;
    let label = &group.selector.label;
    let header = format!("= {label} ");
    let divider = if header.len() < print_width {
      "=".repeat(print_width - header.len())
    } else {
      "".to_string()
    };
    let full_header = format!("{header}{divider}");
    info!("{}", full_header.blue());
  }

  pub fn print_dependency(&self, dependency: &Dependency, group_variant: &VersionGroupVariant) {
    let state_links = dependency
      .get_states()
      .iter()
      .map(|state| {
        let state_name = state.get_name();
        // Issues related to whether a specifier is the highest or lowest semver are
        // all the same logic internally, so we have combined enum branches for
        // them, but from an end user point of view though it is clearer to have a
        // specific status code related to what has happened.
        if matches!(group_variant, VersionGroupVariant::HighestSemver) {
          state_name.replace("HighestOrLowestSemver", "HighestSemver")
        } else if matches!(group_variant, VersionGroupVariant::LowestSemver) {
          state_name.replace("HighestOrLowestSemver", "LowestSemver")
        } else {
          state_name
        }
      })
      .sorted()
      .unique()
      .map(|state_name| self.instance_status_code_link(&state_name))
      .filter(|state_link| !state_link.is_empty())
      .join(", ");
    let state_links = if !state_links.is_empty() {
      format!("({state_links})").dimmed()
    } else {
      state_links.normal()
    };
    let instances_len = dependency.instances.borrow().len();
    let count = self.count_column(instances_len);
    let name = &dependency.name_internal;
    let name = if self.ctx.config.cli.show_hints && dependency.local_instance.borrow().is_some() {
      let local_hint = "(local)".blue();
      format!("{name} {local_hint}").normal()
    } else {
      name.normal()
    };
    let expected = dependency
      .expected
      .borrow()
      .clone()
      .map(|expected| expected.unwrap())
      .unwrap_or("".to_string())
      .dimmed();

    match &dependency.get_state() {
      InstanceState::Valid(variant) => {
        let icon = self.ok_icon();
        match variant {
          ValidInstance::IsIgnored => {
            let icon = self.unknown_icon();
            let name = name.dimmed();
            info!("{count} {icon} {name} {state_links}");
          }
          ValidInstance::IsLocalAndValid => {
            info!("{count} {icon} {name} {expected} {state_links}");
          }
          ValidInstance::IsIdenticalToLocal => {
            info!("{count} {icon} {name} {expected} {state_links}");
          }
          ValidInstance::SatisfiesLocal => {
            info!("{count} {icon} {name} {expected} {state_links}");
          }
          ValidInstance::IsHighestOrLowestSemver => {
            info!("{count} {icon} {name} {expected} {state_links}");
          }
          ValidInstance::SatisfiesHighestOrLowestSemver => {
            info!("{count} {icon} {name} {expected} {state_links}");
          }
          ValidInstance::IsNonSemverButIdentical => {
            info!("{count} {icon} {name} {expected} {state_links}");
          }
          ValidInstance::IsIdenticalToPin => {
            info!("{count} {icon} {name} {expected} {state_links}");
          }
          ValidInstance::SatisfiesSameRangeGroup => {
            info!("{count} {icon} {name} {state_links}");
          }
          ValidInstance::IsIdenticalToSnapTarget => {
            info!("{count} {icon} {name} {expected} {state_links}");
          }
          ValidInstance::SatisfiesSnapTarget => {
            info!("{count} {icon} {name} {expected} {state_links}");
          }
        }
      }
      InstanceState::Invalid(variant) => {
        let name = name.red();
        match variant {
          InvalidInstance::Fixable(variant) => {
            let icon = self.err_icon();
            match variant {
              FixableInstance::IsBanned => {
                info!("{count} {icon} {name} {state_links}");
              }
              FixableInstance::DiffersToLocal => {
                info!("{count} {icon} {name} {expected} {state_links}");
              }
              FixableInstance::DiffersToHighestOrLowestSemver => {
                info!("{count} {icon} {name} {expected} {state_links}");
              }
              FixableInstance::DiffersToSnapTarget => {
                info!("{count} {icon} {name} {expected} {state_links}");
              }
              FixableInstance::DiffersToPin => {
                info!("{count} {icon} {name} {expected} {state_links}");
              }
              FixableInstance::SemverRangeMismatch => {
                info!("{count} {icon} {name} {expected} {state_links}");
              }
              FixableInstance::PinOverridesSemverRange => {
                info!("{count} {icon} {name} {expected} {state_links}");
              }
              FixableInstance::PinOverridesSemverRangeMismatch => {
                info!("{count} {icon} {name} {expected} {state_links}");
              }
            }
          }
          InvalidInstance::Unfixable(variant) => {
            let icon = self.err_icon();
            match variant {
              UnfixableInstance::DependsOnInvalidLocalPackage => {
                info!("{count} {icon} {name} {state_links}");
              }
              UnfixableInstance::NonSemverMismatch => {
                info!("{count} {icon} {name} {state_links}");
              }
              UnfixableInstance::SameRangeMismatch => {
                info!("{count} {icon} {name} {state_links}");
              }
              UnfixableInstance::DependsOnMissingSnapTarget => {
                info!("{count} {icon} {name} {state_links}");
              }
            }
          }
          InvalidInstance::Conflict(variant) => {
            let icon = self.err_icon();
            match variant {
              SemverGroupAndVersionConflict::MatchConflictsWithHighestOrLowestSemver => {
                info!("{count} {icon} {name} {state_links}");
              }
              SemverGroupAndVersionConflict::MismatchConflictsWithHighestOrLowestSemver => {
                info!("{count} {icon} {name} {state_links}");
              }
              SemverGroupAndVersionConflict::MatchConflictsWithSnapTarget => {
                info!("{count} {icon} {name} {state_links}");
              }
              SemverGroupAndVersionConflict::MismatchConflictsWithSnapTarget => {
                info!("{count} {icon} {name} {state_links}");
              }
              SemverGroupAndVersionConflict::MatchConflictsWithLocal => {
                info!("{count} {icon} {name} {state_links}");
              }
              SemverGroupAndVersionConflict::MismatchConflictsWithLocal => {
                info!("{count} {icon} {name} {state_links}");
              }
            }
          }
        }
      }
      InstanceState::Suspect(variant) => {
        let icon = self.warn_icon();
        match variant {
          SuspectInstance::RefuseToBanLocal => {
            info!("{count} {icon} {name} {state_links}");
          }
          SuspectInstance::RefuseToPinLocal => {
            info!("{count} {icon} {name} {state_links}");
          }
          SuspectInstance::RefuseToSnapLocal => {
            info!("{count} {icon} {name} {state_links}");
          }
          SuspectInstance::InvalidLocalVersion => {
            info!("{count} {icon} {name} {state_links}");
          }
        }
      }
      InstanceState::Unknown => {
        panic!("Unknown");
      }
    }
  }

  pub fn print_instance(&self, instance: &Instance, group_variant: &VersionGroupVariant) {
    let state = instance.state.borrow().clone();
    let state_name = state.get_name();
    // Issues related to whether a specifier is the highest or lowest semver are
    // all the same logic internally, so we have combined enum branches for
    // them, but from an end user point of view though it is clearer to have a
    // specific status code related to what has happened.
    let state_name = if matches!(group_variant, VersionGroupVariant::HighestSemver) {
      state_name.replace("HighestOrLowestSemver", "HighestSemver").normal()
    } else if matches!(group_variant, VersionGroupVariant::LowestSemver) {
      state_name.replace("HighestOrLowestSemver", "LowestSemver").normal()
    } else {
      state_name.normal()
    };
    let state_link = self.instance_status_code_link(&state_name);
    let state_link = if !state_link.is_empty() {
      format!("({state_link})").dimmed()
    } else {
      state_link
    };
    let actual = instance.actual_specifier.unwrap();
    let location = self.instance_location(instance).dimmed();
    match &state {
      InstanceState::Valid(variant) => {
        let icon = self.ok_icon();
        let actual = if matches!(variant, ValidInstance::IsIgnored) {
          actual.dimmed()
        } else {
          actual.green()
        };
        match variant {
          ValidInstance::IsIgnored => {
            let icon = self.unknown_icon();
            info!("          {icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsLocalAndValid => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsIdenticalToLocal => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          ValidInstance::SatisfiesLocal => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsHighestOrLowestSemver => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          ValidInstance::SatisfiesHighestOrLowestSemver => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsNonSemverButIdentical => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsIdenticalToPin => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          ValidInstance::SatisfiesSameRangeGroup => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsIdenticalToSnapTarget => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          ValidInstance::SatisfiesSnapTarget => {
            info!("          {icon} {actual} {location} {state_link}");
          }
        }
      }
      InstanceState::Invalid(variant) => {
        let icon = self.err_icon();
        let actual = actual.red();
        match variant {
          InvalidInstance::Fixable(variant) => match variant {
            FixableInstance::IsBanned => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            FixableInstance::DiffersToLocal => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            FixableInstance::DiffersToHighestOrLowestSemver => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            FixableInstance::DiffersToSnapTarget => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            FixableInstance::DiffersToPin => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            FixableInstance::SemverRangeMismatch => {
              let arrow = self.dim_right_arrow();
              let expected = instance.get_specifier_with_preferred_semver_range();
              let expected = expected.unwrap().unwrap();
              let expected = expected.green();
              info!("          {icon} {actual} {arrow} {expected} {location} {state_link}");
            }
            FixableInstance::PinOverridesSemverRange => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            FixableInstance::PinOverridesSemverRangeMismatch => {
              info!("          {icon} {actual} {location} {state_link}");
            }
          },
          InvalidInstance::Unfixable(variant) => match variant {
            UnfixableInstance::DependsOnInvalidLocalPackage => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            UnfixableInstance::NonSemverMismatch => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            UnfixableInstance::SameRangeMismatch => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            UnfixableInstance::DependsOnMissingSnapTarget => {
              info!("          {icon} {actual} {location} {state_link}");
            }
          },
          InvalidInstance::Conflict(variant) => match variant {
            SemverGroupAndVersionConflict::MatchConflictsWithHighestOrLowestSemver => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            SemverGroupAndVersionConflict::MismatchConflictsWithHighestOrLowestSemver => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            SemverGroupAndVersionConflict::MatchConflictsWithSnapTarget => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            SemverGroupAndVersionConflict::MismatchConflictsWithSnapTarget => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            SemverGroupAndVersionConflict::MatchConflictsWithLocal => {
              info!("          {icon} {actual} {location} {state_link}");
            }
            SemverGroupAndVersionConflict::MismatchConflictsWithLocal => {
              info!("          {icon} {actual} {location} {state_link}");
            }
          },
        }
      }
      InstanceState::Suspect(variant) => {
        let icon = self.warn_icon();
        match variant {
          SuspectInstance::RefuseToBanLocal => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          SuspectInstance::RefuseToPinLocal => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          SuspectInstance::RefuseToSnapLocal => {
            info!("          {icon} {actual} {location} {state_link}");
          }
          SuspectInstance::InvalidLocalVersion => {
            info!("          {icon} {actual} {location} {state_link}");
          }
        }
      }
      InstanceState::Unknown => {
        panic!("Unknown");
      }
    }
  }

  pub fn ok_icon(&self) -> ColoredString {
    "✓".green()
  }

  pub fn err_icon(&self) -> ColoredString {
    "✘".red()
  }

  pub fn warn_icon(&self) -> ColoredString {
    "!".yellow()
  }

  fn unknown_icon(&self) -> ColoredString {
    "?".dimmed()
  }

  pub fn dim_right_arrow(&self) -> ColoredString {
    "→".dimmed()
  }

  /// Return a right-aligned column of a count of instances
  /// Example "    38x"
  pub fn count_column(&self, count: usize) -> ColoredString {
    format!("{: >8}x", count).dimmed()
  }

  /// Return a location hint for an instance
  pub fn instance_location(&self, instance: &Instance) -> ColoredString {
    let path_to_prop = instance.dependency_type.path.replace("/", ".");
    let file_link = self.package_json_link(&instance.package.borrow());
    format!("in {file_link} at {path_to_prop}").normal()
  }

  /// Issues related to whether a specifier is the highest or lowest semver are
  /// all the same logic internally, so we have combined enum branches for them.
  ///
  /// From an end user point of view though it is clearer to have a specific
  /// status code related to what has happened.
  fn to_public_status_code(group_variant: &VersionGroupVariant, code: &str) -> ColoredString {
    if matches!(group_variant, VersionGroupVariant::HighestSemver) {
      code.replace("HighestOrLowestSemver", "HighestSemver").normal()
    } else if matches!(group_variant, VersionGroupVariant::LowestSemver) {
      code.replace("HighestOrLowestSemver", "LowestSemver").normal()
    } else {
      code.normal()
    }
  }

  pub fn print_empty_group(&self) {
    warn!("Version Group is empty");
  }

  pub fn print_ignored_group(&self, group: &VersionGroup) {
    let dependencies_count = group.dependencies.borrow().len();
    let count = self.count_column(dependencies_count);
    let icon = self.unknown_icon();
    let message = "Ignored Dependencies".dimmed();
    info!("{count} {icon} {message}");
    let instances_count = group
      .dependencies
      .borrow()
      .values()
      .fold(0, |acc, dep| acc + dep.instances.borrow().len());
    let count = self.count_column(instances_count);
    let message = "Ignored Instances".dimmed();
    info!("{count} {icon} {message}");
  }

  /// Packages which are correctly formatted
  pub fn print_formatted_packages(&self, packages: &[Rc<RefCell<PackageJson>>]) {
    if !packages.is_empty() {
      let icon = self.ok_icon();
      let count = self.count_column(packages.len());
      let status = "Valid".green();
      info!("{count} {icon} {status}");
      if self.ctx.config.cli.show_packages {
        packages
          .iter()
          .sorted_by_key(|package| package.borrow().get_name_unsafe())
          .for_each(|package| {
            self.print_formatted_package(&package.borrow());
          });
      }
    }
  }

  /// Print a package.json which is correctly formatted
  fn print_formatted_package(&self, package: &PackageJson) {
    if package.formatting_mismatches.borrow().is_empty() {
      let icon = "-".dimmed();
      let file_link = self.package_json_link(package).dimmed();
      info!("          {icon} {file_link}");
    }
  }

  /// Print every package.json which has the given formatting mismatch
  pub fn print_formatting_mismatches(&self, variant: &FormatMismatchVariant, mismatches: &[Rc<FormatMismatch>]) {
    let count = self.count_column(mismatches.len());
    let icon = self.err_icon();
    let status_code = format!("{:?}", variant);
    let link = self.status_code_link(&status_code).red();
    info!("{count} {icon} {link}");
    if self.ctx.config.cli.show_packages {
      mismatches
        .iter()
        .sorted_by_key(|mismatch| mismatch.package.borrow().get_name_unsafe())
        .for_each(|mismatch| {
          let icon = "-".dimmed();
          let package = mismatch.package.borrow();
          let property_path = self.format_path(&mismatch.property_path);
          let file_link = self.package_json_link(&package);
          let msg = format!("          {icon} {property_path} of {file_link}").red();
          info!("{msg}");
        });
    }
  }

  /// Render a clickable link to a package.json file
  fn package_json_link(&self, package: &PackageJson) -> ColoredString {
    let name = package.get_name_unsafe();
    let file_path = package.file_path.to_str().unwrap();
    let plain_link = self.link(format!("file:{file_path}"), name);
    format!("{plain_link}").normal()
  }

  /// If enabled, render the reason code as a clickable link
  pub fn instance_status_code_link(&self, pascal_case: &str) -> ColoredString {
    if self.ctx.config.cli.show_status_codes {
      self.status_code_link(pascal_case)
    } else {
      "".normal()
    }
  }

  /// Render the reason code as a clickable link
  fn status_code_link(&self, pascal_case: &str) -> ColoredString {
    let base_url = "https://jamiemason.github.io/syncpack/guide/status-codes/";
    let lower_case = pascal_case.to_lowercase();
    let plain_link = self.link(format!("{base_url}#{lower_case}"), pascal_case);
    format!("{plain_link}").normal()
  }

  /// Render a clickable link
  fn link(&self, url: impl Into<String>, text: impl Into<ColoredString>) -> ColoredString {
    if self.ctx.config.cli.disable_ansi {
      text.into().normal()
    } else {
      format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url.into(), text.into()).normal()
    }
  }

  /// Convert eg. "/dependencies/react" to ".dependencies.react"
  fn format_path(&self, path: &str) -> ColoredString {
    if path == "/" {
      "root".normal()
    } else {
      path.replace("/", ".").normal()
    }
  }
}
