use {
  crate::{
    disk::Disk,
    rcfile::{DEFAULT_MINIMUM_RELEASE_AGE, from_disk::resolve_minimum_release_age},
    test::mock::pnpm_yaml_file_from_str,
  },
  std::path::PathBuf,
};

fn empty_disk() -> Disk {
  Disk {
    cwd: PathBuf::from("/test"),
    lerna_json: None,
    package_json_files: Vec::new(),
    package_json_root_idx: None,
    package_manager: None,
    pnpm_workspace: None,
  }
}

#[test]
fn rcfile_value_wins_over_pnpm_yaml() {
  let mut disk = empty_disk();
  disk.pnpm_workspace = Some(pnpm_yaml_file_from_str("minimumReleaseAge: 60\n"));
  assert_eq!(resolve_minimum_release_age(Some(120), &disk), 120);
}

#[test]
fn rcfile_value_of_zero_wins_over_pnpm_yaml() {
  let mut disk = empty_disk();
  disk.pnpm_workspace = Some(pnpm_yaml_file_from_str("minimumReleaseAge: 60\n"));
  assert_eq!(resolve_minimum_release_age(Some(0), &disk), 0);
}

#[test]
fn pnpm_yaml_used_when_rcfile_silent() {
  let mut disk = empty_disk();
  disk.pnpm_workspace = Some(pnpm_yaml_file_from_str("minimumReleaseAge: 60\n"));
  assert_eq!(resolve_minimum_release_age(None, &disk), 60);
}

#[test]
fn pnpm_yaml_zero_used_when_rcfile_silent() {
  let mut disk = empty_disk();
  disk.pnpm_workspace = Some(pnpm_yaml_file_from_str("minimumReleaseAge: 0\n"));
  assert_eq!(resolve_minimum_release_age(None, &disk), 0);
}

#[test]
fn falls_back_to_default_when_rcfile_silent_and_pnpm_yaml_silent() {
  let mut disk = empty_disk();
  disk.pnpm_workspace = Some(pnpm_yaml_file_from_str("packages:\n  - 'pkgs/*'\n"));
  assert_eq!(resolve_minimum_release_age(None, &disk), DEFAULT_MINIMUM_RELEASE_AGE);
}

#[test]
fn falls_back_to_default_when_no_pnpm_yaml() {
  let disk = empty_disk();
  assert_eq!(resolve_minimum_release_age(None, &disk), DEFAULT_MINIMUM_RELEASE_AGE);
}
