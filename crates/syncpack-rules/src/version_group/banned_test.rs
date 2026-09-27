use {
  crate::{
    inspection::{Inspection, RuleInput},
    test_fixtures::{arena_and_dependency, cfg, dep_type, instance_at, local_version_dep_type},
    version_group::BannedGroup,
  },
  syncpack_model::{
    dependency::DependencyIdx,
    group_selector::GroupSelector,
    instance::{FixableInstance, InstanceIdx, InstanceState, InvalidInstance, SuspectInstance},
  },
  syncpack_vocab::severity::SeverityMap,
};

fn banned_group() -> BannedGroup {
  BannedGroup {
    selector: GroupSelector::new(vec![], vec![], "test".into(), vec![], vec![]),
    dependencies: vec![DependencyIdx(0)],
    severity: SeverityMap::new(),
  }
}

#[test]
fn marks_a_regular_instance_fixable_by_removal() {
  let deps_type = dep_type("/dependencies");
  let (arena, dependency) = arena_and_dependency(vec![instance_at("my-app", &deps_type, "lodash", "4.17.21")]);
  let dependencies = vec![dependency];
  let input = RuleInput {
    instances: &arena,
    dependencies: &dependencies,
    cfg: cfg(),
  };
  let mut out = Inspection::new(arena.len(), dependencies.len(), 0);

  banned_group().visit(&input, &None, &mut out);

  assert_eq!(
    out.state(InstanceIdx(0)),
    &InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::IsBanned)),
  );
}

#[test]
fn refuses_to_ban_the_local_instance_and_marks_it_suspect() {
  let version_type = local_version_dep_type();
  let (arena, dependency) = arena_and_dependency(vec![instance_at("lodash", &version_type, "lodash", "4.17.21")]);
  let dependencies = vec![dependency];
  let input = RuleInput {
    instances: &arena,
    dependencies: &dependencies,
    cfg: cfg(),
  };
  let mut out = Inspection::new(arena.len(), dependencies.len(), 0);

  banned_group().visit(&input, &None, &mut out);

  assert_eq!(
    out.state(InstanceIdx(0)),
    &InstanceState::Suspect(SuspectInstance::RefuseToBanLocal),
  );
}
