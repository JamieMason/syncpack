use {
  crate::context::Discovered,
  colored::*,
  log::info,
  syncpack_rules::version_group::{VersionGroup, VersionGroupBehavior},
};

pub fn print_header(_ctx: &Discovered, group: &VersionGroup) {
  let print_width = 80;
  let label = &group.selector().label;
  let header = format!("= {label} ");
  let divider = if header.len() < print_width {
    "=".repeat(print_width - header.len())
  } else {
    "".to_string()
  };
  let full_header = format!("{header}{divider}");
  info!("{}", full_header.blue());
}
