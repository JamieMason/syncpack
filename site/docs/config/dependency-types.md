---
id: dependency-types
title: dependencyTypes
---

All of the [default dependency types](#dependency-types) are enabled by default,
but can be reduced to a smaller list via the `dependencyTypes` property of your
config file.

In this example, only dependencies found in the
[`dependencies`](https://docs.npmjs.com/cli/v9/configuring-npm/package-json#dependencies)
and
[`devDependencies`](https://docs.npmjs.com/cli/v9/configuring-npm/package-json#devDependencies)
properties of package.json files will be inspected by syncpack:

```json
{
  "dependencyTypes": ["dev", "prod"]
}
```

:::tip

The [default dependency types](#dependency-types) can be extended with your own
[`customTypes`](./custom-types.md), so you can find and fix versions found in
other parts of your package.json files.

:::

:::info

Your `dependencyTypes` configuration in your [config file](../config-file.md)
can be overridden on an ad hoc basis using the [`--types`](../option/types.md)
option.

:::

## Default dependency types

| Value           | Property in package.json                                                                          |
| --------------- | ------------------------------------------------------------------------------------------------- |
| `dev`           | [`devDependencies`](https://docs.npmjs.com/cli/v9/configuring-npm/package-json#devDependencies)   |
| `overrides`     | [`overrides`](https://docs.npmjs.com/cli/v9/configuring-npm/package-json#overrides)               |
| `peer`          | [`peerDependencies`](https://docs.npmjs.com/cli/v9/configuring-npm/package-json#peerDependencies) |
| `pnpmOverrides` | [`pnpm.overrides`](https://pnpm.io/package_json#pnpmoverrides)                                    |
| `prod`          | [`dependencies`](https://docs.npmjs.com/cli/v9/configuring-npm/package-json#dependencies)         |
| `resolutions`   | [`resolutions`](https://docs.npmjs.com/cli/v9/configuring-npm/package-json#resolutions)           |
| `workspace`     | [`version`](https://docs.npmjs.com/cli/v9/configuring-npm/package-json#version)                   |

## The `workspace` type

This option synchronises the versions of your dependencies with the
[`version`](https://docs.npmjs.com/cli/v9/configuring-npm/package-json#version)
properties of the package.json files developed in your own local
workspace/project, when they relate to eachother.

Take this example, `@your-repo/fetch` is developed in your repo:

```jsonc
{
  "name": "@your-repo/fetch",
  "version": "1.0.2"
  // ...rest of the file
}
```

and another package developed in your repo depends on it:

```jsonc
{
  "name": "@your-repo/ui",
  // ...other stuff
  "dependencies": {
    "@your-repo/fetch": "0.9.4"
  }
  // ...rest of the file
}
```

When `workspace` is enabled, syncpack will fix `@your-repo/ui` so it depends on
version `1.0.2` of `@your-repo/fetch`.
