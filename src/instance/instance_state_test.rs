use crate::{
  instance::{FixableInstance, InstanceState, SuspectInstance, UnfixableInstance, ValidInstance},
  specifier::Specifier,
};

fn name_of(state: InstanceState) -> String {
  state.get_name()
}

#[test]
fn state_get_name_strips_payload_for_not_using_catalog_tuple_variant() {
  let s = InstanceState::fixable(FixableInstance::NotUsingCatalog("react18".to_string()));
  assert_eq!(name_of(s), "NotUsingCatalog");
}

#[test]
fn state_get_name_strips_payload_for_missing_from_catalog_struct_variant() {
  let s = InstanceState::fixable(FixableInstance::MissingFromCatalog {
    catalog_name: "default".to_string(),
    winning_specifier: Specifier::new("^18.0.0"),
  });
  assert_eq!(name_of(s), "MissingFromCatalog");
}

#[test]
fn state_get_name_strips_payload_for_missing_from_catalog_and_non_semver_mismatch_tuple_variant() {
  let s = InstanceState::unfixable(UnfixableInstance::MissingFromCatalogAndNonSemverMismatch("react18".to_string()));
  assert_eq!(name_of(s), "MissingFromCatalogAndNonSemverMismatch");
}

#[test]
fn state_get_name_unchanged_for_unit_valid_variant() {
  // Unit variants have no payload — trim is a no-op.
  let s = InstanceState::valid(ValidInstance::IsCatalogDefinition);
  assert_eq!(name_of(s), "IsCatalogDefinition");
}

#[test]
fn state_get_name_unchanged_for_unit_unfixable_variant() {
  let s = InstanceState::unfixable(UnfixableInstance::NotUsingCatalogAndCatalogUnknown);
  assert_eq!(name_of(s), "NotUsingCatalogAndCatalogUnknown");
}

#[test]
fn state_get_name_unchanged_for_unit_suspect_variants() {
  assert_eq!(
    name_of(InstanceState::suspect(SuspectInstance::DependsOnMissingCatalogDefinition)),
    "DependsOnMissingCatalogDefinition"
  );
  assert_eq!(
    name_of(InstanceState::suspect(SuspectInstance::RefuseToCatalogLocal)),
    "RefuseToCatalogLocal"
  );
  assert_eq!(
    name_of(InstanceState::unfixable(UnfixableInstance::CannotInferCatalogFile)),
    "CannotInferCatalogFile"
  );
}

#[test]
fn state_get_name_strips_payload_for_pre_existing_tuple_variants() {
  // Forward-looking guard: no current variant carries a payload, but Debug-leak
  // is a class of bug, so the strip should apply uniformly to whatever the enum
  // grows next.
  let s = InstanceState::fixable(FixableInstance::IsBanned);
  assert_eq!(name_of(s), "IsBanned");
}

#[test]
fn state_get_name_unknown_unchanged() {
  assert_eq!(name_of(InstanceState::Unknown), "Unknown");
}

/// `StatusCode` in `npm/syncpack.ts` is the public type of the `statusCode` printed by
/// `syncpack json`, so it must name every status the enums below can produce.
#[test]
fn typescript_status_code_type_matches_the_status_enums() {
  use std::collections::BTreeSet;
  let rust = include_str!("instance_state.rs");
  let typescript = include_str!("../../npm/syncpack.ts");

  let mut rust_names = BTreeSet::new();
  for enum_name in [
    "ValidInstance",
    "FixableInstance",
    "UnfixableInstance",
    "SemverGroupAndVersionConflict",
    "SuspectInstance",
  ] {
    let start = rust.find(&format!("pub enum {enum_name} {{")).expect(enum_name);
    // variants are the lines indented by exactly two spaces which start with a capital
    rust_names.extend(
      rust[start..]
        .lines()
        .skip(1)
        .take_while(|line| *line != "}")
        .filter_map(|line| line.strip_prefix("  "))
        .filter(|line| line.starts_with(|c: char| c.is_ascii_uppercase()))
        .map(|line| line.chars().take_while(|c| c.is_ascii_alphanumeric()).collect::<String>()),
    );
  }

  let type_body = typescript
    .split("export type StatusCode =")
    .nth(1)
    .and_then(|rest| rest.split(';').next())
    .expect("npm/syncpack.ts defines StatusCode");
  // the names are the text between each pair of single quotes
  let typescript_names: BTreeSet<String> = type_body.split('\'').skip(1).step_by(2).map(String::from).collect();

  let missing: Vec<_> = rust_names.difference(&typescript_names).collect();
  let unknown: Vec<_> = typescript_names.difference(&rust_names).collect();
  assert!(
    missing.is_empty() && unknown.is_empty(),
    "npm/syncpack.ts StatusCode is out of sync. Missing: {missing:?}. Not a real status: {unknown:?}"
  );
}
