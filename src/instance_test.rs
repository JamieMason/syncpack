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

  let catalogs = None;
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client, catalogs);

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
