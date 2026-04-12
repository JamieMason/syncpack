use {
  crate::{
    catalogs::CatalogsByName,
    cli::{Cli, ReporterKind, Subcommand},
    commands::{
      self, fix, fix_mismatches, format, json, lint, lint_semver_ranges, list, list_mismatches, prompt,
      reporter::{JsonFixReporter, JsonFormatReporter, PrettyFixReporter, PrettyFormatReporter},
      set_semver_ranges, update,
    },
    context::{Config, Context},
    disk::{Disk, DiskIo},
    errors::SyncpackError,
    file_paths::get_file_paths,
    logger,
    packages::Packages,
    rcfile::Rcfile,
    registry::{client::RegistryClient, updates::RegistryUpdates},
    source_patterns::get_source_patterns,
    visit_formatting::visit_formatting,
    visit_packages::visit_packages,
  },
  std::sync::Arc,
};

/// Run the full syncpack CLI using injected dependencies
pub async fn syncpack<D: DiskIo>(args: &[String], io: &D, registry_client: &Arc<dyn RegistryClient>) -> Result<Context, SyncpackError> {
  let ctx = analyse(args, io)?;
  let registry_updates = fetch_updates(&ctx, registry_client).await;
  let ctx = inspect(ctx, &registry_updates);
  run(ctx, registry_updates, io)
}

/// Analyse the project, discover config and packages, and return a `Context`
/// struct. All disk reading activity should happening during this phase.
fn analyse<D: DiskIo>(args: &[String], io: &D) -> Result<Context, SyncpackError> {
  let cli = Cli::parse(args)?;
  logger::configure(&cli);
  let disk = Disk::from_workspace(io, &cli.cwd);
  let rcfile = Rcfile::from_disk(&disk, &cli).map_err(SyncpackError::RcfileError)?;
  let config = Config {
    cli,
    rcfile: rcfile.contents,
  };
  let source_patterns = get_source_patterns(&config, &disk);
  let file_paths = get_file_paths(&source_patterns, &disk);
  let packages = Packages::from_config(&disk, &file_paths);
  let catalogs: Option<CatalogsByName> = None; // catalogs::from_config(&config);
  Context::create(config, packages, catalogs).map_err(SyncpackError::ContextError)
}

/// Fetch updates from the npm registry, if applicable
async fn fetch_updates(ctx: &Context, registry_client: &Arc<dyn RegistryClient>) -> Option<RegistryUpdates> {
  match ctx.config.cli.subcommand {
    Subcommand::Update => {
      let registry_updates = RegistryUpdates::fetch(
        registry_client,
        &ctx.version_groups,
        &ctx.instances,
        ctx.config.rcfile.max_concurrent_requests,
      )
      .await;
      Some(registry_updates)
    }
    _ => None,
  }
}

/// Inspect every instance of every dependency within every version group and
/// semver group, and based on the rules of those groups, assign a status code
/// to every instance which represents whether it matches the group's rules.
fn inspect(ctx: Context, registry_updates: &Option<RegistryUpdates>) -> Context {
  match ctx.config.cli.subcommand {
    Subcommand::Fix => visit_packages(ctx, &None),
    Subcommand::Format => visit_formatting(ctx),
    Subcommand::Json => visit_packages(ctx, &None),
    Subcommand::Lint => visit_packages(ctx, &None),
    Subcommand::List => visit_packages(ctx, &None),
    Subcommand::Update => visit_packages(ctx, registry_updates),
    _ => ctx,
  }
}

/// Run the side-effects of the chosen subcommand
fn run<D: DiskIo>(ctx: Context, registry_updates: Option<RegistryUpdates>, io: &D) -> Result<Context, SyncpackError> {
  match ctx.config.cli.subcommand {
    Subcommand::Fix => {
      let pretty = PrettyFixReporter;
      let json_reporter = JsonFixReporter;
      let reporter: &dyn commands::reporter::FixReporter = match ctx.config.cli.reporter {
        ReporterKind::Pretty => &pretty,
        ReporterKind::Json => &json_reporter,
      };
      fix::run(ctx, reporter, io)
    }
    Subcommand::FixMismatches => fix_mismatches::run(ctx),
    Subcommand::Format => {
      let pretty = PrettyFormatReporter;
      let json_reporter = JsonFormatReporter;
      let reporter: &dyn commands::reporter::FormatReporter = match ctx.config.cli.reporter {
        ReporterKind::Pretty => &pretty,
        ReporterKind::Json => &json_reporter,
      };
      format::run(ctx, reporter, io)
    }
    Subcommand::Json => json::run(ctx),
    Subcommand::Lint => lint::run(ctx),
    Subcommand::LintSemverRanges => lint_semver_ranges::run(ctx),
    Subcommand::List => list::run(ctx),
    Subcommand::ListMismatches => list_mismatches::run(ctx),
    Subcommand::Prompt => prompt::run(ctx),
    Subcommand::SetSemverRanges => set_semver_ranges::run(ctx),
    Subcommand::Update => update::run(ctx, registry_updates.expect("registry_updates is None"), io),
  }
}
