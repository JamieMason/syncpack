use {serde::Deserialize, thiserror::Error};

#[derive(Debug, Deserialize)]
#[serde(tag = "_tag")]
pub enum NodeJsResult {
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

#[derive(Debug, Error)]
pub enum RcfileError {
  #[error("Failed to read config file")]
  FileReadFailed(#[from] std::io::Error),
  #[error("Failed to run Node.js/npx/tsx to retrieve JS/TS config file")]
  NodeJsExecutionFailed(#[source] std::io::Error),
  #[error("Node.js/npx/tsx process failed with stderr: {stderr}")]
  ProcessFailed { stderr: String },
  #[error("Config file contains invalid UTF-8")]
  InvalidUtf8(#[from] std::string::FromUtf8Error),
  #[error("Config file failed validation")]
  InvalidConfig(#[from] serde_json::Error),
  #[error("Failed to import or require config file: {import_error} {require_error}")]
  JavaScriptImportFailed { import_error: String, require_error: String },
  #[error("Failed to parse JSON in config file")]
  JsonParseFailed(#[source] serde_json::Error),
  #[error("Failed to parse YAML in config file")]
  YamlParseFailed(#[from] serde_yaml::Error),
  #[error("Config defined as a property in package.json failed validation")]
  PackageJsonConfigInvalid(#[source] serde_json::Error),
}
