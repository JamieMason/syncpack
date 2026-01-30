use colored::Colorize;

/// Run the fix-mismatches command (deprecated in v14)
pub fn run() -> i32 {
  eprintln!("{}", "Deprecated in syncpack v14".red().bold());
  eprintln!();
  eprintln!("{}", "fix-mismatches → fix".yellow().bold());
  eprintln!();
  eprintln!(
    "{} and {} have been merged into a single {} command which autofixes",
    "fix-mismatches".cyan(),
    "set-semver-ranges".cyan(),
    "fix".green()
  );
  eprintln!(
    "issues found by {}. The {} command no longer fixes formatting, which is now handled",
    "syncpack lint".green(),
    "fix".green()
  );
  eprintln!("by {}.", "syncpack format".green());
  eprintln!();
  eprintln!("{}", "Migration Example:".bold());
  eprintln!();
  eprintln!("  {}", "# v13".dimmed());
  eprintln!("  {}", "syncpack fix-mismatches".dimmed());
  eprintln!();
  eprintln!("  {}", "# v14".dimmed());
  eprintln!("  {}", "syncpack fix".cyan());
  eprintln!();
  eprintln!(
    "{}",
    "https://jamiemason.github.io/syncpack/guide/migrate-v14#fix-mismatches-fix"
      .blue()
      .underline()
  );
  eprintln!();
  1
}
