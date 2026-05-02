use crate::{
  disk::{Disk, PackageManager},
  test::mock_disk::MockDiskIo,
};

#[test]
fn detects_pnpm_from_workspace_yaml_without_lockfile() {
  let mut io = MockDiskIo::new();
  io.add_file("pnpm-workspace.yaml", "packages:\n  - 'apps/*'\n".to_string());
  let disk = Disk::from_workspace(&io, io.root());
  assert_eq!(disk.package_manager, Some(PackageManager::Pnpm));
  assert!(disk.pnpm_workspace.is_some(), "pnpm-workspace.yaml should be loaded");
}

#[test]
fn detects_bun_from_legacy_binary_lockfile() {
  let mut io = MockDiskIo::new();
  io.add_file("bun.lockb", String::new());
  let disk = Disk::from_workspace(&io, io.root());
  assert_eq!(disk.package_manager, Some(PackageManager::Bun));
}
