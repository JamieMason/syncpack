use {
  crate::{
    commands::{json::instance_to_json, ui},
    context::Context,
    instance::Instance,
    package_json::{FormatMismatch, PackageJson},
    version_group::{DependencyCore, VersionGroup},
  },
  serde_json::json,
  std::rc::Rc,
};

pub trait FixReporter {
  fn on_group_header(&self, ctx: &Context, group: &VersionGroup);
  fn on_dependency(&self, ctx: &Context, dependency: &DependencyCore, variant: &str);
  fn on_instance(&self, ctx: &Context, instance: &Instance, variant: &str);
  fn on_no_issues(&self);
  fn on_unfixable_warning(&self);
}

pub trait FormatReporter {
  fn on_package_header(&self, ctx: &Context, package: &PackageJson);
  fn on_mismatch_fixed(&self, ctx: &Context, package: &PackageJson, mismatch: &Rc<FormatMismatch>);
  fn on_mismatch_unfixed(&self, ctx: &Context, package: &PackageJson, mismatch: &Rc<FormatMismatch>);
  fn on_no_issues(&self);
}

// — Pretty implementations —

pub struct PrettyFixReporter;

impl FixReporter for PrettyFixReporter {
  fn on_group_header(&self, ctx: &Context, group: &VersionGroup) {
    ui::group::print_header(ctx, group);
  }

  fn on_dependency(&self, ctx: &Context, dependency: &DependencyCore, variant: &str) {
    ui::dependency::print_fixed(ctx, dependency, variant);
  }

  fn on_instance(&self, ctx: &Context, instance: &Instance, _variant: &str) {
    if ctx.config.cli.show_instances {
      ui::instance::print_fixed(ctx, instance);
    }
  }

  fn on_no_issues(&self) {
    ui::util::print_no_issues_found();
  }

  fn on_unfixable_warning(&self) {
    println!(" ");
    log::warn!("Some issues remain which cannot be fixed automatically, run syncpack lint to view them");
  }
}

pub struct PrettyFormatReporter;

impl FormatReporter for PrettyFormatReporter {
  fn on_package_header(&self, ctx: &Context, package: &PackageJson) {
    ui::package::print_invalid_package(ctx, package);
  }

  fn on_mismatch_fixed(&self, ctx: &Context, _package: &PackageJson, mismatch: &Rc<FormatMismatch>) {
    ui::package::print_fixed(ctx, mismatch);
  }

  fn on_mismatch_unfixed(&self, ctx: &Context, _package: &PackageJson, mismatch: &Rc<FormatMismatch>) {
    ui::package::print_invalid(ctx, mismatch);
  }

  fn on_no_issues(&self) {
    ui::util::print_no_issues_found();
  }
}

// — JSON implementations —

pub struct JsonFixReporter;

impl FixReporter for JsonFixReporter {
  fn on_group_header(&self, _ctx: &Context, _group: &VersionGroup) {}

  fn on_dependency(&self, _ctx: &Context, _dependency: &DependencyCore, _variant: &str) {}

  fn on_instance(&self, ctx: &Context, instance: &Instance, variant: &str) {
    let value = instance_to_json(ctx, instance, variant);
    println!("{}", serde_json::to_string(&value).unwrap());
  }

  fn on_no_issues(&self) {}

  fn on_unfixable_warning(&self) {}
}

pub struct JsonFormatReporter;

impl JsonFormatReporter {
  fn print_mismatch_json(&self, _ctx: &Context, package: &PackageJson, mismatch: &Rc<FormatMismatch>) {
    let value = json!({
      "package": &package.name,
      "filePath": package.file_path.to_string_lossy(),
      "property": mismatch.property_path.split('/').filter(|part| !part.is_empty()).collect::<Vec<&str>>(),
      "statusCode": format!("{:?}", mismatch.variant),
    });
    println!("{}", serde_json::to_string(&value).unwrap());
  }
}

impl FormatReporter for JsonFormatReporter {
  fn on_package_header(&self, _ctx: &Context, _package: &PackageJson) {}

  fn on_mismatch_fixed(&self, ctx: &Context, package: &PackageJson, mismatch: &Rc<FormatMismatch>) {
    self.print_mismatch_json(ctx, package, mismatch);
  }

  fn on_mismatch_unfixed(&self, ctx: &Context, package: &PackageJson, mismatch: &Rc<FormatMismatch>) {
    self.print_mismatch_json(ctx, package, mismatch);
  }

  fn on_no_issues(&self) {}
}
