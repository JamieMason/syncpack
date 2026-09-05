use crate::{
  commands::ui,
  context::Inspected,
  errors::SyncpackError,
  instance::Severity,
  version_group::{InstanceAction, action_of},
};

/// Run the lint command side effects
pub fn run(inspected: Inspected) -> Result<Inspected, SyncpackError> {
  let mut is_invalid = false;
  let data = &inspected.data;
  let inspection = &inspected.inspection;

  data.version_groups.iter().for_each(|group| {
    let mut has_printed_group = false;
    for dep_idx in group.get_sorted_dependencies(&data.dependencies, &data.config.cli.sort) {
      let dependency = &data.dependencies[dep_idx.0];
      let mut has_printed_dependency = false;
      for (idx, instance) in dependency.get_sorted_instances(inspection, &data.instances, &data.sources.all) {
        let action = action_of(inspection.severity(idx).unwrap_or(Severity::None));
        if matches!(action, InstanceAction::Valid) {
          continue;
        }
        if !has_printed_group {
          ui::group::print_header(data, group);
          has_printed_group = true;
        }
        if !has_printed_dependency {
          ui::dependency::print(data, inspection, dep_idx, dependency, group.variant_label());
          has_printed_dependency = true;
        }
        if data.config.cli.show_instances {
          ui::instance::print(data, inspection, idx, instance);
        }
        if matches!(action, InstanceAction::Render(Severity::Error) | InstanceAction::Fix(_)) {
          is_invalid = true;
        }
      }
    }
  });

  if is_invalid {
    Err(SyncpackError::IssuesFound)
  } else {
    ui::util::print_no_issues_found();
    Ok(inspected)
  }
}
