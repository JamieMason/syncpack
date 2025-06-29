use {crate::context::Context, serde_json::json};

pub fn run(ctx: Context) -> i32 {
  let mut is_invalid = false;

  ctx
    .get_version_groups()
    .filter(|group| !group.has_ignored_variant() || ctx.config.cli.show_ignored)
    .for_each(|group| {
      group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        dependency.get_sorted_instances().for_each(|instance| {
          if instance.descriptor.matches_cli_filter {
            let package = instance.descriptor.package.borrow();
            let instance_json = json!({
              "dependency": instance.descriptor.name,
              "dependencyGroup": instance.descriptor.internal_name,
              "dependencyType": instance.descriptor.dependency_type.name,
              "package": package.file_path.to_string_lossy(),
              "property": instance.descriptor.dependency_type.path.split('/').filter(|part| !part.is_empty()).collect::<Vec<&str>>(),
              "strategy": instance.descriptor.dependency_type.strategy,
              "versionGroup": format!("{:?}", dependency.variant),
              "preferredSemverRange": instance.preferred_semver_range.as_ref().map(|range|range.unwrap()),
              "statusCode": instance.state.borrow().get_name(),
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
            });
            println!("{}", serde_json::to_string(&instance_json).unwrap());
          }
          if instance.is_invalid() || (instance.is_suspect() && ctx.config.rcfile.strict) {
            is_invalid = true;
          }
        });
      });
    });

  if is_invalid {
    1
  } else {
    0
  }
}
