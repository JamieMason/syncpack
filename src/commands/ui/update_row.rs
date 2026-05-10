use {
  crate::{commands::ui::LINE_ENDING, registry::updates::parse_rfc3339_to_unix_seconds, tui::UpdateRow},
  colored::Colorize,
  log::info,
  node_semver::{Range, Version},
  std::time::{SystemTime, UNIX_EPOCH},
  unicode_width::{UnicodeWidthChar, UnicodeWidthStr},
};

#[cfg(test)]
#[path = "update_row_test.rs"]
mod update_row_test;

const SECONDS_PER_DAY: i64 = 86_400;
const SECONDS_PER_MONTH: i64 = SECONDS_PER_DAY * 30;
const SECONDS_PER_YEAR: i64 = SECONDS_PER_DAY * 365;

/// One rendered terminal line. Picker uses `row_idx` to map between line-space
/// (viewport math) and row-space (selectable rows). `None` for headers.
#[derive(Debug, Clone)]
pub struct RenderedLine {
  pub text: String,
  pub row_idx: Option<usize>,
}

/// Visual width of `s` after stripping CSI SGR escape sequences (the only
/// escapes the `colored` crate emits). Used to budget terminal columns.
pub fn visible_width(s: &str) -> usize {
  strip_ansi(s).width()
}

fn strip_ansi(s: &str) -> String {
  let mut out = String::with_capacity(s.len());
  let mut chars = s.chars().peekable();
  while let Some(c) = chars.next() {
    if c == '\x1b' && chars.peek() == Some(&'[') {
      chars.next();
      for c2 in chars.by_ref() {
        if matches!(c2, '@'..='~') {
          break;
        }
      }
      continue;
    }
    out.push(c);
  }
  out
}

/// Balanced middle-truncate to fit `budget` visual columns. Returns the input
/// unchanged when it already fits. Replaces the middle with `…`.
pub fn middle_truncate(s: &str, budget: usize) -> String {
  let width: usize = s.chars().map(|c| c.width().unwrap_or(0)).sum();
  if width <= budget {
    return s.to_string();
  }
  if budget == 0 {
    return String::new();
  }
  if budget == 1 {
    return "…".to_string();
  }
  let head_budget = (budget - 1) / 2;
  let tail_budget = budget - 1 - head_budget;
  let mut head = String::new();
  let mut head_w = 0;
  for c in s.chars() {
    let w = c.width().unwrap_or(0);
    if head_w + w > head_budget {
      break;
    }
    head.push(c);
    head_w += w;
  }
  let mut tail_chars: Vec<char> = Vec::new();
  let mut tail_w = 0;
  for c in s.chars().rev() {
    let w = c.width().unwrap_or(0);
    if tail_w + w > tail_budget {
      break;
    }
    tail_chars.push(c);
    tail_w += w;
  }
  let tail: String = tail_chars.into_iter().rev().collect();
  format!("{head}…{tail}")
}

/// Wall-clock UNIX seconds. Suspect callers should freeze this value
/// at the top of a render pass so every row is computed against the
/// same instant.
pub fn unix_now() -> i64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map(|d| d.as_secs() as i64)
    .unwrap_or(0)
}

/// Format the difference between an ISO 8601 published time and `now`
/// into a short human label.
///
/// Output mirrors taze: `⩽1d`, `~Nd` (under 30 days), `~Nmo` (under
/// a year), `~N.Ny` (older). Returns `None` when the timestamp can't
/// be parsed or is in the future relative to `now`.
pub fn time_difference(iso: &str, now_unix_seconds: i64) -> Option<String> {
  let published_at = parse_rfc3339_to_unix_seconds(iso)?;
  let elapsed = now_unix_seconds.checked_sub(published_at)?;
  if elapsed < 0 {
    return None;
  }
  if elapsed < SECONDS_PER_DAY {
    return Some("⩽1d".to_string());
  }
  if elapsed < SECONDS_PER_MONTH {
    let days = (elapsed as f64 / SECONDS_PER_DAY as f64).round() as i64;
    return Some(format!("~{days}d"));
  }
  if elapsed < SECONDS_PER_YEAR {
    let months = (elapsed as f64 / SECONDS_PER_MONTH as f64).round() as i64;
    return Some(format!("~{months}mo"));
  }
  let years = (elapsed as f64 / SECONDS_PER_YEAR as f64 * 10.0).round() / 10.0;
  Some(format!("~{years:.1}y"))
}

/// Categorisation of the diff between a current and target version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffKind {
  Major,
  Minor,
  Patch,
  None,
}

impl DiffKind {
  fn from_versions(current: &Version, target: &Version) -> Self {
    if current == target {
      return Self::None;
    }
    let in_tilde = Range::parse(format!("~{current}")).ok().is_some_and(|r| target.satisfies(&r));
    let in_caret = Range::parse(format!("^{current}")).ok().is_some_and(|r| target.satisfies(&r));
    match (in_tilde, in_caret) {
      (true, true) => Self::Patch,
      (false, true) => Self::Minor,
      _ => Self::Major,
    }
  }
}

/// Render `target_raw` with ANSI styling: leading wildcard and the
/// unchanged dotted prefix dimmed, the changed suffix in green. The
/// scope of the green region grows with the diff size — a patch bump
/// greens just the patch segment, a minor bump greens minor + patch,
/// a major bump greens everything from major onwards. Falls back to a
/// fully dimmed string when either side cannot be parsed.
pub fn colorize_diff(current_raw: &str, target_raw: &str) -> String {
  let (current_lead, current_body) = split_leading_range(current_raw);
  let (target_lead, target_body) = split_leading_range(target_raw);

  let parsed = Version::parse(current_body).ok().zip(Version::parse(target_body).ok());

  let Some((current_v, target_v)) = parsed else {
    return target_raw.dimmed().to_string();
  };

  let kind = DiffKind::from_versions(&current_v, &target_v);
  let target_segments: Vec<&str> = target_body.split('.').collect();
  let current_segments: Vec<&str> = current_body.split('.').collect();
  let first_diff = target_segments
    .iter()
    .zip(current_segments.iter())
    .position(|(t, c)| t != c)
    .unwrap_or(target_segments.len());

  let unchanged = target_segments[..first_diff].join(".");
  let changed = target_segments[first_diff..].join(".");
  let separator = if !unchanged.is_empty() && !changed.is_empty() { "." } else { "" };

  let lead_styled = if target_lead.is_empty() {
    String::new()
  } else if target_lead == current_lead {
    target_lead.white().to_string()
  } else {
    target_lead.yellow().to_string()
  };
  let unchanged_styled = if unchanged.is_empty() {
    String::new()
  } else {
    unchanged.white().to_string()
  };
  let changed_styled = if changed.is_empty() {
    String::new()
  } else if matches!(kind, DiffKind::None) {
    changed.dimmed().to_string()
  } else {
    changed.green().to_string()
  };

  format!("{lead_styled}{unchanged_styled}{separator}{changed_styled}")
}

/// Split a specifier like `^1.2.3` into (`^`, `1.2.3`). Strips a single
/// leading semver-range character; everything else is treated as body.
fn split_leading_range(raw: &str) -> (&str, &str) {
  let bytes = raw.as_bytes();
  match bytes.first() {
    Some(b'^') | Some(b'~') => raw.split_at(1),
    _ => ("", raw),
  }
}

/// Aggregated diff counts across a slice of update rows, weighted by
/// the number of instances inside each bucket.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DiffCounts {
  pub major: usize,
  pub minor: usize,
  pub patch: usize,
}

impl DiffCounts {
  pub fn total(&self) -> usize {
    self.major + self.minor + self.patch
  }
}

/// Sum diff kinds across `rows`. A bucket of `Nx` instances contributes
/// `N` to its kind.
pub fn count_diffs(rows: &[UpdateRow]) -> DiffCounts {
  let mut counts = DiffCounts::default();
  for row in rows {
    let kind = diff_kind_of_row(row);
    let weight = row.bucket_count;
    match kind {
      DiffKind::Major => counts.major += weight,
      DiffKind::Minor => counts.minor += weight,
      DiffKind::Patch => counts.patch += weight,
      DiffKind::None => {}
    }
  }
  counts
}

fn diff_kind_of_row(row: &UpdateRow) -> DiffKind {
  let (_, current_body) = split_leading_range(&row.current_raw);
  let (_, target_body) = split_leading_range(&row.target_raw);
  match (Version::parse(current_body).ok(), Version::parse(target_body).ok()) {
    (Some(c), Some(t)) => DiffKind::from_versions(&c, &t),
    _ => DiffKind::None,
  }
}

/// Render every row in `rows`, printing group + dependency headers as
/// boundaries change. Pass `Some(selection)` (one bool per row) to draw
/// `◌`/`◉` glyphs; pass `None` to suppress them (non-interactive).
pub fn render_rows(rows: &[UpdateRow], selection: Option<&[bool]>) {
  for line in format_rows(rows, selection) {
    info!("{line}");
  }
}

/// Produce the lines `render_rows` would emit, without printing them.
/// Used by `LiveTui` to drive the same visual format through crossterm
/// (raw-mode printing won't go through the log macros).
pub fn format_rows(rows: &[UpdateRow], selection: Option<&[bool]>) -> Vec<String> {
  render_lines(rows, selection, None, None).into_iter().map(|l| l.text).collect()
}

/// Walk `rows` once, emitting one `RenderedLine` per terminal row (group
/// header, dep header, solo line, or bucket line). When `max_width` is set,
/// dep names are middle-truncated so each line fits one terminal row; group
/// dividers shorten their `=` padding. The returned `row_idx` field maps
/// selectable lines back to their index in `rows`.
pub fn render_lines(rows: &[UpdateRow], selection: Option<&[bool]>, cursor: Option<usize>, max_width: Option<usize>) -> Vec<RenderedLine> {
  let mut lines = Vec::new();
  let mut last_group_idx: Option<usize> = None;
  let mut i = 0;
  while i < rows.len() {
    let row = &rows[i];
    if last_group_idx != Some(row.group_idx) {
      lines.push(RenderedLine {
        text: format_group_header(&row.group_label, max_width),
        row_idx: None,
      });
      last_group_idx = Some(row.group_idx);
    }
    let mut j = i;
    while j < rows.len() && rows[j].group_idx == row.group_idx && rows[j].dependency_name == row.dependency_name {
      j += 1;
    }
    let bucket_count = j - i;
    let interactive = selection.is_some();
    if bucket_count == 1 {
      let selected = selection.map(|sel| sel[i]);
      lines.push(RenderedLine {
        text: format_solo_line(row, selected, cursor == Some(i), max_width),
        row_idx: Some(i),
      });
    } else {
      lines.push(RenderedLine {
        text: format_dep_header(row, interactive, max_width),
        row_idx: None,
      });
      for k in i..j {
        let selected = selection.map(|sel| sel[k]);
        lines.push(RenderedLine {
          text: format_bucket_line(&rows[k], selected, cursor == Some(k)),
          row_idx: Some(k),
        });
      }
    }
    i = j;
  }
  lines
}

fn format_group_header(label: &str, max_width: Option<usize>) -> String {
  let cap = max_width.map(|w| w.min(80)).unwrap_or(80);
  let label_w: usize = label.chars().map(|c| c.width().unwrap_or(0)).sum();
  // header = `= {label} ` so 3 framing columns around the label.
  let frame = 3;
  let header_w = label_w + frame;
  if header_w >= cap {
    let label_budget = cap.saturating_sub(frame);
    let truncated = middle_truncate(label, label_budget);
    return format!("= {truncated} ").blue().to_string();
  }
  let divider = "=".repeat(cap - header_w);
  format!("= {label} {divider}").blue().to_string()
}

/// Right-aligned `Nx` in a 5-character field (`   1x`, ` 164x`).
/// Matches the existing `count_column` rule but is local so this module
/// stays self-contained.
fn count_cell(count: usize) -> String {
  format!("{count:>4}x").dimmed().to_string()
}

/// Common pieces every row shares: current spec (blue), times (grey),
/// arrow (grey), target spec (diff-coloured).
fn render_specs(row: &UpdateRow) -> String {
  let current = row.current_raw.blue().to_string();
  let current_time = format_time(&row.current_time_label);
  let arrow = "→".dimmed().to_string();
  let target = colorize_diff(&row.current_raw, &row.target_raw);
  let target_time = format_time(&row.target_time_label);
  format!("{current}{current_time} {arrow} {target}{target_time}")
}

fn format_time(label: &Option<String>) -> String {
  match label {
    Some(t) => format!(" ({t})").dimmed().to_string(),
    None => String::new(),
  }
}

/// `◉` (white when selected) or `◌` (dimmed). `None` selection
/// renders as a single space, matching width with the visible glyphs.
fn radio(selected: Option<bool>) -> String {
  match selected {
    Some(true) => "◉".white().to_string(),
    Some(false) => "◌".dimmed().to_string(),
    None => " ".to_string(),
  }
}

/// A dep with a single bucket renders inline. Interactive mode adds a
/// 1-char cursor slot at line start and a radio glyph; non-interactive
/// drops both since neither carries information without a selection.
///
/// Non-interactive: `   1x @azure/identity ^4.13.0 (~7mo) → ^4.13.1 (~2mo)`
/// Interactive:    `{cursor}   1x ◉ @azure/identity ^4.13.0 (~7mo) → ^4.13.1 (~2mo)`
fn format_solo_line(row: &UpdateRow, selected: Option<bool>, focused: bool, max_width: Option<usize>) -> String {
  let count = count_cell(row.dependency_outdated_count);
  let specs = render_specs(row);
  let build = |name: &str| -> String {
    match selected {
      None => format!("{count} {name} {specs}"),
      Some(_) => {
        let cursor = if focused { "▶".cyan().to_string() } else { " ".to_string() };
        format!("{cursor}{count} {} {name} {specs}", radio(selected))
      }
    }
  };
  let line = build(&row.dependency_name);
  let Some(max_w) = max_width else { return line };
  let line_w = visible_width(&line);
  if line_w <= max_w {
    return line;
  }
  let overflow = line_w - max_w;
  let name_w: usize = row.dependency_name.chars().map(|c| c.width().unwrap_or(0)).sum();
  let name_budget = name_w.saturating_sub(overflow);
  build(&middle_truncate(&row.dependency_name, name_budget))
}

/// Header line for a multi-bucket dep:
/// `   3x @azure/identity` (count right-aligned, single-space gap to the
/// dep name). The interactive variant pads the line by 1 char so its
/// bucket rows can use a `▶` cursor and adds a 2-space gap so the dep
/// name aligns with the radio column on solo lines.
fn format_dep_header(row: &UpdateRow, interactive: bool, max_width: Option<usize>) -> String {
  let count = count_cell(row.dependency_outdated_count);
  let build = |name: &str| -> String {
    if interactive {
      format!(" {count}   {name}")
    } else {
      format!("{count} {name}")
    }
  };
  let line = build(&row.dependency_name);
  let Some(max_w) = max_width else { return line };
  let line_w = visible_width(&line);
  if line_w <= max_w {
    return line;
  }
  let overflow = line_w - max_w;
  let name_w: usize = row.dependency_name.chars().map(|c| c.width().unwrap_or(0)).sum();
  let name_budget = name_w.saturating_sub(overflow);
  build(&middle_truncate(&row.dependency_name, name_budget))
}

/// One bucket line under a multi-bucket dep header. The bucket count
/// occupies the same first-column slot as the dep-header / solo-line
/// count, so all `Nx` glyphs stack in a single right-aligned column.
///
/// Non-interactive: `   1x ^4.13.0 (~7mo) → ^4.13.1 (~2mo)`
/// Interactive:     `{cursor}   1x ◉ ^4.13.0 (~7mo) → ^4.13.1 (~2mo)`
///
/// No dep name to truncate here; long bucket lines are accepted as-is.
fn format_bucket_line(row: &UpdateRow, selected: Option<bool>, focused: bool) -> String {
  let count = count_cell(row.bucket_count);
  let specs = render_specs(row);
  match selected {
    None => format!("{count} {specs}"),
    Some(_) => {
      let cursor = if focused { "▶".cyan().to_string() } else { " ".to_string() };
      format!("{cursor}{count} {} {specs}", radio(selected))
    }
  }
}

/// Render the trailing "M major, N minor, P patch update[s]" summary.
/// No-op when nothing was outdated.
pub fn render_summary(counts: DiffCounts) {
  if counts.total() == 0 {
    return;
  }
  let mut parts = vec![];
  if counts.major > 0 {
    parts.push(format!("{} major", counts.major).red().to_string());
  }
  if counts.minor > 0 {
    parts.push(format!("{} minor", counts.minor).cyan().to_string());
  }
  if counts.patch > 0 {
    parts.push(format!("{} patch", counts.patch).green().to_string());
  }
  let suffix = if counts.total() == 1 { "update" } else { "updates" };
  info!("{LINE_ENDING}{} {suffix}", parts.join(", "));
}
