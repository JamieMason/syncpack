use super::*;

mod clamp_viewport {
  use super::*;

  #[test]
  fn cursor_inside_viewport_keeps_top_unchanged() {
    assert_eq!(clamp_viewport(5, 0, 10), 0);
    assert_eq!(clamp_viewport(0, 0, 10), 0);
    assert_eq!(clamp_viewport(9, 0, 10), 0);
  }

  #[test]
  fn cursor_above_viewport_pulls_top_to_cursor() {
    assert_eq!(clamp_viewport(2, 5, 10), 2);
    assert_eq!(clamp_viewport(0, 100, 10), 0);
  }

  #[test]
  fn cursor_below_viewport_pushes_top_to_cursor_minus_visible_plus_one() {
    // visible=10, cursor=10, top=0 → cursor at row 10 is just past last visible (rows 0..=9).
    assert_eq!(clamp_viewport(10, 0, 10), 1);
    // cursor=15, visible=10 → top must be 6 so rows 6..=15 are shown.
    assert_eq!(clamp_viewport(15, 0, 10), 6);
  }

  #[test]
  fn boundary_cursor_at_top_edge_is_visible() {
    // cursor exactly at viewport_top stays in view (no scroll).
    assert_eq!(clamp_viewport(5, 5, 10), 5);
  }

  #[test]
  fn boundary_cursor_at_bottom_edge_is_visible() {
    // cursor at viewport_top + visible - 1 is the last visible line.
    assert_eq!(clamp_viewport(14, 5, 10), 5);
  }

  #[test]
  fn zero_visible_returns_zero() {
    assert_eq!(clamp_viewport(5, 3, 0), 0);
  }
}

mod clamp_after_resize {
  use super::*;

  #[test]
  fn shrinks_top_when_viewport_overshoots_end() {
    // total=20 lines, visible=10 → max_top=10. top=15 must clamp to 10.
    assert_eq!(clamp_after_resize(15, 20, 10), 10);
  }

  #[test]
  fn keeps_top_when_inside_bounds() {
    assert_eq!(clamp_after_resize(3, 20, 10), 3);
  }

  #[test]
  fn small_list_clamps_top_to_zero() {
    // total=5, visible=10 → max_top = 0.
    assert_eq!(clamp_after_resize(7, 5, 10), 0);
  }
}

mod picker_state {
  use super::*;

  #[test]
  fn visible_content_rows_subtracts_three() {
    let rows: Vec<UpdateRow> = vec![];
    let s = PickerState::new(&rows, 80, 24);
    assert_eq!(s.visible_content_rows(), 21);
  }

  #[test]
  fn visible_content_rows_saturates_at_zero() {
    let rows: Vec<UpdateRow> = vec![];
    let s = PickerState::new(&rows, 80, 2);
    assert_eq!(s.visible_content_rows(), 0);
  }
}

mod handle_key {
  use super::*;

  fn fake_row(name: &str) -> UpdateRow {
    UpdateRow {
      group_idx: 0,
      group_label: "Default".to_string(),
      dependency_name: name.to_string(),
      dependency_outdated_count: 1,
      bucket_count: 1,
      current_raw: "^1.0.0".to_string(),
      current_time_label: None,
      target_raw: "^1.0.1".to_string(),
      target_time_label: None,
      instance_indices: vec![],
    }
  }

  fn rows(n: usize) -> Vec<UpdateRow> {
    (0..n).map(|i| fake_row(&format!("dep-{i}"))).collect()
  }

  fn press<'a>(state: &mut PickerState<'a>, code: KeyCode) -> KeyOutcome {
    handle_key(state, code, KeyModifiers::NONE)
  }

  #[test]
  fn down_advances_cursor() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    assert_eq!(press(&mut s, KeyCode::Down), KeyOutcome::Continue);
    assert_eq!(s.cursor_idx, 1);
  }

  #[test]
  fn j_advances_like_down() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    press(&mut s, KeyCode::Char('j'));
    assert_eq!(s.cursor_idx, 1);
  }

  #[test]
  fn down_at_last_wraps_to_zero() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    s.cursor_idx = 2;
    press(&mut s, KeyCode::Down);
    assert_eq!(s.cursor_idx, 0);
  }

  #[test]
  fn up_at_zero_wraps_to_last() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    press(&mut s, KeyCode::Up);
    assert_eq!(s.cursor_idx, 2);
  }

  #[test]
  fn k_retreats_like_up() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    s.cursor_idx = 2;
    press(&mut s, KeyCode::Char('k'));
    assert_eq!(s.cursor_idx, 1);
  }

  #[test]
  fn pgdn_jumps_a_full_page() {
    let rows = rows(50);
    let mut s = PickerState::new(&rows, 80, 24); // visible = 21
    press(&mut s, KeyCode::PageDown);
    assert_eq!(s.cursor_idx, 21);
  }

  #[test]
  fn pgdn_clamps_to_last_row() {
    let rows = rows(10);
    let mut s = PickerState::new(&rows, 80, 24);
    press(&mut s, KeyCode::PageDown);
    assert_eq!(s.cursor_idx, 9);
  }

  #[test]
  fn pgup_jumps_a_full_page_back() {
    let rows = rows(50);
    let mut s = PickerState::new(&rows, 80, 24);
    s.cursor_idx = 30;
    press(&mut s, KeyCode::PageUp);
    assert_eq!(s.cursor_idx, 9);
  }

  #[test]
  fn pgup_saturates_at_zero() {
    let rows = rows(50);
    let mut s = PickerState::new(&rows, 80, 24);
    s.cursor_idx = 5;
    press(&mut s, KeyCode::PageUp);
    assert_eq!(s.cursor_idx, 0);
  }

  #[test]
  fn home_jumps_to_first() {
    let rows = rows(10);
    let mut s = PickerState::new(&rows, 80, 24);
    s.cursor_idx = 7;
    press(&mut s, KeyCode::Home);
    assert_eq!(s.cursor_idx, 0);
  }

  #[test]
  fn end_jumps_to_last() {
    let rows = rows(10);
    let mut s = PickerState::new(&rows, 80, 24);
    press(&mut s, KeyCode::End);
    assert_eq!(s.cursor_idx, 9);
  }

  #[test]
  fn space_toggles_only_focused_row() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    s.cursor_idx = 1;
    press(&mut s, KeyCode::Char(' '));
    assert_eq!(s.selection, vec![true, false, true]);
    press(&mut s, KeyCode::Char(' '));
    assert_eq!(s.selection, vec![true, true, true]);
  }

  #[test]
  fn a_unselects_all_when_some_selected() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    s.selection = vec![true, false, true];
    press(&mut s, KeyCode::Char('a'));
    assert_eq!(s.selection, vec![true, true, true]);
  }

  #[test]
  fn a_unselects_all_when_all_selected() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    press(&mut s, KeyCode::Char('a'));
    assert_eq!(s.selection, vec![false, false, false]);
  }

  #[test]
  fn ctrl_c_cancels() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    let outcome = handle_key(&mut s, KeyCode::Char('c'), KeyModifiers::CONTROL);
    assert_eq!(outcome, KeyOutcome::Cancel);
  }

  #[test]
  fn esc_cancels() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    assert_eq!(press(&mut s, KeyCode::Esc), KeyOutcome::Cancel);
  }

  #[test]
  fn q_cancels() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    assert_eq!(press(&mut s, KeyCode::Char('q')), KeyOutcome::Cancel);
  }

  #[test]
  fn enter_confirms() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    assert_eq!(press(&mut s, KeyCode::Enter), KeyOutcome::Confirm);
  }

  #[test]
  fn unknown_key_is_continue_and_no_state_change() {
    let rows = rows(3);
    let mut s = PickerState::new(&rows, 80, 24);
    let outcome = press(&mut s, KeyCode::F(1));
    assert_eq!(outcome, KeyOutcome::Continue);
    assert_eq!(s.cursor_idx, 0);
    assert_eq!(s.selection, vec![true, true, true]);
  }
}
