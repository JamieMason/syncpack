use {
  crate::{
    cli::Cli,
    rcfile::{
      error::{NodeJsResult, RcfileError},
      RawRcfile,
    },
  },
  log::debug,
  std::{path::Path, process::Command},
};

pub fn from_javascript_path(file_path: &Path) -> Result<RawRcfile, RcfileError> {
  let nodejs_script = build_nodejs_script(file_path);

  let is_typescript = file_path.to_string_lossy().ends_with("ts");
  let mut args = vec![];

  if is_typescript {
    args.push("--experimental-strip-types");
  }

  args.push("--eval");
  args.push(&nodejs_script);

  Command::new("node")
    .args(args)
    .current_dir(file_path.parent().unwrap_or_else(|| Path::new(".")))
    .output()
    .map_err(RcfileError::NodeJsExecutionFailed)
    .and_then(|output| {
      if output.status.success() {
        Ok(output.stdout)
      } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if stderr.contains("experimental-strip-types") {
          Err(RcfileError::NodeJsCannotStripTypes { stderr })
        } else {
          Err(RcfileError::ProcessFailed { stderr })
        }
      }
    })
    .and_then(|stdout| String::from_utf8(stdout).map_err(RcfileError::InvalidUtf8))
    .inspect(|json_str| {
      debug!("Raw output from {:?}: {}", file_path, json_str.trim());
    })
    .and_then(|json_str| serde_json::from_str::<NodeJsResult>(&json_str).map_err(RcfileError::JsonParseFailed))
    .and_then(|response| match response {
      NodeJsResult::Success { value } => serde_json::from_str::<RawRcfile>(&value).map_err(RcfileError::InvalidConfig),
      NodeJsResult::Error {
        import_error,
        require_error,
      } => Err(RcfileError::JavaScriptImportFailed {
        import_error,
        require_error,
      }),
    })
}

fn build_nodejs_script(file_path: &Path) -> String {
  let escaped_file_path_for_nodejs = file_path.to_string_lossy().replace('\\', "\\\\").replace('\'', "\\'");
  format!(
    r#"
    const {{ pathToFileURL }} = require('node:url');
    import(pathToFileURL('{escaped_file_path_for_nodejs}').href)
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
  )
}

pub fn try_from_js_candidates(cli: &Cli) -> Option<Result<RawRcfile, RcfileError>> {
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

#[cfg(test)]
mod tests {
  use {super::*, std::path::PathBuf};

  #[test]
  fn script_uses_path_to_file_url_for_import() {
    let path = PathBuf::from("/some/path/syncpack.config.cjs");
    let script = build_nodejs_script(&path);
    assert!(
      script.contains("pathToFileURL("),
      "import() should use pathToFileURL to convert paths to file:// URLs for Windows compatibility"
    );
    assert!(
      script.contains("require('node:url')"),
      "script should import pathToFileURL from node:url"
    );
  }

  #[test]
  fn script_does_not_import_raw_path() {
    let path = PathBuf::from("/some/path/syncpack.config.cjs");
    let script = build_nodejs_script(&path);
    // Extract the import() argument — should NOT be a bare string literal
    let import_call = script.find("import(").expect("script should contain import()");
    let after_import = &script[import_call + "import(".len()..];
    assert!(
      after_import.starts_with("pathToFileURL("),
      "import() argument should be pathToFileURL(...), got: {}",
      &after_import[..after_import.find(')').unwrap_or(60)]
    );
  }

  #[test]
  fn script_escapes_backslashes_for_windows_paths() {
    let path = PathBuf::from("C:\\Users\\test\\syncpack.config.cjs");
    let script = build_nodejs_script(&path);
    assert!(
      script.contains("C:\\\\Users\\\\test\\\\syncpack.config.cjs"),
      "backslashes in Windows paths should be escaped for the JS string"
    );
  }

  #[test]
  fn script_uses_raw_path_for_require() {
    let path = PathBuf::from("/some/path/syncpack.config.cjs");
    let script = build_nodejs_script(&path);
    assert!(
      script.contains("require('/some/path/syncpack.config.cjs')"),
      "require() should use the raw path since it handles OS paths natively"
    );
  }

  #[test]
  fn script_escapes_single_quotes_in_paths() {
    let path = PathBuf::from("/user's projects/syncpack.config.cjs");
    let script = build_nodejs_script(&path);
    assert!(
      script.contains("/user\\'s projects/syncpack.config.cjs"),
      "single quotes in paths should be escaped to avoid JS syntax errors"
    );
    assert!(
      !script.contains("/user's projects/"),
      "unescaped single quote should not appear in the script"
    );
  }

  #[test]
  fn loads_cjs_config_from_fixture() {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("fixtures/fluid-framework/syncpack.config.cjs")
      .canonicalize()
      .expect("fixture should exist");
    let result = from_javascript_path(&fixture_path);
    assert!(result.is_ok(), "should load .cjs config: {result:?}");
  }
}
