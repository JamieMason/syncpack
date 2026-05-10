use {
  crate::{
    registry::{
      cache::{CACHE_TTL_SECS, Cache, CacheEntry, CacheState},
      client::AllPackageVersions,
    },
    test::mock_disk::MockDiskIo,
  },
  std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
  },
};

fn pkg(name: &str, versions: &[&str]) -> Arc<AllPackageVersions> {
  Arc::new(AllPackageVersions {
    name: name.to_string(),
    versions: versions.iter().map(|s| s.to_string()).collect(),
    times: HashMap::new(),
  })
}

fn unix_now() -> u64 {
  SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn cache_path() -> PathBuf {
  PathBuf::from("/tmp/syncpack-test/cache.json")
}

#[test]
fn load_with_no_existing_file_returns_empty() {
  let disk = MockDiskIo::new();
  let cache = Cache::load(&disk, cache_path());
  assert!(cache.lookup("https://registry.npmjs.org/react").is_none());
}

#[test]
fn store_then_lookup_returns_cached() {
  let disk = MockDiskIo::new();
  let cache = Cache::load(&disk, cache_path());

  cache.store("https://registry.npmjs.org/react", pkg("react", &["18.0.0"]));

  let hit = cache.lookup("https://registry.npmjs.org/react").expect("should hit");
  assert_eq!(hit.versions, vec!["18.0.0"]);
}

#[test]
fn distinct_urls_do_not_collide() {
  let disk = MockDiskIo::new();
  let cache = Cache::load(&disk, cache_path());

  cache.store("https://registry.npmjs.org/react", pkg("react", &["18.0.0"]));
  cache.store("https://registry.npmjs.org/vue", pkg("vue", &["3.0.0"]));

  assert_eq!(cache.lookup("https://registry.npmjs.org/react").unwrap().versions, vec!["18.0.0"]);
  assert_eq!(cache.lookup("https://registry.npmjs.org/vue").unwrap().versions, vec!["3.0.0"]);
}

#[test]
fn save_writes_cache_to_disk() {
  let disk = MockDiskIo::new();
  let path = cache_path();
  let cache = Cache::load(&disk, path.clone());

  cache.store("https://registry.npmjs.org/react", pkg("react", &["18.0.0"]));
  cache.save(&disk);

  let written = disk.written_bytes(&path).expect("cache file written");
  let parsed: CacheState = serde_json::from_slice(&written).expect("valid JSON");
  assert!(parsed.entries.contains_key("https://registry.npmjs.org/react"));
}

#[test]
fn load_uses_existing_cache_file() {
  let mut disk = MockDiskIo::new();
  let mut state = CacheState::default();
  state.entries.insert(
    "https://registry.npmjs.org/react".to_string(),
    CacheEntry {
      cached_at: unix_now(),
      data: pkg("react", &["18.0.0"]),
    },
  );
  disk.add_file("tmp/syncpack-test/cache.json", serde_json::to_string(&state).unwrap());
  let abs_path = disk.root().join("tmp/syncpack-test/cache.json");

  let cache = Cache::load(&disk, abs_path);

  let hit = cache.lookup("https://registry.npmjs.org/react").expect("should hit");
  assert_eq!(hit.versions, vec!["18.0.0"]);
}

#[test]
fn expired_entry_is_evicted_on_lookup() {
  let mut disk = MockDiskIo::new();
  let mut state = CacheState::default();
  state.entries.insert(
    "https://registry.npmjs.org/react".to_string(),
    CacheEntry {
      cached_at: unix_now().saturating_sub(CACHE_TTL_SECS + 60),
      data: pkg("react", &["0.0.1"]),
    },
  );
  disk.add_file("tmp/syncpack-test/cache.json", serde_json::to_string(&state).unwrap());
  let abs_path = disk.root().join("tmp/syncpack-test/cache.json");

  let cache = Cache::load(&disk, abs_path);

  assert!(cache.lookup("https://registry.npmjs.org/react").is_none());
}

#[test]
fn unreadable_cache_file_yields_empty_cache() {
  let mut disk = MockDiskIo::new();
  disk.add_file("tmp/syncpack-test/cache.json", "not valid json".to_string());
  let abs_path = disk.root().join("tmp/syncpack-test/cache.json");

  let cache = Cache::load(&disk, abs_path);

  assert!(cache.lookup("https://registry.npmjs.org/react").is_none());
}

#[test]
fn save_prunes_expired_entries() {
  let mut disk = MockDiskIo::new();
  let mut state = CacheState::default();
  state.entries.insert(
    "https://registry.npmjs.org/stale".to_string(),
    CacheEntry {
      cached_at: unix_now().saturating_sub(CACHE_TTL_SECS + 60),
      data: pkg("stale", &["0.0.1"]),
    },
  );
  let path_str = "tmp/syncpack-test/cache.json";
  disk.add_file(path_str, serde_json::to_string(&state).unwrap());
  let abs_path = disk.root().join(path_str);

  let cache = Cache::load(&disk, abs_path.clone());
  cache.store("https://registry.npmjs.org/fresh", pkg("fresh", &["1.0.0"]));
  cache.save(&disk);

  let written = disk.written_bytes(&abs_path).expect("cache file written");
  let parsed: CacheState = serde_json::from_slice(&written).expect("valid JSON");
  assert!(parsed.entries.contains_key("https://registry.npmjs.org/fresh"));
  assert!(!parsed.entries.contains_key("https://registry.npmjs.org/stale"));
}
