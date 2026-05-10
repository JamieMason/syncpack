use {
  super::{DiffKind, colorize_diff, middle_truncate, render_lines, time_difference, visible_width},
  crate::tui::UpdateRow,
  colored::Colorize,
  node_semver::Version,
};

mod time_difference {
  use super::*;

  /// `2024-01-15T00:00:00Z` in Unix seconds
  const FIXED_NOW: i64 = 1_705_276_800;

  #[test]
  fn under_one_day_returns_lte_label() {
    let four_hours_before_fixed_now = "2024-01-14T20:00:00Z";
    assert_eq!(time_difference(four_hours_before_fixed_now, FIXED_NOW), Some("⩽1d".to_string()));
  }

  #[test]
  fn exactly_one_day_returns_days() {
    let one_day_before_fixed_now = "2024-01-14T00:00:00Z";
    assert_eq!(time_difference(one_day_before_fixed_now, FIXED_NOW), Some("~1d".to_string()));
  }

  #[test]
  fn five_days_ago() {
    let five_days_before_fixed_now = "2024-01-10T00:00:00Z";
    assert_eq!(time_difference(five_days_before_fixed_now, FIXED_NOW), Some("~5d".to_string()));
  }

  #[test]
  fn thirty_one_days_ago_uses_months() {
    let thirty_one_days_before_fixed_now = "2023-12-15T00:00:00Z";
    assert_eq!(
      time_difference(thirty_one_days_before_fixed_now, FIXED_NOW),
      Some("~1mo".to_string())
    );
  }

  #[test]
  fn three_months_ago() {
    let ninety_days_before_fixed_now = "2023-10-17T00:00:00Z";
    assert_eq!(time_difference(ninety_days_before_fixed_now, FIXED_NOW), Some("~3mo".to_string()));
  }

  #[test]
  fn one_year_ago_uses_years() {
    let three_hundred_sixty_five_days_before_fixed_now = "2023-01-15T00:00:00Z";
    assert_eq!(
      time_difference(three_hundred_sixty_five_days_before_fixed_now, FIXED_NOW),
      Some("~1.0y".to_string())
    );
  }

  #[test]
  fn two_point_five_years_ago() {
    let two_and_a_half_years_before_fixed_now = "2021-07-15T00:00:00Z";
    assert_eq!(
      time_difference(two_and_a_half_years_before_fixed_now, FIXED_NOW),
      Some("~2.5y".to_string())
    );
  }

  #[test]
  fn unparseable_returns_none() {
    assert_eq!(time_difference("not-a-date", FIXED_NOW), None);
  }

  #[test]
  fn future_timestamp_returns_none() {
    let one_year_after_fixed_now = "2025-01-15T00:00:00Z";
    assert_eq!(time_difference(one_year_after_fixed_now, FIXED_NOW), None);
  }
}

mod diff_kind {
  use super::*;

  fn kind(current: &str, target: &str) -> DiffKind {
    DiffKind::from_versions(&Version::parse(current).unwrap(), &Version::parse(target).unwrap())
  }

  #[test]
  fn equal_returns_none_kind() {
    assert_eq!(kind("1.2.3", "1.2.3"), DiffKind::None);
  }

  #[test]
  fn patch_bump() {
    assert_eq!(kind("1.2.3", "1.2.4"), DiffKind::Patch);
  }

  #[test]
  fn minor_bump() {
    assert_eq!(kind("1.2.3", "1.3.0"), DiffKind::Minor);
  }

  #[test]
  fn major_bump() {
    assert_eq!(kind("1.2.3", "2.0.0"), DiffKind::Major);
  }

  #[test]
  fn zero_x_patch_bump_is_patch() {
    assert_eq!(kind("0.38.4", "0.38.5"), DiffKind::Patch);
  }

  #[test]
  fn zero_x_minor_bump_is_major() {
    // taze semantics: target outside both ~ and ^ of a 0.x current.
    assert_eq!(kind("0.38.4", "0.39.0"), DiffKind::Major);
  }
}

mod colorize_diff {
  use super::*;

  fn force_colour() {
    colored::control::set_override(true);
  }

  #[test]
  fn unparseable_returns_dimmed_target() {
    force_colour();
    assert_eq!(
      colorize_diff("not-semver", "also-not-semver"),
      "also-not-semver".dimmed().to_string()
    );
  }

  #[test]
  fn patch_bump_colours_changed_suffix_green() {
    force_colour();
    let out = colorize_diff("1.2.3", "1.2.4");
    let expected = format!("{}.{}", "1.2".white(), "4".green());
    assert_eq!(out, expected);
  }

  #[test]
  fn minor_bump_colours_changed_suffix_green() {
    force_colour();
    let out = colorize_diff("1.2.3", "1.3.0");
    let expected = format!("{}.{}", "1".white(), "3.0".green());
    assert_eq!(out, expected);
  }

  #[test]
  fn major_bump_colours_changed_suffix_green() {
    force_colour();
    assert_eq!(colorize_diff("1.2.3", "2.0.0"), "2.0.0".green().to_string());
  }

  #[test]
  fn caret_lead_unchanged_is_white() {
    force_colour();
    let out = colorize_diff("^0.38.4", "^0.38.5");
    let expected = format!("{}{}.{}", "^".white(), "0.38".white(), "5".green());
    assert_eq!(out, expected);
  }

  #[test]
  fn caret_lead_changed_is_yellow() {
    force_colour();
    let out = colorize_diff("^1.2.3", "~1.2.4");
    let expected = format!("{}{}.{}", "~".yellow(), "1.2".white(), "4".green());
    assert_eq!(out, expected);
  }

  #[test]
  fn zero_x_minor_target_coloured_green() {
    force_colour();
    let out = colorize_diff("^0.38.4", "^0.39.0");
    let expected = format!("{}{}.{}", "^".white(), "0".white(), "39.0".green());
    assert_eq!(out, expected);
  }
}

mod visible_width_helper {
  use super::*;

  #[test]
  fn ascii_width_matches_byte_count() {
    assert_eq!(visible_width("hello"), 5);
  }

  #[test]
  fn strips_ansi_csi_sgr_sequences() {
    let red = "x".red().to_string();
    assert_eq!(visible_width(&red), 1);
  }

  #[test]
  fn skips_multiple_ansi_runs() {
    let s = format!("{} {} {}", "a".red(), "b".green(), "c".blue());
    assert_eq!(visible_width(&s), 5);
  }

  #[test]
  fn unicode_arrow_counts_one_column() {
    assert_eq!(visible_width("→"), 1);
  }

  #[test]
  fn ellipsis_counts_one_column() {
    assert_eq!(visible_width("…"), 1);
  }
}

mod middle_truncate_helper {
  use super::*;

  #[test]
  fn returns_input_when_already_under_budget() {
    assert_eq!(middle_truncate("hello", 10), "hello");
    assert_eq!(middle_truncate("hello", 5), "hello");
  }

  #[test]
  fn empty_string_for_zero_budget() {
    assert_eq!(middle_truncate("hello", 0), "");
  }

  #[test]
  fn ellipsis_only_for_budget_one() {
    assert_eq!(middle_truncate("hello", 1), "…");
  }

  #[test]
  fn balanced_middle_truncation_for_long_string() {
    // budget=7 → head=3, ellipsis=1, tail=3.
    assert_eq!(middle_truncate("abcdefghij", 7), "abc…hij");
  }

  #[test]
  fn budget_eight_keeps_three_from_each_end() {
    // budget=8 → head=⌊7/2⌋=3, tail=4. head=abc, tail=ghij.
    assert_eq!(middle_truncate("abcdefghij", 8), "abc…ghij");
  }

  #[test]
  fn truncation_fits_in_budget() {
    let result = middle_truncate("@scoped/very-long-package-name", 14);
    assert!(visible_width(&result) <= 14, "result {result:?} exceeds budget");
    assert!(result.contains('…'));
  }
}

mod render_lines_indexing {
  use super::*;

  fn row(group_idx: usize, name: &str, bucket_count: usize, current: &str, target: &str) -> UpdateRow {
    UpdateRow {
      group_idx,
      group_label: format!("Group {group_idx}"),
      dependency_name: name.into(),
      dependency_outdated_count: bucket_count,
      bucket_count,
      current_raw: current.into(),
      current_time_label: None,
      target_raw: target.into(),
      target_time_label: None,
      instance_indices: vec![],
    }
  }

  #[test]
  fn solo_dep_emits_group_header_and_one_selectable_line() {
    let rows = vec![row(0, "astro", 1, "^1.0.0", "^1.0.1")];
    let lines = render_lines(&rows, Some(&[true]), Some(0), None);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].row_idx, None, "group header is not selectable");
    assert_eq!(lines[1].row_idx, Some(0), "solo line maps to row 0");
  }

  #[test]
  fn multi_bucket_dep_emits_dep_header_then_bucket_lines() {
    let rows = vec![row(0, "react", 1, "^17.0.0", "^17.0.2"), row(0, "react", 1, "^18.0.0", "^18.0.2")];
    let lines = render_lines(&rows, Some(&[true, true]), Some(1), None);
    // group header, dep header, bucket 0, bucket 1
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0].row_idx, None);
    assert_eq!(lines[1].row_idx, None, "dep header is not selectable");
    assert_eq!(lines[2].row_idx, Some(0));
    assert_eq!(lines[3].row_idx, Some(1));
  }

  #[test]
  fn group_change_re_emits_header() {
    let rows = vec![row(0, "astro", 1, "^1.0.0", "^1.0.1"), row(1, "vite", 1, "^5.0.0", "^5.0.1")];
    let lines = render_lines(&rows, Some(&[true, true]), Some(0), None);
    // group0 header, astro line, group1 header, vite line
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0].row_idx, None);
    assert_eq!(lines[1].row_idx, Some(0));
    assert_eq!(lines[2].row_idx, None);
    assert_eq!(lines[3].row_idx, Some(1));
  }

  #[test]
  fn narrow_terminal_truncates_dep_name() {
    let rows = vec![row(0, "@scope/very-long-package-name", 1, "^1.0.0", "^1.0.1")];
    let lines = render_lines(&rows, Some(&[true]), Some(0), Some(30));
    let solo = &lines[1].text;
    assert!(visible_width(solo) <= 30, "line {solo:?} overflows 30 cols");
    assert!(solo.contains('…'), "expected truncation marker in {solo:?}");
  }

  #[test]
  fn wide_terminal_does_not_truncate_short_names() {
    let rows = vec![row(0, "astro", 1, "^1.0.0", "^1.0.1")];
    let lines = render_lines(&rows, Some(&[true]), Some(0), Some(120));
    let solo = &lines[1].text;
    assert!(solo.contains("astro"));
    assert!(!solo.contains('…'));
  }
}
