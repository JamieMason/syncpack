// Single source of truth for which severity keys each version group accepts.
// Used by:
//   - site/src/_partials/severity/group-section.mdx (per-group page rendering)
//   - site/src/content/docs/config/severity.mdx ('Configurable keys per group' table)
// When the Rust validation in src/version_group/severity.rs changes, update this map.

export const KEYS = {
  banned: [{ name: "IsBanned", slug: "is-banned" }],
  pinned: [
    { name: "DiffersToPin", slug: "differs-to-pin" },
    { name: "PinOverridesSemverRange", slug: "pin-overrides-semver-range" },
    {
      name: "PinOverridesSemverRangeMismatch",
      slug: "pin-overrides-semver-range-mismatch",
    },
    { name: "RefuseToPinLocal", slug: "refuse-to-pin-local" },
  ],
  highestSemver: [
    { name: "SemverRangeMismatch", slug: "semver-range-mismatch" },
    { name: "DiffersToLocal", slug: "differs-to-local" },
    { name: "DiffersToCatalog", slug: "differs-to-catalog" },
    {
      name: "DiffersToHighestOrLowestSemver",
      slug: "differs-to-highest-or-lowest-semver",
    },
  ],
  lowestSemver: [
    { name: "SemverRangeMismatch", slug: "semver-range-mismatch" },
    { name: "DiffersToLocal", slug: "differs-to-local" },
    { name: "DiffersToCatalog", slug: "differs-to-catalog" },
    {
      name: "DiffersToHighestOrLowestSemver",
      slug: "differs-to-highest-or-lowest-semver",
    },
  ],
  sameRange: [{ name: "SemverRangeMismatch", slug: "semver-range-mismatch" }],
  semverRangeOnly: [
    { name: "SemverRangeMismatch", slug: "semver-range-mismatch" },
  ],
  sameMinor: [
    {
      name: "DiffersToHighestOrLowestSemverMinor",
      slug: "differs-to-highest-or-lowest-semver-minor",
    },
    { name: "SemverRangeMismatch", slug: "semver-range-mismatch" },
    {
      name: "SameMinorOverridesSemverRange",
      slug: "same-minor-overrides-semver-range",
    },
    {
      name: "SameMinorOverridesSemverRangeMismatch",
      slug: "same-minor-overrides-semver-range-mismatch",
    },
  ],
  snappedTo: [
    { name: "DiffersToSnapTarget", slug: "differs-to-snap-target" },
    { name: "SemverRangeMismatch", slug: "semver-range-mismatch" },
    { name: "RefuseToSnapLocal", slug: "refuse-to-snap-local" },
  ],
  catalog: [
    { name: "NotUsingCatalog", slug: "not-using-catalog" },
    { name: "MissingFromCatalog", slug: "missing-from-catalog" },
  ],
  ignored: [],
};

export const GROUP_LABELS = {
  banned: "Banned",
  pinned: "Pinned",
  highestSemver: "Highest Semver",
  lowestSemver: "Lowest Semver",
  sameRange: "Same Range",
  semverRangeOnly: "Range Only",
  sameMinor: "Same Minor",
  snappedTo: "Snapped To",
  catalog: "Catalog",
  ignored: "Ignored",
};

export const GROUP_LINKS = {
  banned: "/version-groups/banned/",
  pinned: "/version-groups/pinned/",
  highestSemver: "/version-groups/highest-semver/",
  lowestSemver: "/version-groups/lowest-semver/",
  sameRange: "/version-groups/same-range/",
  semverRangeOnly: "/version-groups/range-only/",
  sameMinor: "/version-groups/same-minor/",
  snappedTo: "/version-groups/snapped-to/",
  catalog: "/version-groups/catalog/",
  ignored: "/version-groups/ignored/",
};
