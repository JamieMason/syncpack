use {
  crate::{context::Context, effects::ui},
  colored::*,
  itertools::Itertools,
  log::info,
};

/// Join lines that are not empty with a space separator
pub fn join_line(lines: Vec<&String>) -> String {
  lines.into_iter().filter(|line| !line.is_empty()).join(" ")
}

/// Return a right-aligned column of a count of instances
/// Example "    38x"
pub fn count_column(count: usize) -> String {
  match ui::DEFAULT_INDENT {
    0 => format!("{count: >0}x"),
    1 => format!("{count: >1}x"),
    2 => format!("{count: >2}x"),
    3 => format!("{count: >3}x"),
    4 => format!("{count: >4}x"),
    5 => format!("{count: >5}x"),
    6 => format!("{count: >6}x"),
    _ => format!("{count: >7}x"),
  }
  .dimmed()
  .to_string()
}

/// Render the reason code as a clickable link
pub fn get_status_code_link(ctx: &Context, pascal_case: &str) -> String {
  let base_url = "https://jamiemason.github.io/syncpack/status";
  let kebab_case = pascal_case
    .chars()
    .enumerate()
    .map(|(i, c)| {
      if i == 0 {
        c.to_lowercase().to_string()
      } else if c.is_uppercase() {
        format!("-{}", c.to_lowercase())
      } else {
        c.to_string()
      }
    })
    .collect::<String>();
  get_link(ctx, format!("{base_url}/{kebab_case}"), pascal_case)
}

/// Render a clickable link
pub fn get_link(ctx: &Context, url: impl Into<String>, text: impl Into<ColoredString>) -> String {
  if ctx.config.cli.disable_ansi {
    text.into().to_string()
  } else {
    format!("\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\", url.into(), text.into())
  }
}

/// Convert eg. "/dependencies/react" to ".dependencies.react"
pub fn get_formatted_path(path: &str) -> String {
  if path == "/" {
    "root".to_string()
  } else {
    path.replace("/", ".")
  }
}

pub fn print_no_issues_found() {
  let icon = ui::icon::ok();
  info!("{icon} No issues found");
}
