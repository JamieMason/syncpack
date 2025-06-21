use {
  crate::{context::Context, version_group::VersionGroup},
  colored::*,
  log::info,
};

pub fn print_header(_ctx: &Context, group: &VersionGroup) {
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
