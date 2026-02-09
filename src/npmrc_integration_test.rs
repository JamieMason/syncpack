use {
  crate::registry_client::LiveRegistryClient,
  npmrc_config_rs::{Credentials, LoadOptions, NpmrcConfig},
  std::{collections::HashMap, fs, path::Path},
  tempfile::TempDir,
};

fn setup_config(npmrc_content: &str) -> (TempDir, NpmrcConfig) {
  let dir = TempDir::new().expect("create temp dir");
  fs::write(dir.path().join("package.json"), "{}").expect("write package.json");
  fs::write(dir.path().join(".npmrc"), npmrc_content).expect("write .npmrc");
  let config = NpmrcConfig::load_with_options(LoadOptions {
    cwd: Some(dir.path().to_path_buf()),
    skip_user: true,
    skip_global: true,
    ..Default::default()
  })
  .expect("load npmrc config");
  (dir, config)
}

fn make_client(npmrc_content: &str) -> (TempDir, LiveRegistryClient) {
  let (dir, config) = setup_config(npmrc_content);
  (dir, LiveRegistryClient::new(config))
}

/// Returns (global_dir, user_dir, project_dir, config).
fn setup_multi_layer(
  global_npmrc: &str,
  user_npmrc: &str,
  project_npmrc: &str,
) -> (TempDir, TempDir, TempDir, NpmrcConfig) {
  let global_dir = TempDir::new().expect("global temp dir");
  let user_dir = TempDir::new().expect("user temp dir");
  let project_dir = TempDir::new().expect("project temp dir");

  let etc = global_dir.path().join("etc");
  fs::create_dir_all(&etc).expect("create etc dir");
  fs::write(etc.join("npmrc"), global_npmrc).expect("write global npmrc");
  fs::write(user_dir.path().join(".npmrc"), user_npmrc).expect("write user npmrc");
  fs::write(project_dir.path().join("package.json"), "{}").expect("write package.json");
  fs::write(project_dir.path().join(".npmrc"), project_npmrc).expect("write project .npmrc");

  let config = NpmrcConfig::load_with_options(LoadOptions {
    cwd: Some(project_dir.path().to_path_buf()),
    global_prefix: Some(global_dir.path().to_path_buf()),
    user_config: Some(user_dir.path().join(".npmrc")),
    skip_user: false,
    skip_global: false,
    skip_project: false,
  })
  .expect("load multi-layer config");

  (global_dir, user_dir, project_dir, config)
}

#[test]
fn resolve_url_default_registry_when_empty_npmrc() {
  let (_dir, client) = make_client("");
  let (url, base) = client.resolve_url("react").unwrap();
  assert_eq!(base.host_str().unwrap(), "registry.npmjs.org");
  assert!(url.as_str().ends_with("/react"));
}

#[test]
fn resolve_url_custom_default_registry() {
  let (_dir, client) = make_client("registry=https://custom.example.com/");
  let (url, base) = client.resolve_url("lodash").unwrap();
  assert_eq!(base.host_str().unwrap(), "custom.example.com");
  assert!(url.as_str().ends_with("/lodash"));
}

#[test]
fn resolve_url_scoped_package_uses_scoped_registry() {
  let (_dir, client) = make_client("@myorg:registry=https://myorg.npm.dev/");
  let (url, base) = client.resolve_url("@myorg/utils").unwrap();
  assert_eq!(base.host_str().unwrap(), "myorg.npm.dev");
  assert!(url.as_str().contains("@myorg/utils"));
}

#[test]
fn resolve_url_unscoped_ignores_scoped_registry() {
  let (_dir, client) = make_client("@myorg:registry=https://myorg.npm.dev/");
  let (_url, base) = client.resolve_url("react").unwrap();
  assert_eq!(base.host_str().unwrap(), "registry.npmjs.org");
}

#[test]
fn resolve_url_jsr_fallback_no_explicit_registry() {
  let (_dir, client) = make_client("");
  let (url, base) = client.resolve_url("@jsr/luca__cases").unwrap();
  assert_eq!(base.host_str().unwrap(), "npm.jsr.io");
  assert!(url.as_str().contains("@jsr/luca__cases"));
}

#[test]
fn resolve_url_jsr_uses_explicit_registry_when_set() {
  let (_dir, client) = make_client("@jsr:registry=https://custom.jsr.io/");
  let (_url, base) = client.resolve_url("@jsr/luca__cases").unwrap();
  assert_eq!(base.host_str().unwrap(), "custom.jsr.io");
}

#[test]
fn resolve_url_multiple_scopes_independent() {
  let npmrc = "@a:registry=https://a.example.com/\n@b:registry=https://b.example.com/";
  let (_dir, client) = make_client(npmrc);
  let (_, base_a) = client.resolve_url("@a/pkg").unwrap();
  let (_, base_b) = client.resolve_url("@b/pkg").unwrap();
  assert_eq!(base_a.host_str().unwrap(), "a.example.com");
  assert_eq!(base_b.host_str().unwrap(), "b.example.com");
}

#[test]
fn credentials_token_for_default_registry() {
  let npmrc = "registry=https://custom.example.com/\n//custom.example.com/:_authToken=abc123";
  let (_dir, config) = setup_config(npmrc);
  let reg = config.default_registry();
  let creds = config.credentials_for(&reg).expect("should have credentials");
  match &creds {
    Credentials::Token { token, .. } => assert_eq!(token, "abc123"),
    other => panic!("expected Token, got: {other:?}"),
  }
}

#[test]
fn credentials_basic_auth_for_scoped_registry() {
  let npmrc = "\
@myorg:registry=https://myorg.npm.dev/
//myorg.npm.dev/:username=myuser
//myorg.npm.dev/:_password=cDRzc3dvcmQ=";
  let (_dir, config) = setup_config(npmrc);
  let reg = config.registry_for("@myorg/utils");
  let creds = config.credentials_for(&reg).expect("should have credentials");
  match &creds {
    Credentials::BasicAuth { username, .. } => assert_eq!(username, "myuser"),
    other => panic!("expected BasicAuth, got: {other:?}"),
  }
}

#[test]
fn credentials_legacy_auth() {
  let npmrc = "registry=https://legacy.example.com/\n//legacy.example.com/:_auth=dXNlcjpwYXNz";
  let (_dir, config) = setup_config(npmrc);
  let reg = config.default_registry();
  let creds = config.credentials_for(&reg).expect("should have credentials");
  match &creds {
    Credentials::LegacyAuth { username, .. } => assert_eq!(username, "user"),
    other => panic!("expected LegacyAuth, got: {other:?}"),
  }
}

#[test]
fn credentials_none_for_unconfigured_registry() {
  let (_dir, config) = setup_config("");
  let reg = config.default_registry();
  assert!(config.credentials_for(&reg).is_none());
}

#[test]
fn credentials_token_with_client_cert() {
  let npmrc = "\
registry=https://secure.example.com/
//secure.example.com/:_authToken=tok
//secure.example.com/:certfile=/tmp/cert.pem
//secure.example.com/:keyfile=/tmp/key.pem";
  let (_dir, config) = setup_config(npmrc);
  let reg = config.default_registry();
  let creds = config.credentials_for(&reg).expect("should have credentials");
  match &creds {
    Credentials::Token { token, cert } => {
      assert_eq!(token, "tok");
      assert!(cert.is_some(), "cert should be present");
    }
    other => panic!("expected Token with cert, got: {other:?}"),
  }
}

#[test]
fn credentials_client_cert_only() {
  let npmrc = "\
registry=https://mtls.example.com/
//mtls.example.com/:certfile=/tmp/cert.pem
//mtls.example.com/:keyfile=/tmp/key.pem";
  let (_dir, config) = setup_config(npmrc);
  let reg = config.default_registry();
  let creds = config.credentials_for(&reg).expect("should have credentials");
  match &creds {
    Credentials::ClientCertOnly(cert) => {
      assert_eq!(cert.certfile, Path::new("/tmp/cert.pem"));
      assert_eq!(cert.keyfile, Path::new("/tmp/key.pem"));
    }
    other => panic!("expected ClientCertOnly, got: {other:?}"),
  }
}

#[test]
fn credentials_token_priority_over_basic() {
  let npmrc = "\
registry=https://both.example.com/
//both.example.com/:_authToken=winner
//both.example.com/:username=loser
//both.example.com/:_password=cDRzc3dvcmQ=";
  let (_dir, config) = setup_config(npmrc);
  let reg = config.default_registry();
  let creds = config.credentials_for(&reg).expect("should have credentials");
  match &creds {
    Credentials::Token { token, .. } => assert_eq!(token, "winner"),
    other => panic!("expected Token to win over BasicAuth, got: {other:?}"),
  }
}

#[test]
fn credentials_per_registry_isolation() {
  let npmrc = "\
@a:registry=https://a.example.com/
//a.example.com/:_authToken=token_a
@b:registry=https://b.example.com/
//b.example.com/:_authToken=token_b";
  let (_dir, config) = setup_config(npmrc);
  let reg_a = config.registry_for("@a/pkg");
  let reg_b = config.registry_for("@b/pkg");
  let creds_a = config.credentials_for(&reg_a).expect("creds for @a");
  let creds_b = config.credentials_for(&reg_b).expect("creds for @b");
  match (&creds_a, &creds_b) {
    (Credentials::Token { token: ta, .. }, Credentials::Token { token: tb, .. }) => {
      assert_eq!(ta, "token_a");
      assert_eq!(tb, "token_b");
    }
    _ => panic!("expected both Token, got: {creds_a:?} / {creds_b:?}"),
  }
}

#[test]
fn config_project_overrides_user_registry() {
  let (_g, _u, _p, config) = setup_multi_layer(
    "",
    "registry=https://user.example.com/",
    "registry=https://project.example.com/",
  );
  assert_eq!(config.default_registry().host_str().unwrap(), "project.example.com");
}

#[test]
fn config_user_overrides_global_token() {
  let (_g, _u, _p, config) = setup_multi_layer(
    "registry=https://corp.example.com/\n//corp.example.com/:_authToken=global_tok",
    "//corp.example.com/:_authToken=user_tok",
    "registry=https://corp.example.com/",
  );
  let reg = config.default_registry();
  let creds = config.credentials_for(&reg).expect("should have credentials");
  match &creds {
    Credentials::Token { token, .. } => assert_eq!(token, "user_tok"),
    other => panic!("expected Token, got: {other:?}"),
  }
}

#[test]
fn config_project_scope_with_user_auth() {
  let (_g, _u, _p, config) = setup_multi_layer(
    "",
    "//scoped.example.com/:_authToken=user_secret",
    "@org:registry=https://scoped.example.com/",
  );
  let reg = config.registry_for("@org/pkg");
  assert_eq!(reg.host_str().unwrap(), "scoped.example.com");
  let creds = config.credentials_for(&reg).expect("should have credentials");
  match &creds {
    Credentials::Token { token, .. } => assert_eq!(token, "user_secret"),
    other => panic!("expected Token, got: {other:?}"),
  }
}

#[test]
fn config_skip_project_ignores_project_npmrc() {
  let dir = TempDir::new().expect("temp dir");
  fs::write(dir.path().join("package.json"), "{}").expect("write package.json");
  fs::write(dir.path().join(".npmrc"), "registry=https://project.example.com/").expect("write .npmrc");
  let config = NpmrcConfig::load_with_options(LoadOptions {
    cwd: Some(dir.path().to_path_buf()),
    skip_user: true,
    skip_global: true,
    skip_project: true,
    ..Default::default()
  })
  .expect("load config");
  assert_eq!(config.default_registry().host_str().unwrap(), "registry.npmjs.org");
}

#[test]
fn config_graceful_when_no_npmrc() {
  let dir = TempDir::new().expect("temp dir");
  fs::write(dir.path().join("package.json"), "{}").expect("write package.json");
  let config = NpmrcConfig::load_with_options(LoadOptions {
    cwd: Some(dir.path().to_path_buf()),
    skip_user: true,
    skip_global: true,
    ..Default::default()
  })
  .expect("should load OK without .npmrc");
  assert_eq!(config.default_registry().host_str().unwrap(), "registry.npmjs.org");
}

#[test]
fn scoped_registries_returns_all() {
  let npmrc = "@a:registry=https://a.example.com/\n@b:registry=https://b.example.com/";
  let (_dir, config) = setup_config(npmrc);
  let scoped: HashMap<String, _> = config.scoped_registries();
  assert!(scoped.contains_key("@a"), "missing @a: {scoped:?}");
  assert!(scoped.contains_key("@b"), "missing @b: {scoped:?}");
  assert_eq!(scoped.len(), 2);
}

#[test]
fn scoped_registries_empty_when_none() {
  let (_dir, config) = setup_config("");
  assert!(config.scoped_registries().is_empty());
}

#[test]
fn default_registry_fallback() {
  let (_dir, config) = setup_config("");
  assert_eq!(config.default_registry().host_str().unwrap(), "registry.npmjs.org");
}

#[test]
fn get_raw_value() {
  let (_dir, config) = setup_config("strict-ssl=false");
  assert_eq!(config.get("strict-ssl"), Some("false"));
}

#[test]
fn get_returns_none_for_missing() {
  let (_dir, config) = setup_config("");
  assert!(config.get("nope").is_none());
}

#[test]
fn resolve_url_unix_paths() {
  let (_dir, client) = make_client("");
  let (url, _) = client.resolve_url("react").unwrap();
  assert!(!url.as_str().contains('\\'), "URL should use forward slashes: {url}");
}

#[cfg(windows)]
#[test]
fn config_windows_paths() {
  let dir = TempDir::new().expect("temp dir");
  fs::write(dir.path().join("package.json"), "{}").expect("write package.json");
  fs::write(dir.path().join(".npmrc"), "registry=https://win.example.com/").expect("write .npmrc");
  let config = NpmrcConfig::load_with_options(LoadOptions {
    cwd: Some(dir.path().to_path_buf()),
    skip_user: true,
    skip_global: true,
    ..Default::default()
  })
  .expect("load config on Windows");
  assert_eq!(config.default_registry().host_str().unwrap(), "win.example.com");
}
