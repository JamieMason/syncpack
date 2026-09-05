use {crate::test::builder::TestBuilder, serde_json::json, syncpack_config::Rcfile};

/// The `minimumReleaseAge` resolved by the full pipeline: disk → rcfile loader
/// → `config::parse_value`.
async fn resolved_minimum_release_age(builder: TestBuilder) -> u64 {
  builder
    .with_package(json!({"name": "pkg-a", "version": "0.0.0"}))
    .run()
    .await
    .data
    .config
    .rcfile
    .minimum_release_age
}

#[tokio::test]
async fn rcfile_value_wins_over_pnpm_yaml() {
  let builder = TestBuilder::new()
    .with_pnpm_catalogs("minimumReleaseAge: 60\n")
    .with_config(json!({"minimumReleaseAge": 120}));
  assert_eq!(resolved_minimum_release_age(builder).await, 120);
}

#[tokio::test]
async fn rcfile_value_of_zero_wins_over_pnpm_yaml() {
  let builder = TestBuilder::new()
    .with_pnpm_catalogs("minimumReleaseAge: 60\n")
    .with_config(json!({"minimumReleaseAge": 0}));
  assert_eq!(resolved_minimum_release_age(builder).await, 0);
}

#[tokio::test]
async fn pnpm_yaml_used_when_rcfile_silent() {
  let builder = TestBuilder::new().with_pnpm_catalogs("minimumReleaseAge: 60\n");
  assert_eq!(resolved_minimum_release_age(builder).await, 60);
}

#[tokio::test]
async fn pnpm_yaml_zero_used_when_rcfile_silent() {
  let builder = TestBuilder::new().with_pnpm_catalogs("minimumReleaseAge: 0\n");
  assert_eq!(resolved_minimum_release_age(builder).await, 0);
}

#[tokio::test]
async fn falls_back_to_default_when_rcfile_silent_and_pnpm_yaml_silent() {
  let builder = TestBuilder::new().with_pnpm_catalogs("packages:\n  - 'pkgs/*'\n");
  assert_eq!(resolved_minimum_release_age(builder).await, Rcfile::default().minimum_release_age);
}

#[tokio::test]
async fn falls_back_to_default_when_no_pnpm_yaml() {
  assert_eq!(
    resolved_minimum_release_age(TestBuilder::new()).await,
    Rcfile::default().minimum_release_age
  );
}

#[tokio::test]
async fn yaml_rcfile_number_survives_value_conversion() {
  let builder = TestBuilder::new().with_yaml_config("minimumReleaseAge: 30\n");
  assert_eq!(resolved_minimum_release_age(builder).await, 30);
}
