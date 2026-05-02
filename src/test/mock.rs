use {
  super::registry_client::MockRegistryClient,
  crate::{
    cli::{Cli, ReporterKind, SortBy, Subcommand, UpdateTarget},
    context::{Config, Context},
    dependency::DependencyType,
    disk::{detect_formatting, empty_yaml_file, parse_yaml_file, Disk, File, YamlFile},
    rcfile::Rcfile,
    registry::{client::RegistryClient, updates::RegistryUpdates},
    sources::Sources,
  },
  log::LevelFilter,
  serde_json::Value,
  std::{env, path::PathBuf, sync::Arc},
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
    subcommand: Subcommand::Lint,
    target: UpdateTarget::Latest,
  }
}

/// Create an empty Config struct
pub fn config() -> Config {
  Config {
    cli: cli(),
    rcfile: rcfile(),
  }
}

/// Create a Config struct from a mocked .syncpackrc
pub fn config_from_mock(value: serde_json::Value) -> Config {
  Config {
    cli: cli(),
    rcfile: rcfile_from_mock(value),
  }
}

/// Create an empty Rcfile struct
pub fn rcfile() -> Rcfile {
  Rcfile::default()
}

/// Create an Rcfile struct from a mocked .syncpackrc
pub fn rcfile_from_mock(value: serde_json::Value) -> Rcfile {
  serde_json::from_value::<crate::rcfile::RawRcfile>(value)
    .unwrap()
    .try_into()
    .unwrap()
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

pub fn package_json_from_value(contents: Value) -> Value {
  contents
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
  let sources = Sources::from_disk(&disk, &all_paths);
  (disk, sources)
}

/// Create a Context and RegistryUpdates from mocked npm registry data
pub async fn context_with_registry_updates(
  config: Config,
  disk: Disk,
  sources: Sources,
  mock_updates: serde_json::Value,
  dep_types: Vec<DependencyType>,
) -> (Context, RegistryUpdates) {
  let client: Arc<dyn RegistryClient> = Arc::new(MockRegistryClient::from_json(mock_updates));
  let ctx = Context::create(config, disk, sources, dep_types).unwrap();
  let updates = RegistryUpdates::fetch(
    &client,
    &ctx.version_groups,
    &ctx.instances,
    ctx.config.rcfile.max_concurrent_requests,
  )
  .await;
  (ctx, updates)
}
