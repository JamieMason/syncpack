use {
  crate::{
    cli::Cli,
    rcfile::{
      error::{ConfigError, NodeJsResult},
      Rcfile,
    },
  },
  log::debug,
  std::{path::Path, process::Command},
};

pub fn from_javascript_path(file_path: &Path) -> Result<Rcfile, ConfigError> {
  let escaped_file_path_for_nodejs = file_path.to_string_lossy().replace('\\', "\\\\");
  let nodejs_script = format!(
    r#"
        import('{escaped_file_path_for_nodejs}')
          .then((mod) => mod.default)
          .then((value) => {{
            if (isNonEmptyObject(value)) {{
              console.log(JSON.stringify({{
                _tag: 'Ok',
                value,
              }}));
            }} else {{
              tryRequire('Config expected at default export');
            }}
          }})
          .catch((err) => {{
            tryRequire(err.stack || err.message || 'Unknown error in import()');
          }});

        function tryRequire(importError) {{
          Promise.resolve(null)
            .then(() => require('{escaped_file_path_for_nodejs}'))
            .then((mod) => mod.default || mod)
            .then((value) => {{
              if (isNonEmptyObject(value)) {{
                console.log(JSON.stringify({{
                  _tag: 'Ok',
                  value,
                }}));
              }} else {{
                console.log(JSON.stringify({{
                  _tag: 'Err',
                  importError,
                  requireError: 'Config expected at module.exports',
                }}));
              }}
            }})
            .catch((err) => {{
              console.log(JSON.stringify({{
                _tag: 'Err',
                importError,
                requireError: err.stack || err.message || 'Unknown require error'
              }}));
            }});
        }};

        function isNonEmptyObject(value) {{
          return value && typeof value === 'object' && value.constructor === Object && Object.keys(value).length > 0;
        }}
      "#
  );

  Command::new("npx")
    .args(["tsx", "-e", &nodejs_script])
    .current_dir(file_path.parent().unwrap_or_else(|| Path::new(".")))
    .output()
    .map_err(ConfigError::CommandExecutionFailed)
    .and_then(|output| {
      if output.status.success() {
        Ok(output.stdout)
      } else {
        Err(ConfigError::ProcessFailed {
          stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
      }
    })
    .and_then(|stdout| String::from_utf8(stdout).map_err(ConfigError::InvalidUtf8))
    .inspect(|json_str| {
      debug!("Raw output from {:?}: {}", file_path, json_str.trim());
    })
    .and_then(|json_str| serde_json::from_str::<NodeJsResult>(&json_str).map_err(ConfigError::JsonParseFailed))
    .and_then(|response| match response {
      NodeJsResult::Success { value } => serde_json::from_value(value).map_err(ConfigError::ConfigDeserializationFailed),
      NodeJsResult::Error {
        import_error,
        require_error,
      } => Err(ConfigError::ImportAndRequireFailed {
        import_error,
        require_error,
      }),
    })
}

pub fn try_from_js_candidates(cli: &Cli) -> Option<Result<Rcfile, ConfigError>> {
  let candidates = vec![
    ".syncpackrc.js",
    ".syncpackrc.ts",
    ".syncpackrc.mjs",
    ".syncpackrc.cjs",
    "syncpack.config.js",
    "syncpack.config.ts",
    "syncpack.config.mjs",
    "syncpack.config.cjs",
  ];
  for candidate in candidates {
    let config_path = cli.cwd.join(candidate);
    if config_path.exists() {
      debug!("Found JavaScript/TypeScript config file: {:?}", config_path);
      return Some(from_javascript_path(&config_path));
    }
  }
  None
}
