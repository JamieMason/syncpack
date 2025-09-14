use colored::*;

pub fn ok() -> String {
  "\u{2713}".green().to_string()
}

pub fn err() -> String {
  "\u{2718}".red().to_string()
}

pub fn blue_err() -> String {
  "\u{2718}".blue().to_string()
}

pub fn warn() -> String {
  "!".yellow().to_string()
}

pub fn dim_right_arrow() -> String {
  "\u{2192}".dimmed().to_string()
}

pub fn dim_left_arrow() -> String {
  "\u{2190}".dimmed().to_string()
}
