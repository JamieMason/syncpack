use {
  serde_json::Value,
  std::{path::PathBuf, process::Command},
};

fn fixture_dir() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/json-reporter")
}

fn run_syncpack(args: &[&str]) -> (String, String, i32) {
  let output = Command::new(env!("CARGO_BIN_EXE_syncpack"))
    .args(args)
    .current_dir(fixture_dir())
    .output()
    .expect("failed to run syncpack");
  let stdout = String::from_utf8(output.stdout).unwrap();
  let stderr = String::from_utf8(output.stderr).unwrap();
  let code = output.status.code().unwrap_or(1);
  (stdout, stderr, code)
}

fn parse_ndjson_lines(stdout: &str) -> Vec<Value> {
  stdout
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| serde_json::from_str::<Value>(line).unwrap_or_else(|_| panic!("invalid JSON line: {line}")))
    .collect()
}

// — fix --reporter json —

#[test]
fn fix_json_outputs_one_ndjson_line_per_fixable_instance() {
  let (stdout, _stderr, code) = run_syncpack(&["fix", "--dry-run", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  assert!(!lines.is_empty(), "expected at least one NDJSON line for fixable instance");
  assert_eq!(code, 0, "exit code should be 0 when all fixable issues are resolved");
}

#[test]
fn fix_json_each_line_has_required_fields() {
  let (stdout, _stderr, _code) = run_syncpack(&["fix", "--dry-run", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  for line in &lines {
    assert!(line.get("dependency").is_some(), "missing 'dependency' field: {line}");
    assert!(line.get("dependencyGroup").is_some(), "missing 'dependencyGroup' field: {line}");
    assert!(line.get("dependencyType").is_some(), "missing 'dependencyType' field: {line}");
    assert!(line.get("package").is_some(), "missing 'package' field: {line}");
    assert!(line.get("property").is_some(), "missing 'property' field: {line}");
    assert!(line.get("strategy").is_some(), "missing 'strategy' field: {line}");
    assert!(line.get("versionGroup").is_some(), "missing 'versionGroup' field: {line}");
    assert!(line.get("statusCode").is_some(), "missing 'statusCode' field: {line}");
    assert!(line.get("actual").is_some(), "missing 'actual' field: {line}");
    assert!(line.get("expected").is_some(), "missing 'expected' field: {line}");
  }
}

#[test]
fn fix_json_contains_correct_values_for_fixture() {
  let (stdout, _stderr, _code) = run_syncpack(&["fix", "--dry-run", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  let foo_line = lines
    .iter()
    .find(|line| line["dependency"] == "foo")
    .expect("expected a line for dependency 'foo'");
  assert_eq!(foo_line["actual"]["raw"], "1.0.0");
  assert_eq!(foo_line["expected"]["raw"], "2.0.0");
  assert_eq!(foo_line["actual"]["type"], "exact");
  assert_eq!(foo_line["expected"]["type"], "exact");
  assert_eq!(foo_line["dependencyType"], "prod");
  assert_eq!(foo_line["versionGroup"], "HighestSemver");
  let property = foo_line["property"].as_array().expect("property should be an array");
  assert_eq!(property, &vec![Value::String("dependencies".to_string())]);
}

#[test]
fn fix_json_no_group_headers_or_summary_text_in_stdout() {
  let (stdout, _stderr, _code) = run_syncpack(&["fix", "--dry-run", "--reporter", "json"]);
  for line in stdout.lines() {
    if line.is_empty() {
      continue;
    }
    serde_json::from_str::<Value>(line).unwrap_or_else(|_| panic!("non-JSON line in stdout: {line}"));
  }
}

#[test]
fn fix_json_dry_run_does_not_mutate_fixture() {
  let pkg_a_path = fixture_dir().join("packages/pkg-a/package.json");
  let before = std::fs::read_to_string(&pkg_a_path).unwrap();
  run_syncpack(&["fix", "--dry-run", "--reporter", "json"]);
  let after = std::fs::read_to_string(&pkg_a_path).unwrap();
  assert_eq!(before, after, "fixture should not be mutated with --dry-run");
}

#[test]
fn fix_json_property_is_string_array() {
  let (stdout, _stderr, _code) = run_syncpack(&["fix", "--dry-run", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  for line in &lines {
    let property = line.get("property").expect("missing 'property' field");
    assert!(property.is_array(), "'property' should be an array, got: {property}");
    for element in property.as_array().unwrap() {
      assert!(element.is_string(), "'property' array elements should be strings, got: {element}");
    }
  }
}

// — format --check --reporter json —

#[test]
fn format_check_json_outputs_one_ndjson_line_per_mismatch() {
  let (stdout, _stderr, code) = run_syncpack(&["format", "--check", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  assert_eq!(lines.len(), 3, "expected 3 format mismatches (2 for pkg-b, 1 for pkg-a)");
  assert_eq!(code, 1, "exit code should be 1 when format issues found");
}

#[test]
fn format_check_json_each_line_has_required_fields() {
  let (stdout, _stderr, _code) = run_syncpack(&["format", "--check", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  for line in &lines {
    assert!(line.get("package").is_some(), "missing 'package' field: {line}");
    assert!(line.get("filePath").is_some(), "missing 'filePath' field: {line}");
    assert!(line.get("property").is_some(), "missing 'property' field: {line}");
    assert!(line.get("statusCode").is_some(), "missing 'statusCode' field: {line}");
  }
}

#[test]
fn format_check_json_property_values_are_path_segment_arrays() {
  let (stdout, _stderr, _code) = run_syncpack(&["format", "--check", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  let properties: Vec<&Value> = lines.iter().map(|l| l.get("property").unwrap()).collect();
  let empty_array = serde_json::json!([]);
  let scripts_array = serde_json::json!(["scripts"]);
  assert!(
    properties.contains(&&empty_array),
    "expected root property [] in output, got: {properties:?}"
  );
  assert!(
    properties.contains(&&scripts_array),
    "expected [\"scripts\"] property in output, got: {properties:?}"
  );
  for prop in &properties {
    assert!(prop.is_array(), "'property' should be an array, got: {prop}");
  }
}

#[test]
fn format_check_json_status_codes_are_format_mismatch_variants() {
  let (stdout, _stderr, _code) = run_syncpack(&["format", "--check", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  let valid_status_codes = [
    "BugsPropertyIsNotFormatted",
    "RepositoryPropertyIsNotFormatted",
    "PropertyIsNotSortedAz",
    "PackagePropertiesAreNotSorted",
    "ExportsPropertyIsNotSorted",
  ];
  for line in &lines {
    let status_code = line["statusCode"].as_str().unwrap();
    assert!(valid_status_codes.contains(&status_code), "unexpected statusCode: {status_code}");
  }
}

#[test]
fn format_check_json_pkg_b_has_two_mismatches() {
  let (stdout, _stderr, _code) = run_syncpack(&["format", "--check", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  let pkg_b_lines: Vec<&Value> = lines.iter().filter(|l| l["package"] == "pkg-b").collect();
  assert_eq!(pkg_b_lines.len(), 2, "pkg-b should have 2 mismatches");
  let status_codes: Vec<&str> = pkg_b_lines.iter().map(|l| l["statusCode"].as_str().unwrap()).collect();
  assert!(status_codes.contains(&"PackagePropertiesAreNotSorted"));
  assert!(status_codes.contains(&"PropertyIsNotSortedAz"));
}

#[test]
fn format_check_json_pkg_a_has_one_mismatch() {
  let (stdout, _stderr, _code) = run_syncpack(&["format", "--check", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  let pkg_a_lines: Vec<&Value> = lines.iter().filter(|l| l["package"] == "pkg-a").collect();
  assert_eq!(pkg_a_lines.len(), 1, "pkg-a should have 1 mismatch");
  assert_eq!(pkg_a_lines[0]["statusCode"], "PackagePropertiesAreNotSorted");
}

#[test]
fn format_check_json_no_package_headers_or_summary_in_stdout() {
  let (stdout, _stderr, _code) = run_syncpack(&["format", "--check", "--reporter", "json"]);
  for line in stdout.lines() {
    if line.is_empty() {
      continue;
    }
    serde_json::from_str::<Value>(line).unwrap_or_else(|_| panic!("non-JSON line in stdout: {line}"));
  }
}

// — format --dry-run --reporter json (fix mode) —

#[test]
fn format_fix_json_same_shape_as_check() {
  let (check_stdout, _stderr, _code) = run_syncpack(&["format", "--check", "--reporter", "json"]);
  let (fix_stdout, _stderr2, _code2) = run_syncpack(&["format", "--dry-run", "--reporter", "json"]);
  let check_lines = parse_ndjson_lines(&check_stdout);
  let fix_lines = parse_ndjson_lines(&fix_stdout);
  assert_eq!(
    check_lines.len(),
    fix_lines.len(),
    "check and fix should produce same number of lines"
  );
  for (check_line, fix_line) in check_lines.iter().zip(fix_lines.iter()) {
    assert_eq!(
      check_line["package"], fix_line["package"],
      "package should match between check and fix modes"
    );
    assert_eq!(
      check_line["statusCode"], fix_line["statusCode"],
      "statusCode should match between check and fix modes"
    );
    assert_eq!(
      check_line["property"], fix_line["property"],
      "property should match between check and fix modes"
    );
    assert!(fix_line["property"].is_array(), "property should be an array");
  }
}

#[test]
fn format_fix_json_exits_0() {
  let (_stdout, _stderr, code) = run_syncpack(&["format", "--dry-run", "--reporter", "json"]);
  assert_eq!(code, 0, "format fix mode should exit 0");
}

#[test]
fn format_fix_json_dry_run_does_not_mutate_fixture() {
  let pkg_b_path = fixture_dir().join("packages/pkg-b/package.json");
  let before = std::fs::read_to_string(&pkg_b_path).unwrap();
  run_syncpack(&["format", "--dry-run", "--reporter", "json"]);
  let after = std::fs::read_to_string(&pkg_b_path).unwrap();
  assert_eq!(before, after, "fixture should not be mutated with --dry-run");
}

// — exit code parity between pretty and json reporters —

#[test]
fn fix_exit_code_matches_between_pretty_and_json() {
  let (_stdout1, _stderr1, pretty_code) = run_syncpack(&["fix", "--dry-run"]);
  let (_stdout2, _stderr2, json_code) = run_syncpack(&["fix", "--dry-run", "--reporter", "json"]);
  assert_eq!(
    pretty_code, json_code,
    "exit codes should match between pretty and json reporters for fix"
  );
}

#[test]
fn format_check_exit_code_matches_between_pretty_and_json() {
  let (_stdout1, _stderr1, pretty_code) = run_syncpack(&["format", "--check"]);
  let (_stdout2, _stderr2, json_code) = run_syncpack(&["format", "--check", "--reporter", "json"]);
  assert_eq!(
    pretty_code, json_code,
    "exit codes should match between pretty and json reporters for format --check"
  );
}

// — default reporter (no --reporter flag) —

#[test]
fn fix_default_reporter_is_pretty() {
  let (_stdout, stderr, _code) = run_syncpack(&["fix", "--dry-run", "--show", "all"]);
  assert!(
    !stderr.is_empty(),
    "default reporter should produce output on stderr via log macros"
  );
  assert!(
    stderr.lines().any(|line| serde_json::from_str::<Value>(line).is_err()),
    "default reporter output should not be valid JSON (it should be pretty)"
  );
}

#[test]
fn format_default_reporter_is_pretty() {
  let (_stdout, stderr, _code) = run_syncpack(&["format", "--check"]);
  assert!(
    !stderr.is_empty(),
    "default reporter should produce output on stderr via log macros"
  );
  assert!(
    stderr.lines().any(|line| serde_json::from_str::<Value>(line).is_err()),
    "default reporter output should not be valid JSON (it should be pretty)"
  );
}

// — filePath field contains real paths —

#[test]
fn format_json_filepath_contains_real_path() {
  let (stdout, _stderr, _code) = run_syncpack(&["format", "--check", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  for line in &lines {
    let file_path = line["filePath"].as_str().unwrap();
    assert!(
      file_path.contains("package.json"),
      "filePath should reference a package.json file: {file_path}"
    );
  }
}

#[test]
fn fix_json_package_contains_real_path() {
  let (stdout, _stderr, _code) = run_syncpack(&["fix", "--dry-run", "--reporter", "json"]);
  let lines = parse_ndjson_lines(&stdout);
  for line in &lines {
    let package = line["package"].as_str().unwrap();
    assert!(
      package.contains("package.json"),
      "package field should reference a package.json file: {package}"
    );
  }
}
