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
    let name = &dependency.internal_name;
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
      .map(|expected| expected.get_raw())
      .map(|raw| if raw.is_empty() { raw } else { format!(" {raw} ") })
      .unwrap_or(" ".to_string());

    match &dependency.get_state() {
      InstanceState::Valid(variant) => {
        let expected = expected.dimmed();
        match variant {
          ValidInstance::IsIgnored => {
            let name = name.dimmed();
            info!("{count} {name} {state_links}");
          }
          ValidInstance::IsLocalAndValid => {
            info!("{count} {name}{expected}{state_links}");
          }
          ValidInstance::IsIdenticalToLocal => {
            info!("{count} {name}{expected}{state_links}");
          }
          ValidInstance::SatisfiesLocal => {
            info!("{count} {name}{expected}{state_links}");
          }
          ValidInstance::IsHighestOrLowestSemver => {
            info!("{count} {name}{expected}{state_links}");
          }
          ValidInstance::SatisfiesHighestOrLowestSemver => {
            info!("{count} {name}{expected}{state_links}");
          }
          ValidInstance::IsNonSemverButIdentical => {
            info!("{count} {name}{expected}{state_links}");
          }
          ValidInstance::IsIdenticalToPin => {
            info!("{count} {name}{expected}{state_links}");
          }
          ValidInstance::SatisfiesSameRangeGroup => {
            info!("{count} {name}{state_links}");
          }
          ValidInstance::IsIdenticalToSnapTarget => {
            info!("{count} {name}{expected}{state_links}");
          }
          ValidInstance::SatisfiesSnapTarget => {
            info!("{count} {name}{expected}{state_links}");
          }
        }
      }
      InstanceState::Invalid(variant) => {
        let name = name.red();
        let arrow = self.dim_right_arrow();
        let expected = expected.green();
        match variant {
          InvalidInstance::Fixable(variant) => match variant {
            FixableInstance::IsBanned => {
              info!("{count} {name} {state_links}");
            }
            FixableInstance::DiffersToLocal => {
              info!("{count} {name} {arrow}{expected}{state_links}");
            }
            FixableInstance::DiffersToHighestOrLowestSemver => {
              info!("{count} {name} {arrow}{expected}{state_links}");
            }
            FixableInstance::DiffersToSnapTarget => {
              info!("{count} {name} {arrow}{expected}{state_links}");
            }
            FixableInstance::DiffersToPin => {
              info!("{count} {name} {arrow}{expected}{state_links}");
            }
            FixableInstance::SemverRangeMismatch => {
              info!("{count} {name} {arrow}{expected}{state_links}");
            }
            FixableInstance::PinOverridesSemverRange => {
              info!("{count} {name} {arrow}{expected}{state_links}");
            }
            FixableInstance::PinOverridesSemverRangeMismatch => {
              info!("{count} {name} {arrow}{expected}{state_links}");
            }
          },
          InvalidInstance::Unfixable(variant) => match variant {
            UnfixableInstance::DependsOnInvalidLocalPackage => {
              info!("{count} {name} {state_links}");
            }
            UnfixableInstance::NonSemverMismatch => {
              info!("{count} {name} {state_links}");
            }
            UnfixableInstance::SameRangeMismatch => {
              info!("{count} {name} {state_links}");
            }
          },
          InvalidInstance::Conflict(variant) => match variant {
            SemverGroupAndVersionConflict::MatchConflictsWithHighestOrLowestSemver => {
              info!("{count} {name} {state_links}");
            }
            SemverGroupAndVersionConflict::MismatchConflictsWithHighestOrLowestSemver => {
              info!("{count} {name} {state_links}");
            }
            SemverGroupAndVersionConflict::MatchConflictsWithSnapTarget => {
              info!("{count} {name} {state_links}");
            }
            SemverGroupAndVersionConflict::MismatchConflictsWithSnapTarget => {
              info!("{count} {name} {state_links}");
            }
            SemverGroupAndVersionConflict::MatchConflictsWithLocal => {
              info!("{count} {name} {state_links}");
            }
            SemverGroupAndVersionConflict::MismatchConflictsWithLocal => {
              info!("{count} {name} {state_links}");
            }
          },
        }
      }
      InstanceState::Suspect(variant) => {
        let icon = self.warn_icon();
        match variant {
          SuspectInstance::DependsOnMissingSnapTarget => {
            info!("{count} {name} {state_links}");
          }
          SuspectInstance::RefuseToBanLocal => {
            info!("{count} {name} {state_links}");
          }
          SuspectInstance::RefuseToPinLocal => {
            info!("{count} {name} {state_links}");
          }
          SuspectInstance::RefuseToSnapLocal => {
            info!("{count} {name} {state_links}");
          }
          SuspectInstance::InvalidLocalVersion => {
            info!("{count} {name} {state_links}");
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
    let actual = instance.descriptor.specifier.get_raw();
    let actual = if actual.is_empty() {
      "VERSION_IS_MISSING".yellow()
    } else {
      actual.normal()
    };
    let location = self.instance_location(instance).dimmed();
    let indent = "      ";
    match &state {
      InstanceState::Valid(variant) => {
        let icon = self.ok_icon().dimmed();
        match variant {
          ValidInstance::IsIgnored => {
            let icon = self.unknown_icon();
            let actual = actual.dimmed();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsLocalAndValid => {
            let actual = actual.green();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsIdenticalToLocal => {
            let actual = actual.green();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          ValidInstance::SatisfiesLocal => {
            let actual = actual.green();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsHighestOrLowestSemver => {
            let actual = actual.green();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          ValidInstance::SatisfiesHighestOrLowestSemver => {
            let actual = actual.green();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsNonSemverButIdentical => {
            let actual = actual.green();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsIdenticalToPin => {
            let actual = actual.green();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          ValidInstance::SatisfiesSameRangeGroup => {
            let actual = actual.green();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          ValidInstance::IsIdenticalToSnapTarget => {
            let actual = actual.green();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          ValidInstance::SatisfiesSnapTarget => {
            let actual = actual.green();
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
        }
      }
      InstanceState::Invalid(variant) => {
        let icon = self.err_icon().dimmed();
        let actual = actual.red();
        match variant {
          InvalidInstance::Fixable(variant) => match variant {
            FixableInstance::IsBanned => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            FixableInstance::DiffersToLocal => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            FixableInstance::DiffersToHighestOrLowestSemver => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            FixableInstance::DiffersToSnapTarget => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            FixableInstance::DiffersToPin => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            FixableInstance::SemverRangeMismatch => {
              let arrow = self.dim_right_arrow();
              let expected = instance.get_specifier_with_preferred_semver_range();
              let expected = expected.unwrap().get_raw();
              let expected = expected.green();
              info!("{indent}{icon} {actual} {arrow} {expected} {location} {state_link}");
            }
            FixableInstance::PinOverridesSemverRange => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            FixableInstance::PinOverridesSemverRangeMismatch => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
          },
          InvalidInstance::Unfixable(variant) => match variant {
            UnfixableInstance::DependsOnInvalidLocalPackage => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            UnfixableInstance::NonSemverMismatch => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            UnfixableInstance::SameRangeMismatch => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
          },
          InvalidInstance::Conflict(variant) => match variant {
            SemverGroupAndVersionConflict::MatchConflictsWithHighestOrLowestSemver => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            SemverGroupAndVersionConflict::MismatchConflictsWithHighestOrLowestSemver => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            SemverGroupAndVersionConflict::MatchConflictsWithSnapTarget => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            SemverGroupAndVersionConflict::MismatchConflictsWithSnapTarget => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            SemverGroupAndVersionConflict::MatchConflictsWithLocal => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
            SemverGroupAndVersionConflict::MismatchConflictsWithLocal => {
              info!("{indent}{icon} {actual} {location} {state_link}");
            }
          },
        }
      }
      InstanceState::Suspect(variant) => {
        let icon = self.warn_icon();
        match variant {
          SuspectInstance::DependsOnMissingSnapTarget => {
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          SuspectInstance::RefuseToBanLocal => {
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          SuspectInstance::RefuseToPinLocal => {
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          SuspectInstance::RefuseToSnapLocal => {
            info!("{indent}{icon} {actual} {location} {state_link}");
          }
          SuspectInstance::InvalidLocalVersion => {
            info!("{indent}{icon} {actual} {location} {state_link}");
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
    format!("{: >6}x", count).dimmed()
  }

  /// Return a location hint for an instance
  pub fn instance_location(&self, instance: &Instance) -> ColoredString {
    let path_to_prop = instance.dependency_type.path.replace("/", ".");
    let file_link = self.package_json_link(&instance.package.borrow());
    format!("in {file_link} at {path_to_prop}").normal()
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
          .sorted_by_key(|package| package.borrow().name.clone())
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
        .sorted_by_key(|mismatch| mismatch.package.borrow().name.clone())
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
    let file_path = package.file_path.to_str().unwrap();
    let plain_link = self.link(format!("file:{file_path}"), package.name.clone());
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
