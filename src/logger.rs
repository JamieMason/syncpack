use crate::cli::Cli;

#[cfg(test)]
pub fn init(_: &Cli) {
  use {env_logger::Builder, log::LevelFilter};
  Builder::new().filter_level(LevelFilter::Debug).init();
}

/// Initialize the logger with the given log level
#[cfg(not(test))]
pub fn init(cli: &Cli) {
  use {
    colored::Colorize,
    env_logger::Builder,
    log::{Level, LevelFilter},
    std::io::Write,
  };
  if cli.disable_ansi {
    colored::control::set_override(false);
  }
  if !cli.log_levels.contains(&LevelFilter::Off) {
    let error_enabled = cli.log_levels.contains(&LevelFilter::Error);
    let warn_enabled = cli.log_levels.contains(&LevelFilter::Warn);
    let info_enabled = cli.log_levels.contains(&LevelFilter::Info);
    let debug_enabled = cli.log_levels.contains(&LevelFilter::Debug);
    let trace_enabled = cli.log_levels.contains(&LevelFilter::Trace);
    Builder::new()
      .filter_level(LevelFilter::Debug)
      .format(move |buf, record| {
        let level = record.level();
        if level == Level::Error && error_enabled {
          writeln!(buf, "{}", format!("✗ {}", record.args()).red())
        } else if level == Level::Warn && warn_enabled {
          writeln!(buf, "{}", format!("! {}", record.args()).yellow())
        } else if level == Level::Info && info_enabled {
          writeln!(buf, "{}", record.args())
        } else if level == Level::Debug && debug_enabled {
          writeln!(buf, "{}", format!("? {}", record.args()).cyan())
        } else if level == Level::Trace && trace_enabled {
          writeln!(buf, "{}", record.args())
        } else {
          write!(buf, "")
        }
      })
      .init();
  }
}
