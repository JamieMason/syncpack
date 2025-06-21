use {
  crate::{
    cli::Cli,
    rcfile::{
      error::ConfigError, javascript::try_from_js_candidates, json::try_from_json_candidates,
      package_json::try_from_package_json_config_property, yaml::try_from_yaml_candidates, Rcfile,
    },
  },
  log::{debug, error},
  std::{process::exit, time::Instant},
};

impl Rcfile {
  pub fn from_disk(cli: &Cli) -> Rcfile {
    let start = Instant::now();
    let rcfile = try_from_json_candidates(cli)
      .or_else(|| try_from_yaml_candidates(cli))
      .or_else(|| try_from_package_json_config_property(cli))
      .or_else(|| try_from_js_candidates(cli))
      .map(|result| match result {
        Ok(rcfile) => rcfile,
        Err(err) => {
          match err {
            ConfigError::FileReadFailed(err) => {
              error!("Failed to read config file: {:?}", err);
            }
            ConfigError::CommandExecutionFailed(err) => {
              error!("Failed to run Node.js/npx/tsx to retrieve JS/TS config file: {:?}", err);
            }
            ConfigError::ProcessFailed { stderr } => {
              error!("Node.js/npx/tsx process failed with stderr: {}", stderr);
            }
            ConfigError::InvalidUtf8(err) => {
              error!("Config file contains invalid UTF-8: {:?}", err);
            }
            ConfigError::ConfigDeserializationFailed(err) => {
              error!("Failed to deserialise config file: {:?}", err);
            }
            ConfigError::ImportAndRequireFailed {
              import_error,
              require_error,
            } => {
              if !import_error.is_empty() {
                error!("Failed to read JS/TS config file using import(): {:?}", import_error);
              }
              if !require_error.is_empty() {
                error!("Failed to read JS/TS config file using require(): {:?}", require_error);
              }
            }
            ConfigError::JsonParseFailed(err) => {
              error!("Failed to parse JSON in config file: {:?}", err);
            }
            ConfigError::YamlParseFailed(err) => {
              error!("Failed to parse YAML in config file: {:?}", err);
            }
            ConfigError::PackageJsonConfigInvalid(err) => {
              error!("Invalid .syncpack or .config.syncpack property defined in package.json: {:?}", err);
            }
          }
          exit(1);
        }
      })
      .unwrap_or_else(|| {
        debug!("No config file found, using defaults");
        Rcfile::default()
      });
    debug!("Config discovery completed in {:?}", start.elapsed());
    rcfile
  }
}
