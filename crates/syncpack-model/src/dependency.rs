use {
  crate::instance::{Instance, InstanceIdx},
  std::{collections::BTreeMap, rc::Rc},
  syncpack_specifier::Specifier,
  syncpack_vocab::update_url::UpdateUrl,
};

#[cfg(test)]
#[path = "dependency_test.rs"]
mod dependency_test;

/// Index into the `Discovered.dependencies` arena.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DependencyIdx(pub usize);

/// One dependency within one version group: the set of instances of a single
/// `internal_name` that group claims. Immutable discovery data — inspection
/// outputs (expected specifier, states) are held in `Inspection`, keyed by
/// `DependencyIdx` / `InstanceIdx`.
#[derive(Debug)]
pub struct Dependency {
  pub has_alias: bool,
  pub instances: Vec<InstanceIdx>,
  /// The `/version` instance of this dependency when the dependency is a
  /// package developed locally in this monorepo.
  pub local_instance: Option<InstanceIdx>,
  pub internal_name: String,
}

impl Dependency {
  fn new(internal_name: String) -> Self {
    Self {
      has_alias: false,
      instances: vec![],
      local_instance: None,
      internal_name,
    }
  }

  pub fn get_update_url(&self, arena: &[Instance]) -> Option<UpdateUrl> {
    if self.internal_name_is_supported() {
      self.instances.iter().find_map(|idx| arena[idx.0].get_update_url())
    } else {
      None
    }
  }

  pub fn get_local_specifier(&self, arena: &[Instance]) -> Option<Rc<Specifier>> {
    self.local_instance.map(|idx| Rc::clone(&arena[idx.0].descriptor.specifier))
  }

  pub fn internal_name_is_supported(&self) -> bool {
    !self.internal_name.contains('>') && self.internal_name.rfind('@').unwrap_or(0) == 0
  }

  pub fn has_local_instance(&self) -> bool {
    self.local_instance.is_some()
  }

  pub fn has_local_instance_with_invalid_specifier(&self, arena: &[Instance]) -> bool {
    self
      .get_local_specifier(arena)
      .is_some_and(|local| !matches!(&*local, Specifier::Exact(_)))
  }

  pub fn every_specifier_is_already_identical(&self, arena: &[Instance]) -> bool {
    if let Some(first_actual) = self.instances.first().map(|idx| &arena[idx.0].descriptor.specifier) {
      self.instances.iter().all(|idx| {
        let instance = &arena[idx.0];
        Rc::ptr_eq(&instance.descriptor.specifier, first_actual) || instance.descriptor.specifier.get_raw() == first_actual.get_raw()
      })
    } else {
      false
    }
  }

  pub fn get_unique_specifiers(&self, arena: &[Instance]) -> Vec<Rc<Specifier>> {
    let mut unique = Vec::new();
    for idx in &self.instances {
      let spec = &arena[idx.0].descriptor.specifier;
      if !unique.iter().any(|s: &Rc<Specifier>| s.get_raw() == spec.get_raw()) {
        unique.push(Rc::clone(spec));
      }
    }
    unique
  }

  pub fn get_instances<'a>(&'a self, arena: &'a [Instance]) -> impl Iterator<Item = (InstanceIdx, &'a Instance)> {
    self.instances.iter().map(move |idx| (*idx, &arena[idx.0]))
  }
}

/// Group discovered instances into a per-group map of `Dependency`, keyed by
/// `internal_name`. Called at discovery time; the map is later flattened into
/// the global dependency arena.
pub fn add_instance_to_dep_map(dependencies: &mut BTreeMap<String, Dependency>, idx: InstanceIdx, instance: &Instance) {
  let dep = dependencies
    .entry(instance.descriptor.internal_name.clone())
    .or_insert_with(|| Dependency::new(instance.descriptor.internal_name.clone()));
  dep.instances.push(idx);
  if instance.is_local_instance {
    dep.local_instance = Some(idx);
  }
  if instance.descriptor.name != dep.internal_name {
    dep.has_alias = true;
  }
}
