use super::UpdateTarget;

#[test]
fn stricter_covers_all_nine_pairs() {
  use UpdateTarget::*;
  assert_eq!(Latest.stricter(Latest), Latest);
  assert_eq!(Latest.stricter(Minor), Minor);
  assert_eq!(Latest.stricter(Patch), Patch);
  assert_eq!(Minor.stricter(Latest), Minor);
  assert_eq!(Minor.stricter(Minor), Minor);
  assert_eq!(Minor.stricter(Patch), Patch);
  assert_eq!(Patch.stricter(Latest), Patch);
  assert_eq!(Patch.stricter(Minor), Patch);
  assert_eq!(Patch.stricter(Patch), Patch);
}
