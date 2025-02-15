use crate::context::Context;

/// Runs side-effects to write fixes to disk
pub mod fix;
/// Runs side-effects to lint and fix package.json formatting
pub mod format;
/// Runs side-effects to write lint messages to the UI
pub mod lint;
/// A shared module with methods for printing messages to the console
pub mod ui;
/// Runs side-effects to find and apply updates from the npm registry
pub mod update;

/// Side effects in Syncpack commands are handled by structs which implement
/// this trait. Multiple commands such as `lint`, `fix`, and `json` all depend
/// on the same core logic, but have different side effects.
///
/// This trait allows the core logic to be reused across all commands, while the
/// side effects are handled by the command-specific structs which implement
/// this trait.
pub trait Effects {
  fn run(ctx: Context) -> Context;
}
