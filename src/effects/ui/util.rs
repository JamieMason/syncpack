use {
  crate::{context::Context, effects::ui},
  colored::*,
  itertools::Itertools,
};

/// Join lines that are not empty with a space separator
pub fn join_line(lines: Vec<&String>) -> String {
  lines.into_iter().filter(|line| !line.is_empty()).join(" ")
}

/// Return a right-aligned column of a count of instances
/// Example "    38x"
pub fn count_column(count: usize) -> String {
  match ui::DEFAULT_INDENT {
    0 => format!("{: >0}x", count),
    1 => format!("{: >1}x", count),
    2 => format!("{: >2}x", count),
    3 => format!("{: >3}x", count),
    4 => format!("{: >4}x", count),
    5 => format!("{: >5}x", count),
    6 => format!("{: >6}x", count),
    _ => format!("{: >7}x", count),
  }
  .dimmed()
  .to_string()
}

/// Render the reason code as a clickable link
pub fn status_code_link(ctx: &Context, pascal_case: &str) -> String {
  let base_url = "https://jamiemason.github.io/syncpack/guide/status-codes/";
  let lower_case = pascal_case.to_lowercase();
  link(ctx, format!("{base_url}#{lower_case}"), pascal_case)
}

/// Render a clickable link
pub fn link(ctx: &Context, url: impl Into<String>, text: impl Into<ColoredString>) -> String {
  if ctx.config.cli.disable_ansi {
    text.into().to_string()
  } else {
    format!("\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\", url.into(), text.into())
  }
}

/// Convert eg. "/dependencies/react" to ".dependencies.react"
pub fn format_path(path: &str) -> String {
  if path == "/" {
    "root".to_string()
  } else {
    path.replace("/", ".")
  }
}
