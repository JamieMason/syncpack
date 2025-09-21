use {
  crate::{
    dependency::UpdateUrl,
    test::{self},
    Context,
  },
  serde_json::json,
};

#[test]
fn returns_correct_registry_update_url() {
  let config = test::mock::config_from_mock(json!({}));
  let packages = test::mock::packages_from_mocks(vec![json!({
    "name": "local-package",
    "version": "0.0.0",
    "dependencies": {
      "@jsr/luca__cases": "1",
      "@lit-labs/ssr": "npm:@lit-labs/ssr@3.3.0",
      "@luca/cases": "npm:@jsr/luca__cases@1",
      "@std/fmt": "npm:@jsr/std__fmt@^1.0.3",
      "@std/yaml": "npm:@jsr/std__yaml",
      "lit": "npm:lit@3.2.1",
    }
  })]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);

  let get_update_url_by_name = |name: &str| {
    ctx
      .instances
      .iter()
      .find(|instance| instance.descriptor.internal_name == name)
      .unwrap()
      .get_update_url()
  };

  assert_eq!(get_update_url_by_name("local-package"), None);
  assert_eq!(
    get_update_url_by_name("@jsr/luca__cases"),
    Some(UpdateUrl {
      internal_name: "@jsr/luca__cases".to_string(),
      url: "https://npm.jsr.io/@jsr/luca__cases".to_string()
    })
  );
  assert_eq!(
    get_update_url_by_name("@lit-labs/ssr"),
    Some(UpdateUrl {
      internal_name: "@lit-labs/ssr".to_string(),
      url: "https://registry.npmjs.org/@lit-labs/ssr".to_string()
    })
  );
  assert_eq!(
    get_update_url_by_name("@luca/cases"),
    Some(UpdateUrl {
      internal_name: "@luca/cases".to_string(),
      url: "https://npm.jsr.io/@jsr/luca__cases".to_string()
    })
  );
  assert_eq!(
    get_update_url_by_name("@std/fmt"),
    Some(UpdateUrl {
      internal_name: "@std/fmt".to_string(),
      url: "https://npm.jsr.io/@jsr/std__fmt".to_string()
    })
  );
  assert_eq!(
    get_update_url_by_name("@std/yaml"),
    Some(UpdateUrl {
      internal_name: "@std/yaml".to_string(),
      url: "https://npm.jsr.io/@jsr/std__yaml".to_string()
    })
  );
  assert_eq!(
    get_update_url_by_name("lit"),
    Some(UpdateUrl {
      internal_name: "lit".to_string(),
      url: "https://registry.npmjs.org/lit".to_string()
    })
  );
}

#[test]
fn has_same_major_minor_as_all_with_matching_versions() {
  let config = test::mock::config_from_mock(json!({}));
  let packages = test::mock::packages_from_mocks(vec![json!({
    "name": "local-package",
    "version": "0.0.0",
    "dependencies": {
      "nx": "21.3.0",
      "@nx/js": "21.3.1",
      "@nx/eslint": "21.3.2"
    }
  })]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);

  let instances: Vec<_> = ctx
    .instances
    .iter()
    .filter(|instance| ["nx", "@nx/js", "@nx/eslint"].contains(&instance.descriptor.name.as_str()))
    .collect();

  let nx_instance = instances.iter().find(|i| i.descriptor.name == "nx").unwrap();

  // All versions have major=21, minor=3, so should return true
  assert!(nx_instance.has_same_major_minor_as_all(&instances.iter().map(|i| (*i).clone()).collect::<Vec<_>>()));
}

#[test]
fn has_same_major_minor_as_all_with_mismatched_versions() {
  let config = test::mock::config_from_mock(json!({}));
  let packages = test::mock::packages_from_mocks(vec![json!({
    "name": "local-package",
    "version": "0.0.0",
    "dependencies": {
      "nx": "21.3.0",
      "@nx/js": "21.3.1",
      "@nx/eslint": "21.4.2"
    }
  })]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);

  let instances: Vec<_> = ctx
    .instances
    .iter()
    .filter(|instance| ["nx", "@nx/js", "@nx/eslint"].contains(&instance.descriptor.name.as_str()))
    .collect();

  let nx_instance = instances.iter().find(|i| i.descriptor.name == "nx").unwrap();

  // @nx/eslint has minor=4 while others have minor=3, so should return false
  assert!(!nx_instance.has_same_major_minor_as_all(&instances.iter().map(|i| (*i).clone()).collect::<Vec<_>>()));
}

#[test]
fn has_same_major_minor_as_all_rejects_caret_range() {
  let config = test::mock::config_from_mock(json!({}));
  let packages = test::mock::packages_from_mocks(vec![json!({
    "name": "local-package",
    "version": "0.0.0",
    "dependencies": {
      "lodash": "4.17.21",
      "react": "^18.2.0"
    }
  })]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);

  let instances: Vec<_> = ctx
    .instances
    .iter()
    .filter(|instance| ["lodash", "react"].contains(&instance.descriptor.name.as_str()))
    .collect();

  let lodash_instance = instances.iter().find(|i| i.descriptor.name == "lodash").unwrap();

  // lodash has exact version (4.17.21) but react has caret range (^18.2.0) which
  // could drift to 18.3.0, so should return false
  assert!(!lodash_instance.has_same_major_minor_as_all(&instances.iter().map(|i| (*i).clone()).collect::<Vec<_>>()));
}
