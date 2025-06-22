use {
  crate::{
    cli::Cli,
    rcfile::{error::RcfileError, Rcfile},
  },
  log::debug,
  std::{fs, path::Path},
};

pub fn from_yaml_path(file_path: &Path) -> Result<Rcfile, RcfileError> {
  fs::read_to_string(file_path)
    .map_err(RcfileError::FileReadFailed)
    .and_then(|contents| serde_yaml::from_str::<Rcfile>(&contents).map_err(RcfileError::YamlParseFailed))
}

pub fn try_from_yaml_candidates(cli: &Cli) -> Option<Result<Rcfile, RcfileError>> {
  let candidates = vec![".syncpackrc.yaml", ".syncpackrc.yml"];
  for candidate in candidates {
    let config_path = cli.cwd.join(candidate);
    if config_path.exists() {
      debug!("Found YAML config file: {:?}", config_path);
      return Some(from_yaml_path(&config_path));
    }
  }
  None
}
