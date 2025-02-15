use {
  crate::{
    context::Context,
    effects::ui,
    instance::Instance,
    instance_state::{
      FixableInstance, InstanceState, InvalidInstance, SemverGroupAndVersionConflict, SuspectInstance, UnfixableInstance, ValidInstance,
    },
    version_group::VersionGroupVariant,
  },
  colored::*,
  log::{error, info},
};

pub fn print(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) {
  let state = instance.state.borrow().clone();
  let indent = " ".repeat(ui::DEFAULT_INDENT);
  match &state {
    InstanceState::Valid(variant) => match variant {
      ValidInstance::IsIgnored => {
        let no_icon = " ";
        let actual = get_actual(instance).dimmed();
        let location = get_location(ctx, instance).dimmed();
        let state_link = get_state_link_in_parens(ctx, instance, group_variant);
        info!("{indent}{no_icon} {actual} {location} {state_link}");
      }
      ValidInstance::IsHighestOrLowestSemver
      | ValidInstance::IsIdenticalToLocal
      | ValidInstance::IsIdenticalToPin
      | ValidInstance::IsIdenticalToSnapTarget
      | ValidInstance::IsLocalAndValid
      | ValidInstance::IsNonSemverButIdentical
      | ValidInstance::SatisfiesHighestOrLowestSemver
      | ValidInstance::SatisfiesLocal
      | ValidInstance::SatisfiesSameRangeGroup
      | ValidInstance::SatisfiesSnapTarget => {
        let no_icon = " ";
        let actual = get_actual(instance).dimmed();
        let location = get_location(ctx, instance).dimmed();
        let state_link = get_state_link_in_parens(ctx, instance, group_variant);
        info!("{indent}{no_icon} {actual} {location} {state_link}");
      }
    },
    InstanceState::Invalid(variant) => match variant {
      InvalidInstance::Unfixable(UnfixableInstance::DependsOnInvalidLocalPackage)
      | InvalidInstance::Unfixable(UnfixableInstance::NonSemverMismatch)
      | InvalidInstance::Unfixable(UnfixableInstance::SameRangeMismatch)
      | InvalidInstance::Conflict(SemverGroupAndVersionConflict::MatchConflictsWithHighestOrLowestSemver)
      | InvalidInstance::Conflict(SemverGroupAndVersionConflict::MatchConflictsWithLocal)
      | InvalidInstance::Conflict(SemverGroupAndVersionConflict::MatchConflictsWithSnapTarget)
      | InvalidInstance::Conflict(SemverGroupAndVersionConflict::MismatchConflictsWithHighestOrLowestSemver)
      | InvalidInstance::Conflict(SemverGroupAndVersionConflict::MismatchConflictsWithLocal)
      | InvalidInstance::Conflict(SemverGroupAndVersionConflict::MismatchConflictsWithSnapTarget) => {
        let icon = ui::icon::err();
        let actual = get_actual(instance).red();
        let location = get_location(ctx, instance).dimmed();
        let state_link = get_state_link_in_parens(ctx, instance, group_variant);
        info!("{indent}{icon} {actual} {location} {state_link}");
      }
      InvalidInstance::Fixable(FixableInstance::DiffersToHighestOrLowestSemver)
      | InvalidInstance::Fixable(FixableInstance::DiffersToLocal)
      | InvalidInstance::Fixable(FixableInstance::DiffersToNpmRegistry)
      | InvalidInstance::Fixable(FixableInstance::DiffersToPin)
      | InvalidInstance::Fixable(FixableInstance::DiffersToSnapTarget)
      | InvalidInstance::Fixable(FixableInstance::IsBanned)
      | InvalidInstance::Fixable(FixableInstance::PinOverridesSemverRange)
      | InvalidInstance::Fixable(FixableInstance::PinOverridesSemverRangeMismatch)
      | InvalidInstance::Fixable(FixableInstance::SemverRangeMismatch) => {
        print_fixable(ctx, instance, group_variant);
      }
    },
    InstanceState::Suspect(variant) => match variant {
      SuspectInstance::DependsOnMissingSnapTarget
      | SuspectInstance::InvalidLocalVersion
      | SuspectInstance::RefuseToBanLocal
      | SuspectInstance::RefuseToPinLocal
      | SuspectInstance::RefuseToSnapLocal => {
        let icon = ui::icon::warn();
        let actual = get_actual(instance).yellow();
        let location = get_location(ctx, instance).dimmed();
        let state_link = get_state_link_in_parens(ctx, instance, group_variant);
        info!("{indent}{icon} {actual} {location} {state_link}");
      }
    },
    InstanceState::Unknown => {
      let location = get_location(ctx, instance);
      error!("Instance '{location}' has an unknown state, this is a bug in syncpack");
      panic!("Unknown Instance State");
    }
  }
}

pub fn print_fixable(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) {
  let indent = " ".repeat(ui::DEFAULT_INDENT);
  let icon = ui::icon::err();
  let suggested_fix = get_suggested_fix(instance);
  let location = get_location(ctx, instance).dimmed();
  let state_link = get_state_link_in_parens(ctx, instance, group_variant);
  info!("{indent}{icon} {suggested_fix} {location} {state_link}");
}

pub fn get_actual(instance: &Instance) -> String {
  let actual = instance.descriptor.specifier.get_raw();
  if actual.is_empty() {
    "VERSION_IS_MISSING".yellow().to_string()
  } else {
    actual
  }
}

pub fn get_expected(instance: &Instance) -> String {
  instance.expected_specifier.borrow().as_ref().unwrap().get_raw()
}

pub fn get_suggested_fix(instance: &Instance) -> String {
  let actual = get_actual(instance).red();
  let arrow = ui::icon::dim_right_arrow();
  let expected = get_expected(instance).green();
  format!("{actual} {arrow} {expected}")
}

/// Return a location hint for an instance
pub fn get_location(ctx: &Context, instance: &Instance) -> ColoredString {
  let path_to_prop = instance.descriptor.dependency_type.path.replace("/", ".");
  let file_link = ui::package::package_json_link(ctx, &instance.descriptor.package.borrow());
  format!("in {file_link} at {path_to_prop}").normal()
}

fn get_state_name(instance: &Instance, group_variant: &VersionGroupVariant) -> String {
  let state = instance.state.borrow().clone();
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
}

/// If enabled, render the reason code as a clickable link
pub fn get_state_link(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) -> String {
  if ctx.config.cli.show_status_codes {
    let state_name = get_state_name(instance, group_variant);
    ui::util::status_code_link(ctx, &state_name)
  } else {
    "".to_string()
  }
}

pub fn get_state_link_in_parens(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) -> String {
  let state_link = get_state_link(ctx, instance, group_variant);
  if !state_link.is_empty() {
    format!("({state_link})").dimmed().to_string()
  } else {
    state_link
  }
}
