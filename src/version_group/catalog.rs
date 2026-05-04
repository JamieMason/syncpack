use {
  super::{DependencyCore, L1, L2, L3, L4, L5, add_instance_to_dependencies},
  crate::{
    context::Context,
    disk::PackageManager,
    group_selector::GroupSelector,
    instance::{FixableInstance, Instance, InstanceIdx, SuspectInstance, UnfixableInstance, ValidInstance},
    registry::updates::RegistryUpdates,
    specifier::Specifier,
  },
  log::debug,
  std::{collections::BTreeMap, rc::Rc},
};

#[derive(Debug)]
pub struct CatalogGroup {
  pub selector: GroupSelector,
  pub dependencies: BTreeMap<String, DependencyCore>,
}

impl CatalogGroup {
  pub fn add_instance(&mut self, idx: InstanceIdx, instance: &Instance) {
    add_instance_to_dependencies(&mut self.dependencies, idx, instance);
  }

  pub fn visit(&self, ctx: &Context, _registry_updates: &Option<RegistryUpdates>) {
    let arena = &ctx.instances;
    let distinct_catalog_count = ctx.distinct_catalog_names().len();
    for dep in self.dependencies.values() {
      debug!("visit catalog version group");
      debug!("{L1}visit dependency '{}'", dep.internal_name);
      // Lookup keys by `internal_name` so `dependency_groups` aliasing works
      // — both the def and its consumers carry the alias label.
      let catalog_defs: Vec<&Instance> = ctx.catalog_defs_for(&dep.internal_name).collect();

      // Precompute the missing-from-catalog merge result so every non-def
      // consumer in the dep can be marked consistently. Only used by the
      // 0-catalogs (PM=pnpm/bun) and 1-catalog-without-dep branches.
      let missing_target_catalog_name: Option<String> = if catalog_defs.is_empty() {
        match distinct_catalog_count {
          0 if matches!(ctx.package_manager(), Some(PackageManager::Pnpm) | Some(PackageManager::Bun)) => Some("default".to_string()),
          1 => ctx.distinct_catalog_names().first().map(|s| s.to_string()),
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
          instance.mark_valid(ValidInstance::IsCatalogDefinition, &instance.descriptor.specifier);
          continue;
        }

        // Branch 2: local instance — Syncpack refuses to mark a package's own
        // /version property as a catalog reference.
        if instance.is_local_instance {
          debug!("{L3}it is the local instance of a package developed locally in this monorepo");
          debug!("{L4}refuse to catalog it");
          debug!("{L5}mark as suspect, user should change their config");
          instance.mark_suspect(SuspectInstance::RefuseToCatalogLocal);
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
            instance.mark_valid(ValidInstance::IsCatalog, &target);
          } else {
            // catalog: consumer pointing at a missing def — covers "no such
            // catalog", "dep absent from referenced catalog", and "dep lives in
            // a different catalog".
            debug!("{L3}it references catalog '{referenced_name}' which has no matching def");
            debug!("{L4}mark as DependsOnMissingCatalogDefinition");
            instance.mark_suspect(SuspectInstance::DependsOnMissingCatalogDefinition);
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
          instance.mark_fixable(FixableInstance::NotUsingCatalog(target_catalog_name), &target);
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
              instance.mark_fixable(
                FixableInstance::MissingFromCatalog {
                  catalog_name: catalog_name.clone(),
                  winning_specifier: Rc::clone(winning_specifier),
                },
                &expected,
              );
            }
            Err(()) => {
              debug!("{L3}dep is missing from catalog '{catalog_name}' but specifiers conflict");
              debug!("{L4}mark as MissingFromCatalogAndNonSemverMismatch");
              instance.mark_unfixable(UnfixableInstance::MissingFromCatalogAndNonSemverMismatch(catalog_name.clone()));
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
          instance.mark_unfixable(UnfixableInstance::NotUsingCatalogAndCatalogUnknown);
          continue;
        }

        // Branch 7: 0 catalogs configured AND PM is not pnpm/bun → cannot
        // infer which file to auto-create on fix.
        if catalog_defs.is_empty() && distinct_catalog_count == 0 {
          debug!(
            "{L3}0 catalogs configured + PM={:?} — cannot infer catalog file location",
            ctx.package_manager()
          );
          debug!("{L4}mark as CannotInferCatalogFile");
          instance.mark_unfixable(UnfixableInstance::CannotInferCatalogFile);
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
fn pick_winning_specifier(dep: &DependencyCore, arena: &[Instance]) -> Result<Rc<Specifier>, ()> {
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
