use {
  super::ui::Ui,
  crate::{
    context::Context,
    instance_state::{FixableInstance, InstanceState, InvalidInstance, SuspectInstance},
  },
  colored::*,
  log::{info, warn},
};

/// Run the fix command side effects
pub fn run(ctx: Context) -> Context {
  let ui = Ui { ctx: &ctx };
  let has_cli_filter = ctx.config.cli.filter.is_some();
  let running_multiple_commands = ctx.config.cli.inspect_mismatches && ctx.config.cli.inspect_formatting;

  if ctx.config.cli.inspect_mismatches {
    if running_multiple_commands {
      ui.print_command_header("SEMVER RANGES AND VERSION MISMATCHES");
    }
    let mut valid = 0;
    let mut fixable = 0;
    let mut unfixable = 0;
    let mut suspect = 0;

    ctx.instances.iter().for_each(|instance| {
      let name = &instance.name;

      if has_cli_filter && !*instance.matches_cli_filter.borrow() {
        return;
      }

      let location = ui.instance_location(instance).dimmed();
      let state = instance.state.borrow().clone();
      let state_name = state.get_name();
      let state_link = ui.instance_status_code_link(&state_name);
      let state_link = format!("({state_link})").dimmed();

      match state {
        InstanceState::Unknown => {}
        InstanceState::Valid(variant) => {
          valid += 1;
        }
        InstanceState::Invalid(variant) => match variant {
          InvalidInstance::Fixable(variant) => {
            fixable += 1;
            match variant {
              FixableInstance::IsBanned => instance.remove(),
              _ => {
                let actual = instance.actual_specifier.unwrap().red();
                let arrow = ui.dim_right_arrow();
                let expected = instance.expected_specifier.borrow().as_ref().unwrap().unwrap().green();
                info!("{name} {actual} {arrow} {expected} {location} {state_link}");
                instance.package.borrow().copy_expected_specifier(instance);
              }
            }
          }
          InvalidInstance::Conflict(_) | InvalidInstance::Unfixable(_) => {
            unfixable += 1;
            warn!("Unfixable: {name} {location} {state_link}");
          }
        },
        InstanceState::Suspect(variant) => match variant {
          SuspectInstance::RefuseToBanLocal
          | SuspectInstance::RefuseToPinLocal
          | SuspectInstance::RefuseToSnapLocal
          | SuspectInstance::InvalidLocalVersion => {
            suspect += 1;
            warn!("Suspect: {name} {location} {state_link}");
          }
        },
      }
    });

    info!("{} {} Already Valid", ui.count_column(valid), ui.ok_icon());
    info!("{} {} Fixed", ui.count_column(fixable), ui.ok_icon());
    info!("{} {} Unfixable", ui.count_column(unfixable), ui.err_icon());
    info!("{} {} Suspect", ui.count_column(suspect), ui.warn_icon());
  }

  if ctx.config.cli.inspect_formatting {
    if running_multiple_commands {
      ui.print_command_header("PACKAGE FORMATTING");
    }
    ctx.packages.all.iter().for_each(|package| {
      let package = package.borrow();
      let mut formatting_mismatches = package.formatting_mismatches.borrow_mut();
      formatting_mismatches.iter().for_each(|mismatch| {
        if mismatch.property_path == "/" {
          *package.contents.borrow_mut() = mismatch.expected.clone();
        } else if let Some(value) = package.contents.borrow_mut().pointer_mut(&mismatch.property_path) {
          *value = mismatch.expected.clone();
        }
      });
      *formatting_mismatches = vec![];
    });
    ui.print_formatted_packages(&ctx.packages.all);
  }

  ctx.packages.all.iter().for_each(|package| {
    package.borrow().write_to_disk(&ctx.config);
  });

  ctx
}
