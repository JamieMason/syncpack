use {
  crate::{
    cli::Cli,
    rcfile::{
      error::RcfileError,
      javascript::{from_javascript_path, try_from_js_candidates},
      json::{from_json_path, try_from_json_candidates},
      package_json::try_from_package_json_config_property,
      yaml::{from_yaml_path, try_from_yaml_candidates},
      Rcfile,
    },
  },
  log::{debug, error},
  std::{path::PathBuf, process::exit, time::Instant},
};

impl Rcfile {
  pub fn from_disk(cli: &Cli) -> Rcfile {
    let start = Instant::now();
    let rcfile = Self::try_from_cli_option(cli)
      .or_else(|| try_from_json_candidates(cli))
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
    rcfile.warn_deprecated_v13_config();
    rcfile
  }

  fn try_from_cli_option(cli: &Cli) -> Option<Result<Rcfile, RcfileError>> {
    cli.config_path.as_ref().map(|path| {
      let config_path = PathBuf::from(path);
      let absolute_path = if config_path.is_absolute() {
        config_path
      } else {
        cli.cwd.join(config_path)
      };

      debug!("Using config file from CLI option: {absolute_path:?}");

      if !absolute_path.exists() {
        return Err(RcfileError::FileReadFailed(std::io::Error::new(
          std::io::ErrorKind::NotFound,
          format!("Config file not found: {}", absolute_path.display()),
        )));
      }

      let extension = absolute_path.extension().and_then(|ext| ext.to_str());
      match extension {
        Some("json") => from_json_path(&absolute_path),
        Some("yaml") | Some("yml") => from_yaml_path(&absolute_path),
        Some("js") | Some("cjs") | Some("mjs") | Some("ts") | Some("cts") | Some("mts") => from_javascript_path(&absolute_path),
        _ => from_json_path(&absolute_path),
      }
    })
  }
}
