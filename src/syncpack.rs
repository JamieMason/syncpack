use {
  crate::{
    cli::{Cli, ReporterKind, Subcommand},
    commands::{
      self, fix, fix_mismatches, format, json, lint, lint_semver_ranges, list, list_mismatches, prompt,
      reporter::{JsonFixReporter, JsonFormatReporter, PrettyFixReporter, PrettyFormatReporter},
      set_semver_ranges, update,
    },
    context::{Config, Discovered, Inspected},
    errors::SyncpackError,
    inspection::{FormatConfig, Inspection, RuleInput, RuleInputCfg},
    logger,
    rcfile::Rcfile,
    source::Source,
    source_patterns::get_source_patterns,
    tui::Tui,
    visit_formatting::visit_formatting,
    visit_packages::visit_packages,
  },
  serde_json::Value,
  std::sync::Arc,
  syncpack_disk::{Disk, catalogs},
  syncpack_io::DiskIo,
  syncpack_registry::client::RegistryClient,
  syncpack_vocab::registry_updates::RegistryUpdates,
};

/// Run the full syncpack CLI using injected dependencies. Discovers the
/// project, fetches registry updates when needed, inspects, and returns the
/// bound `Inspected` phase.
pub async fn syncpack<D: DiskIo>(cli: Cli, io: &D, registry_client: &Arc<dyn RegistryClient>) -> Result<Inspected, SyncpackError> {
  let data = analyse(cli, io)?;
  let updates = fetch_updates(&data, registry_client).await;
  let inspection = inspect(&data, &updates);
  Ok(Inspected { data, inspection, updates })
}

/// Analyse the project, discover config and packages, and return a
/// `Discovered`. All disk reads happen here.
fn analyse<D: DiskIo>(cli: Cli, io: &D) -> Result<Discovered, SyncpackError> {
  logger::configure(&cli);
  let mut disk = Disk::from_workspace(io, &cli.cwd);
  let rcfile = Rcfile::from_disk(&disk, io, &cli).map_err(SyncpackError::RcfileError)?;
  let config = Config {
    cli,
    rcfile: rcfile.contents,
  };
  let source_patterns = get_source_patterns(&config, &disk);
  let file_paths = io.find_package_jsons(&disk.cwd, &source_patterns);
  disk.load_package_files(io, &file_paths);
  let dep_types = catalogs::make_catalog_dep_types(&disk)?;
  let sources = crate::context::sources_from_disk(&disk, &file_paths);
  Discovered::create(config, disk, sources, dep_types).map_err(SyncpackError::ContextError)
}

/// Fetch updates from the npm registry, if applicable.
async fn fetch_updates(data: &Discovered, registry_client: &Arc<dyn RegistryClient>) -> Option<RegistryUpdates> {
  match data.config.cli.subcommand {
    Subcommand::Update => {
      let update_urls = crate::version_group::get_unique_update_urls(&data.version_groups, &data.dependencies, &data.instances);
      Some(
        syncpack_registry::updates::fetch_all(
          registry_client,
          update_urls,
          data.config.rcfile.max_concurrent_requests,
          data.config.rcfile.minimum_release_age,
        )
        .await,
      )
    }
    _ => None,
  }
}

/// Inspect every instance and every source, returning one `Inspection` value.
/// Pure: reads discovered data (including already-loaded file contents) and
/// mutates only the accumulator; no disk I/O, no model mutation.
pub fn inspect(data: &Discovered, registry_updates: &Option<RegistryUpdates>) -> Inspection {
  let input = RuleInput {
    instances: &data.instances,
    dependencies: &data.dependencies,
    sources: &data.sources,
    cfg: RuleInputCfg {
      target: data.config.cli.target,
      strict: data.config.rcfile.strict,
      package_manager: data.disk.package_manager,
    },
  };
  let mut inspection = Inspection::new(data.instances.len(), data.dependencies.len(), data.sources.all.len());
  let inspect_start = std::time::Instant::now();
  let version_start = std::time::Instant::now();
  visit_packages(&data.version_groups, &input, registry_updates, &mut inspection);
  log::debug!("Version inspection completed in {:?}", version_start.elapsed());
  let format = FormatConfig {
    sort_packages: data.config.rcfile.sort_packages,
    sort_first: data.config.rcfile.sort_first.clone(),
    format_bugs: data.config.rcfile.format_bugs,
    format_repository: data.config.rcfile.format_repository,
    sort_exports: data.config.rcfile.sort_exports.clone(),
    sort_az: data.config.rcfile.sort_az.clone(),
  };
  let contents: Vec<Option<&Value>> = data
    .sources
    .all
    .iter()
    .map(|source| match source {
      Source::Package { file_idx, .. } => Some(&data.disk.package_json_files[*file_idx].contents),
      Source::PnpmYaml => None,
    })
    .collect();
  let formatting_start = std::time::Instant::now();
  visit_formatting(&data.sources, &contents, &format, &mut inspection);
  log::debug!("Formatting inspection completed in {:?}", formatting_start.elapsed());
  log::debug!("Inspection (version + formatting) completed in {:?}", inspect_start.elapsed());
  inspection
}

/// Run the side-effects of the chosen subcommand. The sole disk writer.
pub fn run<D: DiskIo>(inspected: Inspected, io: &D, tui: &dyn Tui) -> Result<Inspected, SyncpackError> {
  match inspected.data.config.cli.subcommand {
    Subcommand::Fix => {
      let pretty = PrettyFixReporter;
      let json_reporter = JsonFixReporter;
      let reporter: &dyn commands::reporter::FixReporter = match inspected.data.config.cli.reporter {
        ReporterKind::Pretty => &pretty,
        ReporterKind::Json => &json_reporter,
      };
      fix::run(inspected, reporter, io)
    }
    Subcommand::FixMismatches => fix_mismatches::run(inspected),
    Subcommand::Format => {
      let pretty = PrettyFormatReporter;
      let json_reporter = JsonFormatReporter;
      let reporter: &dyn commands::reporter::FormatReporter = match inspected.data.config.cli.reporter {
        ReporterKind::Pretty => &pretty,
        ReporterKind::Json => &json_reporter,
      };
      format::run(inspected, reporter, io)
    }
    Subcommand::Json => json::run(inspected),
    Subcommand::Lint => lint::run(inspected),
    Subcommand::LintSemverRanges => lint_semver_ranges::run(inspected),
    Subcommand::List => list::run(inspected),
    Subcommand::ListMismatches => list_mismatches::run(inspected),
    Subcommand::Prompt => prompt::run(inspected),
    Subcommand::SetSemverRanges => set_semver_ranges::run(inspected),
    Subcommand::Update => update::run(inspected, io, tui),
  }
}
