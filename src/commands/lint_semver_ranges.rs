use colored::Colorize;

/// Run the lint-semver-ranges command (deprecated in v14)
pub fn run() -> i32 {
  eprintln!("{}", "Deprecated in syncpack v14".red().bold());
  eprintln!();
  eprintln!("{}", "lint-semver-ranges → lint".yellow().bold());
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
  eprintln!("It's no longer possible to manage semver ranges without also checking version mismatches,");
  eprintln!("because the two things are so closely linked. Changes to semver ranges affect which");
  eprintln!("versions are considered valid and can indirectly cause version mismatches, so they are");
  eprintln!(
    "now always checked and changed together via the {} and {} commands.",
    "lint".green(),
    "fix".green()
  );
  eprintln!();
  eprintln!("{}", "Migration Example:".bold());
  eprintln!();
  eprintln!("  {}", "# v13".dimmed());
  eprintln!("  {}", "syncpack lint-semver-ranges".dimmed());
  eprintln!();
  eprintln!("  {}", "# v14".dimmed());
  eprintln!("  {}", "syncpack lint".cyan());
  eprintln!();
  eprintln!(
    "{}",
    "https://jamiemason.github.io/syncpack/guide/migrate-v14#lint-semver-ranges-lint"
      .blue()
      .underline()
  );
  eprintln!();
  1
}
