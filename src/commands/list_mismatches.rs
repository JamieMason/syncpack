use colored::Colorize;

/// Run the list-mismatches command (deprecated in v14)
pub fn run() -> i32 {
  eprintln!("{}", "Deprecated in syncpack v14".red().bold());
  eprintln!();
  eprintln!("{}", "list-mismatches → lint".yellow().bold());
  eprintln!();
  eprintln!(
    "{} and {} have been merged into a single {} command which checks",
    "list-mismatches".cyan(),
    "lint-semver-ranges".cyan(),
    "lint".green()
  );
  eprintln!("whether every specifier matches the semver group and version group they belong to.");
  eprintln!(
    "The {} command no longer checks formatting, which is now handled by {}.",
    "lint".green(),
    "syncpack format --check".green()
  );
  eprintln!();
  eprintln!("{}", "Migration Example:".bold());
  eprintln!();
  eprintln!("  {}", "# v13".dimmed());
  eprintln!("  {}", "syncpack list-mismatches --types prod,dev".dimmed());
  eprintln!();
  eprintln!("  {}", "# v14".dimmed());
  eprintln!("  {}", "syncpack lint --dependency-types prod,dev".cyan());
  eprintln!();
  eprintln!(
    "{}",
    "https://jamiemason.github.io/syncpack/guide/migrate-v14#list-mismatches-lint"
      .blue()
      .underline()
  );
  eprintln!();
  1
}
