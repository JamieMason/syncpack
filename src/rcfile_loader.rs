use {
  crate::{
    cli::Cli,
    javascript::{JsResult, get_javascript_contents},
  },
  log::debug,
  serde_json::{Value, json},
  std::{path::Path, time::Instant},
  syncpack_config::{ConfigError, Rcfile, parse_value},
  syncpack_disk::Disk,
  syncpack_io::{DetectedFormatting, DiskIo, DiskIoError, File, NodeJsError, detect_formatting, json_view},
  syncpack_vocab::config_error::UnsupportedConfigErrors,
  thiserror::Error,
};

#[cfg(test)]
#[path = "rcfile_loader_test.rs"]
mod rcfile_loader_test;

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

pub fn rcfile_from_disk<T: DiskIo>(disk: &Disk, io: &T, cli: &Cli) -> Result<File<Rcfile>, RcfileError> {
  let start = Instant::now();

  // Each source path yields the config as a raw `serde_json::Value`; the value
  // is deserialised, validated and resolved into an `Rcfile` by
  // `config::parse_value` at the end. Discovery reads only bytes here; the
  // schema lives in `config`.
  let from_json_path = |filepath: &Path| -> Option<Result<File<Value>, RcfileError>> {
    io.read_json_file::<Value>(filepath)
      .map(|res| res.map_err(RcfileError::DiskIoError))
  };

  let from_yaml_path = |filepath: &Path| -> Option<Result<File<Value>, RcfileError>> {
    io.read_yaml_file(filepath).map(|res| {
      res.map_err(RcfileError::DiskIoError).map(|yaml| File {
        filepath: yaml.filepath.clone(),
        formatting: yaml.formatting.clone(),
        contents: json_view(&yaml),
        dirty: false,
      })
    })
  };

  let from_javascript_path = |filepath: &Path| -> Option<Result<File<Value>, RcfileError>> {
    Some(filepath).filter(|filepath| filepath.exists()).map(|filepath| {
      let nodejs_script = get_javascript_contents(filepath);
      let is_typescript = filepath.to_string_lossy().ends_with("ts");
      let mut args = vec![];

      if is_typescript {
        args.push("--experimental-strip-types");
      }

      args.push("--eval");
      args.push(&nodejs_script);

      io.exec_node_command(&disk.cwd, &args)
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
          JsResult::Success { value } => serde_json::from_str::<Value>(&value)
            .map_err(DiskIoError::JsonParse)
            .map_err(JsRcfileError::DiskIoError)
            .map(|contents| File {
              filepath: filepath.to_path_buf(),
              formatting: detect_formatting(&value),
              contents,
              dirty: false,
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

  let from_any_path = |filepath: &Path| -> Option<Result<File<Value>, RcfileError>> {
    filepath.extension().and_then(|ext| ext.to_str()).and_then(|ext| match ext {
      "syncpackrc" | "json" => from_json_path(filepath),
      "yaml" | "yml" => from_yaml_path(filepath),
      "js" | "cjs" | "mjs" | "ts" | "cts" | "mts" => from_javascript_path(filepath),
      _ => from_json_path(filepath),
    })
  };

  let from_cli_option = || -> Option<Result<File<Value>, RcfileError>> {
    cli.config_path.as_ref().and_then(|config_path| {
      debug!("Using config file from CLI option: {config_path:?}");
      from_any_path(config_path)
    })
  };

  let from_package_json_config_property = || -> Option<Result<File<Value>, RcfileError>> {
    disk.package_json_root().and_then(|file| {
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
          Ok(File {
            filepath: file.filepath.clone(),
            formatting: file.formatting.clone(),
            contents: value,
            dirty: false,
          })
        })
    })
  };

  let raw_rcfile = from_cli_option()
    .or_else(|| from_json_path(&disk.cwd.join(".syncpackrc")))
    .or_else(|| from_json_path(&disk.cwd.join(".syncpackrc.json")))
    .or_else(|| from_yaml_path(&disk.cwd.join(".syncpackrc.yaml")))
    .or_else(|| from_yaml_path(&disk.cwd.join(".syncpackrc.yml")))
    .or_else(|| from_javascript_path(&disk.cwd.join(".syncpackrc.js")))
    .or_else(|| from_javascript_path(&disk.cwd.join(".syncpackrc.ts")))
    .or_else(|| from_javascript_path(&disk.cwd.join(".syncpackrc.mjs")))
    .or_else(|| from_javascript_path(&disk.cwd.join(".syncpackrc.cjs")))
    .or_else(|| from_javascript_path(&disk.cwd.join("syncpack.config.js")))
    .or_else(|| from_javascript_path(&disk.cwd.join("syncpack.config.ts")))
    .or_else(|| from_javascript_path(&disk.cwd.join("syncpack.config.mjs")))
    .or_else(|| from_javascript_path(&disk.cwd.join("syncpack.config.cjs")))
    .or_else(from_package_json_config_property);

  let pnpm_fallback = pnpm_minimum_release_age(disk);

  if let Some(Ok(file)) = raw_rcfile {
    match parse_value(file.contents, pnpm_fallback) {
      Ok(rcfile) => {
        debug!("Config discovery completed in {:?}", start.elapsed());
        return Ok(File {
          filepath: file.filepath,
          formatting: file.formatting,
          contents: rcfile,
          dirty: false,
        });
      }
      Err(ConfigError::Unsupported(errors)) => {
        debug!("Config discovery completed in {:?}", start.elapsed());
        return Err(RcfileError::UnsupportedConfig(errors));
      }
      // A structurally-invalid config value falls through to defaults, matching
      // the historical behaviour where a deserialize failure was not surfaced.
      Err(ConfigError::Deserialize(_)) => {}
    }
  }

  debug!("No config file found, using defaults");
  let rcfile = parse_value(json!({}), pnpm_fallback).expect("an empty config is always valid");
  debug!("Config discovery completed in {:?}", start.elapsed());
  Ok(File {
    filepath: disk.cwd.join(".syncpackrc"),
    formatting: DetectedFormatting::default(),
    contents: rcfile,
    dirty: false,
  })
}

/// The `minimumReleaseAge` (in minutes) declared in `pnpm-workspace.yaml`, if
/// any. The user-value / default precedence is applied by `config::parse_value`
/// which receives this as its fallback.
fn pnpm_minimum_release_age(disk: &Disk) -> Option<u64> {
  disk.pnpm_workspace.as_ref().and_then(|yaml| {
    let json = json_view(yaml);
    let value = json.get("minimumReleaseAge").and_then(|v| v.as_u64());
    if let Some(value) = value {
      debug!("Using minimumReleaseAge={value} from pnpm-workspace.yaml");
    }
    value
  })
}
