use {serde::Deserialize, std::path::Path};

/// The JSON returned to Rust from JavaScript when evaluating a config file
#[derive(Debug, Deserialize)]
#[serde(tag = "_tag")]
pub enum JsResult {
  #[serde(rename = "Ok")]
  Success { value: String },
  #[serde(rename = "Err")]
  Error {
    #[serde(rename = "importError")]
    import_error: String,
    #[serde(rename = "requireError")]
    require_error: String,
  },
}

pub fn get_javascript_contents(file_path: &Path) -> String {
  let escaped_file_path = file_path.to_string_lossy().replace('\\', "\\\\");
  format!(
    r#"
    import('{escaped_file_path}')
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
        .then(() => require('{escaped_file_path}'))
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
