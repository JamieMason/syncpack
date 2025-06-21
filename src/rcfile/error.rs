use {serde::Deserialize, serde_json::Value};

#[derive(Debug, Deserialize)]
#[serde(tag = "_tag")]
pub enum NodeJsResult {
  #[serde(rename = "Ok")]
  Success { value: Value },
  #[serde(rename = "Err")]
  Error {
    #[serde(rename = "importError")]
    import_error: String,
    #[serde(rename = "requireError")]
    require_error: String,
  },
}

#[derive(Debug)]
pub enum ConfigError {
  // File operations
  FileReadFailed(std::io::Error),
  // JavaScript/TypeScript specific
  CommandExecutionFailed(std::io::Error),
  ProcessFailed { stderr: String },
  InvalidUtf8(std::string::FromUtf8Error),
  ConfigDeserializationFailed(serde_json::Error),
  ImportAndRequireFailed { import_error: String, require_error: String },
  // JSON specific
  JsonParseFailed(serde_json::Error),
  // YAML specific
  YamlParseFailed(serde_yaml::Error),
  // Package.json specific
  PackageJsonConfigInvalid(serde_json::Error),
}
