use {
  crate::{
    commands::ui,
    context::Discovered,
    inspection::Inspection,
    instance::{InstanceState, Severity, ValidInstance},
    version_group::{Dependency, DependencyIdx},
  },
  colored::*,
  itertools::Itertools,
  log::{error, info},
};

pub fn print(ctx: &Discovered, inspection: &Inspection, dep_idx: DependencyIdx, dependency: &Dependency, group_variant: &str) {
  match &dependency.get_state(inspection) {
    InstanceState::Valid(ValidInstance::IsIgnored) => print_ignored(ctx, inspection, dep_idx, dependency, group_variant),
    InstanceState::Valid(_) => print_valid(ctx, inspection, dep_idx, dependency, group_variant),
    InstanceState::Invalid(_) => match max_severity(inspection, dependency) {
      // Every Invalid instance downgraded to Warn → render the rollup yellow
      // instead of red so the dep line matches the instance lines below.
      Some(Severity::Warn) | Some(Severity::None) | None => print_suspect(ctx, inspection, dep_idx, dependency, group_variant),
      _ => print_invalid(ctx, inspection, dep_idx, dependency, group_variant),
    },
    InstanceState::Suspect(_) => match max_severity(inspection, dependency) {
      Some(Severity::Error) | Some(Severity::Fix) => print_invalid(ctx, inspection, dep_idx, dependency, group_variant),
      _ => print_suspect(ctx, inspection, dep_idx, dependency, group_variant),
    },
    InstanceState::Unknown => {
      let name = &dependency.internal_name;
      error!("Dependency '{name}' has an unknown state, this is a bug in syncpack");
      unreachable!("Unknown Dependency State");
    }
  }
}

/// Highest resolved severity across this dependency's instances, ranked
/// Error > Fix > Warn > None. `None` when no instance has been resolved.
fn max_severity(inspection: &Inspection, dependency: &Dependency) -> Option<Severity> {
  dependency
    .instances
    .iter()
    .filter_map(|idx| inspection.severity(*idx))
    .max_by_key(|s| severity_rank(*s))
}

fn severity_rank(severity: Severity) -> u8 {
  match severity {
    Severity::None => 0,
    Severity::Warn => 1,
    Severity::Fix => 2,
    Severity::Error => 3,
  }
}

pub fn print_ignored(ctx: &Discovered, inspection: &Inspection, dep_idx: DependencyIdx, dependency: &Dependency, group_variant: &str) {
  let (count_column, name, local_hint, alias_hint, _, _) = get_common_parts(ctx, inspection, dep_idx, dependency, group_variant);
  let name = name.dimmed().to_string();
  let line = ui::util::join_line(vec![&count_column, &name, &local_hint, &alias_hint]);
  info!("{line}");
}

pub fn print_valid(ctx: &Discovered, inspection: &Inspection, dep_idx: DependencyIdx, dependency: &Dependency, group_variant: &str) {
  let (count_column, name, local_hint, alias_hint, expected_specifier, _) =
    get_common_parts(ctx, inspection, dep_idx, dependency, group_variant);
  let expected_specifier = if !expected_specifier.is_empty() {
    expected_specifier.dimmed().to_string()
  } else {
    expected_specifier
  };
  let line = ui::util::join_line(vec![&count_column, &name, &expected_specifier, &local_hint, &alias_hint]);
  info!("{line}");
}

pub fn print_invalid(ctx: &Discovered, inspection: &Inspection, dep_idx: DependencyIdx, dependency: &Dependency, group_variant: &str) {
  let (count_column, name, local_hint, alias_hint, expected_specifier, status_codes) =
    get_common_parts(ctx, inspection, dep_idx, dependency, group_variant);
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

pub fn print_outdated(ctx: &Discovered, inspection: &Inspection, dep_idx: DependencyIdx, dependency: &Dependency, group_variant: &str) {
  print_valid(ctx, inspection, dep_idx, dependency, group_variant);
}

pub fn print_suspect(ctx: &Discovered, inspection: &Inspection, dep_idx: DependencyIdx, dependency: &Dependency, group_variant: &str) {
  let (count_column, name, local_hint, alias_hint, expected_specifier, status_codes) =
    get_common_parts(ctx, inspection, dep_idx, dependency, group_variant);
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

pub fn print_fixed(ctx: &Discovered, inspection: &Inspection, dep_idx: DependencyIdx, dependency: &Dependency, group_variant: &str) {
  let (count_column, name, local_hint, alias_hint, expected_specifier, _) =
    get_common_parts(ctx, inspection, dep_idx, dependency, group_variant);
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

fn get_invalid_status_codes_in_brackets(ctx: &Discovered, inspection: &Inspection, dependency: &Dependency) -> String {
  if !ctx.config.cli.show_status_codes {
    return "".to_string();
  }
  let links = dependency
    .get_states(inspection)
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
  ctx: &Discovered,
  inspection: &Inspection,
  dep_idx: DependencyIdx,
  dependency: &Dependency,
  _group_variant: &str,
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
    get_raw_expected_specifier(inspection, dep_idx)
  };
  let status_codes = if ctx.config.cli.show_instances {
    // don't list statuses when we are listing every instance
    "".to_string()
  } else {
    get_invalid_status_codes_in_brackets(ctx, inspection, dependency)
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

fn get_local_hint(ctx: &Discovered, dependency: &Dependency) -> String {
  if ctx.config.cli.show_hints && dependency.local_instance.is_some() {
    "(local)".purple().to_string()
  } else {
    "".to_string()
  }
}

fn get_raw_expected_specifier(inspection: &Inspection, dep_idx: DependencyIdx) -> String {
  inspection.dep_expected[dep_idx.0]
    .as_ref()
    .map(|expected| expected.get_raw().to_string())
    .unwrap_or_default()
}
