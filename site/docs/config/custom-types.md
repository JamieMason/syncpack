---
id: custom-types
title: customTypes
---

Extend syncpack to find and fix versions in your packages which are not
available by default. Custom types behave like any other dependency, so can be
included in [versionGroups](./version-groups.md) or
[semverGroups](./semver-groups.md) etc.

The example below adds support for synchronising versions found in:

1. The
   [`engines`](https://docs.npmjs.com/cli/v9/configuring-npm/package-json#engines)
   object.
1. The [`packageManager`](https://nodejs.org/api/packages.html#packagemanager)
   string.

```json
{
  "customTypes": {
    "engines": {
      "path": "engines",
      "strategy": "versionsByName"
    },
    "packageManager": {
      "path": "packageManager",
      "strategy": "name@version"
    }
  }
}
```

## customTypes\[name\]

The key of each custom type is its name, this can be used in the following
places to toggle when it is enabled:

1. [`--types`](../option/types.md) and
   [`dependencyTypes`](./dependency-types.md).
1. [`versionGroup.dependencyTypes`](./version-groups.md#dependencytypes)
1. [`semverGroup.dependencyTypes`](./semver-groups.md#dependencytypes)

## customTypes\[name\].path

Where the version can be found in each package.json file, such as `engines`,
`packageManager` or `some.nested.property`.

## customTypes\[name\].strategy

A strategy defines how syncpack needs to read and write dependency names and
versions, there are 3 to choose from:

| Name             | Example                                |
| ---------------- | -------------------------------------- |
| `name@version`   | `pnpm@7.27.0`                          |
| `version`        | `12.4.2`                               |
| `versionsByName` | `{"pnpm":"7.27.0", "semver": "7.3.8"}` |
