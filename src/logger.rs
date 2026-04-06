use {
  crate::cli::Cli,
  log::LevelFilter,
  std::sync::atomic::{AtomicU8, Ordering},
};

/// Bitmask flags for each log level
const ERROR_BIT: u8 = 1 << 0;
const WARN_BIT: u8 = 1 << 1;
const INFO_BIT: u8 = 1 << 2;
const DEBUG_BIT: u8 = 1 << 3;
const TRACE_BIT: u8 = 1 << 4;

/// Default: Info + Warn + Error
const DEFAULT_LEVELS: u8 = ERROR_BIT | WARN_BIT | INFO_BIT;

static LOG_LEVELS: AtomicU8 = AtomicU8::new(DEFAULT_LEVELS);

fn level_filter_to_bit(level: &LevelFilter) -> u8 {
  match level {
    LevelFilter::Error => ERROR_BIT,
    LevelFilter::Warn => WARN_BIT,
    LevelFilter::Info => INFO_BIT,
    LevelFilter::Debug => DEBUG_BIT,
    LevelFilter::Trace => TRACE_BIT,
    LevelFilter::Off => 0,
  }
}

fn levels_to_bitmask(levels: &[LevelFilter]) -> u8 {
  levels.iter().fold(0u8, |mask, l| mask | level_filter_to_bit(l))
}

#[cfg(test)]
pub fn init() {
  use env_logger::Builder;
  LOG_LEVELS.store(ERROR_BIT, Ordering::Relaxed);
  let _ = Builder::new().filter_level(LevelFilter::Error).try_init();
}

/// Initialise the logger with default levels. Call once at startup before
/// anything else. The format closure reads LOG_LEVELS on every log call.
#[cfg(not(test))]
pub fn init() {
  use {colored::Colorize, env_logger::Builder, log::Level, std::io::Write};
  Builder::new()
    .filter_level(LevelFilter::Trace)
    .format(move |buf, record| {
      let mask = LOG_LEVELS.load(Ordering::Relaxed);
      let level = record.level();
      if level == Level::Error && mask & ERROR_BIT != 0 {
        writeln!(buf, "{}", format!("✗ {}", record.args()).red())
      } else if level == Level::Warn && mask & WARN_BIT != 0 {
        writeln!(buf, "{}", format!("! {}", record.args()).yellow())
      } else if level == Level::Info && mask & INFO_BIT != 0 {
        writeln!(buf, "{}", record.args())
      } else if level == Level::Debug && mask & DEBUG_BIT != 0 {
        writeln!(buf, "{}", format!("? {}", record.args()).cyan())
      } else if level == Level::Trace && mask & TRACE_BIT != 0 {
        writeln!(buf, "{}", record.args())
      } else {
        write!(buf, "")
      }
    })
    .init();
}

/// Update log levels and ANSI settings after CLI parse succeeds.
pub fn configure(cli: &Cli) {
  if cli.log_levels.contains(&LevelFilter::Off) {
    LOG_LEVELS.store(0, Ordering::Relaxed);
  } else {
    LOG_LEVELS.store(levels_to_bitmask(&cli.log_levels), Ordering::Relaxed);
  }
  if cli.disable_ansi {
    colored::control::set_override(false);
  }
}
