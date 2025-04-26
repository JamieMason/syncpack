/// Indent level used across UI formatting
const DEFAULT_INDENT: usize = 4;

#[cfg(windows)]
pub const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
pub const LINE_ENDING: &str = "\n";

pub mod dependency;
pub mod group;
pub mod icon;
pub mod instance;
pub mod package;
pub mod util;
