use {
  crate::{
    context::Context,
    effects::ui,
    package_json::{FormatMismatch, PackageJson},
  },
  colored::*,
  log::info,
  std::rc::Rc,
};

pub fn print_invalid_package(ctx: &Context, package: &PackageJson) {
  let count = package.formatting_mismatches.borrow().len();
  let count_column = ui::util::count_column(count);
  let name = &package.name;
  let file_link = get_package_json_link(ctx, package);
  let location = format!("at {file_link}").dimmed();
  info!("{count_column} {name} {location}");
}

pub fn print_fixed_package(ctx: &Context, package: &PackageJson) {
  print_invalid_package(ctx, package);
}

pub fn print_invalid(ctx: &Context, mismatch: &Rc<FormatMismatch>) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = ui::icon::err();
  let status_code = format!("{:?}", mismatch.variant);
  let status_code_link = ui::util::get_status_code_link(ctx, &status_code).red();
  let property_path = ui::util::get_formatted_path(&mismatch.property_path);
  let location = format!("at {property_path}").dimmed();
  info!("{indent} {icon} {status_code_link} {location}");
}

pub fn print_fixed(ctx: &Context, mismatch: &Rc<FormatMismatch>) {
  let indent = " ".repeat(ui::DEFAULT_INDENT + 1);
  let icon = ui::icon::ok();
  let status_code = format!("{:?}", mismatch.variant);
  let status_code_link = ui::util::get_status_code_link(ctx, &status_code).dimmed();
  let property_path = ui::util::get_formatted_path(&mismatch.property_path);
  let location = format!("at {property_path}").dimmed();
  info!("{indent} {icon} {status_code_link} {location}");
}

/// Render a clickable link to a package.json file
pub fn get_package_json_link(ctx: &Context, package: &PackageJson) -> String {
  let file_path = package.file_path.to_str().unwrap();
  let relative_file_path = package.get_relative_file_path(&ctx.config.cli.cwd);
  ui::util::get_link(ctx, format!("file:{file_path}"), relative_file_path)
}
