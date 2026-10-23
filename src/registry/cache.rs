#[cfg(test)]
#[path = "cache_test.rs"]
mod cache_test;

use {
  crate::{disk::DiskIo, registry::client::AllPackageVersions},
  log::debug,
  serde::{Deserialize, Serialize},
  std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
  },
};

/// 30 minutes, mirroring the taze cache TTL.
pub(crate) const CACHE_TTL_SECS: u64 = 30 * 60;

/// Persistable on-disk cache of npm registry responses, keyed by URL.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CacheState {
  pub entries: HashMap<String, CacheEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheEntry {
  /// Seconds since UNIX epoch when this entry was written.
  pub cached_at: u64,
  pub data: Arc<AllPackageVersions>,
}

/// Per-user cache location: `$XDG_CACHE_HOME/syncpack/cache.json`, else
/// `$HOME/.cache/syncpack/cache.json`, else `<tmpdir>/syncpack/cache.json`.
/// The temp dir is world-shared on Linux, so it is a last resort.
pub fn default_cache_filepath() -> PathBuf {
  cache_filepath(
    std::env::var_os("XDG_CACHE_HOME").map(PathBuf::from),
    std::env::var_os("HOME").map(PathBuf::from),
    std::env::temp_dir(),
  )
}

/// Empty env vars count as unset, as the XDG spec requires.
pub(crate) fn cache_filepath(xdg_cache_home: Option<PathBuf>, home: Option<PathBuf>, temp_dir: PathBuf) -> PathBuf {
  let non_empty = |p: PathBuf| (!p.as_os_str().is_empty()).then_some(p);
  xdg_cache_home
    .and_then(non_empty)
    .or_else(|| home.and_then(non_empty).map(|h| h.join(".cache")))
    .unwrap_or(temp_dir)
    .join("syncpack")
    .join("cache.json")
}

/// In-memory cache backed by a JSON file on disk. Read and write
/// failures are swallowed so a missing or unwritable cache degrades
/// gracefully to direct fetches.
#[derive(Debug)]
pub struct Cache {
  state: Mutex<CacheState>,
  filepath: PathBuf,
}

impl Cache {
  /// Load any existing cache file via `io`. Any read or parse error
  /// yields an empty cache.
  pub fn load<D: DiskIo + ?Sized>(io: &D, filepath: PathBuf) -> Self {
    let state = match io.read_bytes(&filepath) {
      Some(Ok(bytes)) => match serde_json::from_slice::<CacheState>(&bytes) {
        Ok(parsed) => {
          debug!("registry cache loaded from {}", filepath.display());
          parsed
        }
        Err(err) => {
          debug!("registry cache unparsable at {}: {err}", filepath.display());
          CacheState::default()
        }
      },
      Some(Err(err)) => {
        debug!("registry cache unreadable at {}: {err}", filepath.display());
        CacheState::default()
      }
      None => CacheState::default(),
    };
    Self {
      state: Mutex::new(state),
      filepath,
    }
  }

  /// Prune expired entries, then write the in-memory cache to disk via
  /// `io`. Errors are swallowed.
  pub fn save<D: DiskIo + ?Sized>(&self, io: &D) {
    let snapshot = match self.state.lock() {
      Ok(mut guard) => {
        let now = unix_now();
        guard
          .entries
          .retain(|_, entry| now.saturating_sub(entry.cached_at) < CACHE_TTL_SECS);
        guard.clone()
      }
      Err(err) => {
        debug!("registry cache mutex poisoned: {err}");
        return;
      }
    };
    let bytes = match serde_json::to_vec(&snapshot) {
      Ok(b) => b,
      Err(err) => {
        debug!("registry cache serialize failed: {err}");
        return;
      }
    };
    match io.write_bytes(&self.filepath, &bytes) {
      Ok(()) => debug!("registry cache saved to {}", self.filepath.display()),
      Err(err) => debug!("registry cache write failed at {}: {err}", self.filepath.display()),
    }
  }

  /// Return a cached response for `url` when present and not expired.
  /// Expired entries are evicted.
  pub fn lookup(&self, url: &str) -> Option<Arc<AllPackageVersions>> {
    let now = unix_now();
    let mut state = self.state.lock().ok()?;
    let entry = state.entries.get(url)?;
    if now.saturating_sub(entry.cached_at) < CACHE_TTL_SECS {
      return Some(Arc::clone(&entry.data));
    }
    state.entries.remove(url);
    None
  }

  /// Insert or replace the cached response for `url`.
  pub fn store(&self, url: &str, data: Arc<AllPackageVersions>) {
    let Ok(mut state) = self.state.lock() else { return };
    state.entries.insert(
      url.to_string(),
      CacheEntry {
        cached_at: unix_now(),
        data,
      },
    );
  }
}

fn unix_now() -> u64 {
  SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}
