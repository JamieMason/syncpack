use {
  crate::commands::ui::update_row,
  colored::Colorize,
  crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
  },
  std::io::{IsTerminal, Write, stdout},
};

#[cfg(test)]
#[path = "tui_test.rs"]
mod tui_test;

/// Result of probing the terminal environment before a `pick` call.
/// Lets callers fan out to a uniform set of warnings/dispatches without
/// `pick` having to mix env-readiness with picker logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiReadiness {
  /// Picker is good to go.
  Ready { cols: u16, rows: u16 },
  /// No rows to pick from — caller should skip the picker.
  Empty,
  /// stdout is not connected to a terminal.
  NotTty,
  /// Terminal is too small for the picker layout.
  TooSmall {
    cols: u16,
    rows: u16,
    min_cols: u16,
    min_rows: u16,
  },
  /// `terminal::size()` failed; we have no dimensions to work with.
  CannotMeasure,
}

/// Interactive picker abstraction for `syncpack update --interactive`.
///
/// Mirrors the trait pattern used for `RegistryClient` and `DiskIo`:
/// a real `LiveTui` impl drives a crossterm-based terminal UI, while
/// tests inject a `MockTui` to exercise the apply path with a canned
/// selection.
pub trait Tui {
  /// Probe the terminal environment. The caller hands the row count so
  /// the trait can branch on `Empty` without a second method.
  fn readiness(&self, row_count: usize) -> TuiReadiness;
  /// Present the user with a checklist of update rows. Returns the
  /// indices of selected rows, in their original order. Returns `None`
  /// when the user cancels (e.g. `Esc`). Caller must ensure readiness
  /// is `Ready` before invoking — the implementation does not re-check.
  fn pick(&self, rows: &[UpdateRow], cols: u16, rows_term: u16) -> Option<Vec<usize>>;
}

/// A single row of the update table, one per (group, dependency,
/// current raw specifier) bucket.
#[derive(Debug, Clone)]
pub struct UpdateRow {
  /// Index into `ctx.version_groups`. Used to print the group header.
  pub group_idx: usize,
  /// Pre-rendered group header label (e.g. "Default Version Group").
  pub group_label: String,
  /// Internal name of the dependency (the registry key), e.g. `astro`.
  pub dependency_name: String,
  /// Total number of outdated instances of this dependency in this
  /// group. Drives the `Nx dep_name` header line.
  pub dependency_outdated_count: usize,
  /// Number of instances in this bucket — i.e. instances sharing the
  /// same `current_raw` specifier. Drives the `Nx` on the row.
  pub bucket_count: usize,
  /// Raw specifier as written in package.json today, e.g. `^6.2.1`.
  pub current_raw: String,
  /// Pre-formatted "how stale" label for the current version, e.g.
  /// `~15d`. `None` when the registry response had no time data.
  pub current_time_label: Option<String>,
  /// Raw specifier we'd update to, e.g. `^6.3.1`.
  pub target_raw: String,
  /// Pre-formatted "how stale" label for the target version.
  pub target_time_label: Option<String>,
  /// Backing `InstanceIdx`s the apply step needs to write back to disk.
  pub instance_indices: Vec<crate::instance::InstanceIdx>,
}

/// Minimum terminal size for the interactive picker. Smaller than this
/// and the layout (2 hint rows + 1 status row + at least one content row)
/// can't fit; readiness reports `TooSmall` so the caller can warn.
pub const MIN_TERMINAL_HEIGHT: u16 = 4;
pub const MIN_TERMINAL_WIDTH: u16 = 30;

/// Crossterm-driven implementation. Hand-rolled to match the spec's
/// row layout: group/dep headers, indented rows with `◌`/`◉` mid-row,
/// `▶` on the focused row.
pub struct LiveTui;

impl LiveTui {
  pub fn new() -> Self {
    Self
  }
}

impl Default for LiveTui {
  fn default() -> Self {
    Self::new()
  }
}

impl Tui for LiveTui {
  fn readiness(&self, row_count: usize) -> TuiReadiness {
    if row_count == 0 {
      return TuiReadiness::Empty;
    }
    if !stdout().is_terminal() {
      return TuiReadiness::NotTty;
    }
    let Some((cols, rows)) = terminal::size().ok() else {
      return TuiReadiness::CannotMeasure;
    };
    if rows < MIN_TERMINAL_HEIGHT || cols < MIN_TERMINAL_WIDTH {
      return TuiReadiness::TooSmall {
        cols,
        rows,
        min_cols: MIN_TERMINAL_WIDTH,
        min_rows: MIN_TERMINAL_HEIGHT,
      };
    }
    TuiReadiness::Ready { cols, rows }
  }

  fn pick(&self, rows: &[UpdateRow], cols: u16, rows_term: u16) -> Option<Vec<usize>> {
    run_picker(rows, cols, rows_term).unwrap_or_else(|err| {
      eprintln!("interactive picker failed: {err}");
      None
    })
  }
}

/// In-flight state for one picker session. `cursor_idx` walks selectable
/// rows (row-space); `viewport_top` tracks the top *line* shown
/// (line-space, including header lines). `selection` is keyed by absolute
/// row index so scrolling can never lose toggles.
struct PickerState<'a> {
  rows: &'a [UpdateRow],
  selection: Vec<bool>,
  cursor_idx: usize,
  viewport_top: usize,
  terminal_height: u16,
  terminal_width: u16,
}

impl<'a> PickerState<'a> {
  fn new(rows: &'a [UpdateRow], terminal_width: u16, terminal_height: u16) -> Self {
    Self {
      rows,
      selection: vec![true; rows.len()],
      cursor_idx: 0,
      viewport_top: 0,
      terminal_height,
      terminal_width,
    }
  }

  /// Hint rows (2) + status row (1) are always pinned; the rest is
  /// scrollable content. Saturating so a 3-row terminal returns 0
  /// (we'd have already bailed in `readiness`, but stay defensive).
  fn visible_content_rows(&self) -> usize {
    self.terminal_height.saturating_sub(3) as usize
  }
}

fn run_picker(rows: &[UpdateRow], terminal_width: u16, terminal_height: u16) -> std::io::Result<Option<Vec<usize>>> {
  let mut state = PickerState::new(rows, terminal_width, terminal_height);
  let _guard = TermGuard::enter()?;

  loop {
    redraw(&mut state)?;
    match event::read()? {
      Event::Key(KeyEvent { code, modifiers, kind, .. }) => {
        if kind != KeyEventKind::Press {
          continue;
        }
        match handle_key(&mut state, code, modifiers) {
          KeyOutcome::Continue => {}
          KeyOutcome::Cancel => return Ok(None),
          KeyOutcome::Confirm => {
            let picks: Vec<usize> = state.selection.iter().enumerate().filter(|(_, s)| **s).map(|(i, _)| i).collect();
            return Ok(Some(picks));
          }
        }
      }
      Event::Resize(w, h) => {
        state.terminal_width = w;
        state.terminal_height = h;
      }
      _ => {}
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
enum KeyOutcome {
  Continue,
  Cancel,
  Confirm,
}

fn handle_key(state: &mut PickerState, code: KeyCode, modifiers: KeyModifiers) -> KeyOutcome {
  let row_count = state.rows.len();
  match code {
    KeyCode::Up | KeyCode::Char('k') => {
      state.cursor_idx = (state.cursor_idx + row_count - 1) % row_count;
    }
    KeyCode::Down | KeyCode::Char('j') => {
      state.cursor_idx = (state.cursor_idx + 1) % row_count;
    }
    KeyCode::PageUp => {
      let step = state.visible_content_rows().max(1);
      state.cursor_idx = state.cursor_idx.saturating_sub(step);
    }
    KeyCode::PageDown => {
      let step = state.visible_content_rows().max(1);
      state.cursor_idx = (state.cursor_idx + step).min(row_count - 1);
    }
    KeyCode::Home => {
      state.cursor_idx = 0;
    }
    KeyCode::End => {
      state.cursor_idx = row_count - 1;
    }
    KeyCode::Char(' ') => {
      state.selection[state.cursor_idx] = !state.selection[state.cursor_idx];
    }
    KeyCode::Char('a') if !modifiers.contains(KeyModifiers::CONTROL) => {
      let any_unselected = state.selection.iter().any(|s| !s);
      state.selection.fill(any_unselected);
    }
    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
      return KeyOutcome::Cancel;
    }
    KeyCode::Esc | KeyCode::Char('q') => {
      return KeyOutcome::Cancel;
    }
    KeyCode::Enter => {
      return KeyOutcome::Confirm;
    }
    _ => {}
  }
  KeyOutcome::Continue
}

/// Re-clamp `viewport_top` so that `cursor_line` is on-screen. Hard edge:
/// no scroll-off margin; the cursor sits exactly at the boundary when
/// scrolling.
fn clamp_viewport(cursor_line: usize, viewport_top: usize, visible: usize) -> usize {
  if visible == 0 {
    return 0;
  }
  if cursor_line < viewport_top {
    cursor_line
  } else if cursor_line >= viewport_top + visible {
    cursor_line + 1 - visible
  } else {
    viewport_top
  }
}

/// Re-clamp on resize: viewport may now overshoot the end of the list.
fn clamp_after_resize(viewport_top: usize, total_lines: usize, visible: usize) -> usize {
  let max_top = total_lines.saturating_sub(visible);
  viewport_top.min(max_top)
}

fn redraw(state: &mut PickerState) -> std::io::Result<()> {
  let mut out = stdout();
  let visible = state.visible_content_rows();
  let lines = update_row::render_lines(
    state.rows,
    Some(&state.selection),
    Some(state.cursor_idx),
    Some(state.terminal_width as usize),
  );

  // row_to_line[row_idx] = the line index of the selectable line for that row.
  let mut row_to_line = vec![0usize; state.rows.len()];
  for (line_idx, line) in lines.iter().enumerate() {
    if let Some(row_idx) = line.row_idx {
      row_to_line[row_idx] = line_idx;
    }
  }
  let cursor_line = row_to_line[state.cursor_idx];
  state.viewport_top = clamp_after_resize(state.viewport_top, lines.len(), visible);
  state.viewport_top = clamp_viewport(cursor_line, state.viewport_top, visible);

  execute!(out, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All))?;

  let hints = [
    format!(
      "{} {} {} {}",
      "↑↓".bold().green(),
      "select,".dimmed(),
      "space".bold().green(),
      "toggle,".dimmed(),
    ),
    format!(
      "{} {} {} {} {} {}",
      "a".bold().green(),
      "all,".dimmed(),
      "enter".bold().green(),
      "confirm,".dimmed(),
      "esc or q".bold().green(),
      "cancel".dimmed(),
    ),
  ];
  for hint in &hints {
    writeln!(out, "{hint}\r")?;
  }

  for r in 0..visible {
    let line_idx = state.viewport_top + r;
    if line_idx < lines.len() {
      writeln!(out, "{}\r", lines[line_idx].text)?;
    } else {
      writeln!(out, "\r")?;
    }
  }

  let selected_count = state.selection.iter().filter(|s| **s).count();
  let status = format!("[{}/{}] • {} selected", state.cursor_idx + 1, state.rows.len(), selected_count,)
    .dimmed()
    .to_string();
  write!(out, "{status}\r")?;
  out.flush()?;
  Ok(())
}

/// RAII guard that toggles raw mode + alt-screen + cursor visibility.
/// Restores terminal state on Drop, including panic unwinds.
struct TermGuard;

impl TermGuard {
  fn enter() -> std::io::Result<Self> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
    Ok(Self)
  }
}

impl Drop for TermGuard {
  fn drop(&mut self) {
    let _ = execute!(stdout(), cursor::Show, LeaveAlternateScreen);
    let _ = terminal::disable_raw_mode();
  }
}
