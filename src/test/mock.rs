use {
  crate::{
    cli::{Cli, ReporterKind, SortBy, Subcommand},
    discovery::{Config, Discovered},
  },
  log::LevelFilter,
  serde_json::Value,
  std::{env, path::PathBuf, sync::Arc},
  syncpack_config::Rcfile,
  syncpack_disk::Disk,
  syncpack_io::{File, YamlFile, detect_formatting, empty_yaml_file, parse_yaml_file},
  syncpack_model::sources::Sources,
  syncpack_registry::{client::RegistryClient, mock::MockRegistryClient},
  syncpack_specifier::update_target::UpdateTarget,
  syncpack_vocab::{dependency_type::DependencyType, registry_updates::RegistryUpdates},
};

pub fn cli() -> Cli {
  Cli {
    check: true,
    config_path: None,
    cwd: env::current_dir().unwrap(),
    disable_ansi: true,
    dry_run: true,
    filters: None,
    log_levels: vec![LevelFilter::Error],
    reporter: ReporterKind::Pretty,
    show_hints: false,
    show_ignored: false,
    show_instances: false,
    show_status_codes: false,
    sort: SortBy::Name,
    source_patterns: vec![],
    source_mode: None,
    subcommand: Subcommand::Lint,
    target: UpdateTarget::Latest,
    interactive: false,
    no_cache: false,
  }
}

/// Create a Config struct from a mocked .syncpackrc
pub fn config_from_mock(value: serde_json::Value) -> Config {
  Config {
    cli: cli(),
    rcfile: rcfile_from_mock(value),
  }
}

/// Create an Rcfile struct from a mocked .syncpackrc
pub fn rcfile_from_mock(value: serde_json::Value) -> Rcfile {
  syncpack_config::rcfile_from(value)
}

/// Parse a package.json value into a `File<Value>` with a synthetic file
/// path derived from the package's name.
pub fn package_json_file_from_value(contents: Value) -> File<Value> {
  let name = contents
    .pointer("/name")
    .and_then(|name| name.as_str())
    .unwrap_or("NAME_IS_MISSING")
    .to_string();
  let file_path = PathBuf::from(format!("/packages/{name}/package.json"));
  let raw = serde_json::to_string_pretty(&contents).unwrap_or_default();
  File {
    filepath: file_path,
    formatting: detect_formatting(&raw),
    contents,
    dirty: false,
  }
}

/// Parse raw yaml into a `YamlFile` with a synthetic file path
/// (`/test/pnpm-workspace.yaml`). Falls back to an empty `YamlFile`
/// when the input fails to parse.
pub fn pnpm_yaml_file_from_str(yaml_content: &str) -> YamlFile {
  parse_yaml_file(yaml_content.to_string(), PathBuf::from("/test/pnpm-workspace.yaml"))
    .unwrap_or_else(|| empty_yaml_file(PathBuf::from("/test/pnpm-workspace.yaml")))
}

/// Build a `Sources` arena and a synthetic `Disk` from mocked package.json
/// values. Every package gets a `Source::Package` entry; no pnpm yaml is
/// constructed by this helper. All package paths are added to
/// `user_source_indices` so iteration sees them in pass 2.
pub fn disk_and_sources_from_mocks(values: Vec<serde_json::Value>) -> (Disk, Sources) {
  let cwd = PathBuf::from("/test");
  let mut package_json_files: Vec<File<serde_json::Value>> = Vec::new();
  let mut all_paths: Vec<PathBuf> = Vec::new();
  for value in values {
    let file = package_json_file_from_value(value);
    all_paths.push(file.filepath.clone());
    package_json_files.push(file);
  }
  let disk = Disk {
    cwd,
    lerna_json: None,
    package_json_files,
    package_json_root_idx: None,
    package_manager: None,
    pnpm_workspace: None,
  };
  let sources = crate::discovery::sources_from_disk(&disk, &all_paths);
  (disk, sources)
}

/// Create a Discovered and RegistryUpdates from mocked npm registry data
pub async fn discovered_with_registry_updates(
  config: Config,
  disk: Disk,
  sources: Sources,
  mock_updates: serde_json::Value,
  dep_types: Vec<DependencyType>,
) -> (Discovered, RegistryUpdates) {
  let client: Arc<dyn RegistryClient> = Arc::new(MockRegistryClient::from_json(mock_updates));
  let data = Discovered::create(config, disk, sources, dep_types).unwrap();
  let update_urls = syncpack_rules::version_group::get_unique_update_urls(&data.version_groups, &data.dependencies, &data.instances);
  let updates = syncpack_registry::updates::fetch_all(
    &client,
    update_urls,
    data.config.rcfile.max_concurrent_requests,
    data.config.rcfile.minimum_release_age,
  )
  .await;
  (data, updates)
}
