use crate::{dependency::Dependency, version_group::VersionGroupVariant};

#[test]
fn internal_name_is_supported() {
  let scenarios = vec![
    (true, "@fluid-private/changelog-generator-wrapper"),
    (true, "@fluid-tools/markdown-magic"),
    (true, "@types/events_pkg"),
    (true, "@types/node"),
    (true, "get-tsconfig"),
    (true, "node-fetch"),
    (true, "nodegit"),
    (true, "qs"),
    (true, "sharp"),
    (true, "socket.io-client"),
    (true, "socket.io-parser"),
    (false, "@fluentui/react-positioning>@floating-ui/dom"),
    (false, "@types/node@<18"),
    (false, "good-fences>nodegit"),
    (false, "json5@<1.0.2"),
    (false, "json5@>=2.0.0 <2.2.2"),
    (false, "oclif>@aws-sdk/client-cloudfront"),
    (false, "oclif>@aws-sdk/client-s3"),
    (false, "simplemde>codemirror"),
    (false, "simplemde>marked"),
  ];
  for (expected, name) in scenarios {
    let dependency = Dependency::new(name.to_string(), VersionGroupVariant::HighestSemver, None, None);
    assert_eq!(expected, dependency.internal_name_is_supported());
  }
}
