use {
  crate::{
    context::{Discovered, Inspected},
    errors::SyncpackError,
    inspection::Inspection,
    instance::{Instance, InstanceIdx, Severity},
    source::Source,
    version_group::{InstanceAction, action_of},
  },
  serde_json::{Value, json},
};

pub fn instance_to_json(ctx: &Discovered, inspection: &Inspection, idx: InstanceIdx, instance: &Instance, variant_label: &str) -> Value {
  let package_path = match &ctx.sources.all[instance.source_idx().0] {
    Source::Package { file_idx, .. } => ctx.disk.package_json_files[*file_idx].filepath.to_string_lossy().to_string(),
    Source::PnpmYaml => "pnpm-workspace.yaml".to_string(),
  };
  let severity = inspection.severity(idx).unwrap_or(Severity::None);
  json!({
    "dependency": instance.descriptor.name,
    "dependencyGroup": instance.descriptor.internal_name,
    "dependencyType": instance.descriptor.dependency_type.name,
    "package": package_path,
    "property": instance.descriptor.dependency_type.path.split('/').filter(|part| !part.is_empty()).collect::<Vec<&str>>(),
    "strategy": instance.descriptor.dependency_type.strategy,
    "versionGroup": variant_label,
    "preferredSemverRange": instance.preferred_semver_range.as_ref().map(|range| range.unwrap()),
    "statusCode": inspection.state(idx).get_name(),
    "statusType": inspection.state(idx).get_status_type(),
    "severity": severity,
    "actual": json!({
      "raw": instance.descriptor.specifier.get_raw(),
      "type": instance.descriptor.specifier.get_config_identifier(),
    }),
    "expected": inspection.expected(idx).map(|expected|
      json!({
        "raw": expected.get_raw(),
        "type": expected.get_config_identifier(),
      }),
    ),
  })
}

pub fn run(inspected: Inspected) -> Result<Inspected, SyncpackError> {
  let mut is_invalid = false;
  let data = &inspected.data;
  let inspection = &inspected.inspection;

  data
    .version_groups
    .iter()
    .filter(|group| !group.is_ignored() || data.config.cli.show_ignored)
    .for_each(|group| {
      let variant_label = group.variant_label();
      for dep_idx in group.get_sorted_dependencies(&data.dependencies, &data.config.cli.sort) {
        let dependency = &data.dependencies[dep_idx.0];
        for (idx, instance) in dependency.get_sorted_instances(inspection, &data.instances, &data.sources.all) {
          let action = action_of(inspection.severity(idx).unwrap_or(Severity::None));
          let instance_json = instance_to_json(data, inspection, idx, instance, variant_label);
          println!("{}", serde_json::to_string(&instance_json).unwrap());
          if matches!(action, InstanceAction::Render(Severity::Error) | InstanceAction::Fix(_)) {
            is_invalid = true;
          }
        }
      }
    });

  if is_invalid {
    Err(SyncpackError::IssuesFound)
  } else {
    Ok(inspected)
  }
}
