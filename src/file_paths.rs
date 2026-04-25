use {
  crate::disk::{Disk, DiskIo},
  globset::{Glob, GlobSet, GlobSetBuilder},
  log::debug,
  std::{collections::VecDeque, path::PathBuf},
};

/// Resolve every source glob pattern into their absolute file paths of
/// package.json files
pub fn get_file_paths<T: DiskIo>(all_patterns: &[String], disk: &Disk<'_, T>) -> Vec<PathBuf> {
  let (negatives, positives): (Vec<_>, Vec<_>) = all_patterns.iter().partition(|p| p.starts_with('!'));
  let build_globset = |patterns: &[&String], strip_prefix: &str| -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
      let p = pattern.trim_start_matches(strip_prefix);
      match Glob::new(p) {
        Ok(g) => {
          builder.add(g);
        }
        Err(err) => debug!("Invalid glob pattern '{p}': {err}"),
      }
    }
    builder.build().unwrap_or_else(|_| GlobSet::empty())
  };

  let include_set = build_globset(&positives, "");
  let exclude_set = build_globset(&negatives, "!");

  walk_matching(disk, &include_set, &exclude_set)
}

/// Walk a directory tree, skipping `node_modules` and other irrelevant
/// directories, and return every `package.json` path that matches
/// `include_set` and does not match `exclude_set`.
fn walk_matching<T: DiskIo>(disk: &Disk<'_, T>, include_set: &GlobSet, exclude_set: &GlobSet) -> Vec<PathBuf> {
  let mut results = Vec::new();
  let mut queue: VecDeque<PathBuf> = VecDeque::new();
  queue.push_back(disk.cwd.clone());
  while let Some(dir) = queue.pop_front() {
    let entries = match disk.io.read_dir(&dir) {
      Ok(e) => e,
      Err(err) => {
        debug!("Could not read directory '{}': {err}", dir.display());
        continue;
      }
    };
    for entry in entries {
      let name = entry.file_name().to_string_lossy().to_string();
      if entry.is_dir() {
        // Prune directories that can never contain relevant package.json files
        if name == "node_modules" || name == ".git" {
          continue;
        }
        queue.push_back(entry.path().to_path_buf());
      } else if name == "package.json" {
        let path = entry.path();
        let rel = path.strip_prefix(&disk.cwd).unwrap_or(path);
        if include_set.is_match(rel) && !exclude_set.is_match(rel) {
          results.push(path.to_path_buf());
        }
      }
    }
  }
  results
}
