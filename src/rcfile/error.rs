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
  #[error("Failed to read config file: {0}")]
  FileReadFailed(#[from] std::io::Error),
  #[error("Failed to run node to retrieve JS/TS config file: {0}")]
  NodeJsExecutionFailed(#[source] std::io::Error),
  #[error("Executing a JavaScript config file failed with stderr: {stderr}")]
  ProcessFailed { stderr: String },
  #[error("Config file contains invalid UTF-8: {0}")]
  InvalidUtf8(#[from] std::string::FromUtf8Error),
  #[error("Config file failed validation: {0}")]
  InvalidConfig(#[from] serde_json::Error),
  #[error("Failed to import or require config file: {import_error} {require_error}")]
  JavaScriptImportFailed { import_error: String, require_error: String },
  #[error("Failed to parse JSON in config file: {0}")]
  JsonParseFailed(#[source] serde_json::Error),
  #[error("Failed to parse YAML in config file: {0}")]
  YamlParseFailed(#[from] serde_yaml::Error),
  #[error("Config defined as a property in package.json failed validation: {0}")]
  PackageJsonConfigInvalid(#[source] serde_json::Error),
}
