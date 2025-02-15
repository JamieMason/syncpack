use {
  crate::{
    context::Context,
    dependency::Dependency,
    effects::ui,
    instance_state::{InstanceState, ValidInstance},
    version_group::VersionGroupVariant,
  },
  colored::*,
  log::{error, info},
};

pub fn print(ctx: &Context, dependency: &Dependency, group_variant: &VersionGroupVariant) {
  let instances_len = dependency.instances.len();
  let count = ui::util::count_column(instances_len);
  let name = &dependency.internal_name;
  let local_hint = get_local_hint(ctx, dependency);

  match &dependency.get_state() {
    InstanceState::Valid(variant) => match variant {
      ValidInstance::IsIgnored => {
        print_ignored(ctx, dependency, group_variant);
      }
      ValidInstance::IsHighestOrLowestSemver
      | ValidInstance::IsIdenticalToLocal
      | ValidInstance::IsIdenticalToPin
      | ValidInstance::IsIdenticalToSnapTarget
      | ValidInstance::IsLocalAndValid
      | ValidInstance::IsNonSemverButIdentical
      | ValidInstance::SatisfiesHighestOrLowestSemver
      | ValidInstance::SatisfiesLocal
      | ValidInstance::SatisfiesSnapTarget => {
        print_valid(ctx, dependency, group_variant);
      }
      ValidInstance::SatisfiesSameRangeGroup => {
        let line = ui::util::join_line(vec![&count, name, &local_hint]);
        info!("{line}");
      }
    },
    InstanceState::Invalid(variant) => {
      let name = name.red().to_string();
      let line = ui::util::join_line(vec![&count, &name, &local_hint]);
      info!("{line}");
    }
    InstanceState::Suspect(variant) => {
      let name = name.yellow().to_string();
      let line = ui::util::join_line(vec![&count, &name, &local_hint]);
      info!("{line}");
    }
    InstanceState::Unknown => {
      error!("Dependency '{name}' has an unknown state, this is a bug in syncpack");
      panic!("Unknown Dependency State");
    }
  }
}

pub fn print_ignored(ctx: &Context, dependency: &Dependency, group_variant: &VersionGroupVariant) {
  let instances_len = dependency.instances.len();
  let count = ui::util::count_column(instances_len);
  let name = &dependency.internal_name.dimmed().to_string();
  let local_hint = get_local_hint(ctx, dependency);
  let line = ui::util::join_line(vec![&count, &name]);
  info!("{line}");
}

pub fn print_valid(ctx: &Context, dependency: &Dependency, group_variant: &VersionGroupVariant) {
  let instances_len = dependency.instances.len();
  let count = ui::util::count_column(instances_len);
  let name = &dependency.internal_name;
  let local_hint = get_local_hint(ctx, dependency);
  let expected = get_raw_expected_specifier(dependency);
  let expected = expected.dimmed().to_string();
  let line = ui::util::join_line(vec![&count, name, &expected, &local_hint]);
  info!("{line}");
}

pub fn get_alias_hint(dependency: &Dependency) -> String {
  if dependency.has_alias {
    "[alias]".magenta().to_string()
  } else {
    "".to_string()
  }
}

fn get_local_hint(ctx: &Context, dependency: &Dependency) -> String {
  if ctx.config.cli.show_hints && dependency.local_instance.borrow().is_some() {
    "[local]".blue().to_string()
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
