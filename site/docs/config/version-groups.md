---
id: version-groups
title: versionGroups
---

The most common use case for version groups is when some of the packages in your
Monorepo are considered alpha (or legacy). Since those packages are much further
ahead (or behind) the other packages, the dependencies within those packages
need to be managed differently to the rest of the Monorepo.

Your alpha packages might use unstable versions of some dependencies, while the
rest of the repo might need to remain on stable versions.

You don't want mismatches within your alpha packages, you don't want mismatches
within the other packages, but you _do_ want those groups to use different
versions _to each other_ and not have `syncpack` make them all the same.

In the following example, 2 of our packages are using different versions of
`react` and `react-dom` to the rest of the project.

```json
{
  "versionGroups": [
    {
      "dependencies": ["react", "react-dom"],
      "packages": ["@alpha/server", "@alpha/ui"]
    }
  ]
}
```

> 👋 The `dependencies` and `packages` fields are processed using
> [minimatch](https://github.com/isaacs/minimatch), so the above example can
> also be written as `"packages": ["@alpha/**"]`.

`syncpack` will make ensure that:

- The versions of `react` and `react-dom` are the same within `@alpha/server`
  and `@alpha/ui`.
- The versions of `react` and `react-dom` are the same across every package
  except `@alpha/server` and `@alpha/ui`.
- The versions of `react` and `react-dom` within `@alpha/server` and `@alpha/ui`
  can be different to the other packages in the monorepo.
- The versions of every other dependency in the monorepo (eg `lodash`) are the
  same across every package including `@alpha/server` and `@alpha/ui`.

Each dependency can only belong to one version group, the first rule which
matches a given dependency and package will apply.

You can be quite granular with these rules, so the partitioning doesn't _have_
to apply to an entire package:

- A specific dependency in a specific package.
- A specific dependency in some specific packages only.
- Any dependency who name matches a pattern such as `@aws-sdk/**`.

See [`semverGroups`](./semver-groups.md) for more examples, they work the same
way.

## versionGroup.dependencies

Required. An array of minimatch glob patterns which should match the key of
dependencies defined in your package.json files.

| Pattern                  | Matches                                  |
| ------------------------ | ---------------------------------------- |
| `["**"]`                 | Any dependency                           |
| `["@aws-sdk/**"]`        | Any dependency with the scope `@aws-sdk` |
| `["react", "react-dom"]` | Specific dependencies by name            |

## versionGroup.packages

Required. An array of minimatch glob patterns which should match the `name`
property of packages developed within your monorepo.

| Pattern                      | Matches                               |
| ---------------------------- | ------------------------------------- |
| `["**"]`                     | Any package                           |
| `["@my-repo/**"]`            | Any package with the scope `@my-repo` |
| `["my-server", "my-client"]` | Specific packages by name             |

## versionGroup.dependencyTypes

Optional. If set, will result in only the dependency types included in that
array being considered a match for this version group.

In this example we define that all dependencies within `peerDependencies` in the
repo must match, regardless of what versions of the same dependencies might be
used in `dependencies` or `devDependencies`.

```json
{
  "versionGroups": [
    {
      "dependencies": ["**"],
      "dependencyTypes": ["peerDependencies"],
      "packages": ["**"]
    }
  ]
}
```

## versionGroup.isBanned

Remove dependencies which you've decided should never be allowed.

```json
{
  "versionGroups": [
    {
      "dependencies": ["never-gonna"],
      "isBanned": true,
      "packages": ["**"]
    }
  ]
}
```

## versionGroup.isIgnored

Have syncpack ignore these dependencies completely.

```json
{
  "versionGroups": [
    {
      "dependencies": ["**"],
      "isIgnored": true,
      "packages": ["oops-moment", "workaround"]
    }
  ]
}
```

## versionGroup.pinVersion

Pin the version of all dependencies in this group to match this specific version
you've defined.

```json
{
  "versionGroups": [
    {
      "dependencies": ["@aws-sdk/**"],
      "packages": ["**"],
      "pinVersion": "3.55.0"
    }
  ]
}
```
