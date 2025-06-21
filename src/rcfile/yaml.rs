use {
  crate::{
    cli::Cli,
    rcfile::{error::ConfigError, Rcfile},
  },
  log::debug,
  std::{fs, path::Path},
};

pub fn from_yaml_path(file_path: &Path) -> Result<Rcfile, ConfigError> {
  fs::read_to_string(file_path)
    .map_err(ConfigError::FileReadFailed)
    .and_then(|contents| serde_yaml::from_str::<Rcfile>(&contents).map_err(ConfigError::YamlParseFailed))
}

pub fn try_from_yaml_candidates(cli: &Cli) -> Option<Result<Rcfile, ConfigError>> {
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
