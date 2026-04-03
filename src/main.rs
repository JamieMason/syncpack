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
    errors::SyncpackError,
    packages::Packages,
    rcfile::Rcfile,
    registry::{client::LiveRegistryClient, updates::RegistryUpdates},
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
mod errors;
mod group_selector;
mod instance;
mod logger;
mod package_json;
mod packages;
mod rcfile;
mod registry;
pub(crate) use syncpack_specifier::{self as specifier, semver_range};
mod version_group;
mod visit_formatting;
mod visit_packages;

#[tokio::main]
async fn main() {
  if let Err(e) = run().await {
    let msg = e.to_string();
    if !msg.is_empty() {
      error!("{e}");
    }
    exit(1);
  }
}

async fn run() -> Result<(), SyncpackError> {
  let cli = Cli::parse()?;
  logger::init(&cli);
  let rcfile = Rcfile::from_disk(&cli)?;
  let config = Config { rcfile, cli };
  let packages = Packages::from_config(&config);
  let catalogs: Option<CatalogsByName> = None; // catalogs::from_config(&config);
  let ctx = Context::create(config, packages, catalogs)?;

  match ctx.config.cli.subcommand {
    Subcommand::Fix => {
      let ctx = visit_packages(ctx, None);
      let pretty = PrettyFixReporter;
      let json_reporter = JsonFixReporter;
      let reporter: &dyn commands::reporter::FixReporter = match ctx.config.cli.reporter {
        ReporterKind::Pretty => &pretty,
        ReporterKind::Json => &json_reporter,
      };
      fix::run(ctx, reporter)?;
    }
    Subcommand::Format => {
      let ctx = visit_formatting(ctx);
      let pretty = PrettyFormatReporter;
      let json_reporter = JsonFormatReporter;
      let reporter: &dyn commands::reporter::FormatReporter = match ctx.config.cli.reporter {
        ReporterKind::Pretty => &pretty,
        ReporterKind::Json => &json_reporter,
      };
      format::run(ctx, reporter)?;
    }
    Subcommand::Lint => {
      let ctx = visit_packages(ctx, None);
      lint::run(ctx)?;
    }
    Subcommand::Update => {
      let client: Arc<dyn registry::client::RegistryClient> = Arc::new(LiveRegistryClient::new());
      let updates = RegistryUpdates::fetch(
        &client,
        &ctx.version_groups,
        &ctx.instances,
        ctx.config.rcfile.max_concurrent_requests,
      )
      .await;
      let ctx = visit_packages(ctx, Some(&updates));
      update::run(ctx, &updates)?;
    }
    Subcommand::List => {
      let ctx = visit_packages(ctx, None);
      list::run(ctx)?;
    }
    Subcommand::Json => {
      let ctx = visit_packages(ctx, None);
      json::run(ctx)?;
    }
    Subcommand::ListMismatches => list_mismatches::run()?,
    Subcommand::LintSemverRanges => lint_semver_ranges::run()?,
    Subcommand::FixMismatches => fix_mismatches::run()?,
    Subcommand::SetSemverRanges => set_semver_ranges::run()?,
    Subcommand::Prompt => prompt::run()?,
  }

  Ok(())
}
