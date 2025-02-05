use {
  crate::{
    cli::{Cli, SortBy, Subcommand},
    config::Config,
    package_json::PackageJson,
    packages::Packages,
    rcfile::Rcfile,
  },
  log::LevelFilter,
  serde_json::Value,
  std::{cell::RefCell, env, path::PathBuf},
};

pub fn cli() -> Cli {
  Cli {
    check: true,
    cwd: env::current_dir().unwrap(),
    filter: None,
    disable_ansi: true,
    inspect_formatting: false,
    inspect_mismatches: true,
    log_levels: vec![LevelFilter::Error],
    show_ignored: false,
    show_instances: false,
    show_hints: false,
    show_packages: false,
    show_status_codes: false,
    source_patterns: vec![],
    sort: SortBy::Name,
    subcommand: Subcommand::Lint,
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
  let empty_json = "{}".to_string();
  serde_json::from_str::<Rcfile>(&empty_json).unwrap()
}

/// Create an Rcfile struct from a mocked .syncpackrc
pub fn rcfile_from_mock(value: serde_json::Value) -> Rcfile {
  serde_json::from_value::<Rcfile>(value).unwrap()
}

/// Parse a package.json string
pub fn package_json_from_value(contents: Value) -> PackageJson {
  PackageJson {
    file_path: PathBuf::new(),
    formatting_mismatches: RefCell::new(vec![]),
    json: RefCell::new(contents.to_string()),
    contents: RefCell::new(contents),
  }
}

/// Create an collection of package.json files from mocked values
pub fn packages_from_mocks(values: Vec<serde_json::Value>) -> Packages {
  let mut packages = Packages::new();
  for value in values {
    packages.add_package(package_json_from_value(value));
  }
  packages
}
