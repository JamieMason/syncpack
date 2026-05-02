use {
  crate::{commands::ui, context::Context, source::FormatMismatch},
  colored::*,
  log::info,
  std::path::Path,
};

pub fn print_invalid_package(ctx: &Context, name: &str, filepath: &Path, mismatch_count: usize) {
  let count_column = ui::util::count_column(mismatch_count);
  let file_link = get_package_json_link(ctx, filepath);
  let location = format!("at {file_link}").dimmed();
  info!("{count_column} {name} {location}");
}

pub fn print_invalid(ctx: &Context, mismatch: &FormatMismatch) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = ui::icon::err();
  let status_code = format!("{:?}", mismatch.variant);
  let status_code_link = ui::util::get_status_code_link(ctx, &status_code).red();
  let property_path = ui::util::get_formatted_path(&mismatch.property_path);
  let location = format!("at {property_path}").dimmed();
  info!("{indent} {icon} {status_code_link} {location}");
}

pub fn print_fixed(ctx: &Context, mismatch: &FormatMismatch) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = ui::icon::ok();
  let status_code = format!("{:?}", mismatch.variant);
  let status_code_link = ui::util::get_status_code_link(ctx, &status_code).dimmed();
  let property_path = ui::util::get_formatted_path(&mismatch.property_path);
  let location = format!("at {property_path}").dimmed();
  info!("{indent} {icon} {status_code_link} {location}");
}

/// Render a clickable link to a package.json file
pub fn get_package_json_link(ctx: &Context, package_file_path: &Path) -> String {
  let file_path = package_file_path.to_str().unwrap();
  let relative_file_path = get_relative_file_path(&ctx.disk.cwd, package_file_path);
  ui::util::get_link(ctx, format!("file:{file_path}"), relative_file_path)
}

fn get_relative_file_path(cwd: &Path, file_path: &Path) -> String {
  file_path
    .strip_prefix(cwd)
    .ok()
    .and_then(|path| path.to_str().map(|path_str| path_str.to_string()))
    .expect("Failed to create relative file path")
}
