use {
  crate::{
    cli::Cli,
    rcfile::{
      error::{NodeJsResult, RcfileError},
      Rcfile,
    },
  },
  log::debug,
  std::{path::Path, process::Command},
};

pub fn from_javascript_path(file_path: &Path) -> Result<Rcfile, RcfileError> {
  let escaped_file_path_for_nodejs = file_path.to_string_lossy().replace('\\', "\\\\");
  let nodejs_script = format!(
    r#"
    import('tsx')
      .catch(() => null)
      .then(() => import('{escaped_file_path_for_nodejs}'))
      .then(findConfig)
      .then((value) => {{
        if (isNonEmptyObject(value)) {{
          console.log(JSON.stringify({{
            _tag: 'Ok',
            value: JSON.stringify(value),
            source: 'import',
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
        .then(findConfig)
        .then((value) => {{
          if (isNonEmptyObject(value)) {{
            console.log(JSON.stringify({{
              _tag: 'Ok',
              value: JSON.stringify(value),
              source: 'require',
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

    function findConfig(mod) {{
      return mod.default && mod.default.default ? mod.default.default : mod.default;
    }}
    "#
  );

  Command::new("node")
    .args(vec!["-e", &nodejs_script])
    .current_dir(file_path.parent().unwrap_or_else(|| Path::new(".")))
    .output()
    .map_err(RcfileError::NodeJsExecutionFailed)
    .and_then(|output| {
      if output.status.success() {
        Ok(output.stdout)
      } else {
        Err(RcfileError::ProcessFailed {
          stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
      }
    })
    .and_then(|stdout| String::from_utf8(stdout).map_err(RcfileError::InvalidUtf8))
    .inspect(|json_str| {
      debug!("Raw output from {:?}: {}", file_path, json_str.trim());
    })
    .and_then(|json_str| serde_json::from_str::<NodeJsResult>(&json_str).map_err(RcfileError::JsonParseFailed))
    .and_then(|response| match response {
      NodeJsResult::Success { value } => serde_json::from_str::<Rcfile>(&value).map_err(RcfileError::InvalidConfig),
      NodeJsResult::Error {
        import_error,
        require_error,
      } => Err(RcfileError::JavaScriptImportFailed {
        import_error,
        require_error,
      }),
    })
}

pub fn try_from_js_candidates(cli: &Cli) -> Option<Result<Rcfile, RcfileError>> {
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
      debug!("Found JavaScript/TypeScript config file: {config_path:?}");
      return Some(from_javascript_path(&config_path));
    }
  }
  None
}
