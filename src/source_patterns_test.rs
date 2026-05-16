use crate::source_patterns::normalise_pattern;

#[test]
fn normalizes_backslashes_to_forward_slashes() {
  let windows_backslashes = [
    ("projects\\apps\\*", "projects/apps/*/package.json"),
    ("projects\\libs\\lib1", "projects/libs/lib1/package.json"),
    ("apps\\*\\src", "apps/*/src/package.json"),
  ];
  let mixed_slashes = [
    ("projects\\mixed/pkg1", "projects/mixed/pkg1/package.json"),
    ("apps/test\\utils", "apps/test/utils/package.json"),
  ];
  let forward_slashes = [
    ("projects/apps/*", "projects/apps/*/package.json"),
    ("packages/*", "packages/*/package.json"),
  ];
  let backslashes_with_package_json = [
    ("apps\\*/package.json", "apps/*/package.json"),
    ("projects\\libs\\*\\package.json", "projects/libs/*/package.json"),
  ];
  let forward_slashes_with_package_json = [
    ("apps/*/package.json", "apps/*/package.json"),
    ("packages/*/package.json", "packages/*/package.json"),
  ];
  let bare_package_json = [("package.json", "/package.json")];
  let glob_patterns = [
    ("**\\*\\package.json", "**/*/package.json"),
    ("src\\**\\tests", "src/**/tests/package.json"),
  ];
  let negated_globs = [
    ("!apps/test2", "!apps/test2/package.json"),
    ("!packages/*", "!packages/*/package.json"),
    ("!apps/test2/package.json", "!apps/test2/package.json"),
    ("!projects\\apps\\*", "!projects/apps/*/package.json"),
  ];
  let non_standard_manifest_names = [
    ("packages/*/package.public.json", "packages/*/package.public.json"),
    ("packages/*/package.private.json", "packages/*/package.private.json"),
    ("packages/foo/manifest.json", "packages/foo/manifest.json"),
    ("package.public.json", "/package.public.json"),
    ("packages\\*\\package.public.json", "packages/*/package.public.json"),
    ("packages/*/*.json", "packages/*/*.json"),
    ("!packages/foo/package.public.json", "!packages/foo/package.public.json"),
    ("!packages/*/*.json", "!packages/*/*.json"),
  ];

  let cases = windows_backslashes
    .iter()
    .chain(mixed_slashes.iter())
    .chain(forward_slashes.iter())
    .chain(backslashes_with_package_json.iter())
    .chain(forward_slashes_with_package_json.iter())
    .chain(bare_package_json.iter())
    .chain(glob_patterns.iter())
    .chain(negated_globs.iter())
    .chain(non_standard_manifest_names.iter());

  for (input, expected) in cases {
    let result = normalise_pattern(input.to_string());
    assert_eq!(result, *expected, "normalize_pattern({input:?}) should return {expected:?}");
  }
}
