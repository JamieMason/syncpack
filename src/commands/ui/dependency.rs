use {
  crate::{
    commands::ui,
    context::Context,
    dependency::Dependency,
    instance_state::{InstanceState, ValidInstance},
    version_group::VersionGroupVariant,
  },
  colored::*,
  itertools::Itertools,
  log::{error, info},
};

pub fn print(ctx: &Context, dependency: &Dependency, group_variant: &VersionGroupVariant) {
  match &dependency.get_state() {
    InstanceState::Valid(ValidInstance::IsIgnored) => print_ignored(ctx, dependency, group_variant),
    InstanceState::Valid(_) => print_valid(ctx, dependency, group_variant),
    InstanceState::Invalid(_) => print_invalid(ctx, dependency, group_variant),
    InstanceState::Suspect(_) => print_suspect(ctx, dependency, group_variant),
    InstanceState::Unknown => {
      let name = &dependency.internal_name;
      error!("Dependency '{name}' has an unknown state, this is a bug in syncpack");
      panic!("Unknown Dependency State");
    }
  }
}

pub fn print_ignored(ctx: &Context, dependency: &Dependency, group_variant: &VersionGroupVariant) {
  let (count_column, name, local_hint, alias_hint, _, _) = get_common_parts(ctx, dependency, group_variant);
  let name = name.dimmed().to_string();
  let line = ui::util::join_line(vec![&count_column, &name, &local_hint, &alias_hint]);
  info!("{line}");
}

pub fn print_valid(ctx: &Context, dependency: &Dependency, group_variant: &VersionGroupVariant) {
  let (count_column, name, local_hint, alias_hint, expected_specifier, _) = get_common_parts(ctx, dependency, group_variant);
  let expected_specifier = if !expected_specifier.is_empty() {
    expected_specifier.dimmed().to_string()
  } else {
    expected_specifier
  };
  let line = ui::util::join_line(vec![&count_column, &name, &expected_specifier, &local_hint, &alias_hint]);
  info!("{line}");
}

pub fn print_invalid(ctx: &Context, dependency: &Dependency, group_variant: &VersionGroupVariant) {
  let (count_column, name, local_hint, alias_hint, expected_specifier, status_codes) = get_common_parts(ctx, dependency, group_variant);
  let expected_specifier = if !expected_specifier.is_empty() {
    expected_specifier.red().to_string()
  } else {
    expected_specifier
  };
  let status_codes = if !status_codes.is_empty() {
    status_codes.dimmed().to_string()
  } else {
    status_codes
  };
  let line = ui::util::join_line(vec![
    &count_column,
    &name,
    &expected_specifier,
    &local_hint,
    &alias_hint,
    &status_codes,
  ]);
  info!("{line}");
}

pub fn print_outdated(ctx: &Context, dependency: &Dependency, group_variant: &VersionGroupVariant) {
  print_valid(ctx, dependency, group_variant);
}

pub fn print_suspect(ctx: &Context, dependency: &Dependency, group_variant: &VersionGroupVariant) {
  let (count_column, name, local_hint, alias_hint, expected_specifier, status_codes) = get_common_parts(ctx, dependency, group_variant);
  // let name = name.yellow().to_string();
  let expected_specifier = if !expected_specifier.is_empty() {
    expected_specifier.yellow().to_string()
  } else {
    expected_specifier
  };
  let status_codes = if !status_codes.is_empty() {
    status_codes.dimmed().to_string()
  } else {
    status_codes
  };
  let line = ui::util::join_line(vec![
    &count_column,
    &name,
    &expected_specifier,
    &local_hint,
    &alias_hint,
    &status_codes,
  ]);
  info!("{line}");
}

pub fn print_fixed(ctx: &Context, dependency: &Dependency, group_variant: &VersionGroupVariant) {
  let (count_column, name, local_hint, alias_hint, expected_specifier, _) = get_common_parts(ctx, dependency, group_variant);
  let icon = if ctx.config.cli.show_instances {
    "".to_string()
  } else {
    ui::icon::ok()
  };
  let expected_specifier = if !expected_specifier.is_empty() {
    expected_specifier.dimmed().to_string()
  } else {
    expected_specifier
  };
  let line = ui::util::join_line(vec![&count_column, &icon, &name, &expected_specifier, &local_hint, &alias_hint]);
  info!("{line}");
}

fn get_invalid_status_codes_in_brackets(ctx: &Context, dependency: &Dependency) -> String {
  if !ctx.config.cli.show_status_codes {
    return "".to_string();
  }
  let links = dependency
    .get_states()
    .iter()
    .filter(|state| matches!(state, InstanceState::Invalid(_) | InstanceState::Suspect(_)))
    .map(|state| state.get_name())
    .unique()
    .map(|state_name| ui::util::get_status_code_link(ctx, &state_name))
    .sorted()
    .collect::<Vec<String>>();
  if links.is_empty() {
    "".to_string()
  } else {
    let links = links.join(", ");
    format!("({links})")
  }
}

fn get_common_parts(
  ctx: &Context,
  dependency: &Dependency,
  _group_variant: &VersionGroupVariant,
) -> (String, String, String, String, String, String) {
  let instances_len = dependency.instances.len();
  let count_column = ui::util::count_column(instances_len);
  let name = dependency.internal_name.to_string();
  let local_hint = get_local_hint(ctx, dependency);
  let alias_hint = get_alias_hint(dependency);
  let expected_specifier = if ctx.config.cli.show_instances {
    // don't repeat expected specifier when we are listing every instance
    "".to_string()
  } else {
    get_raw_expected_specifier(dependency)
  };
  let status_codes = if ctx.config.cli.show_instances {
    // don't list statuses when we are listing every instance
    "".to_string()
  } else {
    get_invalid_status_codes_in_brackets(ctx, dependency)
  };
  (count_column, name, local_hint, alias_hint, expected_specifier, status_codes)
}

pub fn get_alias_hint(dependency: &Dependency) -> String {
  if dependency.has_alias {
    "(aliased)".purple().to_string()
  } else {
    "".to_string()
  }
}

fn get_local_hint(ctx: &Context, dependency: &Dependency) -> String {
  if ctx.config.cli.show_hints && dependency.local_instance.borrow().is_some() {
    "(local)".purple().to_string()
  } else {
    "".to_string()
  }
}

fn get_raw_expected_specifier(dependency: &Dependency) -> String {
  dependency
    .expected
    .borrow()
    .as_ref()
    .map(|expected| expected.get_raw())
    .unwrap_or_default()
}
