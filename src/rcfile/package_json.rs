use {
  crate::{
    cli::Cli,
    rcfile::{error::ConfigError, Rcfile},
  },
  log::debug,
  serde_json::Value,
  std::fs,
};

pub fn try_from_package_json_config_property(cli: &Cli) -> Option<Result<Rcfile, ConfigError>> {
  let package_json_path = cli.cwd.join("package.json");
  package_json_path
    .exists()
    .then_some(&package_json_path)
    .and_then(|path| fs::read_to_string(path).ok())
    .and_then(|contents| serde_json::from_str::<Value>(&contents).ok())
    .and_then(|mut package_json| {
      package_json
        .get_mut("syncpack")
        .inspect(|_| debug!("Found .syncpack property in package.json"))
        .map(|config| config.take())
        .or_else(|| {
          package_json
            .pointer_mut("/config/syncpack")
            .inspect(|_| debug!("Found .config.syncpack property in package.json"))
            .map(|config| config.take())
        })
        .map(|syncpack_config| serde_json::from_value(syncpack_config).map_err(ConfigError::PackageJsonConfigInvalid))
    })
}
