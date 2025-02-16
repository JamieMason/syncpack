#[cfg(test)]
#[path = "visit_packages_test.rs"]
mod visit_packages_test;

use {
  crate::{
    cli::SortBy,
    context::Context,
    format,
    instance_state::{FixableInstance::*, SemverGroupAndVersionConflict::*, SuspectInstance::*, UnfixableInstance::*, ValidInstance::*},
    package_json::{FormatMismatch, FormatMismatchVariant::*, PackageJson},
    specifier::{semver_range::SemverRange, Specifier},
    version_group::VersionGroupVariant,
  },
  itertools::Itertools,
  log::debug,
  std::{cell::RefCell, cmp::Ordering, rc::Rc},
};

const L1: &str = "  ";
const L2: &str = "    ";
const L3: &str = "      ";
const L4: &str = "        ";
const L5: &str = "          ";
const L6: &str = "            ";
const L7: &str = "              ";
const L8: &str = "                ";
const L9: &str = "                  ";
const L10: &str = "                    ";

pub fn visit_packages(ctx: Context) -> Context {
  if ctx.config.cli.inspect_mismatches {
    debug!("visit versions");

    ctx
      .version_groups
      .iter()
      // @TODO: can moving snapped to groups last be done when reading config at
      // the start?
      //
      // fix snapped to groups last, so that the packages they're snapped to
      // have any fixes applied to them first
      .sorted_by(|a, b| {
        if matches!(a.variant, VersionGroupVariant::SnappedTo) {
          Ordering::Greater
        } else if matches!(b.variant, VersionGroupVariant::SnappedTo) {
          Ordering::Less
        } else {
          Ordering::Equal
        }
      })
      .for_each(|group| {
        group.for_each_dependency(&SortBy::Name, |dependency| match dependency.variant {
          VersionGroupVariant::Banned => {
            visit_banned(dependency);
          }
          VersionGroupVariant::HighestSemver | VersionGroupVariant::LowestSemver => {
            visit_preferred_semver(dependency, &ctx);
          }
          VersionGroupVariant::Ignored => {
            visit_ignored(dependency);
          }
          VersionGroupVariant::Pinned => {
            visit_pinned(dependency);
          }
          VersionGroupVariant::SameRange => {
            visit_same_range(dependency);
          }
          VersionGroupVariant::SnappedTo => {
            visit_snapped_to(dependency, &ctx);
          }
        });
      });
  }

  if ctx.config.cli.inspect_formatting {
    let add_mismatch = |package: &Rc<RefCell<PackageJson>>, mismatch: FormatMismatch| {
      let mismatch = Rc::new(mismatch);
      package.borrow().formatting_mismatches.borrow_mut().push(Rc::clone(&mismatch));
    };

    ctx.packages.all.iter().for_each(|package| {
      if ctx.config.rcfile.sort_packages || !ctx.config.rcfile.sort_first.is_empty() {
        if let Some(expected) = format::get_sorted_first(&ctx.config.rcfile, &package.borrow()) {
          add_mismatch(
            package,
            FormatMismatch {
              expected,
              package: Rc::clone(package),
              property_path: "/".to_string(),
              variant: PackagePropertiesAreNotSorted,
            },
          );
        }
      }
      if ctx.config.rcfile.format_bugs {
        if let Some(expected) = format::get_formatted_bugs(&package.borrow()) {
          add_mismatch(
            package,
            FormatMismatch {
              expected,
              package: Rc::clone(package),
              property_path: "/bugs".to_string(),
              variant: BugsPropertyIsNotFormatted,
            },
          );
        }
      }
      if ctx.config.rcfile.format_repository {
        if let Some(expected) = format::get_formatted_repository(&package.borrow()) {
          add_mismatch(
            package,
            FormatMismatch {
              expected,
              package: Rc::clone(package),
              property_path: "/repository".to_string(),
              variant: RepositoryPropertyIsNotFormatted,
            },
          );
        }
      }
      if !ctx.config.rcfile.sort_exports.is_empty() {
        if let Some(expected) = format::get_sorted_exports(&ctx.config.rcfile, &package.borrow()) {
          add_mismatch(
            package,
            FormatMismatch {
              expected,
              package: Rc::clone(package),
              property_path: "/exports".to_string(),
              variant: ExportsPropertyIsNotSorted,
            },
          );
        }
      }
      if !ctx.config.rcfile.sort_az.is_empty() {
        for key in ctx.config.rcfile.sort_az.iter() {
          if let Some(expected) = format::get_sorted_az(key, &package.borrow()) {
            add_mismatch(
              package,
              FormatMismatch {
                expected,
                package: Rc::clone(package),
                property_path: format!("/{}", key),
                variant: PropertyIsNotSortedAz,
              },
            );
          }
        }
      }
    });
  }

  ctx
}

fn visit_banned(dependency: &crate::dependency::Dependency) {
  debug!("visit banned version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  dependency.instances.borrow().iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
    if instance.is_local {
      debug!("{L3}it is the local instance of a package developed locally in this monorepo");
      debug!("{L4}refuse to change it");
      debug!("{L5}mark as suspect, user should change their config");
      instance.mark_suspect(RefuseToBanLocal);
    } else {
      debug!("{L3}it should be removed");
      debug!("{L4}mark as error");
      instance.mark_fixable(IsBanned, &Specifier::None);
    }
  });
}

fn visit_preferred_semver(dependency: &crate::dependency::Dependency, ctx: &Context) {
  debug!("visit standard version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  if dependency.has_local_instance_with_invalid_specifier() {
    debug!("{L2}it has an invalid local instance");
    dependency.instances.borrow().iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
      if instance.is_local {
        debug!("{L4}it is the invalid local instance");
        debug!("{L5}mark as suspect");
        instance.mark_suspect(InvalidLocalVersion);
      } else {
        debug!("{L4}it depends on an unknowable version of an invalid local instance");
        debug!("{L5}mark as error");
        instance.mark_unfixable(DependsOnInvalidLocalPackage);
      }
    });
  } else if dependency.has_local_instance() {
    debug!("{L2}it is a package developed locally in this monorepo");
    let local_specifier = dependency.get_local_specifier().unwrap();
    dependency.set_expected_specifier(&local_specifier);
    dependency.instances.borrow().iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
      if instance.is_local {
        debug!("{L4}it is the valid local instance");
        instance.mark_valid(IsLocalAndValid, &local_specifier);
        return;
      }
      debug!("{L4}it depends on the local instance");
      if instance.descriptor.specifier.is_workspace_protocol() {
        debug!("{L5}it is using the workspace protocol");
        if !ctx.config.rcfile.strict {
          debug!("{L6}strict mode is off");
          debug!("{L7}mark as satisfying local");
          instance.mark_valid(SatisfiesLocal, &instance.descriptor.specifier);
          return;
        }
        debug!("{L6}strict mode is on");
      } else {
        debug!("{L5}it is not using the workspace protocol");
      }
      debug!("{L5}its version number (without a range):");
      if !instance.descriptor.specifier.has_same_version_number_as(&local_specifier) {
        debug!("{L6}differs to the local instance");
        debug!("{L7}mark as error");
        instance.mark_fixable(DiffersToLocal, &local_specifier);
        return;
      }
      debug!("{L6}is the same as the local instance");
      if instance.must_match_preferred_semver_range_which_is_not(&SemverRange::Exact) {
        let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
        debug!("{L7}it is in a semver group which prefers a different semver range to the local instance ({preferred_semver_range:?})");
        if instance.matches_preferred_semver_range() {
          debug!("{L8}its semver range matches its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&local_specifier) {
            debug!("{L9}the semver range satisfies the local version");
            debug!("{L10}mark as suspect (the config is asking for an inexact match)");
            instance.mark_valid(SatisfiesLocal, &instance.get_specifier_with_preferred_semver_range().unwrap());
          } else {
            debug!("{L9}the preferred semver range will not satisfy the local version");
            debug!("{L10}mark as unfixable error");
            instance.mark_conflict(MatchConflictsWithLocal);
          }
        } else {
          debug!("{L8}its semver range does not match its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&local_specifier) {
            debug!("{L9}the preferred semver range will satisfy the local version");
            debug!("{L10}mark as fixable error");
            instance.mark_fixable(SemverRangeMismatch, &instance.get_specifier_with_preferred_semver_range().unwrap());
          } else {
            debug!("{L9}the preferred semver range will not satisfy the local version");
            debug!("{L10}mark as unfixable error");
            instance.mark_conflict(MismatchConflictsWithLocal);
          }
        }
        return;
      }
      debug!("{L7}it is not in a semver group which prefers a different semver range to the local instance");
      if instance.already_equals(&local_specifier) {
        debug!("{L8}its semver range matches the local instance");
        debug!("{L9}mark as valid");
        instance.mark_valid(IsIdenticalToLocal, &local_specifier);
      } else {
        debug!("{L8}its semver range differs to the local instance");
        debug!("{L9}mark as error");
        instance.mark_fixable(DiffersToLocal, &local_specifier);
      }
    });
  } else if let Some(highest_specifier) = dependency.get_highest_or_lowest_specifier() {
    debug!("{L2}a highest semver version was found ({highest_specifier:?})");
    dependency.set_expected_specifier(&highest_specifier);
    dependency.instances.borrow().iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
      debug!("{L4}its version number (without a range):");
      if !instance.descriptor.specifier.has_same_version_number_as(&highest_specifier) {
        debug!("{L5}differs to the highest semver version");
        debug!("{L6}mark as error");
        instance.mark_fixable(DiffersToHighestOrLowestSemver, &highest_specifier);
        return;
      }
      debug!("{L5}is the same as the highest semver version");
      let range_of_highest_specifier = highest_specifier.get_semver_range().unwrap();
      if instance.must_match_preferred_semver_range_which_is_not(range_of_highest_specifier) {
        let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
        debug!(
          "{L6}it is in a semver group which prefers a different semver range to the highest semver version ({preferred_semver_range:?})"
        );
        if instance.matches_preferred_semver_range() {
          debug!("{L7}its semver range matches its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&highest_specifier) {
            debug!("{L8}the semver range satisfies the highest semver version");
            debug!("{L9}mark as suspect (the config is asking for an inexact match)");
            instance.mark_valid(SatisfiesHighestOrLowestSemver, &instance.descriptor.specifier);
          } else {
            debug!("{L8}the preferred semver range will not satisfy the highest semver version");
            debug!("{L9}mark as unfixable error");
            instance.mark_conflict(MatchConflictsWithHighestOrLowestSemver);
          }
        } else {
          debug!("{L7}its semver range does not match its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&highest_specifier) {
            debug!("{L8}the preferred semver range will satisfy the highest semver version");
            debug!("{L9}mark as fixable error");
            instance.mark_fixable(SemverRangeMismatch, &instance.get_specifier_with_preferred_semver_range().unwrap());
          } else {
            debug!("{L8}the preferred semver range will not satisfy the highest semver version");
            debug!("{L9}mark as unfixable error");
            instance.mark_conflict(MismatchConflictsWithHighestOrLowestSemver);
          }
        }
      } else {
        debug!("{L4}it is not in a semver group which prefers a different semver range to the highest semver version");
        if instance.already_equals(&highest_specifier) {
          debug!("{L5}it is identical to the highest semver version");
          debug!("{L6}mark as valid");
          instance.mark_valid(IsHighestOrLowestSemver, &highest_specifier);
        } else {
          debug!("{L5}it is different to the highest semver version");
          debug!("{L6}mark as error");
          instance.mark_fixable(DiffersToHighestOrLowestSemver, &highest_specifier);
        }
      }
    });
  } else {
    debug!("{L2}no instances have a semver version");
    if dependency.every_specifier_is_already_identical() {
      debug!("{L3}but all are identical");
      dependency.instances.borrow().iter().for_each(|instance| {
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
        debug!("{L5}it is identical to every other instance");
        debug!("{L6}mark as valid");
        instance.mark_valid(IsNonSemverButIdentical, &instance.descriptor.specifier);
      });
    } else {
      debug!("{L3}and they differ");
      dependency.instances.borrow().iter().for_each(|instance| {
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L4}visit instance '{}' ({actual_specifier:?})", instance.id);
        debug!("{L5}it depends on a currently unknowable correct version from a set of unsupported version specifiers");
        debug!("{L6}mark as error");
        instance.mark_unfixable(NonSemverMismatch);
      });
    }
  }
}

fn visit_ignored(dependency: &crate::dependency::Dependency) {
  debug!("visit ignored version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  dependency.instances.borrow().iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
    instance.mark_valid(IsIgnored, &instance.descriptor.specifier);
  });
}

fn visit_pinned(dependency: &crate::dependency::Dependency) {
  debug!("visit pinned version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  let pinned_specifier = dependency.pinned_specifier.clone().unwrap();
  dependency.set_expected_specifier(&pinned_specifier);
  dependency.instances.borrow().iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
    if instance.is_local {
      debug!("{L3}it is the local instance of a package developed locally in this monorepo");
      debug!("{L4}refuse to change it");
      debug!("{L5}mark as error, user should change their config");
      instance.mark_suspect(RefuseToPinLocal);
      return;
    }
    if instance.already_equals(&pinned_specifier) {
      debug!("{L3}it is identical to the pinned version");
      debug!("{L4}mark as valid");
      instance.mark_valid(IsIdenticalToPin, &pinned_specifier);
      return;
    }
    debug!("{L3}it depends on the local instance");
    debug!("{L4}its version number (without a range):");
    if !instance.descriptor.specifier.has_same_version_number_as(&pinned_specifier) {
      debug!("{L5}differs to the pinned version");
      debug!("{L6}mark as error");
      instance.mark_fixable(DiffersToPin, &pinned_specifier);
      return;
    }
    debug!("{L5}is the same as the pinned version");
    if instance.must_match_preferred_semver_range_which_differs_to(&pinned_specifier) {
      let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
      debug!("{L6}it is in a semver group which prefers a different semver range to the pinned version ({preferred_semver_range:?})");
      if instance.matches_preferred_semver_range() {
        debug!("{L7}its semver range matches its semver group");
        debug!("{L8}1. pin it and ignore the semver group");
        debug!("{L8}2. mark as suspect (the config is asking for a different range AND they want to pin it)");
        instance.mark_fixable(PinOverridesSemverRange, &pinned_specifier);
      } else {
        debug!("{L7}its semver range does not match its semver group or the pinned version's");
        debug!("{L8}1. pin it and ignore the semver group");
        debug!("{L8}2. mark as suspect (the config is asking for a different range AND they want to pin it)");
        instance.mark_fixable(PinOverridesSemverRangeMismatch, &pinned_specifier);
      }
      return;
    }
    debug!("{L6}it is not in a semver group which prefers a different semver range to the pinned version");
    debug!("{L7}it differs to the pinned version");
    debug!("{L8}mark as error");
    instance.mark_fixable(DiffersToPin, &pinned_specifier);
  });
}

fn visit_same_range(dependency: &crate::dependency::Dependency) {
  debug!("visit same range version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  dependency.instances.borrow().iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;
    debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);
    if instance.already_satisfies_all(&dependency.instances.borrow()) {
      debug!("{L3}its specifier satisfies all other instances in the group");
      if instance.must_match_preferred_semver_range() {
        debug!("{L4}it belongs to a semver group");
        if instance.matches_preferred_semver_range() {
          debug!("{L5}its specifier matches its semver group");
          instance.mark_valid(SatisfiesSameRangeGroup, actual_specifier);
        } else {
          debug!("{L5}its specifier mismatches its semver group");
          instance.mark_fixable(SemverRangeMismatch, &instance.get_specifier_with_preferred_semver_range().unwrap());
        }
      } else {
        debug!("{L4}it does not belong to a semver group");
        instance.mark_valid(SatisfiesSameRangeGroup, actual_specifier);
      }
    } else {
      debug!("{L3}its specifier does not satisfy all other instances in the group");
      instance.mark_unfixable(SameRangeMismatch);
    }
  });
}

fn visit_snapped_to(dependency: &crate::dependency::Dependency, ctx: &Context) {
  debug!("visit snapped to version group");
  debug!("{L1}visit dependency '{}'", dependency.internal_name);
  if let Some(snapped_to_specifier) = dependency.get_snapped_to_specifier(&ctx.instances) {
    debug!("{L2}a target version was found ({snapped_to_specifier:?})");
    dependency.set_expected_specifier(&snapped_to_specifier);
    dependency.instances.borrow().iter().for_each(|instance| {
      let actual_specifier = &instance.descriptor.specifier;
      debug!("{L3}visit instance '{}' ({actual_specifier:?})", instance.id);
      if instance.is_local && !instance.already_equals(&snapped_to_specifier) {
        debug!("{L4}it is the local instance of a package developed locally in this monorepo");
        debug!("{L5}refuse to change it");
        debug!("{L6}mark as error, user should change their config");
        instance.mark_suspect(RefuseToSnapLocal);
        return;
      }
      debug!("{L4}it is not a local instance of a package developed locally in this monorepo");
      debug!("{L5}its version number (without a range):");
      if !instance.descriptor.specifier.has_same_version_number_as(&snapped_to_specifier) {
        debug!("{L6}differs to the target version");
        debug!("{L7}mark as error");
        instance.mark_fixable(DiffersToSnapTarget, &snapped_to_specifier);
        return;
      }
      debug!("{L6}is the same as the target version");
      let range_of_snapped_to_specifier = snapped_to_specifier.get_semver_range().unwrap();
      if instance.must_match_preferred_semver_range_which_is_not(range_of_snapped_to_specifier) {
        let preferred_semver_range = &instance.preferred_semver_range.clone().unwrap();
        debug!("{L7}it is in a semver group which prefers a different semver range to the target version ({preferred_semver_range:?})");
        if instance.matches_preferred_semver_range() {
          debug!("{L8}its semver range matches its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&snapped_to_specifier) {
            debug!("{L9}the semver range satisfies the target version");
            debug!("{L10}mark as suspect (the config is asking for an inexact match)");
            instance.mark_valid(SatisfiesSnapTarget, &instance.descriptor.specifier);
          } else {
            debug!("{L9}the preferred semver range will not satisfy the target version");
            debug!("{L10}mark as unfixable error");
            instance.mark_conflict(MatchConflictsWithSnapTarget);
          }
        } else {
          debug!("{L8}its semver range does not match its semver group");
          if instance.specifier_with_preferred_semver_range_will_satisfy(&snapped_to_specifier) {
            debug!("{L9}the preferred semver range will satisfy the target version");
            debug!("{L10}mark as fixable error");
            instance.mark_fixable(SemverRangeMismatch, &instance.get_specifier_with_preferred_semver_range().unwrap());
          } else {
            debug!("{L9}the preferred semver range will not satisfy the target version");
            debug!("{L10}mark as unfixable error");
            instance.mark_conflict(MismatchConflictsWithSnapTarget);
          }
        }
      } else {
        debug!("{L5}it is not in a semver group which prefers a different semver range to the target version");
        if instance.already_equals(&snapped_to_specifier) {
          debug!("{L6}it is identical to the target version");
          debug!("{L7}mark as valid");
          instance.mark_valid(IsIdenticalToSnapTarget, &snapped_to_specifier);
        } else {
          debug!("{L6}it is different to the target version");
          debug!("{L7}mark as error");
          instance.mark_fixable(DiffersToSnapTarget, &snapped_to_specifier);
        }
      }
    });
  } else {
    debug!("{L2}no target version was found");
    dependency.instances.borrow().iter().for_each(|instance| {
      instance.mark_suspect(DependsOnMissingSnapTarget);
    });
  }
}
