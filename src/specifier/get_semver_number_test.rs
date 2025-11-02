use crate::specifier::Specifier;

#[test]
fn returns_correct_semver_numbers() {
  let cases: Vec<(&str, Option<&str>)> = vec![
    ("workspace:*", None),
    ("workspace:^", None),
    ("workspace:~", None),
    ("npm:foo", None),
    ("npm:foo@1.2.3", Some("1.2.3")),
    ("npm:@foo/bar@1.2.3", Some("1.2.3")),
    ("1.2.3", Some("1.2.3")),
    ("^1.2.3", Some("1.2.3")),
  ];
  for (value, expected) in cases {
    assert_eq!(Specifier::new(value).get_semver_number(), expected);
  }
}
