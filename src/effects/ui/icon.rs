use colored::*;

pub fn ok() -> ColoredString {
  "\u{2713}".green()
}

pub fn err() -> ColoredString {
  "\u{2718}".red()
}

pub fn warn() -> ColoredString {
  "!".yellow()
}

pub fn unknown() -> ColoredString {
  "?".dimmed()
}

pub fn dim_right_arrow() -> ColoredString {
  "\u{2192}".dimmed()
}
