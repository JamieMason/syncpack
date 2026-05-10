use crate::tui::{Tui, TuiReadiness, UpdateRow};

/// Canned response for `MockTui::pick`.
pub enum MockPick {
  /// Select every row.
  All,
  /// Select only these row indices.
  Some(Vec<usize>),
  /// Simulate the user pressing `Esc`.
  Cancel,
}

/// Test-time `Tui` impl. Drives both `readiness` and `pick` through
/// canned values so the apply path can be exercised without a real
/// terminal.
pub struct MockTui {
  pub response: MockPick,
  /// Override for `readiness`. `None` means "default to `Ready 80x24`".
  pub readiness: Option<TuiReadiness>,
}

impl MockTui {
  pub fn select_all() -> Self {
    Self {
      response: MockPick::All,
      readiness: None,
    }
  }

  pub fn cancel() -> Self {
    Self {
      response: MockPick::Cancel,
      readiness: None,
    }
  }

  pub fn select(indices: Vec<usize>) -> Self {
    Self {
      response: MockPick::Some(indices),
      readiness: None,
    }
  }

  /// Stub `readiness` to report no TTY. `pick` should never be called.
  pub fn not_tty() -> Self {
    Self {
      response: MockPick::Cancel,
      readiness: Some(TuiReadiness::NotTty),
    }
  }

  /// Stub `readiness` to report a too-small terminal. `pick` should never
  /// be called.
  pub fn too_small() -> Self {
    Self {
      response: MockPick::Cancel,
      readiness: Some(TuiReadiness::TooSmall {
        cols: 10,
        rows: 2,
        min_cols: crate::tui::MIN_TERMINAL_WIDTH,
        min_rows: crate::tui::MIN_TERMINAL_HEIGHT,
      }),
    }
  }

  /// Stub `readiness` to report `terminal::size()` failed. `pick` should
  /// never be called.
  pub fn cannot_measure() -> Self {
    Self {
      response: MockPick::Cancel,
      readiness: Some(TuiReadiness::CannotMeasure),
    }
  }
}

impl Tui for MockTui {
  fn readiness(&self, row_count: usize) -> TuiReadiness {
    if let Some(r) = self.readiness {
      return r;
    }
    if row_count == 0 {
      return TuiReadiness::Empty;
    }
    TuiReadiness::Ready { cols: 80, rows: 24 }
  }

  fn pick(&self, rows: &[UpdateRow], _cols: u16, _rows_term: u16) -> Option<Vec<usize>> {
    match &self.response {
      MockPick::All => Some((0..rows.len()).collect()),
      MockPick::Some(indices) => Some(indices.iter().copied().filter(|i| *i < rows.len()).collect()),
      MockPick::Cancel => None,
    }
  }
}
