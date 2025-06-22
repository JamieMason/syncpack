use {
  crate::{
    cli::Cli,
    rcfile::{error::RcfileError, Rcfile},
  },
  log::debug,
  std::{fs, path::Path},
};

pub fn from_json_path(file_path: &Path) -> Result<Rcfile, RcfileError> {
  fs::read_to_string(file_path)
    .map_err(RcfileError::FileReadFailed)
    .and_then(|contents| serde_json::from_str::<Rcfile>(&contents).map_err(RcfileError::JsonParseFailed))
}

pub fn try_from_json_candidates(cli: &Cli) -> Option<Result<Rcfile, RcfileError>> {
  let candidates = vec![".syncpackrc", ".syncpackrc.json"];
  for candidate in candidates {
    let config_path = cli.cwd.join(candidate);
    if config_path.exists() {
      debug!("Found JSON config file: {:?}", config_path);
      return Some(from_json_path(&config_path));
    }
  }
  None
}
