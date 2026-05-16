use {
  crate::{dependency::Strategy, instance::Instance, specifier::Specifier},
  detect_indent::detect_indent,
  detect_newline_style::LineEnding,
  serde_json::Value as JsonValue,
  std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
  },
  thiserror::Error,
  yaml_serde::{Mapping, Value as YamlValue},
};

/// A YAML file held in memory alongside its raw text and a queue of
/// pending edit operations. The dual model lets reads (`json_view`,
/// `pnpm_catalog_names`, etc.) hit the parsed `yaml_serde::Value` while
/// writes replay `patches` against the original `raw` text via the
/// `yamlpatch` crate so comments / blank lines / indent are preserved.
#[derive(Debug)]
pub struct YamlFile {
  pub filepath: PathBuf,
  pub formatting: DetectedFormatting,
  pub contents: yaml_serde::Value,
  /// Original on-disk text. Empty for auto-created files.
  pub raw: String,
  /// Edit operations recorded since load (or since last write).
  pub patches: Vec<PendingYamlOp>,
  /// `true` once `contents` has been mutated and not yet written to disk.
  pub dirty: bool,
}

impl YamlFile {
  pub fn is_dirty(&self) -> bool {
    self.dirty
  }

  pub fn mark_dirty(&mut self) {
    self.dirty = true;
  }
}

/// A single recorded edit. `segments` is the route to the target
/// expressed as owned strings — `yamlpath::Route`s are built lazily at
/// write time.
#[derive(Debug, Clone)]
pub enum PendingYamlOp {
  /// Replace the value at `segments`.
  Replace { segments: Vec<String>, value: yaml_serde::Value },
  /// Add `key: value` at the mapping referenced by `segments`. Empty
  /// `segments` targets the document root.
  Add {
    segments: Vec<String>,
    key: String,
    value: yaml_serde::Value,
  },
  /// Remove the entry at `segments`.
  Remove { segments: Vec<String> },
}

#[cfg(test)]
#[path = "disk_test.rs"]
mod disk_test;

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
  YamlParse(#[source] yaml_serde::Error),
  #[error("Failed to serialise YAML:\n\n{0}")]
  YamlSerialize(#[source] yaml_serde::Error),
  #[error("Failed to parse YAML for format-preserving write:\n\n{0}")]
  YamlPatchParse(#[source] yamlpath::QueryError),
  #[error("Failed to apply YAML patch:\n\n{0}")]
  YamlPatchApply(#[source] yamlpatch::Error),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
  pub filepath: PathBuf,
  pub formatting: DetectedFormatting,
  pub contents: T,
  /// `true` once `contents` has been mutated and not yet written to disk.
  pub dirty: bool,
}

impl<T> File<T> {
  pub fn is_dirty(&self) -> bool {
    self.dirty
  }

  pub fn mark_dirty(&mut self) {
    self.dirty = true;
  }
}

/// Indent and newline style detected from a package.json file
#[derive(Clone, Debug)]
pub struct DetectedFormatting {
  /// e.g. `"  "`, `"    "`, `"\t"`.
  pub indent: String,
  /// e.g. `"\n"`, `"\r\n"`.
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
pub struct Disk {
  pub cwd: PathBuf,
  pub lerna_json: Option<File<serde_json::Value>>,
  /// Includes the root; `package_json_root_idx` points at it.
  pub package_json_files: Vec<File<serde_json::Value>>,
  pub package_json_root_idx: Option<usize>,
  pub package_manager: Option<PackageManager>,
  pub pnpm_workspace: Option<YamlFile>,
}

impl Disk {
  pub fn from_workspace<T: DiskIo>(io: &T, directory: &Path) -> Self {
    let lerna_json = io.read_json_file(&directory.join("lerna.json"));
    let package_json_root_file = io.read_json_file(&directory.join("package.json"));
    let package_manager = if io.path_exists(&directory.join("pnpm-lock.yaml")) || io.path_exists(&directory.join("pnpm-workspace.yaml")) {
      Some(PackageManager::Pnpm)
    } else if io.path_exists(&directory.join("yarn.lock")) {
      Some(PackageManager::Yarn)
    } else if io.path_exists(&directory.join("package-lock.json")) {
      Some(PackageManager::Npm)
    } else if io.path_exists(&directory.join("bun.lock")) || io.path_exists(&directory.join("bun.lockb")) {
      Some(PackageManager::Bun)
    } else {
      Some(PackageManager::Unknown)
    };
    let pnpm_workspace = if let Some(PackageManager::Pnpm) = package_manager {
      io.read_yaml_file(&directory.join("pnpm-workspace.yaml"))
    } else {
      None
    };

    let mut package_json_files = Vec::new();
    let mut package_json_root_idx = None;
    if let Some(Ok(root_file)) = package_json_root_file {
      package_json_root_idx = Some(package_json_files.len());
      package_json_files.push(root_file);
    }

    Self {
      cwd: directory.to_path_buf(),
      lerna_json: lerna_json.and_then(Result::ok),
      package_json_files,
      package_json_root_idx,
      package_manager,
      pnpm_workspace: pnpm_workspace.and_then(Result::ok),
    }
  }

  /// Borrow the root package.json if one was loaded.
  pub fn package_json_root(&self) -> Option<&File<serde_json::Value>> {
    self.package_json_root_idx.and_then(|i| self.package_json_files.get(i))
  }

  /// Default formatting fallback used when a file's own formatting is empty.
  /// Prefers the root pkg.json's formatting; falls back to defaults.
  pub fn formatting_fallback(&self) -> DetectedFormatting {
    self.package_json_root().map(|f| f.formatting.clone()).unwrap_or_default()
  }

  /// Read every package.json named in `paths`. Skips a path that equals the
  /// root's filepath so the root is not parsed twice. Successfully parsed
  /// files are appended to `package_json_files`. Failures are logged and
  /// skipped.
  pub fn load_package_files<T: DiskIo>(&mut self, io: &T, paths: &[PathBuf]) {
    let root_filepath = self.package_json_root().map(|f| f.filepath.clone());
    for path in paths {
      if root_filepath.as_ref() == Some(path) {
        continue;
      }
      match io.read_textfile(path) {
        Some(Ok(file)) => {
          if let Some(parsed) = parse_json_file(file.contents, file.filepath) {
            self.package_json_files.push(parsed);
          }
        }
        Some(Err(err)) => {
          log::error!("{err}");
        }
        None => {
          log::error!("package.json not readable at {}", path.to_str().unwrap_or("unknown"));
        }
      }
    }
  }
}

/// Parse a package.json from a raw JSON string and a file path. Returns
/// `None` if the JSON is invalid; the error is logged.
pub fn parse_json_file(raw: String, filepath: PathBuf) -> Option<File<serde_json::Value>> {
  match serde_json::from_str::<serde_json::Value>(&raw) {
    Ok(contents) => Some(File {
      filepath,
      formatting: detect_formatting(&raw),
      contents,
      dirty: false,
    }),
    Err(err) => {
      log::error!("Invalid JSON at {}: {err}", filepath.to_str().unwrap_or("unknown"));
      None
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
  /// Read a file from disk as raw bytes. Used by callers (e.g. the
  /// registry cache) that don't need formatting detection or the
  /// `File<V>` envelope. `None` when the file does not exist.
  fn read_bytes(&self, filepath: &Path) -> Option<Result<Vec<u8>, DiskIoError>>;
  /// Read a file from disk to a string
  fn read_textfile(&self, filepath: &Path) -> Option<Result<File<String>, DiskIoError>>;
  /// Read a YAML file from disk
  fn read_yaml_file(&self, filepath: &Path) -> Option<Result<YamlFile, DiskIoError>>;
  /// Read a YAML file as a typed value. Used for read-only typed
  /// configs (e.g. `.syncpackrc.yaml`). Distinct from `read_yaml_file`
  /// which carries the raw text + patch queue used for format-preserving
  /// writes — typed reads do not need that machinery.
  fn read_yaml_typed<V: serde::de::DeserializeOwned>(&self, filepath: &Path) -> Option<Result<File<V>, DiskIoError>>;
  /// Write raw bytes to disk, creating any missing parent directories.
  /// Used by callers that don't need the `File<V>` envelope.
  fn write_bytes(&self, filepath: &Path, bytes: &[u8]) -> Result<(), DiskIoError>;
  /// Write a JSON file to disk
  fn write_json_file<V: serde::ser::Serialize>(&self, file: &File<V>) -> Result<(), DiskIoError>;
  /// Write a YAML file to disk. Format-preserving via `yamlpatch` when
  /// `file.raw` is non-empty and `file.patches` is non-empty; otherwise
  /// the in-memory `contents` is serialised fresh via `yaml_serde`.
  fn write_yaml_file(&self, file: &YamlFile) -> Result<(), DiskIoError>;
  /// Find every `package.json` under `root` that matches `patterns`,
  /// honouring `.gitignore` and skipping `node_modules`/`.git`. Patterns
  /// use Override semantics: bare globs include, `!`-prefixed globs
  /// exclude. `*` matches a single path segment; `**` spans separators.
  fn find_package_jsons(&self, root: &Path, patterns: &[String]) -> Vec<PathBuf>;
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

  fn read_bytes(&self, filepath: &Path) -> Option<Result<Vec<u8>, DiskIoError>> {
    Some(filepath)
      .filter(|f| self.path_exists(f))
      .map(|filepath| fs::read(filepath).map_err(DiskIoError::FileRead))
  }

  fn read_textfile(&self, filepath: &Path) -> Option<Result<File<String>, DiskIoError>> {
    Some(filepath).filter(|f| self.path_exists(f)).map(|filepath| {
      fs::read_to_string(filepath).map_err(DiskIoError::FileRead).map(|raw| File {
        filepath: filepath.to_path_buf(),
        formatting: detect_formatting(&raw),
        contents: raw,
        dirty: false,
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
            dirty: false,
          })
      })
    })
  }

  fn read_yaml_file(&self, filepath: &Path) -> Option<Result<YamlFile, DiskIoError>> {
    self
      .read_textfile(filepath)
      .map(|res| res.and_then(|file| parse_yaml_file_strict(file.contents, file.filepath)))
  }

  fn read_yaml_typed<V: serde::de::DeserializeOwned>(&self, filepath: &Path) -> Option<Result<File<V>, DiskIoError>> {
    self.read_textfile(filepath).map(|res| {
      res.and_then(|file| {
        yaml_serde::from_str::<V>(&file.contents)
          .map_err(DiskIoError::YamlParse)
          .map(|parsed| File {
            filepath: file.filepath,
            formatting: detect_formatting(&file.contents),
            contents: parsed,
            dirty: false,
          })
      })
    })
  }

  fn write_bytes(&self, filepath: &Path, bytes: &[u8]) -> Result<(), DiskIoError> {
    ensure_parent_dir(filepath)?;
    fs::write(filepath, bytes).map_err(DiskIoError::FileWrite)
  }

  fn write_json_file<V: serde::ser::Serialize>(&self, file: &File<V>) -> Result<(), DiskIoError> {
    let pretty_bytes = get_pretty_json_bytes(file)?;
    ensure_parent_dir(&file.filepath)?;
    fs::write(&file.filepath, &pretty_bytes).map_err(DiskIoError::FileWrite)
  }

  fn write_yaml_file(&self, file: &YamlFile) -> Result<(), DiskIoError> {
    let bytes = render_yaml_bytes(file)?;
    ensure_parent_dir(&file.filepath)?;
    fs::write(&file.filepath, &bytes).map_err(DiskIoError::FileWrite)
  }

  fn find_package_jsons(&self, root: &Path, patterns: &[String]) -> Vec<PathBuf> {
    let mut builder = ignore::overrides::OverrideBuilder::new(root);
    for pattern in patterns {
      if let Err(err) = builder.add(pattern) {
        log::debug!("Invalid source pattern '{pattern}': {err}");
      }
    }
    let overrides = builder.build().unwrap_or_else(|err| {
      log::debug!("Failed to build source pattern overrides: {err}");
      ignore::overrides::Override::empty()
    });

    ignore::WalkBuilder::new(root)
      .require_git(false)
      .overrides(overrides)
      .filter_entry(|entry| {
        let name = entry.file_name();
        name != "node_modules" && name != ".git"
      })
      .build()
      .filter_map(|result| match result {
        Ok(entry) => Some(entry),
        Err(err) => {
          log::debug!("Walk error: {err}");
          None
        }
      })
      .filter(|entry| entry.file_type().is_some_and(|t| t.is_file()) && entry.file_name().to_string_lossy().ends_with(".json"))
      .map(|entry| entry.into_path())
      .collect()
  }
}

fn ensure_parent_dir(filepath: &Path) -> Result<(), DiskIoError> {
  let Some(parent) = filepath.parent() else { return Ok(()) };
  if parent.as_os_str().is_empty() || parent.exists() {
    return Ok(());
  }
  fs::create_dir_all(parent).map_err(DiskIoError::FileWrite)
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

/// Render a `YamlFile` to bytes for writing.
///
/// Two paths:
///
/// - **Empty `raw` (auto-created file) OR no patches recorded**: serialize `contents` fresh via `yaml_serde::to_string`. Nothing exists to
///   preserve.
/// - **Non-empty `raw` AND patches recorded**: build a `yamlpath::Document` from the original text, replay each `PendingYamlOp` as a
///   `yamlpatch::Patch`, and return the resulting `.source()` text. Comments, blank lines, key order, and indent are preserved.
///
/// Errors from the format-preserving path surface as
/// `DiskIoError::YamlPatch{Parse,Apply}` so callers refuse the write
/// instead of silently degrading.
pub fn render_yaml_bytes(file: &YamlFile) -> Result<Vec<u8>, DiskIoError> {
  if file.raw.is_empty() || file.patches.is_empty() {
    let yaml = yaml_serde::to_string(&file.contents).map_err(DiskIoError::YamlSerialize)?;
    let mut bytes = yaml.into_bytes();
    bytes.extend(file.formatting.newline.as_bytes());
    return Ok(bytes);
  }
  let document = yamlpath::Document::new(file.raw.clone()).map_err(DiskIoError::YamlPatchParse)?;
  let patches: Vec<yamlpatch::Patch<'static>> = file.patches.iter().map(pending_op_to_patch).collect();
  let result = yamlpatch::apply_yaml_patches(&document, &patches).map_err(DiskIoError::YamlPatchApply)?;
  Ok(result.source().as_bytes().to_vec())
}

/// Convert a syncpack-side `PendingYamlOp` into a `yamlpatch::Patch`
/// using owned segment strings (so the resulting `Patch` does not
/// borrow from the source `PendingYamlOp`).
fn pending_op_to_patch(op: &PendingYamlOp) -> yamlpatch::Patch<'static> {
  match op {
    PendingYamlOp::Replace { segments, value } => yamlpatch::Patch {
      route: route_from_segments(segments),
      operation: yamlpatch::Op::Replace(value.clone()),
    },
    PendingYamlOp::Add { segments, key, value } => yamlpatch::Patch {
      route: route_from_segments(segments),
      operation: yamlpatch::Op::Add {
        key: yaml_quote_key(key),
        value: value.clone(),
      },
    },
    PendingYamlOp::Remove { segments } => yamlpatch::Patch {
      route: route_from_segments(segments),
      operation: yamlpatch::Op::Remove,
    },
  }
}

fn route_from_segments(segments: &[String]) -> yamlpath::Route<'static> {
  let components: Vec<yamlpath::Component<'static>> = segments.iter().cloned().map(yamlpath::Component::from).collect();
  yamlpath::Route::from(components)
}

/// Render a string as a YAML mapping key, adding quotes when the key
/// starts with a reserved indicator (`@`, `` ` ``) or contains
/// syntactically significant punctuation. yamlpatch's `Op::Add`
/// inserts the key verbatim into the output text, so any quoting must
/// be baked into the string we hand it. Defers to `yaml_serde` for the
/// quote decision so we match standard scalar emission rules.
fn yaml_quote_key(key: &str) -> String {
  let value = YamlValue::String(key.to_string());
  let serialised = yaml_serde::to_string(&value).expect("string serialisation cannot fail");
  serialised.trim_end().to_string()
}

/// Parse a `pnpm-workspace.yaml` from a raw string. Returns `None` if
/// the yaml fails to parse; the error is logged. The original `raw`
/// text is stored on the returned `YamlFile` so writes can preserve
/// formatting via `yamlpatch`.
pub fn parse_yaml_file(raw: String, filepath: PathBuf) -> Option<YamlFile> {
  match parse_yaml_file_strict(raw, filepath.clone()) {
    Ok(file) => Some(file),
    Err(err) => {
      log::error!("Invalid YAML at {}: {err}", filepath.to_str().unwrap_or("unknown"));
      None
    }
  }
}

/// Strict parse: returns `Err` on invalid YAML. Used by `LiveDiskIo`
/// where the I/O layer already wraps errors.
fn parse_yaml_file_strict(raw: String, filepath: PathBuf) -> Result<YamlFile, DiskIoError> {
  let formatting = detect_formatting(&raw);
  let contents = yaml_serde::from_str::<YamlValue>(&raw).map_err(DiskIoError::YamlParse)?;
  Ok(YamlFile {
    filepath,
    formatting,
    contents,
    raw,
    patches: Vec::new(),
    dirty: false,
  })
}

/// Construct an empty `pnpm-workspace.yaml` shell. Used by the fix-time
/// auto-create path when `policy: "catalog"` enforces a catalog but no
/// yaml exists. `dirty` defaults to `false`; the caller flips it on
/// first insert. `raw` is empty so writes go through the fresh-serialize
/// path (nothing to preserve).
pub fn empty_yaml_file(filepath: PathBuf) -> YamlFile {
  YamlFile {
    filepath,
    formatting: detect_formatting(""),
    contents: YamlValue::Mapping(Mapping::new()),
    raw: String::new(),
    patches: Vec::new(),
    dirty: false,
  }
}

/// Read the package's `name` property, falling back to `"NAME_IS_MISSING"`.
pub fn package_name(file: &File<JsonValue>) -> &str {
  file
    .contents
    .pointer("/name")
    .and_then(|name| name.as_str())
    .unwrap_or("NAME_IS_MISSING")
}

/// Does a property exist at the given JSON pointer?
pub fn has_prop(file: &File<JsonValue>, pointer: &str) -> bool {
  file.contents.pointer(pointer).is_some()
}

/// Read the JSON value at the given pointer (cloned, so callers don't hold
/// a borrow on the file).
pub fn get_prop(file: &File<JsonValue>, pointer: &str) -> Option<JsonValue> {
  file.contents.pointer(pointer).cloned()
}

/// Deeply set a property in the parsed JSON. Marks the file dirty only when
/// the value actually changed (order-aware comparison).
pub fn set_prop(file: &mut File<JsonValue>, pointer: &str, next_value: JsonValue) {
  if pointer == "/" {
    if json_values_differ(&file.contents, &next_value) {
      file.contents = next_value;
      file.dirty = true;
    }
  } else if let Some(value) = file.contents.pointer_mut(pointer)
    && json_values_differ(value, &next_value)
  {
    *value = next_value;
    file.dirty = true;
  }
}

/// Set a key on a parent object. Uses direct Map access so keys may contain
/// `/` (eg. scoped npm packages like `@scope/name`). Marks dirty only when
/// the value actually changed.
pub fn set_nested_prop(file: &mut File<JsonValue>, parent_pointer: &str, key: &str, next_value: JsonValue) {
  if let Some(JsonValue::Object(obj)) = file.contents.pointer_mut(parent_pointer) {
    let is_changed = obj.get(key).is_none_or(|value| json_values_differ(value, &next_value));
    if is_changed {
      obj.insert(key.to_string(), next_value);
      file.dirty = true;
    }
  }
}

/// Remove a key from a parent object. Marks dirty only when the key existed.
pub fn remove_prop(file: &mut File<JsonValue>, parent_pointer: &str, key: &str) {
  if let Some(JsonValue::Object(obj)) = file.contents.pointer_mut(parent_pointer)
    && obj.remove(key).is_some()
  {
    file.dirty = true;
  }
}

/// Make sure every segment of `pointer` exists as a `Value::Object`. Walks
/// one segment at a time, creating empty objects en route. The empty-string
/// parent (`""`) resolves to the document root per RFC 6901.
pub fn ensure_object_path(file: &mut File<JsonValue>, pointer: &str) {
  let segments: Vec<&str> = pointer.split('/').filter(|s| !s.is_empty()).collect();
  let mut current = String::new();
  for segment in segments {
    let parent = current.clone();
    current.push('/');
    current.push_str(segment);
    if !matches!(file.contents.pointer(&current), Some(JsonValue::Object(_))) {
      set_nested_prop(file, &parent, segment, JsonValue::Object(Default::default()));
    }
  }
}

/// Apply an instance's `expected_specifier` to the package.json. Dispatches
/// over the dependency type's `Strategy`. Marks the file dirty only when the
/// stored value actually changed.
pub fn copy_expected_specifier_json(file: &mut File<JsonValue>, instance: &Instance) {
  let path_to_prop_str = &instance.descriptor.dependency_type.path.as_str();
  let raw_specifier = instance.expected_specifier.borrow().as_ref().unwrap().get_raw().to_string();
  match instance.descriptor.dependency_type.strategy {
    Strategy::NameAndVersionProps => {
      set_prop(file, path_to_prop_str, JsonValue::String(raw_specifier));
    }
    Strategy::NamedVersionString => {
      let full_value = format!("{}@{}", instance.descriptor.name, raw_specifier);
      set_prop(file, path_to_prop_str, JsonValue::String(full_value));
    }
    Strategy::UnnamedVersionString => {
      set_prop(file, path_to_prop_str, JsonValue::String(raw_specifier));
    }
    Strategy::VersionsByName => {
      set_nested_prop(file, path_to_prop_str, &instance.descriptor.name, JsonValue::String(raw_specifier));
    }
    Strategy::InvalidConfig => {
      unreachable!("unrecognised strategy");
    }
  };
}

/// Eagerly convert the parsed yaml into an owned `serde_json::Value`. No
/// caching — every call allocates a fresh Value. Caller is expected to
/// materialise once per pass and reuse via `.as_ref()`.
pub fn json_view(file: &YamlFile) -> JsonValue {
  yaml_to_json(&file.contents)
}

/// Insert `dep_name → specifier` under the named catalog block. Idempotent —
/// if the value already matches, no mutation occurs and `dirty` stays as it
/// was. Returns `true` when in-memory state changed.
///
/// `catalog_name == "default"` writes under the bare `catalog:` block; any
/// other name writes under `catalogs.{name}`. Each non-idempotent call also
/// pushes a `PendingYamlOp` describing the edit so the write path can replay
/// it via `yamlpatch` and preserve formatting.
pub fn insert_catalog_definition(file: &mut YamlFile, catalog_name: &str, dep_name: &str, specifier: &Rc<Specifier>) -> bool {
  let raw = specifier.get_raw().to_string();
  let next_value = YamlValue::String(raw);
  // Choose patch shape from PRE-mutation state. The in-memory `contents`
  // is mutated below via `ensure_catalog_block`/`insert`.
  let pending = build_insert_patch(file, catalog_name, dep_name, &next_value);
  let block = ensure_catalog_block(file, catalog_name);
  let current = block.get(dep_name);
  if current == Some(&next_value) {
    return false;
  }
  block.insert(YamlValue::String(dep_name.to_string()), next_value);
  if let Some(op) = pending {
    file.patches.push(op);
  }
  file.dirty = true;
  true
}

/// Determine the right `PendingYamlOp` for an insert based on the
/// pre-mutation state of `file.contents`. Returns `None` when the
/// existing value already equals `next_value` (idempotent case).
fn build_insert_patch(file: &YamlFile, catalog_name: &str, dep_name: &str, next_value: &YamlValue) -> Option<PendingYamlOp> {
  let root = match &file.contents {
    YamlValue::Mapping(map) => Some(map),
    _ => None,
  };
  if catalog_name == "default" {
    let existing_block = root.and_then(|r| r.get("catalog")).and_then(|v| v.as_mapping());
    let existing_value = existing_block.and_then(|b| b.get(dep_name));
    if existing_value == Some(next_value) {
      return None;
    }
    if existing_value.is_some() {
      return Some(PendingYamlOp::Replace {
        segments: vec!["catalog".to_string(), dep_name.to_string()],
        value: next_value.clone(),
      });
    }
    if existing_block.is_some() {
      return Some(PendingYamlOp::Add {
        segments: vec!["catalog".to_string()],
        key: dep_name.to_string(),
        value: next_value.clone(),
      });
    }
    let mut nested = Mapping::new();
    nested.insert(YamlValue::String(dep_name.to_string()), next_value.clone());
    Some(PendingYamlOp::Add {
      segments: Vec::new(),
      key: "catalog".to_string(),
      value: YamlValue::Mapping(nested),
    })
  } else {
    let catalogs = root.and_then(|r| r.get("catalogs")).and_then(|v| v.as_mapping());
    let existing_named = catalogs.and_then(|c| c.get(catalog_name)).and_then(|v| v.as_mapping());
    let existing_value = existing_named.and_then(|n| n.get(dep_name));
    if existing_value == Some(next_value) {
      return None;
    }
    if existing_value.is_some() {
      return Some(PendingYamlOp::Replace {
        segments: vec!["catalogs".to_string(), catalog_name.to_string(), dep_name.to_string()],
        value: next_value.clone(),
      });
    }
    if existing_named.is_some() {
      return Some(PendingYamlOp::Add {
        segments: vec!["catalogs".to_string(), catalog_name.to_string()],
        key: dep_name.to_string(),
        value: next_value.clone(),
      });
    }
    let mut named_map = Mapping::new();
    named_map.insert(YamlValue::String(dep_name.to_string()), next_value.clone());
    if catalogs.is_some() {
      return Some(PendingYamlOp::Add {
        segments: vec!["catalogs".to_string()],
        key: catalog_name.to_string(),
        value: YamlValue::Mapping(named_map),
      });
    }
    let mut catalogs_map = Mapping::new();
    catalogs_map.insert(YamlValue::String(catalog_name.to_string()), YamlValue::Mapping(named_map));
    Some(PendingYamlOp::Add {
      segments: Vec::new(),
      key: "catalogs".to_string(),
      value: YamlValue::Mapping(catalogs_map),
    })
  }
}

/// Remove `dep_name` from the named catalog block. If removal empties the
/// block, the block (and `catalogs` parent map when emptied of all named
/// catalogs) is also pruned. Returns `true` when anything was removed.
///
/// Each remove also pushes one or more `PendingYamlOp::Remove` entries:
/// the dep first, then any parent that became empty as a result. This
/// keeps the `yamlpatch` replay aligned with the in-memory state.
pub fn remove_catalog_definition(file: &mut YamlFile, catalog_name: &str, dep_name: &str) -> bool {
  let key_dep = YamlValue::String(dep_name.to_string());
  let key_name = YamlValue::String(catalog_name.to_string());
  let YamlValue::Mapping(ref mut root) = file.contents else {
    return false;
  };
  let mut new_patches: Vec<PendingYamlOp> = Vec::new();
  let removed = if catalog_name == "default" {
    let block = root.get_mut("catalog").and_then(|v| v.as_mapping_mut());
    let Some(block) = block else { return false };
    let removed = block.remove(&key_dep).is_some();
    if removed {
      new_patches.push(PendingYamlOp::Remove {
        segments: vec!["catalog".to_string(), dep_name.to_string()],
      });
    }
    if removed && block.is_empty() {
      root.remove("catalog");
      new_patches.push(PendingYamlOp::Remove {
        segments: vec!["catalog".to_string()],
      });
    }
    removed
  } else {
    let catalogs = root.get_mut("catalogs").and_then(|v| v.as_mapping_mut());
    let Some(catalogs) = catalogs else { return false };
    let block = catalogs.get_mut(&key_name).and_then(|v| v.as_mapping_mut());
    let Some(block) = block else { return false };
    let removed = block.remove(&key_dep).is_some();
    if removed {
      new_patches.push(PendingYamlOp::Remove {
        segments: vec!["catalogs".to_string(), catalog_name.to_string(), dep_name.to_string()],
      });
    }
    if removed && block.is_empty() {
      catalogs.remove(&key_name);
      new_patches.push(PendingYamlOp::Remove {
        segments: vec!["catalogs".to_string(), catalog_name.to_string()],
      });
    }
    if catalogs.is_empty() {
      root.remove("catalogs");
      new_patches.push(PendingYamlOp::Remove {
        segments: vec!["catalogs".to_string()],
      });
    }
    removed
  };
  if removed {
    file.patches.extend(new_patches);
    file.dirty = true;
  }
  removed
}

/// Apply a consumer instance's `expected_specifier` to the yaml via the
/// catalog-definition route. No-op when the instance is not a catalog
/// instance or has no expected specifier.
pub fn copy_expected_specifier_yaml(file: &mut YamlFile, instance: &Instance) {
  let Some(catalog_name) = instance.catalog_name() else {
    return;
  };
  let Some(expected) = instance.expected_specifier.borrow().clone() else {
    return;
  };
  let dep_name = instance.descriptor.name.clone();
  insert_catalog_definition(file, catalog_name, &dep_name, &expected);
}

/// Persist a JSON file when dirty. Returns `Ok(true)` on actual write,
/// `Ok(false)` when no-op. Resets `dirty = false` post-write.
pub fn write_json_file<D: DiskIo>(
  file: &mut File<JsonValue>,
  io: &D,
  indent_override: Option<&str>,
  formatting_fallback: &DetectedFormatting,
) -> Result<bool, DiskIoError> {
  if !file.dirty {
    return Ok(false);
  }
  let effective_formatting = match indent_override {
    Some(indent) => DetectedFormatting {
      indent: indent.to_string(),
      newline: file.formatting.newline.clone(),
    },
    None if file.formatting.indent.is_empty() => formatting_fallback.clone(),
    None => file.formatting.clone(),
  };
  let snapshot = File {
    filepath: file.filepath.clone(),
    formatting: effective_formatting,
    contents: &file.contents,
    dirty: false,
  };
  io.write_json_file(&snapshot)?;
  file.dirty = false;
  Ok(true)
}

/// Persist a YAML file when dirty. Returns `Ok(true)` on actual write,
/// `Ok(false)` when no-op. Resets `dirty = false` post-write.
///
/// `indent_override` only takes effect when `file.raw` is empty — i.e.
/// when the file is being serialised fresh because there's no original
/// text to preserve. For existing files the on-disk indent is kept by
/// `yamlpatch`.
pub fn write_yaml_file<D: DiskIo>(
  file: &mut YamlFile,
  io: &D,
  indent_override: Option<&str>,
  formatting_fallback: &DetectedFormatting,
) -> Result<bool, DiskIoError> {
  if !file.dirty {
    return Ok(false);
  }
  if file.raw.is_empty() {
    let effective_formatting = match indent_override {
      Some(indent) => DetectedFormatting {
        indent: indent.to_string(),
        newline: file.formatting.newline.clone(),
      },
      None if file.formatting.indent.is_empty() => formatting_fallback.clone(),
      None => file.formatting.clone(),
    };
    file.formatting = effective_formatting;
  }
  io.write_yaml_file(file)?;
  file.dirty = false;
  file.patches.clear();
  Ok(true)
}

/// Order-aware comparison of two JSON values. Unlike `JsonValue::eq`, this
/// treats objects with different key order as different, matching
/// serialisation behaviour.
fn json_values_differ(a: &JsonValue, b: &JsonValue) -> bool {
  match (a, b) {
    (JsonValue::Object(a), JsonValue::Object(b)) => {
      a.len() != b.len()
        || a
          .iter()
          .zip(b.iter())
          .any(|((k1, v1), (k2, v2))| k1 != k2 || json_values_differ(v1, v2))
    }
    (JsonValue::Array(a), JsonValue::Array(b)) => a.len() != b.len() || a.iter().zip(b.iter()).any(|(a, b)| json_values_differ(a, b)),
    _ => a != b,
  }
}

/// Get-or-create the catalog block (top-level `catalog` for default, or a
/// nested entry under `catalogs.{name}` for named catalogs) as a mapping.
fn ensure_catalog_block<'a>(file: &'a mut YamlFile, catalog_name: &str) -> &'a mut Mapping {
  // Promote root to a Mapping if it was Null (freshly created file).
  if !matches!(file.contents, YamlValue::Mapping(_)) {
    file.contents = YamlValue::Mapping(Mapping::new());
  }
  let YamlValue::Mapping(ref mut root) = file.contents else {
    unreachable!("just promoted to Mapping");
  };
  if catalog_name == "default" {
    let key = YamlValue::String("catalog".to_string());
    if !matches!(root.get(&key), Some(YamlValue::Mapping(_))) {
      root.insert(key.clone(), YamlValue::Mapping(Mapping::new()));
    }
    let YamlValue::Mapping(block) = root.get_mut(&key).expect("just inserted") else {
      unreachable!();
    };
    block
  } else {
    let catalogs_key = YamlValue::String("catalogs".to_string());
    if !matches!(root.get(&catalogs_key), Some(YamlValue::Mapping(_))) {
      root.insert(catalogs_key.clone(), YamlValue::Mapping(Mapping::new()));
    }
    let YamlValue::Mapping(catalogs) = root.get_mut(&catalogs_key).expect("just inserted") else {
      unreachable!();
    };
    let name_key = YamlValue::String(catalog_name.to_string());
    if !matches!(catalogs.get(&name_key), Some(YamlValue::Mapping(_))) {
      catalogs.insert(name_key.clone(), YamlValue::Mapping(Mapping::new()));
    }
    let YamlValue::Mapping(block) = catalogs.get_mut(&name_key).expect("just inserted") else {
      unreachable!();
    };
    block
  }
}

/// Convert a `yaml_serde::Value` into a `serde_json::Value` for JSON pointer
/// access. Hand-rolled to avoid any reliance on the yaml crate's `Serialize`
/// quirks. Yaml mappings with non-string keys produce `null` for that entry.
pub(crate) fn yaml_to_json(yaml: &YamlValue) -> JsonValue {
  match yaml {
    YamlValue::Null => JsonValue::Null,
    YamlValue::Bool(b) => JsonValue::Bool(*b),
    YamlValue::Number(n) => {
      if let Some(i) = n.as_i64() {
        JsonValue::Number(i.into())
      } else if let Some(u) = n.as_u64() {
        JsonValue::Number(u.into())
      } else if let Some(f) = n.as_f64() {
        serde_json::Number::from_f64(f).map(JsonValue::Number).unwrap_or(JsonValue::Null)
      } else {
        JsonValue::Null
      }
    }
    YamlValue::String(s) => JsonValue::String(s.clone()),
    YamlValue::Sequence(seq) => JsonValue::Array(seq.iter().map(yaml_to_json).collect()),
    YamlValue::Mapping(map) => {
      let mut out = serde_json::Map::with_capacity(map.len());
      for (k, v) in map {
        let key = match k {
          YamlValue::String(s) => s.clone(),
          YamlValue::Bool(b) => b.to_string(),
          YamlValue::Number(n) => n.to_string(),
          _ => continue,
        };
        out.insert(key, yaml_to_json(v));
      }
      JsonValue::Object(out)
    }
    YamlValue::Tagged(tagged) => yaml_to_json(&tagged.value),
  }
}
