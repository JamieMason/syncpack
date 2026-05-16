use {
  crate::disk::{DiskDirEntry, DiskIo, DiskIoError, File, NodeJsError, YamlFile, detect_formatting},
  std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
  },
};

/// In-memory filesystem for testing.
///
/// Captures writes so tests can assert on serialised output. Reads
/// pull from the mock files map.
pub struct MockDiskIo {
  root: PathBuf,
  files: HashMap<PathBuf, String>,
  writes: RefCell<HashMap<PathBuf, Vec<u8>>>,
}

impl MockDiskIo {
  pub fn new() -> Self {
    Self {
      root: std::env::current_dir().unwrap(),
      files: HashMap::new(),
      writes: RefCell::new(HashMap::new()),
    }
  }

  pub fn root(&self) -> &Path {
    &self.root
  }

  /// Add a file to the mock filesystem. `relative_path` is relative to root.
  pub fn add_file(&mut self, relative_path: &str, contents: String) {
    let abs_path = self.root.join(relative_path);
    self.files.insert(abs_path, contents);
  }

  pub fn add_json<V: serde::Serialize>(&mut self, relative_path: &str, value: &V) {
    let raw = serde_json::to_string_pretty(value).unwrap();
    self.add_file(relative_path, raw);
  }

  /// Inspect captured write bytes for a path. `None` if nothing was
  /// written there.
  pub fn written_bytes(&self, path: &Path) -> Option<Vec<u8>> {
    self.writes.borrow().get(path).cloned()
  }

  /// Inspect captured write text for a path (UTF-8). `None` if nothing
  /// was written.
  pub fn written_text(&self, path: &Path) -> Option<String> {
    self
      .writes
      .borrow()
      .get(path)
      .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
  }

  fn has_children(&self, dir: &Path) -> bool {
    self.files.keys().any(|f| f.starts_with(dir) && f != dir)
  }

  fn record_write(&self, path: &Path, bytes: Vec<u8>) {
    self.writes.borrow_mut().insert(path.to_path_buf(), bytes);
  }
}

impl DiskIo for MockDiskIo {
  fn exec_node_command(&self, _current_dir: &Path, _args: &[&str]) -> Result<String, NodeJsError> {
    Err(NodeJsError::CannotStripTypes)
  }

  fn path_exists(&self, filepath: &Path) -> bool {
    self.files.contains_key(filepath) || self.has_children(filepath)
  }

  fn read_dir(&self, path: &Path) -> Result<Vec<DiskDirEntry>, std::io::Error> {
    let mut seen = HashSet::new();
    let mut entries = Vec::new();
    for file_path in self.files.keys() {
      if let Ok(relative) = file_path.strip_prefix(path)
        && let Some(first_component) = relative.iter().next()
      {
        let child_path = path.join(first_component);
        if seen.insert(child_path.clone()) {
          let is_dir = relative.iter().count() > 1;
          entries.push(DiskDirEntry::new(child_path, is_dir));
        }
      }
    }
    Ok(entries)
  }

  fn read_json_file<V: serde::de::DeserializeOwned>(&self, filepath: &Path) -> Option<Result<File<V>, DiskIoError>> {
    self.files.get(filepath).map(|raw| {
      serde_json::from_str::<V>(raw).map_err(DiskIoError::JsonParse).map(|contents| File {
        filepath: filepath.to_path_buf(),
        formatting: detect_formatting(raw),
        contents,
        dirty: false,
      })
    })
  }

  fn read_bytes(&self, filepath: &Path) -> Option<Result<Vec<u8>, DiskIoError>> {
    self.files.get(filepath).map(|raw| Ok(raw.as_bytes().to_vec()))
  }

  fn read_textfile(&self, filepath: &Path) -> Option<Result<File<String>, DiskIoError>> {
    self.files.get(filepath).map(|raw| {
      Ok(File {
        filepath: filepath.to_path_buf(),
        formatting: detect_formatting(raw),
        contents: raw.clone(),
        dirty: false,
      })
    })
  }

  fn read_yaml_file(&self, filepath: &Path) -> Option<Result<YamlFile, DiskIoError>> {
    self.files.get(filepath).map(|raw| {
      let formatting = detect_formatting(raw);
      serde_yaml::from_str::<serde_yaml::Value>(raw)
        .map_err(DiskIoError::YamlParse)
        .map(|contents| YamlFile {
          filepath: filepath.to_path_buf(),
          formatting,
          contents,
          raw: raw.clone(),
          patches: Vec::new(),
          dirty: false,
        })
    })
  }

  fn read_yaml_typed<V: serde::de::DeserializeOwned>(&self, filepath: &Path) -> Option<Result<File<V>, DiskIoError>> {
    self.files.get(filepath).map(|raw| {
      serde_yaml::from_str::<V>(raw).map_err(DiskIoError::YamlParse).map(|contents| File {
        filepath: filepath.to_path_buf(),
        formatting: detect_formatting(raw),
        contents,
        dirty: false,
      })
    })
  }

  fn write_bytes(&self, filepath: &Path, bytes: &[u8]) -> Result<(), DiskIoError> {
    self.record_write(filepath, bytes.to_vec());
    Ok(())
  }

  fn write_json_file<V: serde::ser::Serialize>(&self, file: &File<V>) -> Result<(), DiskIoError> {
    let buffer: Vec<u8> = Vec::new();
    let indent = file.formatting.indent.replace("\\t", "\t");
    let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
    let mut serializer = serde_json::ser::Serializer::with_formatter(buffer, formatter);
    file.contents.serialize(&mut serializer).map_err(DiskIoError::JsonSerialize)?;
    let mut bytes = serializer.into_inner();
    bytes.extend(file.formatting.newline.as_bytes());
    self.record_write(&file.filepath, bytes);
    Ok(())
  }

  fn write_yaml_file(&self, file: &YamlFile) -> Result<(), DiskIoError> {
    let bytes = crate::disk::render_yaml_bytes(file)?;
    self.record_write(&file.filepath, bytes);
    Ok(())
  }

  fn find_package_jsons(&self, root: &Path, patterns: &[String]) -> Vec<PathBuf> {
    let mut builder = ignore::overrides::OverrideBuilder::new(root);
    for pattern in patterns {
      let _ = builder.add(pattern);
    }
    let overrides = builder.build().unwrap_or_else(|_| ignore::overrides::Override::empty());

    self
      .files
      .keys()
      .filter(|path| path.file_name().is_some_and(|n| n == "package.json"))
      .filter(|path| {
        let rel = path.strip_prefix(root).unwrap_or(path);
        overrides.matched(rel, false).is_whitelist()
      })
      .cloned()
      .collect()
  }
}
