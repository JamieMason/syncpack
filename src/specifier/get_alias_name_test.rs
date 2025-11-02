use crate::specifier::Specifier;

#[test]
fn returns_correct_alias_names() {
  let cases: Vec<(&str, Option<&str>)> = vec![
    ("npm:foo", Some("foo")),
    ("npm:foo@1.2.3", Some("foo")),
    ("npm:@foo/bar@1.2.3", Some("@foo/bar")),
    ("1.2.3", None),
  ];
  for (value, expected) in cases {
    assert_eq!(Specifier::new(value).get_alias_name(), expected);
  }
}
