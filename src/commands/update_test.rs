use {
  crate::{
    commands::update::{build_update_rows, filter_rows_for_display},
    test::{builder::TestBuilder, mock_tui::MockTui},
  },
  serde_json::json,
  std::collections::HashMap,
};

/// `2024-01-15T00:00:00Z` in Unix seconds — used as the frozen `now`
/// reference across tests in this file.
const FROZEN_NOW: i64 = 1_705_276_800;

fn times(pairs: &[(&str, &str)]) -> HashMap<String, String> {
  pairs.iter().map(|(v, t)| (v.to_string(), t.to_string())).collect()
}

#[tokio::test]
async fn no_registry_updates_returns_empty_rows() {
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "dependencies": {"foo": "^1.0.0"}
    })])
    .with_registry_updates(json!({}))
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  assert!(rows.is_empty());
}

#[tokio::test]
async fn one_outdated_dep_produces_one_row() {
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "dependencies": {"foo": "^1.0.0"}
    })])
    .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}))
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  assert_eq!(rows.len(), 1);
  let row = &rows[0];
  assert_eq!(row.dependency_name, "foo");
  assert_eq!(row.dependency_outdated_count, 1);
  assert_eq!(row.bucket_count, 1);
  assert_eq!(row.current_raw, "^1.0.0");
  assert_eq!(row.target_raw, "^1.0.1");
}

#[tokio::test]
async fn instances_with_same_current_specifier_collapse_into_one_bucket() {
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.0.0"}
      }),
      json!({
        "name": "package-b",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.0.0"}
      }),
    ])
    .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}))
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  assert_eq!(rows.len(), 1);
  assert_eq!(rows[0].bucket_count, 2);
  assert_eq!(rows[0].dependency_outdated_count, 2);
  assert_eq!(rows[0].instance_indices.len(), 2);
}

#[tokio::test]
async fn different_current_specifiers_become_separate_buckets() {
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.0.0"}
      }),
      json!({
        "name": "package-b",
        "version": "1.0.0",
        "dependencies": {"foo": "~1.0.0"}
      }),
      json!({
        "name": "package-c",
        "version": "1.0.0",
        "dependencies": {"foo": "1.0.0"}
      }),
    ])
    .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}))
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  assert_eq!(rows.len(), 3);
  for row in &rows {
    assert_eq!(row.bucket_count, 1);
    assert_eq!(row.dependency_outdated_count, 3);
  }
  let currents: Vec<&str> = rows.iter().map(|r| r.current_raw.as_str()).collect();
  assert!(currents.contains(&"^1.0.0"));
  assert!(currents.contains(&"~1.0.0"));
  assert!(currents.contains(&"1.0.0"));
}

#[tokio::test]
async fn buckets_sort_by_count_desc_then_version_desc() {
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.0.0"}
      }),
      json!({
        "name": "package-b",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.0.0"}
      }),
      json!({
        "name": "package-c",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.5.0"}
      }),
    ])
    .with_registry_updates(json!({"foo": ["1.0.0", "1.5.0", "1.5.1"]}))
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  assert_eq!(rows.len(), 2);
  // Larger bucket first (count desc).
  assert_eq!(rows[0].current_raw, "^1.0.0");
  assert_eq!(rows[0].bucket_count, 2);
  assert_eq!(rows[1].current_raw, "^1.5.0");
  assert_eq!(rows[1].bucket_count, 1);
}

#[tokio::test]
async fn registry_times_populate_time_labels() {
  let fourteen_days_before_frozen_now = "2024-01-01T00:00:00Z";
  let three_days_before_frozen_now = "2024-01-12T00:00:00Z";
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "dependencies": {"foo": "^1.0.0"}
    })])
    .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}))
    .with_registry_times(
      "foo",
      times(&[("1.0.0", fourteen_days_before_frozen_now), ("1.0.1", three_days_before_frozen_now)]),
    )
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  assert_eq!(rows.len(), 1);
  assert_eq!(rows[0].current_time_label.as_deref(), Some("~14d"));
  assert_eq!(rows[0].target_time_label.as_deref(), Some("~3d"));
}

#[tokio::test]
async fn ignored_dep_via_update_group_does_not_appear_in_rows() {
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "dependencies": {"foo": "^1.0.0"}
    })])
    .with_update_group(json!({
      "dependencies": ["foo"],
      "isIgnored": true
    }))
    .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}))
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  assert!(rows.is_empty(), "ignored deps must not appear in update rows");
}

/// Severity is owned by `versionGroups` and gates lint/fix/list/json only.
/// `update` behaviour is owned by `updateGroups`. Setting severity on a
/// version group must NOT affect what update rows are produced — plan §3.6.
#[tokio::test]
async fn severity_on_version_group_does_not_affect_update_rows() {
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "dependencies": {"foo": "^1.0.0"}
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "severity": {"DiffersToHighestOrLowestSemver": "warn"}
    }))
    .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}))
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  assert_eq!(rows.len(), 1, "update rows are unaffected by severity config");
  assert_eq!(rows[0].dependency_name, "foo");
  assert_eq!(rows[0].current_raw, "^1.0.0");
  assert_eq!(rows[0].target_raw, "^1.0.1");
}

#[tokio::test]
async fn missing_registry_times_yield_none_labels() {
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "dependencies": {"foo": "^1.0.0"}
    })])
    .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}))
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  assert_eq!(rows.len(), 1);
  assert!(rows[0].current_time_label.is_none());
  assert!(rows[0].target_time_label.is_none());
}

#[tokio::test]
async fn mock_tui_select_all_picks_every_row() {
  use crate::tui::Tui;
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "dependencies": {"foo": "^1.0.0", "bar": "^2.0.0"}
    })])
    .with_registry_updates(json!({
      "foo": ["1.0.0", "1.0.1"],
      "bar": ["2.0.0", "2.1.0"],
    }))
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  let tui = MockTui::select_all();
  let picks = tui.pick(&rows, 80, 24).expect("MockTui::select_all returns Some");
  assert_eq!(picks.len(), rows.len());
}

#[tokio::test]
async fn mock_tui_cancel_returns_none() {
  use crate::tui::Tui;
  let rows = vec![];
  let tui = MockTui::cancel();
  assert!(tui.pick(&rows, 80, 24).is_none());
}

#[tokio::test]
async fn filter_rows_for_display_drops_unselected_and_recomputes_dep_total() {
  let (ctx, updates) = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.0.0"}
      }),
      json!({
        "name": "package-b",
        "version": "1.0.0",
        "dependencies": {"foo": "~1.0.0"}
      }),
      json!({
        "name": "package-c",
        "version": "1.0.0",
        "dependencies": {"foo": "1.0.0"}
      }),
    ])
    .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}))
    .run_with_updates()
    .await;
  let rows = build_update_rows(&ctx, &updates.unwrap(), FROZEN_NOW);
  assert_eq!(rows.len(), 3, "three buckets for foo");
  for row in &rows {
    assert_eq!(row.dependency_outdated_count, 3, "pre-filter total reflects all buckets");
  }

  let selection = vec![true, false, false];
  let filtered = filter_rows_for_display(&rows, &selection);
  assert_eq!(filtered.len(), 1, "only the selected bucket survives");
  assert_eq!(
    filtered[0].dependency_outdated_count, 1,
    "dep total recomputed to selected bucket counts"
  );
  assert_eq!(filtered[0].bucket_count, 1);
}

mod apply {
  use {
    super::*,
    crate::{commands::update, errors::SyncpackError, registry::updates::RegistryUpdates, test::mock_disk::MockDiskIo},
  };

  /// Run the full `update::run` flow with a frozen MockTui and return
  /// the resulting context so callers can inspect which package_json
  /// files got marked dirty by the apply step. Returns the raw `Result`
  /// so cancel-style tests can match `Err(Cancelled)` directly.
  async fn try_run_update(builder: TestBuilder, tui: MockTui) -> Result<crate::context::Context, SyncpackError> {
    let (mut ctx, updates) = builder.run_with_updates().await;
    // Force apply path: turn off --check, keep dry-run on so the
    // mutation stays visible without touching disk.
    ctx.config.cli.check = false;
    ctx.config.cli.dry_run = true;
    ctx.config.cli.interactive = true;
    let disk = MockDiskIo::new();
    let updates = updates.unwrap_or_else(|| RegistryUpdates {
      updates_by_internal_name: Default::default(),
      times_by_internal_name: Default::default(),
      failed: vec![],
    });
    update::run(ctx, updates, &disk, &tui)
  }

  async fn run_update(builder: TestBuilder, tui: MockTui) -> crate::context::Context {
    try_run_update(builder, tui).await.expect("update::run failed")
  }

  fn dirty_files(ctx: &crate::context::Context) -> Vec<String> {
    ctx
      .disk
      .package_json_files
      .iter()
      .filter(|f| f.is_dirty())
      .map(|f| f.filepath.display().to_string())
      .collect()
  }

  #[tokio::test]
  async fn select_all_marks_every_outdated_file_dirty() {
    let builder = TestBuilder::new()
      .with_packages(vec![
        json!({
          "name": "package-a",
          "version": "1.0.0",
          "dependencies": {"foo": "^1.0.0"}
        }),
        json!({
          "name": "package-b",
          "version": "1.0.0",
          "dependencies": {"foo": "^1.0.0"}
        }),
      ])
      .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}));
    let ctx = run_update(builder, MockTui::select_all()).await;
    let dirty = dirty_files(&ctx);
    assert_eq!(dirty.len(), 2);
  }

  #[tokio::test]
  async fn cancel_returns_cancelled_error() {
    let builder = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.0.0"}
      })])
      .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}));
    let result = try_run_update(builder, MockTui::cancel()).await;
    assert!(matches!(result, Err(SyncpackError::Cancelled)));
  }

  #[tokio::test]
  async fn select_specific_indices_only_writes_chosen_buckets() {
    let pkg = json!({
      "name": "package-a",
      "version": "1.0.0",
      "dependencies": {"foo": "^1.0.0", "bar": "^2.0.0"}
    });
    let updates = json!({
      "foo": ["1.0.0", "1.0.1"],
      "bar": ["2.0.0", "2.1.0"],
    });
    // Build rows once to learn their order, then pick the first.
    let (ctx, registry_updates) = TestBuilder::new()
      .with_packages(vec![pkg.clone()])
      .with_registry_updates(updates.clone())
      .run_with_updates()
      .await;
    let rows = build_update_rows(&ctx, &registry_updates.unwrap(), FROZEN_NOW);
    assert_eq!(rows.len(), 2);
    let chosen_dep = rows[0].dependency_name.clone();
    let chosen_target = rows[0].target_raw.clone();
    let unchosen_dep = rows[1].dependency_name.clone();
    let unchosen_current = rows[1].current_raw.clone();

    let builder = TestBuilder::new().with_packages(vec![pkg]).with_registry_updates(updates);
    let ctx_after = run_update(builder, MockTui::select(vec![0])).await;

    let deps = ctx_after.disk.package_json_files[0]
      .contents
      .pointer("/dependencies")
      .expect("dependencies present");
    assert_eq!(deps[&chosen_dep].as_str(), Some(chosen_target.as_str()), "chosen row written");
    assert_eq!(
      deps[&unchosen_dep].as_str(),
      Some(unchosen_current.as_str()),
      "unchosen row preserved"
    );
  }

  #[tokio::test]
  async fn not_tty_warns_and_writes_nothing() {
    let builder = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.0.0"}
      })])
      .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}));
    let ctx = run_update(builder, MockTui::not_tty()).await;
    assert!(dirty_files(&ctx).is_empty());
  }

  #[tokio::test]
  async fn too_small_terminal_warns_and_writes_nothing() {
    let builder = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.0.0"}
      })])
      .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}));
    let ctx = run_update(builder, MockTui::too_small()).await;
    assert!(dirty_files(&ctx).is_empty());
  }

  #[tokio::test]
  async fn cannot_measure_terminal_warns_and_writes_nothing() {
    let builder = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {"foo": "^1.0.0"}
      })])
      .with_registry_updates(json!({"foo": ["1.0.0", "1.0.1"]}));
    let ctx = run_update(builder, MockTui::cannot_measure()).await;
    assert!(dirty_files(&ctx).is_empty());
  }
}
