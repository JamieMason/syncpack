use {
  crate::{
    commands::ui,
    context::Context,
    instance::Instance,
    instance_state::{InstanceState, InvalidInstance, ValidInstance},
    version_group::VersionGroupVariant,
  },
  colored::*,
  log::{error, info},
};

pub fn print(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) {
  match &instance.state.borrow().clone() {
    InstanceState::Valid(ValidInstance::IsIgnored) => print_ignored(ctx, instance, group_variant),
    InstanceState::Valid(_) => print_valid(ctx, instance, group_variant),
    InstanceState::Invalid(InvalidInstance::Unfixable(_)) => print_unfixable(ctx, instance, group_variant),
    InstanceState::Invalid(InvalidInstance::Conflict(_)) => print_unfixable(ctx, instance, group_variant),
    InstanceState::Invalid(InvalidInstance::Fixable(_)) => print_fixable(ctx, instance, group_variant),
    InstanceState::Suspect(_) => print_suspect(ctx, instance, group_variant),
    InstanceState::Unknown => {
      let location = get_location(ctx, instance);
      error!("Instance '{location}' has an unknown state, this is a bug in syncpack");
      panic!("Unknown Instance State");
    }
  }
}

pub fn print_ignored(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = "-".dimmed().to_string();
  let (actual, location, state_link, local_hint) = get_common_parts(ctx, instance, group_variant);
  let actual = actual.dimmed().to_string();
  let location = location.dimmed().to_string();
  let line = ui::util::join_line(vec![&indent, &icon, &actual, &location, &state_link, &local_hint]);
  info!("{line}");
}

pub fn print_valid(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = "-".dimmed().to_string();
  let (actual, location, state_link, local_hint) = get_common_parts(ctx, instance, group_variant);
  let location = location.dimmed().to_string();
  let line = ui::util::join_line(vec![&indent, &icon, &actual, &location, &state_link, &local_hint]);
  info!("{line}");
}

pub fn print_unfixable(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = ui::icon::err();
  let (actual, location, state_link, local_hint) = get_common_parts(ctx, instance, group_variant);
  let actual = actual.red().to_string();
  let location = location.dimmed().to_string();
  let line = ui::util::join_line(vec![&indent, &icon, &actual, &location, &state_link, &local_hint]);
  info!("{line}");
}

pub fn print_fixable(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = ui::icon::err();
  let actual = get_actual(instance).red();
  let arrow = ui::icon::dim_right_arrow();
  let expected = get_expected(instance).dimmed();
  let suggested_fix = format!("{actual} {arrow} {expected}");
  let (_, location, state_link, local_hint) = get_common_parts(ctx, instance, group_variant);
  let location = location.dimmed().to_string();
  let line = ui::util::join_line(vec![&indent, &icon, &suggested_fix, &location, &state_link, &local_hint]);
  info!("{line}");
}

pub fn print_outdated(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = ui::icon::blue_err();
  let expected = get_expected(instance).blue();
  let arrow = ui::icon::dim_left_arrow();
  let actual = get_actual(instance).dimmed();
  let suggested_fix = format!("{expected} {arrow} {actual}");
  let (_, location, state_link, local_hint) = get_common_parts(ctx, instance, group_variant);
  let location = location.dimmed().to_string();
  let line = ui::util::join_line(vec![&indent, &icon, &suggested_fix, &location, &state_link, &local_hint]);
  info!("{line}");
}

pub fn print_suspect(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = ui::icon::warn();
  let (actual, location, state_link, local_hint) = get_common_parts(ctx, instance, group_variant);
  let actual = actual.yellow().to_string();
  let location = location.dimmed().to_string();
  let line = ui::util::join_line(vec![&indent, &icon, &actual, &location, &state_link, &local_hint]);
  info!("{line}");
}

pub fn print_fixed(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = ui::icon::ok();
  let expected = get_expected(instance);
  let arrow = ui::icon::dim_left_arrow();
  let actual = get_actual(instance).dimmed();
  let applied_fix = format!("{expected} {arrow} {actual}");
  let (_, location, state_link, local_hint) = get_common_parts(ctx, instance, group_variant);
  let location = location.dimmed().to_string();
  let line = ui::util::join_line(vec![&indent, &icon, &applied_fix, &location, &state_link, &local_hint]);
  info!("{line}");
}

fn get_common_parts(ctx: &Context, instance: &Instance, group_variant: &VersionGroupVariant) -> (String, String, String, String) {
  let actual = get_actual(instance);
  let location = get_location(ctx, instance);
  let state_link = get_state_link_in_parens(ctx, instance, group_variant);
  let local_hint = get_local_hint(ctx, instance);
  (actual, location, state_link, local_hint)
}

pub fn get_actual(instance: &Instance) -> String {
  let actual = instance.descriptor.specifier.get_raw();
  if actual.is_empty() {
    "VERSION_IS_MISSING".yellow().to_string()
  } else {
    actual
  }
}

fn get_local_hint(ctx: &Context, instance: &Instance) -> String {
  if ctx.config.cli.show_hints && instance.is_local {
    "(local)".purple().to_string()
  } else {
    "".to_string()
  }
}

pub fn get_expected(instance: &Instance) -> String {
  instance.expected_specifier.borrow().as_ref().unwrap().get_raw()
}

pub fn get_location(ctx: &Context, instance: &Instance) -> String {
  let alias_info = if instance.descriptor.name != instance.descriptor.internal_name {
    format!("of {}", instance.descriptor.name)
  } else {
    "".to_string()
  };
  let path_to_prop = instance.descriptor.dependency_type.path.replace("/", ".");
  let file_link = ui::package::get_package_json_link(ctx, &instance.descriptor.package.borrow());
  ui::util::join_line(vec![&alias_info, &"in".to_string(), &file_link, &"at".to_string(), &path_to_prop])
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
    ui::util::get_status_code_link(ctx, &state_name)
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
