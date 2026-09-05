use syncpack_specifier::update_target::UpdateTarget;

#[derive(Debug, Clone)]
pub enum UpdatePolicy {
  Skip,
  /// Clamp eligible registry updates to no greater than this target.
  UpTo(UpdateTarget),
}
