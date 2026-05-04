#![allow(dead_code)]

// Synthetic Bun root contract:
// When `with_bun_catalogs` (or `with_bun_workspaces_catalogs`) is called, a
// synthetic root package is injected at `sources.all[0]`:
// - `name = "bun-root"`
// - `file_path = "/bun-root/package.json"`
// - `contents = { "name": "bun-root", ...catalog_blocks }` (or wrapped in `workspaces` for the workspaces variant)
// Real workspace packages are appended at `sources.all[1..]`. Every Bun
// catalog instance carries `source_idx = SourceIdx(0)` directly — no
// fallback chain runs in tests.
//
// dry_run defaults to true (set by `mock::config_from_mock`), so fix tests
// can assert `is_dirty()` and post-fix contents without action. Flip
// `ctx.config.cli.dry_run = false` only when asserting writes through a
// recording `MockDiskIo` — `write_to_disk` resets `dirty = false` after
// persisting, hiding the mutation otherwise.

use {
  super::mock,
  crate::{
    catalogs,
    cli::UpdateTarget,
    context::Context,
    disk::{Disk, File, PackageManager, detect_formatting, parse_yaml_file},
    sources::Sources,
    visit_formatting::visit_formatting,
    visit_packages::visit_packages,
  },
  serde_json::{Value, json},
  std::path::PathBuf,
};

#[cfg(test)]
#[path = "builder_test.rs"]
mod builder_test;

/// Builder pattern for creating test contexts with reduced boilerplate
pub struct TestBuilder {
  config: Value,
  dependency_groups: Vec<Value>,
  package_manager: Option<PackageManager>,
  packages: Vec<Value>,
  pnpm_yaml: Option<String>,
  bun_root: Option<Value>,
  registry_updates: Option<Value>,
  subcommand: Option<String>,
  semver_groups: Vec<Value>,
  strict: Option<bool>,
  update_target: Option<UpdateTarget>,
  version_groups: Vec<Value>,
}

impl TestBuilder {
  pub fn new() -> Self {
    Self {
      config: json!({}),
      dependency_groups: vec![],
      package_manager: None,
      packages: vec![],
      pnpm_yaml: None,
      bun_root: None,
      registry_updates: None,
      subcommand: None,
      semver_groups: vec![],
      strict: None,
      update_target: None,
      version_groups: vec![],
    }
  }

  /// Inject pnpm-workspace.yaml content. The yaml lands as
  /// `Source::PnpmYaml` in `ctx.sources.all` (last slot). Catalog instances
  /// carry `descriptor.source_idx` pointing at that slot and the dep type
  /// flag `is_catalog_definition == true`. Implies
  /// `with_pnpm_package_manager()` so discovery actually runs.
  pub fn with_pnpm_catalogs(mut self, yaml_content: &str) -> Self {
    self.pnpm_yaml = Some(yaml_content.to_string());
    if self.package_manager.is_none() {
      self.package_manager = Some(PackageManager::Pnpm);
    }
    self
  }

  /// Inject Bun catalog blocks at top-level (`/catalog`, `/catalogs/{name}`)
  /// of a synthetic root package.json at `packages[0]`. Implies
  /// `with_bun_package_manager()`.
  pub fn with_bun_catalogs(mut self, root_pkg_json_fragment: Value) -> Self {
    let mut root = root_pkg_json_fragment;
    if root.get("name").is_none() {
      root["name"] = json!("bun-root");
    }
    self.bun_root = Some(root);
    if self.package_manager.is_none() {
      self.package_manager = Some(PackageManager::Bun);
    }
    self
  }

  /// Same as `with_bun_catalogs` but wraps the fragment inside `/workspaces/`
  /// (`/workspaces/catalog`, `/workspaces/catalogs/{name}`).
  pub fn with_bun_workspaces_catalogs(mut self, workspaces_fragment: Value) -> Self {
    self.bun_root = Some(json!({
      "name": "bun-root",
      "workspaces": workspaces_fragment,
    }));
    if self.package_manager.is_none() {
      self.package_manager = Some(PackageManager::Bun);
    }
    self
  }

  /// Set Context.package_manager = Some(PackageManager::Pnpm).
  pub fn with_pnpm_package_manager(mut self) -> Self {
    self.package_manager = Some(PackageManager::Pnpm);
    self
  }

  /// Set Context.package_manager = Some(PackageManager::Bun).
  pub fn with_bun_package_manager(mut self) -> Self {
    self.package_manager = Some(PackageManager::Bun);
    self
  }

  /// Set Context.package_manager = Some(PackageManager::Npm).
  pub fn with_npm_package_manager(mut self) -> Self {
    self.package_manager = Some(PackageManager::Npm);
    self
  }

  /// Set Context.package_manager = Some(PackageManager::Yarn).
  pub fn with_yarn_package_manager(mut self) -> Self {
    self.package_manager = Some(PackageManager::Yarn);
    self
  }

  /// Set Context.package_manager = Some(PackageManager::Unknown).
  pub fn with_unknown_package_manager(mut self) -> Self {
    self.package_manager = Some(PackageManager::Unknown);
    self
  }

  pub fn with_package(mut self, package: Value) -> Self {
    self.packages.push(package);
    self
  }

  pub fn with_packages(mut self, packages: Vec<Value>) -> Self {
    self.packages.extend(packages);
    self
  }

  pub fn with_version_group(mut self, group: Value) -> Self {
    self.version_groups.push(group);
    self
  }

  pub fn with_version_groups(mut self, groups: Vec<Value>) -> Self {
    self.version_groups.extend(groups);
    self
  }

  pub fn with_semver_group(mut self, group: Value) -> Self {
    self.semver_groups.push(group);
    self
  }

  pub fn with_strict(mut self, strict: bool) -> Self {
    self.strict = Some(strict);
    self
  }

  pub fn with_update_target(mut self, target: UpdateTarget) -> Self {
    self.update_target = Some(target);
    self
  }

  pub fn with_registry_updates(mut self, updates: Value) -> Self {
    self.registry_updates = Some(updates);
    self
  }

  pub fn with_config(mut self, config: Value) -> Self {
    self.config = config;
    self
  }

  pub fn with_subcommand(mut self, subcommand: &str) -> Self {
    self.subcommand = Some(subcommand.to_string());
    self
  }

  /// Build the final configuration from all the builder settings
  fn build_config(&self) -> Value {
    let mut config = self.config.clone();
    if !self.version_groups.is_empty() {
      config["versionGroups"] = Value::Array(self.version_groups.clone());
    }
    if !self.semver_groups.is_empty() {
      config["semverGroups"] = Value::Array(self.semver_groups.clone());
    }
    if !self.dependency_groups.is_empty() {
      config["dependencyGroups"] = Value::Array(self.dependency_groups.clone());
    }
    if let Some(strict) = self.strict {
      config["strict"] = Value::Bool(strict);
    }
    config
  }

  pub async fn run(self) -> Context {
    use {
      crate::{
        registry::client::RegistryClient,
        syncpack,
        test::{mock_disk::MockDiskIo, registry_client::MockRegistryClient},
      },
      std::sync::Arc,
    };

    let mut disk = MockDiskIo::new();

    // Add package.json files at packages/{name}/package.json
    for pkg in &self.packages {
      let name = pkg["name"].as_str().unwrap_or("unknown");
      let path = format!("packages/{name}/package.json");
      disk.add_json(&path, pkg);
    }

    // Synthetic Bun root + bun.lock to trigger PM=Bun + catalog discovery.
    if let Some(ref root) = self.bun_root {
      disk.add_json("package.json", root);
      disk.add_file("bun.lock", "{}".to_string());
    }

    // Synthetic pnpm-workspace.yaml + pnpm-lock.yaml to trigger PM=Pnpm + discovery.
    if let Some(ref yaml) = self.pnpm_yaml {
      disk.add_file("pnpm-workspace.yaml", yaml.clone());
      disk.add_file("pnpm-lock.yaml", "lockfileVersion: '6.0'\n".to_string());
    }

    // Honour `with_*_package_manager(...)` when no catalog artifact already
    // wrote a lock file. PM is detected purely from disk in `.run()`.
    if self.bun_root.is_none() && self.pnpm_yaml.is_none() {
      match self.package_manager {
        Some(PackageManager::Pnpm) => disk.add_file("pnpm-lock.yaml", "lockfileVersion: '6.0'\n".to_string()),
        Some(PackageManager::Bun) => disk.add_file("bun.lock", "{}".to_string()),
        Some(PackageManager::Npm) => disk.add_file("package-lock.json", "{}".to_string()),
        Some(PackageManager::Yarn) => disk.add_file("yarn.lock", "".to_string()),
        Some(PackageManager::Unknown) | None => {}
      }
    }

    // Add .syncpackrc if config has custom settings
    let config = self.build_config();
    if config != json!({}) {
      disk.add_json(".syncpackrc", &config);
    }

    let subcommand = self
      .subcommand
      .as_deref()
      .unwrap_or(if self.registry_updates.is_some() { "update" } else { "lint" });
    let mut args: Vec<String> = vec!["syncpack".into(), subcommand.into()];
    if let Some(ref target) = self.update_target {
      args.push("--target".into());
      args.push(
        match target {
          crate::cli::UpdateTarget::Latest => "latest",
          crate::cli::UpdateTarget::Minor => "minor",
          crate::cli::UpdateTarget::Patch => "patch",
        }
        .into(),
      );
    }

    let registry_client: Arc<dyn RegistryClient> = if let Some(ref updates) = self.registry_updates {
      Arc::new(MockRegistryClient::from_json(updates.clone()))
    } else {
      Arc::new(MockRegistryClient::from_json(json!({})))
    };

    let (ctx, _registry_updates) = syncpack::syncpack(&args, &disk, &registry_client)
      .await
      .expect("syncpack analyse/inspect failed");
    ctx
  }

  pub fn build(self) -> Context {
    self.try_build().unwrap()
  }

  pub fn try_build(self) -> Result<Context, crate::context::ContextError> {
    let mut config = mock::config_from_mock(self.build_config());
    let disk = self.build_disk();
    let sources = build_sources(&disk);
    if let Some(target) = self.update_target {
      match target {
        UpdateTarget::Latest => config.cli.target = UpdateTarget::Latest,
        UpdateTarget::Minor => config.cli.target = UpdateTarget::Minor,
        UpdateTarget::Patch => config.cli.target = UpdateTarget::Patch,
      }
    }
    let dep_types = catalogs::make_catalog_dep_types(&disk).unwrap_or_default();
    Context::create(config, disk, sources, dep_types)
  }

  pub fn build_and_visit_packages(self) -> Context {
    let ctx = self.build();
    visit_packages(ctx, &None)
  }

  pub fn build_and_visit_formatting(self) -> Context {
    let ctx = self.build();
    visit_formatting(ctx)
  }

  pub async fn build_with_registry_and_visit(self) -> Context {
    let mut config = mock::config_from_mock(self.build_config());
    let disk = self.build_disk();
    let sources = build_sources(&disk);
    let dep_types = catalogs::make_catalog_dep_types(&disk).unwrap_or_default();

    if let Some(target) = self.update_target {
      match target {
        UpdateTarget::Latest => config.cli.target = UpdateTarget::Latest,
        UpdateTarget::Minor => config.cli.target = UpdateTarget::Minor,
        UpdateTarget::Patch => config.cli.target = UpdateTarget::Patch,
      }
    }

    if let Some(mock_updates) = self.registry_updates {
      let (ctx, updates) = mock::context_with_registry_updates(config, disk, sources, mock_updates, dep_types).await;
      visit_packages(ctx, &Some(updates))
    } else {
      let ctx = Context::create(config, disk, sources, dep_types).unwrap();
      visit_packages(ctx, &None)
    }
  }

  /// Synthetic Disk for build paths: includes the bun_root (if any) at slot 0,
  /// real workspace packages at slots 1..N, and pnpm yaml when set. Test infra
  /// uses /test/* paths because no real fs reads happen on this code path.
  fn build_disk(&self) -> Disk {
    let cwd = PathBuf::from("/test");
    let mut package_json_files: Vec<File<serde_json::Value>> = Vec::new();
    let mut package_json_root_idx: Option<usize> = None;
    if let Some(ref root) = self.bun_root {
      let raw = serde_json::to_string_pretty(root).unwrap_or_default();
      package_json_files.push(File {
        filepath: PathBuf::from("/bun-root/package.json"),
        formatting: detect_formatting(&raw),
        contents: root.clone(),
        dirty: false,
      });
      package_json_root_idx = Some(0);
    }
    for pkg in &self.packages {
      let name = pkg
        .pointer("/name")
        .and_then(|n| n.as_str())
        .unwrap_or("NAME_IS_MISSING")
        .to_string();
      let raw = serde_json::to_string_pretty(pkg).unwrap_or_default();
      package_json_files.push(File {
        filepath: PathBuf::from(format!("/packages/{name}/package.json")),
        formatting: detect_formatting(&raw),
        contents: pkg.clone(),
        dirty: false,
      });
    }
    let pnpm_workspace = self
      .pnpm_yaml
      .as_ref()
      .and_then(|raw| parse_yaml_file(raw.clone(), PathBuf::from("/test/pnpm-workspace.yaml")));
    Disk {
      cwd,
      lerna_json: None,
      package_json_files,
      package_json_root_idx,
      package_manager: self.package_manager,
      pnpm_workspace,
    }
  }
}

/// Assemble `Sources` for tests by mirroring `disk.package_json_files` 1:1
/// and treating every file as user-pattern-matched (so iteration's pass 2
/// sees them all). Yaml is appended at the tail when present.
fn build_sources(disk: &Disk) -> Sources {
  let all_paths: Vec<PathBuf> = disk.package_json_files.iter().map(|f| f.filepath.clone()).collect();
  Sources::from_disk(disk, &all_paths)
}

impl Default for TestBuilder {
  fn default() -> Self {
    Self::new()
  }
}
