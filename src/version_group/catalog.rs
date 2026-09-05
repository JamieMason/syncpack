use {
  crate::{
    inspection::{Inspection, RuleInput},
    version_group::{L1, L2, L3, L4, L5},
  },
  log::debug,
  std::rc::Rc,
  syncpack_model::{
    dependency::{Dependency, DependencyIdx},
    group_selector::GroupSelector,
    instance::{
      FixableInstance, Instance, InstanceState, InvalidInstance, SuspectInstance, UnfixableInstance, ValidInstance, catalog_defs_for,
      distinct_catalog_names,
    },
  },
  syncpack_specifier::Specifier,
  syncpack_vocab::{package_manager::PackageManager, registry_updates::RegistryUpdates, severity::SeverityMap},
};

#[derive(Debug)]
pub struct CatalogGroup {
  pub selector: GroupSelector,
  pub dependencies: Vec<DependencyIdx>,
  pub severity: SeverityMap,
}

impl CatalogGroup {
  pub fn visit(&self, input: &RuleInput, _registry_updates: &Option<RegistryUpdates>, out: &mut Inspection) {
    let arena = input.instances;
    let strict = input.cfg.strict;
    let distinct_catalog_count = distinct_catalog_names(arena).len();
    for &dep_idx in &self.dependencies {
      let dep = &input.dependencies[dep_idx.0];
      debug!("visit catalog version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      // Lookup keys by `internal_name` so `dependency_groups` aliasing works
      // — both the def and its consumers carry the alias label.
      let catalog_defs: Vec<&Instance> = catalog_defs_for(arena, &dep.internal_name).collect();

      // Precompute the missing-from-catalog merge result so every non-def
      // consumer in the dep can be marked consistently. Only used by the
      // 0-catalogs (PM=pnpm/bun) and 1-catalog-without-dep branches.
      let missing_target_catalog_name: Option<String> = if catalog_defs.is_empty() {
        match distinct_catalog_count {
          0 if matches!(input.cfg.package_manager, Some(PackageManager::Pnpm) | Some(PackageManager::Bun)) => Some("default".to_string()),
          1 => distinct_catalog_names(arena).first().map(|s| s.to_string()),
          _ => None,
        }
      } else {
        None
      };
      let merged_winning_specifier: Option<Result<Rc<Specifier>, ()>> =
        missing_target_catalog_name.as_deref().map(|_| pick_winning_specifier(dep, arena));

      for &idx in &dep.instances {
        let instance = &arena[idx.0];
        let actual_specifier = &instance.descriptor.specifier;
        debug!("{L2}visit instance '{}' ({actual_specifier:?})", instance.id);

        // Branch 1: catalog definition → IsCatalogDefinition (def's own spec).
        if instance.is_catalog_instance() {
          debug!("{L3}it is a catalog definition");
          debug!("{L4}mark as catalog definition");
          out.classify(
            idx,
            InstanceState::Valid(ValidInstance::IsCatalogDefinition),
            Some(Rc::clone(actual_specifier)),
            &self.severity,
            strict,
          );
          continue;
        }

        // Branch 2: local instance — Syncpack refuses to mark a package's own
        // /version property as a catalog reference.
        if instance.is_local_instance {
          debug!("{L3}it is the local instance of a package developed locally in this monorepo");
          debug!("{L4}refuse to catalog it");
          debug!("{L5}mark as suspect, user should change their config");
          out.classify(
            idx,
            InstanceState::Suspect(SuspectInstance::RefuseToCatalogLocal),
            Some(Rc::clone(actual_specifier)),
            &self.severity,
            strict,
          );
          continue;
        }

        // Branch 3: instance uses the `catalog:` protocol.
        if let Specifier::Catalog(catalog) = &**actual_specifier {
          let referenced_name = catalog.name.as_deref().unwrap_or("default");
          let matching_def = catalog_defs.iter().find(|def| def.catalog_name() == Some(referenced_name));
          if let Some(def) = matching_def {
            debug!("{L3}it uses the catalog: protocol referencing existing def '{referenced_name}'");
            let target = def.catalog_target_specifier().unwrap_or_else(|| Rc::clone(actual_specifier));
            debug!("{L4}mark as IsCatalog (target {target:?})");
            out.classify(
              idx,
              InstanceState::Valid(ValidInstance::IsCatalog),
              Some(target),
              &self.severity,
              strict,
            );
          } else {
            // catalog: consumer pointing at a missing def — covers "no such
            // catalog", "dep absent from referenced catalog", and "dep lives in
            // a different catalog".
            debug!("{L3}it references catalog '{referenced_name}' which has no matching def");
            debug!("{L4}mark as DependsOnMissingCatalogDefinition");
            out.classify(
              idx,
              InstanceState::Suspect(SuspectInstance::DependsOnMissingCatalogDefinition),
              Some(Rc::clone(actual_specifier)),
              &self.severity,
              strict,
            );
          }
          continue;
        }

        // Branch 4: real specifier, dep defined in exactly one catalog.
        if catalog_defs.len() == 1 {
          let def = catalog_defs[0];
          let target_catalog_name = def.catalog_name().unwrap_or("default").to_string();
          let target = def.catalog_target_specifier().unwrap_or_else(|| Rc::clone(actual_specifier));
          debug!("{L3}dep is in exactly one catalog '{target_catalog_name}'");
          debug!("{L4}mark as NotUsingCatalog (target {target:?})");
          out.classify(
            idx,
            InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::NotUsingCatalog(target_catalog_name))),
            Some(target),
            &self.severity,
            strict,
          );
          continue;
        }

        // Branch 5: dep is not defined in any catalog. Two routes reach this
        // point: 0 catalogs project-wide (PM=pnpm/bun → implicit default) OR
        // exactly 1 catalog exists and dep is absent from it.
        if let (Some(catalog_name), Some(merge_result)) = (&missing_target_catalog_name, &merged_winning_specifier) {
          match merge_result {
            Ok(winning_specifier) => {
              let expected = catalog_target_for(catalog_name);
              debug!("{L3}dep is missing from catalog '{catalog_name}' (winning {winning_specifier:?})");
              debug!("{L4}mark as MissingFromCatalog (expected {expected:?})");
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::MissingFromCatalog {
                  catalog_name: catalog_name.clone(),
                  winning_specifier: Rc::clone(winning_specifier),
                })),
                Some(expected),
                &self.severity,
                strict,
              );
            }
            Err(()) => {
              debug!("{L3}dep is missing from catalog '{catalog_name}' but specifiers conflict");
              debug!("{L4}mark as MissingFromCatalogAndNonSemverMismatch");
              out.classify(
                idx,
                InstanceState::Invalid(InvalidInstance::Unfixable(
                  UnfixableInstance::MissingFromCatalogAndNonSemverMismatch(catalog_name.clone()),
                )),
                Some(Rc::clone(actual_specifier)),
                &self.severity,
                strict,
              );
            }
          }
          continue;
        }

        // Branch 6: 2+ catalogs exist, dep defined in 0 or 2+ of them.
        // Reaching here means: not a def, not local, not a catalog: consumer,
        // not exactly 1 catalog def — i.e. ambiguous which catalog to point
        // the consumer at. Syncpack does not auto-pick.
        if distinct_catalog_count >= 2 {
          debug!("{L3}dep is defined in zero or 2+ of {distinct_catalog_count} catalogs — ambiguous");
          debug!("{L4}mark as NotUsingCatalogAndCatalogUnknown");
          out.classify(
            idx,
            InstanceState::Invalid(InvalidInstance::Unfixable(UnfixableInstance::NotUsingCatalogAndCatalogUnknown)),
            Some(Rc::clone(actual_specifier)),
            &self.severity,
            strict,
          );
          continue;
        }

        // Branch 7: 0 catalogs configured AND PM is not pnpm/bun → cannot
        // infer which file to auto-create on fix.
        if catalog_defs.is_empty() && distinct_catalog_count == 0 {
          debug!(
            "{L3}0 catalogs configured + PM={:?} — cannot infer catalog file location",
            input.cfg.package_manager
          );
          debug!("{L4}mark as CannotInferCatalogFile");
          out.classify(
            idx,
            InstanceState::Invalid(InvalidInstance::Unfixable(UnfixableInstance::CannotInferCatalogFile)),
            Some(Rc::clone(actual_specifier)),
            &self.severity,
            strict,
          );
          continue;
        }

        debug!("{L5}fell through every catalog branch — stays Unknown (unexpected)");
      }
    }
  }
}

/// Build the `catalog:` / `catalog:{name}` Specifier consumers should switch to.
fn catalog_target_for(catalog_name: &str) -> Rc<Specifier> {
  let raw = if catalog_name == "default" {
    "catalog:".to_string()
  } else {
    format!("catalog:{catalog_name}")
  };
  Specifier::new(&raw)
}

/// Resolve the specifier to enshrine in the catalog when the dep is missing.
/// - All non-catalog instances agree on a single raw specifier (byte-identical) → `Ok(that specifier)`.
/// - All non-catalog instances are simple semver → `Ok(highest semver)`.
/// - Otherwise (mixed semver/non-semver, or all-non-semver-but-different) → `Err(())` to signal `MissingFromCatalogAndNonSemverMismatch`.
fn pick_winning_specifier(dep: &Dependency, arena: &[Instance]) -> Result<Rc<Specifier>, ()> {
  let consumer_specifiers: Vec<Rc<Specifier>> = dep
    .instances
    .iter()
    .map(|idx| &arena[idx.0])
    .filter(|i| !i.is_catalog_instance())
    .map(|i| Rc::clone(&i.descriptor.specifier))
    .collect();
  if consumer_specifiers.is_empty() {
    // No consumers means no-one to mark as missing — caller skips this branch.
    // Return Err so the unfixable path is not taken either.
    return Err(());
  }
  let first_raw = consumer_specifiers[0].get_raw().to_string();
  let all_identical = consumer_specifiers.iter().all(|s| s.get_raw() == first_raw);
  if all_identical {
    return Ok(Rc::clone(&consumer_specifiers[0]));
  }
  let all_semver = consumer_specifiers.iter().all(|s| s.get_node_version().is_some());
  if all_semver {
    let highest = consumer_specifiers
      .iter()
      .max_by(|a, b| a.cmp(b))
      .map(Rc::clone)
      .expect("non-empty list");
    return Ok(highest);
  }
  Err(())
}
