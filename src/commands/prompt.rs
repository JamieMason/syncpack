use colored::Colorize;

/// Run the prompt command (deprecated in v14)
pub fn run() -> i32 {
  eprintln!("{}", "Deprecated in syncpack v14".red().bold());
  eprintln!();
  eprintln!("{}", "prompt → Removed".yellow().bold());
  eprintln!();
  eprintln!(
    "The {} command is an interactive prompt which lists all current issues which syncpack",
    "prompt".cyan()
  );
  eprintln!("can't fix automatically. It is not yet available in v14 and will be added at a later date.");
  eprintln!();
  eprintln!("Syncpack can't automatically fix mismatches between specifiers it does not support,");
  eprintln!("which are usually specifiers which are not semver, such as pnpm overrides, or complex");
  eprintln!("semver specifiers like {}.", "^1.2.3 || ^2.0.0".cyan());
  eprintln!();
  eprintln!("{}", "Status:".bold());
  eprintln!();
  eprintln!("  {}", "# v13".dimmed());
  eprintln!("  {}", "syncpack prompt".dimmed());
  eprintln!();
  eprintln!("  {}", "# v14".dimmed());
  eprintln!("  {}", "# Not yet implemented".red());
  eprintln!();
  eprintln!("{}", "https://syncpack.dev/guide/migrate-v14#prompt-removed".blue().underline());
  eprintln!();
  1
}
