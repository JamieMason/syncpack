use {
  crate::{
    cli::Cli,
    disk::{detect_formatting, DetectedFormatting, Disk, DiskIo, DiskIoError, File, NodeJsError},
    errors::UnsupportedConfigErrors,
    rcfile::{
      from_disk::javascript::{get_javascript_contents, JsResult},
      RawRcfile, Rcfile,
    },
  },
  log::debug,
  std::{path::Path, time::Instant},
  thiserror::Error,
};

#[path = "javascript.rs"]
mod javascript;

#[derive(Debug, Error)]
pub enum JsRcfileError {
  #[error(transparent)]
  DiskIoError(DiskIoError),
  #[error("Node.js threw when trying to import() your config file:\n\n{import_error}\n\n{require_error}")]
  ImportError { import_error: String, require_error: String },
  #[error("Config file failed validation:\n\n{0}")]
  InvalidConfig(#[from] serde_json::Error),
  #[error(transparent)]
  NodeJsError(NodeJsError),
}

#[derive(Debug, Error)]
pub enum RcfileError {
  #[error(transparent)]
  DiskIoError(DiskIoError),
  #[error(transparent)]
  JsRcfileError(JsRcfileError),
  #[error(transparent)]
  UnsupportedConfig(UnsupportedConfigErrors),
}

impl Rcfile {
  pub fn from_disk<T: DiskIo>(disk: &Disk<T>, cli: &Cli) -> Result<File<Rcfile>, RcfileError> {
    let start = Instant::now();

    let from_json_path = |filepath: &Path| -> Option<Result<File<RawRcfile>, RcfileError>> {
      disk
        .io
        .read_json_file::<RawRcfile>(filepath)
        .map(|res| res.map_err(RcfileError::DiskIoError))
    };

    let from_yaml_path = |filepath: &Path| -> Option<Result<File<RawRcfile>, RcfileError>> {
      disk
        .io
        .read_yaml_file::<RawRcfile>(filepath)
        .map(|res| res.map_err(RcfileError::DiskIoError))
    };

    let from_javascript_path = |filepath: &Path| -> Option<Result<File<RawRcfile>, RcfileError>> {
      Some(filepath).filter(|filepath| filepath.exists()).map(|filepath| {
        let nodejs_script = get_javascript_contents(filepath);
        let is_typescript = filepath.to_string_lossy().ends_with("ts");
        let mut args = vec![];

        if is_typescript {
          args.push("--experimental-strip-types");
        }

        args.push("--eval");
        args.push(&nodejs_script);

        disk
          .io
          .exec_node_command(&cli.cwd, &args)
          .map_err(JsRcfileError::NodeJsError)
          .inspect(|stdout| {
            debug!("Raw output from {:?}: {}", filepath, stdout.trim());
          })
          .and_then(|stdout| {
            serde_json::from_str::<JsResult>(&stdout)
              .map_err(DiskIoError::JsonParse)
              .map_err(JsRcfileError::DiskIoError)
          })
          .and_then(|js_result| match js_result {
            JsResult::Success { value } => serde_json::from_str::<RawRcfile>(&value)
              .map_err(DiskIoError::JsonParse)
              .map_err(JsRcfileError::DiskIoError)
              .map(|contents| File {
                filepath: filepath.to_path_buf(),
                formatting: detect_formatting(&value),
                contents,
              }),
            JsResult::Error {
              import_error,
              require_error,
            } => Err(JsRcfileError::ImportError {
              import_error,
              require_error,
            }),
          })
          .map_err(RcfileError::JsRcfileError)
      })
    };

    let from_any_path = |filepath: &Path| -> Option<Result<File<RawRcfile>, RcfileError>> {
      filepath.extension().and_then(|ext| ext.to_str()).and_then(|ext| match ext {
        "syncpackrc" | "json" => from_json_path(filepath),
        "yaml" | "yml" => from_yaml_path(filepath),
        "js" | "cjs" | "mjs" | "ts" | "cts" | "mts" => from_javascript_path(filepath),
        _ => from_json_path(filepath),
      })
    };

    let from_cli_option = || -> Option<Result<File<RawRcfile>, RcfileError>> {
      cli.config_path.as_ref().and_then(|config_path| {
        debug!("Using config file from CLI option: {config_path:?}");
        from_any_path(config_path)
      })
    };

    let from_package_json_config_property = || -> Option<Result<File<RawRcfile>, RcfileError>> {
      disk.package_json_root.as_ref().and_then(|file| {
        file
          .contents
          .get("syncpack")
          .inspect(|_| debug!("Found .syncpack property in package.json"))
          .or_else(|| {
            file
              .contents
              .pointer("/config/syncpack")
              .inspect(|_| debug!("Found .config.syncpack property in package.json"))
          })
          .cloned()
          .map(|value| {
            serde_json::from_value::<RawRcfile>(value)
              .map_err(DiskIoError::JsonParse)
              .map_err(RcfileError::DiskIoError)
              .map(|contents| File {
                filepath: file.filepath.clone(),
                formatting: file.formatting.clone(),
                contents,
              })
          })
      })
    };

    let raw_rcfile = from_cli_option()
      .or_else(|| from_json_path(&cli.cwd.join(".syncpackrc")))
      .or_else(|| from_json_path(&cli.cwd.join(".syncpackrc.json")))
      .or_else(|| from_yaml_path(&cli.cwd.join(".syncpackrc.yaml")))
      .or_else(|| from_yaml_path(&cli.cwd.join(".syncpackrc.yml")))
      .or_else(|| from_javascript_path(&cli.cwd.join(".syncpackrc.js")))
      .or_else(|| from_javascript_path(&cli.cwd.join(".syncpackrc.ts")))
      .or_else(|| from_javascript_path(&cli.cwd.join(".syncpackrc.mjs")))
      .or_else(|| from_javascript_path(&cli.cwd.join(".syncpackrc.cjs")))
      .or_else(|| from_javascript_path(&cli.cwd.join("syncpack.config.js")))
      .or_else(|| from_javascript_path(&cli.cwd.join("syncpack.config.ts")))
      .or_else(|| from_javascript_path(&cli.cwd.join("syncpack.config.mjs")))
      .or_else(|| from_javascript_path(&cli.cwd.join("syncpack.config.cjs")))
      .or_else(from_package_json_config_property);

    if let Some(Ok(file)) = raw_rcfile {
      let filepath = file.filepath;
      let raw_rcfile = file.contents;

      // @TODO: See if this can be done whenever serde deserializes a RawRcfile
      if let Err(config_errors) = raw_rcfile.validate_unknown_fields() {
        debug!("Config discovery completed in {:?}", start.elapsed());
        return Err(RcfileError::UnsupportedConfig(UnsupportedConfigErrors(config_errors)));
      }

      if let Ok(rcfile) = Rcfile::try_from(raw_rcfile) {
        debug!("Config discovery completed in {:?}", start.elapsed());
        return Ok(File {
          filepath,
          formatting: file.formatting.clone(),
          contents: rcfile,
        });
      }
    }

    debug!("No config file found, using defaults");
    debug!("Config discovery completed in {:?}", start.elapsed());
    Ok(File {
      filepath: cli.cwd.join(".syncpackrc"),
      formatting: DetectedFormatting::default(),
      contents: Rcfile::default(),
    })
  }
}
