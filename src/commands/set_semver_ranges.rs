use colored::Colorize;

/// Run the set-semver-ranges command (deprecated in v14)
pub fn run() -> i32 {
  eprintln!("{}", "Deprecated in syncpack v14".red().bold());
  eprintln!();
  eprintln!("{}", "set-semver-ranges → fix".yellow().bold());
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
  eprintln!("  {}", "syncpack set-semver-ranges".dimmed());
  eprintln!();
  eprintln!("  {}", "# v14".dimmed());
  eprintln!("  {}", "syncpack fix".cyan());
  eprintln!();
  eprintln!(
    "{}",
    "https://syncpack.dev/guide/migrate-v14#set-semver-ranges-fix".blue().underline()
  );
  eprintln!();
  1
}
