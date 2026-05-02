use crate::{
  errors::UnsupportedConfigError,
  source::{Source, SourceKind},
};

#[test]
fn source_kind_parses_pascal_case() {
  assert!(matches!(SourceKind::parse("PackageJson"), Ok(SourceKind::PackageJson)));
  assert!(matches!(SourceKind::parse("PnpmWorkspace"), Ok(SourceKind::PnpmWorkspace)));
  match SourceKind::parse("InvalidValue") {
    Err(UnsupportedConfigError::InvalidSource { value }) => assert_eq!(value, "InvalidValue"),
    other => panic!("expected InvalidSource error, got {other:?}"),
  }
}

#[test]
fn source_kind_returns_correct_variant() {
  let pkg_source = Source::Package {
    file_idx: 0,
    name: "x".to_string(),
    formatting_mismatches: vec![],
  };
  assert_eq!(pkg_source.kind(), SourceKind::PackageJson);

  let yaml_source = Source::PnpmYaml;
  assert_eq!(yaml_source.kind(), SourceKind::PnpmWorkspace);
}

#[test]
fn source_pnpm_yaml_is_unit_variant() {
  let s = Source::PnpmYaml;
  match s {
    Source::Package { .. } => panic!("not a package"),
    Source::PnpmYaml => {}
  }
}

#[test]
fn source_package_is_struct_variant() {
  let s = Source::Package {
    file_idx: 0,
    name: "pkg".to_string(),
    formatting_mismatches: vec![],
  };
  match &s {
    Source::Package {
      file_idx,
      name,
      formatting_mismatches,
    } => {
      assert_eq!(*file_idx, 0);
      assert_eq!(name, "pkg");
      assert!(formatting_mismatches.is_empty());
    }
    Source::PnpmYaml => panic!("not yaml"),
  }
  assert_eq!(s.name(), "pkg");
}
