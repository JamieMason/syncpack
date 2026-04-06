#![allow(dead_code)]

use {
  crate::{
    catalogs::CatalogsByName,
    cli::{Cli, ReporterKind, Subcommand},
    commands::{
      fix, fix_mismatches, format, json, lint, lint_semver_ranges, list, list_mismatches, prompt,
      reporter::{JsonFixReporter, JsonFormatReporter, PrettyFixReporter, PrettyFormatReporter},
      set_semver_ranges, update,
    },
    context::{Config, Context},
    disk::{Disk, DiskIo, LiveDiskIo},
    errors::SyncpackError,
    file_paths::get_file_paths,
    packages::Packages,
    rcfile::Rcfile,
    registry::{
      client::{LiveRegistryClient, RegistryClient},
      updates::RegistryUpdates,
    },
    source_patterns::get_source_patterns,
    visit_formatting::visit_formatting,
    visit_packages::visit_packages,
  },
  log::error,
  std::{process::exit, sync::Arc},
};

#[cfg(test)]
#[path = "test/test.rs"]
mod test;

mod catalogs;
mod cli;
mod commands;
mod context;
mod dependency;
mod disk;
mod errors;
mod file_paths;
mod group_selector;
mod instance;
mod logger;
mod package_json;
mod packages;
mod rcfile;
mod registry;
mod source_patterns;
pub(crate) use syncpack_specifier::{self as specifier, semver_range};
mod version_group;
mod visit_formatting;
mod visit_packages;

#[tokio::main]
async fn main() {
  let result = async {
    logger::init();
    let args: Vec<String> = std::env::args().collect();
    let io = LiveDiskIo::new();
    let registry_client = LiveRegistryClient::new();
    syncpack(&args, io, registry_client).await
  }
  .await;

  if let Err(e) = result {
    let msg = e.to_string();
    if !msg.is_empty() {
      error!("{e}");
    }
    exit(1);
  }
}

async fn syncpack<D: DiskIo, R: RegistryClient + 'static>(args: &[String], io: D, registry_client: R) -> Result<Context, SyncpackError> {
  let ctx = analyse(args, io)?;
  let registry_updates = fetch_updates(&ctx, registry_client).await;
  let ctx = inspect(ctx, &registry_updates);
  run(ctx, registry_updates)
}

/// Analyse the project, discover config and packages, and return a `Context`
/// struct. All disk reading activity should happening during this phase.
fn analyse<D: DiskIo>(args: &[String], io: D) -> Result<Context, SyncpackError> {
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
async fn fetch_updates<R: RegistryClient + 'static>(ctx: &Context, registry_client: R) -> Option<RegistryUpdates> {
  match ctx.config.cli.subcommand {
    Subcommand::Update => {
      let client: Arc<dyn RegistryClient> = Arc::new(registry_client);
      let registry_updates = RegistryUpdates::fetch(
        &client,
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
fn run(ctx: Context, registry_updates: Option<RegistryUpdates>) -> Result<Context, SyncpackError> {
  match ctx.config.cli.subcommand {
    Subcommand::Fix => {
      let pretty = PrettyFixReporter;
      let json_reporter = JsonFixReporter;
      let reporter: &dyn commands::reporter::FixReporter = match ctx.config.cli.reporter {
        ReporterKind::Pretty => &pretty,
        ReporterKind::Json => &json_reporter,
      };
      fix::run(ctx, reporter)
    }
    Subcommand::FixMismatches => fix_mismatches::run(ctx),
    Subcommand::Format => {
      let pretty = PrettyFormatReporter;
      let json_reporter = JsonFormatReporter;
      let reporter: &dyn commands::reporter::FormatReporter = match ctx.config.cli.reporter {
        ReporterKind::Pretty => &pretty,
        ReporterKind::Json => &json_reporter,
      };
      format::run(ctx, reporter)
    }
    Subcommand::Json => json::run(ctx),
    Subcommand::Lint => lint::run(ctx),
    Subcommand::LintSemverRanges => lint_semver_ranges::run(ctx),
    Subcommand::List => list::run(ctx),
    Subcommand::ListMismatches => list_mismatches::run(ctx),
    Subcommand::Prompt => prompt::run(ctx),
    Subcommand::SetSemverRanges => set_semver_ranges::run(ctx),
    Subcommand::Update => update::run(ctx, registry_updates.expect("registry_updates is None")),
  }
}
