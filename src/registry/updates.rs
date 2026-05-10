use {
  crate::{
    dependency::UpdateUrl,
    instance::Instance,
    registry::client::{AllPackageVersions, RegistryClient, RegistryError},
    specifier::Specifier,
    version_group::VersionGroup,
  },
  indicatif::{MultiProgress, ProgressBar, ProgressStyle},
  log::debug,
  std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
  },
  tokio::{
    sync::Semaphore,
    task::{JoinHandle, spawn},
  },
};

#[cfg(test)]
#[path = "updates_test.rs"]
mod updates_test;

/// The result of fetching package versions from the npm registry.
pub struct RegistryUpdates {
  /// All updates from the npm registry keyed by internal dependency name
  pub updates_by_internal_name: HashMap<String, Vec<Rc<Specifier>>>,
  /// Per-version publish timestamps (ISO 8601) keyed by internal
  /// dependency name. Used by the `update` UI to render a `~Nd` /
  /// `~Nmo` / `~N.Ny` "how stale" hint next to each version.
  pub times_by_internal_name: HashMap<String, HashMap<String, String>>,
  /// The internal names of all failed updates
  pub failed: Vec<String>,
}

impl RegistryUpdates {
  /// Fetch every version specifier ever published for all updateable
  /// dependencies in the project.
  pub async fn fetch(
    client: &Arc<dyn RegistryClient>,
    version_groups: &[VersionGroup],
    arena: &[Instance],
    max_concurrent_requests: usize,
    minimum_release_age_minutes: u64,
  ) -> Self {
    let client = Arc::clone(client);
    let semaphore = Arc::new(Semaphore::new(max_concurrent_requests));
    let progress_bars = Arc::new(MultiProgress::new());
    let mut handles: Vec<(String, JoinHandle<Result<Arc<AllPackageVersions>, RegistryError>>)> = vec![];
    let mut updates_by_internal_name: HashMap<String, Vec<Rc<Specifier>>> = HashMap::new();
    let mut times_by_internal_name: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut failed: Vec<String> = vec![];
    let cutoff_unix_seconds = age_cutoff_unix_seconds(minimum_release_age_minutes);

    for update_url in get_unique_update_urls(version_groups, arena) {
      let permit = Arc::clone(&semaphore).acquire_owned().await;
      let client = Arc::clone(&client);
      let progress_bars = Arc::clone(&progress_bars);

      handles.push((
        update_url.internal_name.clone(),
        spawn(async move {
          let _permit = permit;
          let progress_bar = progress_bars.add(ProgressBar::new_spinner());
          progress_bar.enable_steady_tick(Duration::from_millis(100));
          progress_bar.set_style(ProgressStyle::default_spinner());
          progress_bar.set_message(update_url.internal_name.clone());
          let package_meta = client.fetch(&update_url).await;
          progress_bar.finish_and_clear();
          progress_bars.remove(&progress_bar);
          package_meta
        }),
      ));
    }

    for (internal_name, handle) in handles {
      match handle.await {
        Ok(result) => match result {
          Ok(package_meta) => {
            let all_updates = updates_by_internal_name.entry(internal_name.clone()).or_default();
            for version in package_meta.versions.iter() {
              if !is_too_recent(version, &package_meta.times, cutoff_unix_seconds) {
                all_updates.push(Specifier::new(version));
              }
            }
            times_by_internal_name.insert(internal_name.clone(), package_meta.times.clone());
          }
          Err(err) => {
            debug!("{err}");
            failed.push(internal_name);
          }
        },
        Err(err) => {
          debug!("{err}");
          failed.push(internal_name);
        }
      }
    }

    Self {
      updates_by_internal_name,
      times_by_internal_name,
      failed,
    }
  }
}

/// Compute the UNIX-second cutoff for the age filter. Returns `None` when
/// filtering is disabled (`minimum_release_age_minutes == 0`) or when the
/// system clock is before the UNIX epoch.
pub(crate) fn age_cutoff_unix_seconds(minimum_release_age_minutes: u64) -> Option<i64> {
  if minimum_release_age_minutes == 0 {
    return None;
  }
  let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
  Some(now - (minimum_release_age_minutes as i64) * 60)
}

/// `true` when the version was published after the cutoff and so should
/// be filtered out. Versions absent from `times` are kept.
pub(crate) fn is_too_recent(version: &str, times: &HashMap<String, String>, cutoff_unix_seconds: Option<i64>) -> bool {
  let Some(cutoff) = cutoff_unix_seconds else { return false };
  let Some(timestamp) = times.get(version) else { return false };
  match parse_rfc3339_to_unix_seconds(timestamp) {
    Some(published_at) => published_at > cutoff,
    None => false,
  }
}

/// Parse an RFC 3339 / ISO 8601 timestamp such as
/// `"2024-01-15T10:30:00.000Z"` into UNIX seconds. Returns `None` for
/// any input that does not match the npm registry's fixed format.
///
/// Implements Howard Hinnant's date algorithm
/// (<https://howardhinnant.github.io/date_algorithms.html>) for the
/// civil → days-since-epoch conversion so we avoid pulling in a date
/// crate for one call site.
pub(crate) fn parse_rfc3339_to_unix_seconds(s: &str) -> Option<i64> {
  let bytes = s.as_bytes();
  if bytes.len() < 20 || !s.ends_with('Z') {
    return None;
  }
  let year: i32 = std::str::from_utf8(&bytes[0..4]).ok()?.parse().ok()?;
  if bytes[4] != b'-' {
    return None;
  }
  let month: u32 = std::str::from_utf8(&bytes[5..7]).ok()?.parse().ok()?;
  if bytes[7] != b'-' {
    return None;
  }
  let day: u32 = std::str::from_utf8(&bytes[8..10]).ok()?.parse().ok()?;
  if bytes[10] != b'T' {
    return None;
  }
  let hour: u32 = std::str::from_utf8(&bytes[11..13]).ok()?.parse().ok()?;
  if bytes[13] != b':' {
    return None;
  }
  let minute: u32 = std::str::from_utf8(&bytes[14..16]).ok()?.parse().ok()?;
  if bytes[16] != b':' {
    return None;
  }
  let second: u32 = std::str::from_utf8(&bytes[17..19]).ok()?.parse().ok()?;
  if !(1..=12).contains(&month) || day == 0 || day > 31 || hour > 23 || minute > 59 || second > 60 {
    return None;
  }
  let y = if month <= 2 { year - 1 } else { year };
  let era = if y >= 0 { y } else { y - 399 } / 400;
  let yoe = (y - era * 400) as u32;
  let m = if month > 2 { month - 3 } else { month + 9 };
  let doy = (153 * m + 2) / 5 + day - 1;
  let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
  let days_since_epoch = era as i64 * 146097 + doe as i64 - 719468;
  Some(days_since_epoch * 86400 + hour as i64 * 3600 + minute as i64 * 60 + second as i64)
}

/// Return a list of every dependency we should query the registry for
/// updates. We use internal names in order to support dependency groups,
/// where many dependencies can be aliased as one.
fn get_unique_update_urls(version_groups: &[VersionGroup], arena: &[Instance]) -> HashSet<UpdateUrl> {
  version_groups.iter().fold(HashSet::new(), |mut unique_update_urls, group| {
    group.get_update_urls(arena).inspect(|update_urls| {
      update_urls.iter().for_each(|url| {
        unique_update_urls.insert(url.clone());
      });
    });
    unique_update_urls
  })
}
