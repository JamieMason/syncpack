//! Minimal builders for `Instance`/`Dependency`/`RuleInput`, for tests that
//! exercise this crate's own visitors directly — no `Disk`, no config
//! parsing, no JSON. Mirrors the same grouping step `Discovered::create`
//! uses (`add_instance_to_dep_map`), just without the surrounding pipeline.

use {
  crate::inspection::RuleInputCfg,
  std::{collections::BTreeMap, rc::Rc},
  syncpack_model::{
    dependency::{Dependency, add_instance_to_dep_map},
    instance::{Instance, InstanceDescriptor, InstanceIdx},
    sources::SourceIdx,
  },
  syncpack_specifier::{Specifier, update_target::UpdateTarget},
  syncpack_vocab::dependency_type::{DependencyType, Strategy},
};

pub(crate) fn dep_type(path: &str) -> Rc<DependencyType> {
  Rc::new(DependencyType {
    name_path: None,
    name: path.to_string(),
    path: path.to_string(),
    strategy: Strategy::VersionsByName,
    source: syncpack_vocab::source_kind::SourceKind::PackageJson,
    is_catalog_definition: false,
  })
}

/// The `/version` dep type — instances built from this are `is_local_instance`.
pub(crate) fn local_version_dep_type() -> Rc<DependencyType> {
  dep_type("/version")
}

pub(crate) fn instance_at(package_name: &str, dependency_type: &Rc<DependencyType>, name: &str, specifier: &str) -> Instance {
  let descriptor = InstanceDescriptor {
    dependency_type: Rc::clone(dependency_type),
    internal_name: name.to_string(),
    is_local_dependency: false,
    name: name.to_string(),
    source_idx: SourceIdx(0),
    specifier: Specifier::new(specifier),
  };
  Instance::new(descriptor, package_name, None, None)
}

/// Build the instance arena and the single `Dependency` claiming every
/// instance passed in. All instances must share one `internal_name` (i.e. be
/// the same dependency) — this is a one-dependency-at-a-time fixture, not a
/// general discovery replacement.
pub(crate) fn arena_and_dependency(instances: Vec<Instance>) -> (Vec<Instance>, Dependency) {
  let mut dep_map: BTreeMap<String, Dependency> = BTreeMap::new();
  let mut arena = Vec::with_capacity(instances.len());
  for instance in instances {
    let idx = InstanceIdx(arena.len());
    add_instance_to_dep_map(&mut dep_map, idx, &instance);
    arena.push(instance);
  }
  let dependency = dep_map.into_values().next().expect("at least one instance");
  (arena, dependency)
}

pub(crate) fn cfg() -> RuleInputCfg {
  RuleInputCfg {
    target: UpdateTarget::Latest,
    strict: false,
    package_manager: None,
  }
}
