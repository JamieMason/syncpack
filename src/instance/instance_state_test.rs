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
