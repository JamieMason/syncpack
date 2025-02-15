use {
  crate::{
    context::Context,
    effects::ui,
    package_json::{FormatMismatch, FormatMismatchVariant, PackageJson},
  },
  colored::*,
  itertools::Itertools,
  log::info,
  std::{cell::RefCell, rc::Rc},
};

/// Packages which are correctly formatted
pub fn print_formatted(ctx: &Context, packages: &[Rc<RefCell<PackageJson>>]) {
  if !packages.is_empty() {
    let icon = ui::icon::ok();
    let count = ui::util::count_column(packages.len());
    let status = "Valid".green();
    info!("{count} {icon} {status}");
    if ctx.config.cli.show_packages {
      packages
        .iter()
        .sorted_by_key(|package| package.borrow().name.clone())
        .for_each(|package| {
          print_formatted_package(ctx, &package.borrow());
        });
    }
  }
}

/// Print a package.json which is correctly formatted
fn print_formatted_package(ctx: &Context, package: &PackageJson) {
  if package.formatting_mismatches.borrow().is_empty() {
    let icon = "-".dimmed();
    let file_link = package_json_link(ctx, package).dimmed();
    info!("          {icon} {file_link}");
  }
}

/// Print every package.json which has the given formatting mismatch
pub fn print_formatting_mismatches(ctx: &Context, variant: &FormatMismatchVariant, mismatches: &[Rc<FormatMismatch>]) {
  let count = ui::util::count_column(mismatches.len());
  let icon = ui::icon::err();
  let status_code = format!("{:?}", variant);
  let link = ui::util::status_code_link(ctx, &status_code).red();
  info!("{count} {icon} {link}");
  if ctx.config.cli.show_packages {
    mismatches
      .iter()
      .sorted_by_key(|mismatch| mismatch.package.borrow().name.clone())
      .for_each(|mismatch| {
        let icon = "-".dimmed();
        let package = mismatch.package.borrow();
        let property_path = ui::util::format_path(&mismatch.property_path);
        let file_link = package_json_link(ctx, &package);
        let msg = format!("          {icon} {property_path} of {file_link}").red();
        info!("{msg}");
      });
  }
}

/// Render a clickable link to a package.json file
pub fn package_json_link(ctx: &Context, package: &PackageJson) -> String {
  let file_path = package.file_path.to_str().unwrap();
  ui::util::link(ctx, format!("file:{file_path}"), package.name.clone())
}
