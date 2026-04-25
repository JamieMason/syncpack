use {
  detect_indent::detect_indent,
  detect_newline_style::LineEnding,
  std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
  },
  thiserror::Error,
};

#[derive(Debug, Error)]
pub enum NodeJsError {
  #[error("Node.js v22.6.0 or higher is needed for a TypeScript config file, try a JavaScript or JSON config file")]
  CannotStripTypes,
  #[error("Failed to run Node.js:\n\n{0}")]
  ExecutionFailed(#[source] std::io::Error),
  #[error("Node.js stdout contains invalid UTF-8:\n\n{0}")]
  InvalidUtf8(#[from] std::string::FromUtf8Error),
  #[error("Executing Node.js failed with stderr:\n\n{0}")]
  ProcessFailed(String),
}

#[derive(Debug, Error)]
pub enum DiskIoError {
  #[error("Failed to read file:\n\n{0}")]
  FileRead(#[source] std::io::Error),
  #[error("Failed to write file:\n\n{0}")]
  FileWrite(#[source] std::io::Error),
  #[error("Failed to parse JSON:\n\n{0}")]
  JsonParse(#[source] serde_json::Error),
  #[error("Failed to serialise JSON:\n\n{0}")]
  JsonSerialize(#[source] serde_json::Error),
  #[error("Failed to parse YAML:\n\n{0}")]
  YamlParse(#[source] serde_yaml::Error),
  #[error("Failed to serialise YAML:\n\n{0}")]
  YamlSerialize(#[source] serde_yaml::Error),
}

#[derive(Debug)]
pub enum PackageManager {
  Bun,
  Npm,
  Pnpm,
  /// We have looked but could not determine the package manager
  Unknown,
  Yarn,
}

#[derive(Debug)]
pub struct File<T> {
  /// Absolute path to the file on disk
  pub filepath: PathBuf,
  /// Detected indentation and newline style
  pub formatting: DetectedFormatting,
  /// Parsed contents of the file
  pub contents: T,
}

/// Indent and newline style detected from a package.json file
#[derive(Clone, Debug)]
pub struct DetectedFormatting {
  /// Indentation detected from the file's raw content (e.g. "  ", "    ", "\t")
  pub indent: String,
  /// Newline style detected from the file's raw content (e.g. "\n", "\r\n")
  pub newline: String,
}

impl Default for DetectedFormatting {
  fn default() -> Self {
    Self {
      indent: "  ".to_string(),
      newline: "\n".to_string(),
    }
  }
}

/// Detect indent and newline style from a raw JSON string
pub fn detect_formatting(raw: &str) -> DetectedFormatting {
  let indent = detect_indent(raw).indent().to_string();
  let indent = if indent.is_empty() { "  ".to_string() } else { indent };
  let newline = match LineEnding::find_or_use_lf(raw) {
    LineEnding::CRLF => "\r\n".to_string(),
    LineEnding::CR => "\r".to_string(),
    LineEnding::LF => "\n".to_string(),
  };
  DetectedFormatting { indent, newline }
}

/// The owner of all writable files in the workspace
#[derive(Debug)]
pub struct Disk<'a, T: DiskIo> {
  /// The root directory of the workspace
  pub cwd: PathBuf,
  /// The underlying disk IO implementation
  pub io: &'a T,
  /// The lerna.json file, if found
  pub lerna_json: Option<File<serde_json::Value>>,
  /// All package.json files found in the workspace except the root package.json
  pub package_json_files: Vec<File<serde_json::Value>>,
  /// The root package.json file, if found
  pub package_json_root: Option<File<serde_json::Value>>,
  /// The package manager used in the workspace, if knowable
  pub package_manager: Option<PackageManager>,
  /// The pnpm-workspace.yaml file, if found
  pub pnpm_workspace: Option<File<serde_yaml::Value>>,
}

impl<'a, T: DiskIo> Disk<'a, T> {
  pub fn from_workspace(io: &'a T, directory: &Path) -> Self {
    let package_json_files = vec![];
    let lerna_json = io.read_json_file(&directory.join("lerna.json"));
    let package_json_root = io.read_json_file(&directory.join("package.json"));
    let package_manager = if io.path_exists(&directory.join("pnpm-lock.yaml")) {
      Some(PackageManager::Pnpm)
    } else if io.path_exists(&directory.join("yarn.lock")) {
      Some(PackageManager::Yarn)
    } else if io.path_exists(&directory.join("package-lock.json")) {
      Some(PackageManager::Npm)
    } else if io.path_exists(&directory.join("bun.lock")) {
      Some(PackageManager::Bun)
    } else {
      Some(PackageManager::Unknown)
    };
    let pnpm_workspace = if let Some(PackageManager::Pnpm) = package_manager {
      io.read_yaml_file(&directory.join("pnpm-workspace.yaml"))
    } else {
      None
    };

    Self {
      cwd: directory.to_path_buf(),
      io,
      lerna_json: lerna_json.and_then(Result::ok),
      package_json_files,
      package_json_root: package_json_root.and_then(Result::ok),
      package_manager,
      pnpm_workspace: pnpm_workspace.and_then(Result::ok),
    }
  }
}

/// Abstract directory entry returned by DiskIo::read_dir
pub struct DiskDirEntry {
  path: PathBuf,
  is_dir: bool,
}

impl DiskDirEntry {
  pub fn new(path: PathBuf, is_dir: bool) -> Self {
    Self { path, is_dir }
  }

  pub fn path(&self) -> &Path {
    &self.path
  }

  pub fn file_name(&self) -> std::ffi::OsString {
    self.path.file_name().unwrap_or_default().to_owned()
  }

  pub fn is_dir(&self) -> bool {
    self.is_dir
  }
}

/// Allow DI of the disk IO layer
pub trait DiskIo {
  /// Execute a NodeJS command and return the stdout
  fn exec_node_command(&self, current_dir: &Path, args: &[&str]) -> Result<String, NodeJsError>;
  /// Check if a file exists on disk
  fn path_exists(&self, filepath: &Path) -> bool;
  /// Returns the entries within a directory.
  fn read_dir(&self, path: &Path) -> Result<Vec<DiskDirEntry>, std::io::Error>;
  /// Read a JSON file from disk
  fn read_json_file<V: serde::de::DeserializeOwned>(&self, filepath: &Path) -> Option<Result<File<V>, DiskIoError>>;
  /// Read a file from disk to a string
  fn read_textfile(&self, filepath: &Path) -> Option<Result<File<String>, DiskIoError>>;
  /// Read a YAML file from disk
  fn read_yaml_file<V: serde::de::DeserializeOwned>(&self, filepath: &Path) -> Option<Result<File<V>, DiskIoError>>;
  /// Write a JSON file to disk
  fn write_json_file<V: serde::ser::Serialize>(&self, file: &File<V>) -> Result<(), DiskIoError>;
  /// Write a YAML file to disk
  fn write_yaml_file<V: serde::ser::Serialize>(&self, file: File<V>) -> Result<(), DiskIoError>;
}

#[derive(Debug)]
pub struct LiveDiskIo {}

impl LiveDiskIo {
  pub fn new() -> Self {
    Self {}
  }
}

impl DiskIo for LiveDiskIo {
  fn exec_node_command(&self, current_dir: &Path, args: &[&str]) -> Result<String, NodeJsError> {
    Command::new("node")
      .args(args)
      .current_dir(current_dir)
      .output()
      .map_err(NodeJsError::ExecutionFailed)
      .and_then(|output| {
        if output.status.success() {
          String::from_utf8(output.stdout).map_err(NodeJsError::InvalidUtf8)
        } else {
          let stderr = String::from_utf8_lossy(&output.stderr).to_string();
          if stderr.contains("experimental-strip-types") {
            Err(NodeJsError::CannotStripTypes)
          } else {
            Err(NodeJsError::ProcessFailed(stderr))
          }
        }
      })
  }

  fn path_exists(&self, filepath: &Path) -> bool {
    filepath.exists()
  }

  fn read_dir(&self, path: &Path) -> Result<Vec<DiskDirEntry>, std::io::Error> {
    let entries = fs::read_dir(path)?;
    Ok(
      entries
        .flatten()
        .map(|entry| {
          let path = entry.path();
          let is_dir = path.is_dir();
          DiskDirEntry::new(path, is_dir)
        })
        .collect(),
    )
  }

  fn read_textfile(&self, filepath: &Path) -> Option<Result<File<String>, DiskIoError>> {
    Some(filepath).filter(|f| self.path_exists(f)).map(|filepath| {
      fs::read_to_string(filepath).map_err(DiskIoError::FileRead).map(|raw| File {
        filepath: filepath.to_path_buf(),
        formatting: detect_formatting(&raw),
        contents: raw,
      })
    })
  }

  fn read_json_file<V: serde::de::DeserializeOwned>(&self, filepath: &Path) -> Option<Result<File<V>, DiskIoError>> {
    self.read_textfile(filepath).map(|res| {
      res.and_then(|file| {
        serde_json::from_str::<V>(&file.contents)
          .map_err(DiskIoError::JsonParse)
          .map(|parsed| File {
            filepath: file.filepath,
            formatting: detect_formatting(&file.contents),
            contents: parsed,
          })
      })
    })
  }

  fn read_yaml_file<V: serde::de::DeserializeOwned>(&self, filepath: &Path) -> Option<Result<File<V>, DiskIoError>> {
    self.read_textfile(filepath).map(|res| {
      res.and_then(|file| {
        serde_yaml::from_str::<V>(&file.contents)
          .map_err(DiskIoError::YamlParse)
          .map(|parsed| File {
            filepath: file.filepath,
            formatting: detect_formatting(&file.contents),
            contents: parsed,
          })
      })
    })
  }

  fn write_json_file<V: serde::ser::Serialize>(&self, file: &File<V>) -> Result<(), DiskIoError> {
    let pretty_bytes = get_pretty_json_bytes(file)?;
    fs::write(&file.filepath, &pretty_bytes).map_err(DiskIoError::FileWrite)
  }

  fn write_yaml_file<V: serde::ser::Serialize>(&self, file: File<V>) -> Result<(), DiskIoError> {
    let pretty_bytes = get_pretty_yaml_bytes(&file)?;
    fs::write(&file.filepath, &pretty_bytes).map_err(DiskIoError::FileWrite)
  }
}

/// Serialize a JSON Value back into pretty JSON as bytes.
pub(crate) fn get_pretty_json_bytes<V: serde::ser::Serialize>(file: &File<V>) -> Result<Vec<u8>, DiskIoError> {
  let buffer = Vec::new();
  let indent = &file.formatting.indent.replace("\\t", "\t");
  let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
  let mut serializer = serde_json::ser::Serializer::with_formatter(buffer, formatter);
  file.contents.serialize(&mut serializer).map_err(DiskIoError::JsonSerialize)?;
  let mut bytes = serializer.into_inner();
  bytes.extend(file.formatting.newline.as_bytes());
  Ok(bytes)
}

/// Serialize a YAML Value back into pretty YAML as bytes.
fn get_pretty_yaml_bytes<V: serde::ser::Serialize>(file: &File<V>) -> Result<Vec<u8>, DiskIoError> {
  let yaml = serde_yaml::to_string(&file.contents).map_err(DiskIoError::YamlSerialize)?;
  let mut bytes = yaml.into_bytes();
  bytes.extend(file.formatting.newline.as_bytes());
  Ok(bytes)
}
