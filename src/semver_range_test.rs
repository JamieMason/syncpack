use {
  super::*,
  std::{cmp::Ordering, collections::HashMap},
};

#[test]
fn creates_a_semver_range_from_a_string() {
  let cases: Vec<(&str, SemverRange)> = vec![
    ("*", SemverRange::Any),
    ("^", SemverRange::Minor),
    ("", SemverRange::Exact),
    (">", SemverRange::Gt),
    (">=", SemverRange::Gte),
    ("<", SemverRange::Lt),
    ("<=", SemverRange::Lte),
    ("~", SemverRange::Patch),
  ];
  for (input, expected) in cases {
    let parsed = SemverRange::new(input).unwrap();
    assert_eq!(parsed, expected, "'{input}' should be '{expected:?}'");
    assert_eq!(parsed.unwrap(), input, "'{parsed:?}' should unwrap to '{input}'");
  }
}

#[test]
fn returns_none_for_unrecognised_ranges() {
  let parsed = SemverRange::new("wat");
  assert_eq!(parsed, None);
}

#[test]
fn compares_ranges_according_to_their_greediness() {
  let cases: Vec<(&str, &str, Ordering)> = vec![
    ("", "", Ordering::Equal),
    ("", "<", Ordering::Greater),
    ("*", "*", Ordering::Equal),
    ("*", ">", Ordering::Greater),
    ("<", "<=", Ordering::Less),
    ("<=", "<", Ordering::Greater),
    (">", ">=", Ordering::Greater),
    (">=", ">", Ordering::Less),
    ("^", "", Ordering::Greater),
    ("^", "~", Ordering::Greater),
  ];
  for (a, b, expected) in cases {
    let parsed = SemverRange::new(a);
    let ordering = parsed.cmp(&SemverRange::new(b));
    assert_eq!(ordering, expected, "'{a}' should be {expected:?} '{b}'");
  }
}

#[test]
fn sorts_ranges_according_to_their_greediness() {
  fn to_ranges(ranges: Vec<&str>) -> Vec<SemverRange> {
    ranges.iter().map(|r| SemverRange::new(r).unwrap()).collect()
  }
  let mut ranges = to_ranges(vec!["", "<", "*", ">", ">=", "<=", "^", "~"]);
  let expected = to_ranges(vec!["<", "<=", "", "~", "^", ">=", ">", "*"]);

  ranges.sort();
  assert_eq!(ranges, expected, "{ranges:?}, {expected:?}");
}

#[test]
fn implements_hash() {
  let semver1 = SemverRange::new("^").unwrap();
  let semver2 = SemverRange::new("~").unwrap();
  let mut map = HashMap::new();

  map.insert(&semver1, "value1");
  map.insert(&semver2, "value2");

  // Retrieve values from the map to verify the hash implementation
  assert_eq!(map.get(&semver1), Some(&"value1"));
  assert_eq!(map.get(&semver2), Some(&"value2"));
}
