use crate::{cli::Cli, rcfile::Rcfile};

#[derive(Debug)]
pub struct Config {
  pub cli: Cli,
  pub rcfile: Rcfile,
}

impl Config {
  /// Read the rcfile from stdin and fall back to defaults if none was sent
  pub fn from_cli(cli: Cli) -> Config {
    Config {
      rcfile: Rcfile::from_disk(&cli),
      cli,
    }
  }
}
