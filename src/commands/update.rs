use {
  crate::{
    commands::ui::{self, LINE_ENDING, update_row},
    context::Context,
    disk::{DiskIo, copy_expected_specifier_json, copy_expected_specifier_yaml, write_json_file, write_yaml_file},
    errors::SyncpackError,
    instance::InstanceIdx,
    registry::updates::RegistryUpdates,
    source::Source,
    tui::{Tui, TuiReadiness, UpdateRow},
    version_group::{VersionGroup, VersionGroupBehavior},
  },
  log::{error, info, warn},
};

#[cfg(test)]
#[path = "update_test.rs"]
mod update_test;

/// Build the table of rows for the update view. Pure data transform —
/// no rendering, no I/O — so it can be unit-tested independently.
///
/// Outdated instances of each dependency are bucketed by their raw
/// current specifier (e.g. `^6.2.1` and `~6.2.1` are two buckets).
/// Each bucket becomes one `UpdateRow`. Buckets sort by count desc,
/// then by current version desc.
pub fn build_update_rows(ctx: &Context, registry_updates: &RegistryUpdates, now_unix_seconds: i64) -> Vec<UpdateRow> {
  let mut rows: Vec<UpdateRow> = Vec::new();

  for (group_idx, group) in ctx.version_groups.iter().enumerate() {
    let is_relevant_group = matches!(group, VersionGroup::PreferredSemver(g) if g.prefer_highest)
      || matches!(group, VersionGroup::CatalogDefs(_))
      || matches!(group, VersionGroup::SemverRangeOnly(_));
    if !is_relevant_group {
      continue;
    }
    let group_label = group.selector().label.clone();

    for dep in group.get_sorted_dependencies(&ctx.config.cli.sort) {
      let outdated: Vec<(InstanceIdx, &crate::instance::Instance)> = dep
        .get_sorted_instances(&ctx.instances, &ctx.sources.all)
        .filter(|(_, instance)| instance.is_outdated())
        .collect();
      if outdated.is_empty() {
        continue;
      }
      let dep_outdated_count = outdated.len();
      let times = registry_updates.times_by_internal_name.get(&dep.internal_name);

      // Bucket by raw current specifier, preserving discovery order
      // so two buckets with equal count fall back to a stable order
      // before we re-sort.
      let mut buckets: Vec<Bucket> = Vec::new();
      for (idx, instance) in outdated {
        let current_raw = instance.descriptor.specifier.get_raw().to_string();
        if let Some(b) = buckets.iter_mut().find(|b| b.current_raw == current_raw) {
          b.indices.push(idx);
          continue;
        }
        let expected = instance.expected_specifier.borrow();
        let expected_specifier = expected.as_ref().expect("outdated instance must have an expected specifier");
        let target_raw = expected_specifier.get_raw().to_string();
        let current_version = instance.descriptor.specifier.get_semver_number().map(String::from);
        let target_version = expected_specifier.get_semver_number().map(String::from);
        buckets.push(Bucket {
          current_raw,
          target_raw,
          current_version,
          target_version,
          indices: vec![idx],
        });
      }

      buckets.sort_by(|a, b| {
        b.indices
          .len()
          .cmp(&a.indices.len())
          .then_with(|| b.current_version.cmp(&a.current_version))
      });

      for bucket in buckets {
        let current_time_label = bucket
          .current_version
          .as_deref()
          .and_then(|v| times.and_then(|t| t.get(v)))
          .and_then(|iso| update_row::time_difference(iso, now_unix_seconds));
        let target_time_label = bucket
          .target_version
          .as_deref()
          .and_then(|v| times.and_then(|t| t.get(v)))
          .and_then(|iso| update_row::time_difference(iso, now_unix_seconds));

        rows.push(UpdateRow {
          group_idx,
          group_label: group_label.clone(),
          dependency_name: dep.internal_name.clone(),
          dependency_outdated_count: dep_outdated_count,
          bucket_count: bucket.indices.len(),
          current_raw: bucket.current_raw,
          current_time_label,
          target_raw: bucket.target_raw,
          target_time_label,
          instance_indices: bucket.indices,
        });
      }
    }
  }

  rows
}

struct Bucket {
  current_raw: String,
  target_raw: String,
  current_version: Option<String>,
  target_version: Option<String>,
  indices: Vec<InstanceIdx>,
}

/// Keep only the rows the user picked, then rewrite each remaining row's
/// `dependency_outdated_count` to the sum of bucket counts that survived
/// the filter for its (group, dependency). Otherwise a multi-bucket dep
/// with one bucket left would still print the original total.
pub fn filter_rows_for_display(rows: &[UpdateRow], selection: &[bool]) -> Vec<UpdateRow> {
  let mut filtered: Vec<UpdateRow> = rows
    .iter()
    .zip(selection.iter())
    .filter(|&(_, &picked)| picked)
    .map(|(row, _)| row.clone())
    .collect();
  let mut totals: std::collections::HashMap<(usize, String), usize> = std::collections::HashMap::new();
  for row in &filtered {
    *totals.entry((row.group_idx, row.dependency_name.clone())).or_insert(0) += row.bucket_count;
  }
  for row in &mut filtered {
    if let Some(total) = totals.get(&(row.group_idx, row.dependency_name.clone())) {
      row.dependency_outdated_count = *total;
    }
  }
  filtered
}

pub fn run<D: DiskIo>(mut ctx: Context, registry_updates: RegistryUpdates, io: &D, tui: &dyn Tui) -> Result<Context, SyncpackError> {
  let now = update_row::unix_now();
  let rows = build_update_rows(&ctx, &registry_updates, now);
  let was_outdated = !rows.is_empty();

  // Resolve which rows the user actually wants to apply.
  let (selection, applying): (Option<Vec<bool>>, Vec<usize>) = if !was_outdated || ctx.config.cli.check {
    (None, vec![])
  } else if ctx.config.cli.interactive {
    match tui.readiness(rows.len()) {
      TuiReadiness::Empty => (None, vec![]),
      TuiReadiness::NotTty => {
        warn!("--interactive requires a TTY");
        (None, vec![])
      }
      TuiReadiness::CannotMeasure => {
        warn!("--interactive cannot measure terminal size");
        (None, vec![])
      }
      TuiReadiness::TooSmall { min_cols, min_rows, .. } => {
        warn!("--interactive needs at least {min_rows} rows and {min_cols} columns");
        (None, vec![])
      }
      TuiReadiness::Ready { cols, rows: r } => match tui.pick(&rows, cols, r) {
        Some(picks) => {
          let mut sel = vec![false; rows.len()];
          for i in picks.iter().copied() {
            if i < rows.len() {
              sel[i] = true;
            }
          }
          let applying = picks.into_iter().filter(|i| *i < rows.len()).collect();
          (Some(sel), applying)
        }
        None => return Err(SyncpackError::Cancelled),
      },
    }
  } else {
    (None, (0..rows.len()).collect())
  };

  let filtered_for_display: Option<Vec<UpdateRow>> = selection.as_deref().map(|sel| filter_rows_for_display(&rows, sel));
  let display_rows: &[UpdateRow] = filtered_for_display.as_deref().unwrap_or(&rows);

  update_row::render_rows(display_rows, None);

  let counts = update_row::count_diffs(display_rows);
  update_row::render_summary(counts);

  // Apply selected rows by copying expected → actual through the
  // existing per-source dispatch.
  let mut copy_actions: Vec<InstanceIdx> = vec![];
  for &row_idx in &applying {
    copy_actions.extend(rows[row_idx].instance_indices.iter().copied());
  }

  for inst_idx in copy_actions {
    let source_idx = ctx.instances[inst_idx.0].source_idx();
    let consumer_file_idx: Option<usize> = match &ctx.sources.all[source_idx.0] {
      Source::Package { file_idx, .. } => Some(*file_idx),
      Source::PnpmYaml => None,
    };
    let instance = &ctx.instances[inst_idx.0];
    if let Some(fi) = consumer_file_idx {
      copy_expected_specifier_json(&mut ctx.disk.package_json_files[fi], instance);
    } else if let Some(yaml) = ctx.disk.pnpm_workspace.as_mut() {
      copy_expected_specifier_yaml(yaml, instance);
    }
  }

  if !registry_updates.failed.is_empty() {
    info!(" ");
    registry_updates.failed.iter().for_each(|name| {
      error!("Failed to fetch {name}");
    });
    warn!(
      "Syncpack does not yet support custom npm registries{LINE_ENDING}  Subscribe to https://github.com/JamieMason/syncpack/issues/220"
    );
  } else if !was_outdated {
    ui::util::print_no_issues_found();
  }

  if ctx.config.cli.check {
    return if was_outdated { Err(SyncpackError::IssuesFound) } else { Ok(ctx) };
  }

  if !ctx.config.cli.dry_run {
    let indent = ctx.config.rcfile.indent.as_deref();
    let fallback = ctx.disk.formatting_fallback();
    for file in ctx.disk.package_json_files.iter_mut() {
      write_json_file(file, io, indent, &fallback)?;
    }
    if let Some(yaml) = ctx.disk.pnpm_workspace.as_mut() {
      write_yaml_file(yaml, io, indent, &fallback)?;
    }
  }

  Ok(ctx)
}
