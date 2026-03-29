use {
  crate::{
    context::{Context, SyncpackError},
    instance::Instance,
  },
  serde_json::{json, Value},
};

pub fn instance_to_json(_ctx: &Context, instance: &Instance, variant_label: &str) -> Value {
  let package = instance.descriptor.package.borrow();
  json!({
    "dependency": instance.descriptor.name,
    "dependencyGroup": instance.descriptor.internal_name,
    "dependencyType": instance.descriptor.dependency_type.name,
    "package": package.file_path.to_string_lossy(),
    "property": instance.descriptor.dependency_type.path.split('/').filter(|part| !part.is_empty()).collect::<Vec<&str>>(),
    "strategy": instance.descriptor.dependency_type.strategy,
    "versionGroup": variant_label,
    "preferredSemverRange": instance.preferred_semver_range.as_ref().map(|range| range.unwrap()),
    "statusCode": instance.state.borrow().get_name(),
    "statusType": instance.state.borrow().get_status_type(),
    "actual": json!({
      "raw": instance.descriptor.specifier.get_raw(),
      "type": instance.descriptor.specifier.get_config_identifier(),
    }),
    "expected": instance.expected_specifier.borrow().as_ref().map(|expected|
      json!({
        "raw": expected.get_raw(),
        "type": expected.get_config_identifier(),
      }),
    ),
  })
}

pub fn run(ctx: Context) -> Result<Context, SyncpackError> {
  let mut is_invalid = false;

  ctx
    .version_groups
    .iter()
    .filter(|group| !group.is_ignored() || ctx.config.cli.show_ignored)
    .for_each(|group| {
      let variant_label = group.variant_label();
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dep| {
        dep.get_sorted_instances(&ctx.instances).for_each(|instance| {
          let instance_json = instance_to_json(&ctx, instance, variant_label);
          println!("{}", serde_json::to_string(&instance_json).unwrap());
          if instance.is_invalid() || (instance.is_suspect() && ctx.config.rcfile.strict) {
            is_invalid = true;
          }
        });
      });
    });

  if is_invalid {
    Err(SyncpackError::IssuesFound)
  } else {
    Ok(ctx)
  }
}
