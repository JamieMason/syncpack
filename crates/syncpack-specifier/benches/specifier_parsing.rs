use {
  criterion::{criterion_group, criterion_main, BenchmarkId, Criterion},
  std::{hint::black_box, time::Duration},
  syncpack_specifier::Specifier,
};

/// Inputs covering every Specifier variant
const INPUTS: &[(&str, &str)] = &[
  // Simple semver
  ("exact", "1.2.3"),
  ("exact_tag", "1.2.3-beta.1"),
  ("major", "1"),
  ("minor", "1.2"),
  ("latest_star", "*"),
  ("latest_keyword", "latest"),
  // Ranges
  ("range_caret", "^1.2.3"),
  ("range_tilde", "~1.2.3"),
  ("range_gt", ">1.2.3"),
  ("range_gte", ">=1.2.3"),
  ("range_lt", "<1.2.3"),
  ("range_lte", "<=1.2.3"),
  ("range_caret_tag", "^1.2.3-beta.1"),
  ("range_major_caret", "^1"),
  ("range_minor_tilde", "~1.2"),
  // Complex
  ("complex_or", ">=1.0.0 <2.0.0 || >=3.0.0"),
  ("complex_and", ">=1.0.0 <2.0.0"),
  // Workspace protocol
  ("workspace_star", "workspace:*"),
  ("workspace_caret", "workspace:^1.2.3"),
  // Non-semver
  ("catalog", "catalog:react18"),
  ("alias", "npm:lodash@^4.17.21"),
  ("tag", "beta"),
  ("git_github", "github:user/repo#v1.2.3"),
  ("file", "file:../packages/foo"),
  ("link", "link:../packages/foo"),
  ("url", "https://example.com/package.tgz"),
  ("unsupported", "}wat{"),
  ("empty", ""),
];

fn bench_specifier_create(c: &mut Criterion) {
  let mut group = c.benchmark_group("Specifier::create");
  group.warm_up_time(Duration::from_secs(1));
  group.measurement_time(Duration::from_secs(2));
  for &(name, input) in INPUTS {
    group.bench_with_input(BenchmarkId::new("create", name), input, |b, input| {
      b.iter(|| Specifier::create(black_box(input)))
    });
  }
  group.finish();
}

criterion_group!(benches, bench_specifier_create);
criterion_main!(benches);
