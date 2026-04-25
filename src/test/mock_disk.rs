use {
  crate::disk::{detect_formatting, DiskDirEntry, DiskIo, DiskIoError, File, NodeJsError},
  std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
  },
};

/// In-memory filesystem for testing
pub struct MockDiskIo {
  root: PathBuf,
  files: HashMap<PathBuf, String>,
}

impl MockDiskIo {
  pub fn new() -> Self {
    Self {
      root: std::env::current_dir().unwrap(),
      files: HashMap::new(),
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

  fn has_children(&self, dir: &Path) -> bool {
    self.files.keys().any(|f| f.starts_with(dir) && f != dir)
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
      if let Ok(relative) = file_path.strip_prefix(path) {
        if let Some(first_component) = relative.iter().next() {
          let child_path = path.join(first_component);
          if seen.insert(child_path.clone()) {
            let is_dir = relative.iter().count() > 1;
            entries.push(DiskDirEntry::new(child_path, is_dir));
          }
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
      })
    })
  }

  fn read_textfile(&self, filepath: &Path) -> Option<Result<File<String>, DiskIoError>> {
    self.files.get(filepath).map(|raw| {
      Ok(File {
        filepath: filepath.to_path_buf(),
        formatting: detect_formatting(raw),
        contents: raw.clone(),
      })
    })
  }

  fn read_yaml_file<V: serde::de::DeserializeOwned>(&self, filepath: &Path) -> Option<Result<File<V>, DiskIoError>> {
    self.files.get(filepath).map(|raw| {
      serde_yaml::from_str::<V>(raw).map_err(DiskIoError::YamlParse).map(|contents| File {
        filepath: filepath.to_path_buf(),
        formatting: detect_formatting(raw),
        contents,
      })
    })
  }

  fn write_json_file<V: serde::ser::Serialize>(&self, _file: &File<V>) -> Result<(), DiskIoError> {
    Ok(())
  }

  fn write_yaml_file<V: serde::ser::Serialize>(&self, _file: File<V>) -> Result<(), DiskIoError> {
    Ok(())
  }
}
