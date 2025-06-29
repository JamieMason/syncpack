use {
  crate::{
    cli::Cli,
    rcfile::{
      javascript::try_from_js_candidates, json::try_from_json_candidates, package_json::try_from_package_json_config_property,
      yaml::try_from_yaml_candidates, Rcfile,
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
          error!("{err}");
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
