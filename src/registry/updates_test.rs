use {
  crate::registry::updates::{age_cutoff_unix_seconds, is_too_recent, parse_rfc3339_to_unix_seconds},
  std::collections::HashMap,
};

#[test]
fn parses_rfc3339_with_milliseconds() {
  // 2024-01-15T10:30:00.000Z = 1705314600
  assert_eq!(parse_rfc3339_to_unix_seconds("2024-01-15T10:30:00.000Z"), Some(1705314600));
}

#[test]
fn parses_rfc3339_without_milliseconds() {
  assert_eq!(parse_rfc3339_to_unix_seconds("2024-01-15T10:30:00Z"), Some(1705314600));
}

#[test]
fn parses_unix_epoch() {
  assert_eq!(parse_rfc3339_to_unix_seconds("1970-01-01T00:00:00Z"), Some(0));
}

#[test]
fn rejects_non_z_timezone() {
  assert!(parse_rfc3339_to_unix_seconds("2024-01-15T10:30:00+00:00").is_none());
}

#[test]
fn rejects_too_short_input() {
  assert!(parse_rfc3339_to_unix_seconds("2024-01-15").is_none());
}

#[test]
fn rejects_invalid_month() {
  assert!(parse_rfc3339_to_unix_seconds("2024-13-15T10:30:00Z").is_none());
}

#[test]
fn cutoff_returns_none_when_filter_disabled() {
  assert_eq!(age_cutoff_unix_seconds(0), None);
}

#[test]
fn cutoff_subtracts_minutes_from_now() {
  let cutoff = age_cutoff_unix_seconds(60).expect("non-zero minutes produces a cutoff");
  let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64;
  // cutoff should be roughly 3600s in the past (1 hour)
  assert!((now - 3700..=now - 3500).contains(&cutoff));
}

#[test]
fn version_with_no_known_publish_time_is_kept() {
  let times = HashMap::new();
  // any non-None cutoff
  assert!(!is_too_recent("1.0.0", &times, Some(0)));
}

#[test]
fn version_published_before_cutoff_is_kept() {
  let mut times = HashMap::new();
  times.insert("1.0.0".to_string(), "2020-01-01T00:00:00Z".to_string());
  // cutoff way in the future means published well before
  assert!(!is_too_recent("1.0.0", &times, Some(2_000_000_000)));
}

#[test]
fn version_published_after_cutoff_is_filtered() {
  let mut times = HashMap::new();
  // 2030-01-01
  times.insert("1.0.0".to_string(), "2030-01-01T00:00:00Z".to_string());
  // cutoff at 2020-01-01
  let cutoff_2020 = parse_rfc3339_to_unix_seconds("2020-01-01T00:00:00Z").unwrap();
  assert!(is_too_recent("1.0.0", &times, Some(cutoff_2020)));
}

#[test]
fn filter_disabled_keeps_recent_versions() {
  let mut times = HashMap::new();
  times.insert("1.0.0".to_string(), "2030-01-01T00:00:00Z".to_string());
  assert!(!is_too_recent("1.0.0", &times, None));
}

#[test]
fn malformed_timestamp_keeps_version() {
  let mut times = HashMap::new();
  times.insert("1.0.0".to_string(), "not-a-date".to_string());
  assert!(!is_too_recent("1.0.0", &times, Some(0)));
}
