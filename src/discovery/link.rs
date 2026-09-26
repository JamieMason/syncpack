use {
  std::path::{Path, PathBuf},
  syncpack_disk::Disk,
  syncpack_model::{
    dependency::Dependency,
    instance::{Instance, InstanceIdx},
    source::Source,
    sources::Sources,
  },
  syncpack_specifier::Specifier,
};

/// Resolve, at discovery time, whether each `link:` instance of a
/// locally-developed dependency resolves to that package's directory, so
/// inspection needs no disk I/O.
pub(super) fn compute_link_resolution(
  dependencies: &[Dependency],
  instances: &[Instance],
  sources: &Sources,
  disk: &Disk,
) -> Vec<(InstanceIdx, bool)> {
  let mut resolutions = Vec::new();
  for dep in dependencies {
    if let Some(local_idx) = dep.local_instance {
      let local_instance = &instances[local_idx.0];
      for &idx in &dep.instances {
        let instance = &instances[idx.0];
        if instance.descriptor.specifier.is_link() {
          resolutions.push((idx, link_resolves_to_local(instance, local_instance, &sources.all, disk)));
        }
      }
    }
  }
  resolutions
}

/// Whether a `link:` instance of a locally-developed dependency resolves to
/// that package's directory. Discovery-time filesystem read, so inspection
/// stays pure. `false` for non-link instances and pnpm-catalog sources (which
/// do not live in a package.json).
fn link_resolves_to_local(instance: &Instance, local_instance: &Instance, sources: &[Source], disk: &Disk) -> bool {
  let Specifier::Link(link) = &*instance.descriptor.specifier else {
    return false;
  };
  let Source::Package {
    file_idx: consuming_idx, ..
  } = &sources[instance.source_idx().0]
  else {
    return false;
  };
  let Source::Package { file_idx: local_idx, .. } = &sources[local_instance.source_idx().0] else {
    return false;
  };
  let consuming_package_path = &disk.package_json_files[*consuming_idx].filepath;
  let consuming_package_dir = consuming_package_path.parent().unwrap_or_else(|| Path::new(""));
  let link_path = link.raw.strip_prefix("link:").unwrap_or(&link.raw);
  let resolved_link_path = consuming_package_dir.join(link_path);
  let local_package_path = &disk.package_json_files[*local_idx].filepath;
  let local_package_dir = local_package_path.parent().unwrap_or_else(|| Path::new(""));

  if let (Ok(resolved_canonical), Ok(local_canonical)) = (resolved_link_path.canonicalize(), local_package_dir.canonicalize()) {
    resolved_canonical == local_canonical
  } else {
    normalize_path(&resolved_link_path) == normalize_path(local_package_dir)
  }
}

/// Normalize a path by resolving `.` and `..` components without touching the
/// filesystem.
fn normalize_path(path: &Path) -> PathBuf {
  let mut components = Vec::new();
  for component in path.components() {
    match component {
      std::path::Component::ParentDir => {
        components.pop();
      }
      std::path::Component::CurDir => {}
      _ => components.push(component),
    }
  }
  components.iter().collect()
}
