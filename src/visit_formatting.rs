use {
  crate::{
    context::Context,
    package_json::{FormatMismatch, FormatMismatchVariant::*, PackageJson},
  },
  std::{cell::RefCell, rc::Rc},
};

mod format;

pub fn visit_formatting(ctx: Context) -> Context {
  let add_mismatch = |package: &Rc<RefCell<PackageJson>>, mismatch: FormatMismatch| {
    let mismatch = Rc::new(mismatch);
    package.borrow().formatting_mismatches.borrow_mut().push(Rc::clone(&mismatch));
  };

  ctx.packages.all.iter().for_each(|package| {
    if ctx.config.rcfile.sort_packages || !ctx.config.rcfile.sort_first.is_empty() {
      if let Some(expected) = format::get_sorted_first(&ctx.config.rcfile, &package.borrow()) {
        add_mismatch(
          package,
          FormatMismatch {
            expected,
            property_path: "/".to_string(),
            variant: PackagePropertiesAreNotSorted,
          },
        );
      }
    }
    if ctx.config.rcfile.format_bugs {
      if let Some(expected) = format::get_formatted_bugs(&package.borrow()) {
        add_mismatch(
          package,
          FormatMismatch {
            expected,
            property_path: "/bugs".to_string(),
            variant: BugsPropertyIsNotFormatted,
          },
        );
      }
    }
    if ctx.config.rcfile.format_repository {
      if let Some(expected) = format::get_formatted_repository(&package.borrow()) {
        add_mismatch(
          package,
          FormatMismatch {
            expected,
            property_path: "/repository".to_string(),
            variant: RepositoryPropertyIsNotFormatted,
          },
        );
      }
    }
    if !ctx.config.rcfile.sort_exports.is_empty() {
      if let Some(expected) = format::get_sorted_exports(&ctx.config.rcfile, &package.borrow()) {
        add_mismatch(
          package,
          FormatMismatch {
            expected,
            property_path: "/exports".to_string(),
            variant: ExportsPropertyIsNotSorted,
          },
        );
      }
    }
    if !ctx.config.rcfile.sort_az.is_empty() {
      for key in ctx.config.rcfile.sort_az.iter() {
        if let Some(expected) = format::get_sorted_az(key, &package.borrow()) {
          add_mismatch(
            package,
            FormatMismatch {
              expected,
              property_path: format!("/{}", key),
              variant: PropertyIsNotSortedAz,
            },
          );
        }
      }
    }
  });

  ctx
}
