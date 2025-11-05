use crate::packages::normalize_pattern;

#[test]
fn normalizes_backslashes_to_forward_slashes() {
  let cases = vec![
    // Windows-style backslashes
    ("projects\\apps\\*", "projects/apps/*/package.json"),
    ("projects\\libs\\lib1", "projects/libs/lib1/package.json"),
    ("apps\\*\\src", "apps/*/src/package.json"),
    // Mixed slashes
    ("projects\\mixed/pkg1", "projects/mixed/pkg1/package.json"),
    ("apps/test\\utils", "apps/test/utils/package.json"),
    // Already normalized (forward slashes)
    ("projects/apps/*", "projects/apps/*/package.json"),
    ("packages/*", "packages/*/package.json"),
    // Already includes package.json with backslashes
    ("apps\\*/package.json", "apps/*/package.json"),
    ("projects\\libs\\*\\package.json", "projects/libs/*/package.json"),
    // Already includes package.json with forward slashes
    ("apps/*/package.json", "apps/*/package.json"),
    ("packages/*/package.json", "packages/*/package.json"),
    // Just package.json
    ("package.json", "package.json"),
    // Complex patterns
    ("**\\*\\package.json", "**/*/package.json"),
    ("src\\**\\tests", "src/**/tests/package.json"),
  ];

  for (input, expected) in cases {
    let result = normalize_pattern(input.to_string());
    assert_eq!(result, expected, "normalize_pattern({input:?}) should return {expected:?}");
  }
}
