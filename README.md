# syncpack

> Manage multiple package.json files, such as in Lerna Monorepos and Yarn/Pnpm
> Workspaces

[![NPM version](http://img.shields.io/npm/v/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![NPM downloads](http://img.shields.io/npm/dm/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![Build Status](https://img.shields.io/github/workflow/status/JamieMason/syncpack/ci)](https://github.com/JamieMason/syncpack/actions)
[![Maintainability](https://api.codeclimate.com/v1/badges/516439365fdd0e3c6526/maintainability)](https://codeclimate.com/github/JamieMason/syncpack/maintainability)

## 🌩 Installation

```bash
npm install --global syncpack
```

## 🤖 GitHub Action

As of May 2022 there is now a
[Syncpack GitHub Action](https://github.com/marketplace/actions/syncpack-synchronise-monorepo-dependency-versions).
It is new and less stable than syncpack itself, but please give it a try and
[give your feedback](https://github.com/JamieMason/syncpack-github-action/issues/new).

## 📝 Commands

### fix-mismatches

Ensure that multiple packages requiring the same dependency define the same
version, so that every package requires eg. `react@16.4.2`, instead of a
combination of `react@16.4.2`, `react@0.15.9`, and `react@16.0.0`.

See [`versionGroups`](#versiongroups) if you have advanced requirements.

<details>
<summary>Options</summary>

```
-s, --source [pattern]  glob pattern for package.json files to read from
-f, --filter [pattern]  only include dependencies whose name matches this regex
-p, --prod              include dependencies
-d, --dev               include devDependencies
-P, --peer              include peerDependencies
-R, --resolutions       include resolutions (yarn)
-o, --overrides         include overrides (npm)
-O, --pnpmOverrides     include overrides (pnpm)
-w, --workspace         include locally developed package versions
-i, --indent [value]    override indentation. defaults to "  "
-c, --config <path>     path to a syncpack config file
-h, --help              display help for command
```

</details>

<details>
<summary>Examples</summary>

```bash
# uses defaults for resolving packages
syncpack fix-mismatches
# uses packages defined by --source when provided
syncpack fix-mismatches --source "apps/*/package.json"
# multiple globs can be provided like this
syncpack fix-mismatches --source "apps/*/package.json" --source "core/*/package.json"
# uses dependencies regular expression defined by --filter when provided
syncpack fix-mismatches --filter "typescript|tslint"
# only inspect "devDependencies"
syncpack fix-mismatches --dev
# only inspect "devDependencies" and "peerDependencies"
syncpack fix-mismatches --dev --peer
# indent package.json with 4 spaces instead of 2
syncpack fix-mismatches --indent "    "
```

</details>

### format

Organise package.json files according to a conventional format, where fields
appear in a predictable order and nested fields are ordered alphabetically.
Shorthand properties are used where available, such as the `"repository"` and
`"bugs"` fields.

<details>
<summary>Options</summary>

```
-s, --source [pattern]  glob pattern for package.json files to read from
-i, --indent [value]    override indentation. defaults to "  "
-c, --config <path>     path to a syncpack config file
-h, --help              display help for command
```

</details>

<details>
<summary>Examples</summary>

```bash
# uses defaults for resolving packages
syncpack format
# uses packages defined by --source when provided
syncpack format --source "apps/*/package.json"
# multiple globs can be provided like this
syncpack format --source "apps/*/package.json" --source "core/*/package.json"
# indent package.json with 4 spaces instead of 2
syncpack format --indent "    "
```

</details>

### lint-semver-ranges

Check whether dependency versions used within "dependencies", "devDependencies",
and "peerDependencies" follow a consistent format.

See [`semverGroups`](#semvergroups) if you have advanced requirements.

<details>
<summary>Options</summary>

```
-s, --source [pattern]      glob pattern for package.json files to read from
-p, --prod                  include dependencies
-d, --dev                   include devDependencies
-P, --peer                  include peerDependencies
-R, --resolutions           include resolutions (yarn)
-o, --overrides             include overrides (npm)
-O, --pnpmOverrides         include overrides (pnpm)
-f, --filter [pattern]      only include dependencies whose name matches this regex
-r, --semver-range <range>  see supported ranges below. defaults to ""
-c, --config <path>         path to a syncpack config file
-w, --workspace             include locally developed package versions
-h, --help                  display help for command
```

</details>

<details>
<summary>Examples</summary>

```bash
# uses defaults for resolving packages
syncpack lint-semver-ranges
# uses packages defined by --source when provided
syncpack lint-semver-ranges --source "apps/*/package.json"
# multiple globs can be provided like this
syncpack lint-semver-ranges --source "apps/*/package.json" --source "core/*/package.json"
# uses dependencies regular expression defined by --filter when provided
syncpack lint-semver-ranges --filter "typescript|tslint"
# use ~ range instead of default ""
syncpack lint-semver-ranges --semver-range ~
# use ~ range in "devDependencies"
syncpack lint-semver-ranges --dev --semver-range ~
# use ~ range in "devDependencies" and "peerDependencies"
syncpack lint-semver-ranges --dev --peer --semver-range ~
```

</details>

### list

List all dependencies required by your packages.

<details>
<summary>Options</summary>

```
-s, --source [pattern]  glob pattern for package.json files to read from
-p, --prod              include dependencies
-d, --dev               include devDependencies
-P, --peer              include peerDependencies
-R, --resolutions       include resolutions (yarn)
-o, --overrides         include overrides (npm)
-O, --pnpmOverrides     include overrides (pnpm)
-f, --filter [pattern]  only include dependencies whose name matches this regex
-c, --config <path>     path to a syncpack config file
-w, --workspace         include locally developed package versions
-h, --help              display help for command
```

</details>

<details>
<summary>Examples</summary>

```bash
# uses defaults for resolving packages
syncpack list
# uses packages defined by --source when provided
syncpack list --source "apps/*/package.json"
# multiple globs can be provided like this
syncpack list --source "apps/*/package.json" --source "core/*/package.json"
# uses dependencies regular expression defined by --filter when provided
syncpack list --filter "typescript|tslint"
# only inspect "devDependencies"
syncpack list --dev
# only inspect "devDependencies" and "peerDependencies"
syncpack list --dev --peer
```

</details>

### list-mismatches

List dependencies which are required by multiple packages, where the version is
not the same across every package.

See [`versionGroups`](#versiongroups) if you have advanced requirements.

<details>
<summary>Options</summary>

```
-s, --source [pattern]  glob pattern for package.json files to read from
-p, --prod              include dependencies
-d, --dev               include devDependencies
-P, --peer              include peerDependencies
-R, --resolutions       include resolutions (yarn)
-o, --overrides         include overrides (npm)
-O, --pnpmOverrides     include overrides (pnpm)
-f, --filter [pattern]  only include dependencies whose name matches this regex
-c, --config <path>     path to a syncpack config file
-w, --workspace         include locally developed package versions
-h, --help              display help for command
```

</details>

<details>
<summary>Examples</summary>

```bash
# uses defaults for resolving packages
syncpack list-mismatches
# uses packages defined by --source when provided
syncpack list-mismatches --source "apps/*/package.json"
# multiple globs can be provided like this
syncpack list-mismatches --source "apps/*/package.json" --source "core/*/package.json"
# uses dependencies regular expression defined by --filter when provided
syncpack list-mismatches --filter "typescript|tslint"
# only inspect "devDependencies"
syncpack list-mismatches --dev
# only inspect "devDependencies" and "peerDependencies"
syncpack list-mismatches --dev --peer
```

</details>

### set-semver-ranges

Ensure dependency versions used within `"dependencies"`, `"devDependencies"`,
and `"peerDependencies"` follow a consistent format.

See [`semverGroups`](#semvergroups) if you have advanced requirements.

<details>
<summary>Options</summary>

```
-s, --source [pattern]      glob pattern for package.json files to read from
-r, --semver-range <range>  see supported ranges below. defaults to ""
-f, --filter [pattern]      only include dependencies whose name matches this regex
-p, --prod                  include dependencies
-d, --dev                   include devDependencies
-P, --peer                  include peerDependencies
-R, --resolutions           include resolutions (yarn)
-o, --overrides             include overrides (npm)
-O, --pnpmOverrides         include overrides (pnpm)
-w, --workspace             include locally developed package versions
-i, --indent [value]        override indentation. defaults to "  "
-r, --semver-range <range>  see supported ranges below. defaults to ""
-c, --config <path>         path to a syncpack config file
-h, --help                  display help for command
```

</details>

<details>
<summary>Examples</summary>

```bash
# uses defaults for resolving packages
syncpack set-semver-ranges
# uses packages defined by --source when provided
syncpack set-semver-ranges --source "apps/*/package.json"
# multiple globs can be provided like this
syncpack set-semver-ranges --source "apps/*/package.json" --source "core/*/package.json"
# uses dependencies regular expression defined by --filter when provided
syncpack set-semver-ranges --filter "typescript|tslint"
# use ~ range instead of default ""
syncpack set-semver-ranges --semver-range ~
# set ~ range in "devDependencies"
syncpack set-semver-ranges --dev --semver-range ~
# set ~ range in "devDependencies" and "peerDependencies"
syncpack set-semver-ranges --dev --peer --semver-range ~
# indent package.json with 4 spaces instead of 2
syncpack set-semver-ranges --indent "    "
```

</details>

## 🛠 Configuration File

Creating a configuration file is optional, syncpack will search up the directory
tree in the following places:

- a `syncpack` property in `package.json`
- a `.syncpackrc` file in JSON or YAML format
- a `.syncpackrc.json`, `.syncpackrc.yaml`, `.syncpackrc.yml`, `.syncpackrc.js`,
  or `.syncpackrc.cjs` file
- a `syncpack.config.js` or `syncpack.config.cjs` CommonJS module exporting an
  object

If you want to specify a path to a configuration file, overriding the discovered
configuration file (if present), you can use the `--config` option.

### Default Configuration

```json
{
  "dev": true,
  "filter": ".",
  "indent": "  ",
  "overrides": true,
  "peer": true,
  "pnpmOverrides": true,
  "prod": true,
  "resolutions": true,
  "workspace": true,
  "semverGroups": [],
  "semverRange": "",
  "sortAz": [
    "contributors",
    "dependencies",
    "devDependencies",
    "keywords",
    "peerDependencies",
    "resolutions",
    "scripts"
  ],
  "sortFirst": ["name", "description", "version", "author"],
  "source": [],
  "versionGroups": []
}
```

### `dev`, `peer`, `prod`, `resolutions`, `overrides`, `pnpmOverrides`, and `workspace`

Whether to search within `devDependencies`, `peerDependencies`, `dependencies`,
`resolutions` (Yarn), `overrides` (npm), `pnpmOverrides` (pnpm), and the
`version` property of the package.json files of your own packages developed
within your `workspace` respectively.

All of these locations are searched by default but they can be disabled
individually in your config file. If any are set via the command line options
`--dev`, `--peer`, `--prod`, `--resolutions`, `--overrides`, `--pnpmOverrides`,
or `--workspace` then only the options you provide will be searched.

### `filter`

A string which will be passed to `new RegExp()` to match against package names
that should be included.

### `indent`

The character(s) to be used to indent your package.json files when writing to
disk.

### `semverRange`

Defaulted to `""` to ensure that exact dependency versions are used instead of
loose ranges, but this can be overridden in your config file or via the
`--semver-range` command line option.

#### Supported Ranges

```
<  <1.4.2
<= <=1.4.2
"" 1.4.2
~  ~1.4.2
^  ^1.4.2
>= >=1.4.2
>  >1.4.2
*  *
```

### `sortAz`

When using the `format` command, determines which fields within package.json
files should be sorted alphabetically. When the value is an Object, its keys are
sorted alphabetically. When the value is an Array, its values are sorted
alphabetically. There is no equivalent CLI Option for this configuration.

### `sortFirst`

When using the `format` command, determines which fields within package.json
files should appear at the top, and in what order. There is no equivalent CLI
Option for this configuration.

### `source`

Defaults to `["package.json", "packages/*/package.json"]` to match most Projects
using Lerna or Yarn Workspaces, but this can be overridden in your config file
or via multiple `--source` command line options. Supports any patterns supported
by [glob](https://github.com/isaacs/node-glob).

### `versionGroups`

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

See [`semverGroups`](#semverGroups) for more examples, they work the same way.

#### `versionGroup.dependencies`

Required. An array of minimatch glob patterns which should match the key of
dependencies defined in your package.json files.

| Pattern                  | Matches                                  |
| ------------------------ | ---------------------------------------- |
| `["**"]`                 | Any dependency                           |
| `["@aws-sdk/**"]`        | Any dependency with the scope `@aws-sdk` |
| `["react", "react-dom"]` | Specific dependencies by name            |

#### `versionGroup.packages`

Required. An array of minimatch glob patterns which should match the `name`
property of packages developed within your monorepo.

| Pattern                      | Matches                               |
| ---------------------------- | ------------------------------------- |
| `["**"]`                     | Any package                           |
| `["@my-repo/**"]`            | Any package with the scope `@my-repo` |
| `["my-server", "my-client"]` | Specific packages by name             |

#### `versionGroup.dependencyTypes`

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

#### `versionGroup.isBanned`

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

#### `versionGroup.pinVersion`

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

### `semverGroups`

Allow some packages to have different semver range rules to the rest of your
monorepo. Each dependency can only belong to one semver group, the first rule
which matches a given dependency and package will apply.

#### Example use cases

1: Every dependency of `@myrepo/library` should have a semver range of `~`,
regardless of what the rest of the monorepo uses:

```json
{
  "semverGroups": [
    {
      "range": "~",
      "dependencies": ["**"],
      "packages": ["@myrepo/library"]
    }
  ]
}
```

2: Every dependency of `@myrepo/library` whose name matches `@alpha/**` should
have a semver range of `^`, regardless of what the rest of that package or the
rest of the monorepo uses:

```json
{
  "semverGroups": [
    {
      "range": "^",
      "dependencies": ["@alpha/**"],
      "packages": ["@myrepo/library"]
    }
  ]
}
```

3: Every dependency in the monorepo whose name matches `@alpha/**` should have a
semver range of `~`, regardless of what the rest of the monorepo uses:

```json
{
  "semverGroups": [
    {
      "range": "~",
      "dependencies": ["@alpha/**"],
      "packages": ["**"]
    }
  ]
}
```

3: Production dependencies should have fixed version numbers, but development
and peer dependencies can be broader.

```json
{
  "semverGroups": [
    {
      "range": "",
      "dependencyTypes": [
        "prod",
        "resolutions",
        "overrides",
        "pnpmOverrides",
        "workspace"
      ],
      "dependencies": ["**"],
      "packages": ["**"]
    },
    {
      "range": "~",
      "dependencyTypes": ["dev"],
      "dependencies": ["**"],
      "packages": ["**"]
    },
    {
      "range": "^",
      "dependencyTypes": ["peer"],
      "dependencies": ["**"],
      "packages": ["**"]
    }
  ]
}
```

#### `semverGroup.range`

Which of the [Supported Ranges](#supported-ranges) this group should use.

#### `semverGroup.dependencies`

Works the same as [`semverGroup.dependencies`](#semvergroupdependencies).

#### `semverGroup.packages`

Works the same as [`semverGroup.packages`](#semvergrouppackages).

#### `semverGroup.dependencyTypes`

Works the same as [`semverGroup.dependencyTypes`](#semvergroupdependencytypes).

## 🕵🏾‍♀️ Resolving Packages

package.json files are resolved in this order of precendence:

1.  If `--source`
    [glob patterns](https://github.com/isaacs/node-glob#glob-primer) are
    provided, use those.
2.  If using [Yarn Workspaces](https://yarnpkg.com/lang/en/docs/workspaces/),
    read `workspaces` from `./package.json`.
3.  If using [Lerna](https://lerna.js.org/), read `packages` from
    `./lerna.json`.
4.  If using [Pnpm](https://pnpm.js.org/), read `packages` from
    `./pnpm-workspace.yaml`.
5.  Default to `'package.json'` and `'packages/*/package.json'`.

> 👋 Always add quotes around your `--source` patterns [[more info](https://github.com/JamieMason/syncpack/issues/66#issuecomment-1146011769)].

## 🙋🏿‍♀️ Getting Help

Get help with issues by creating a [Bug Report] or discuss ideas by opening a
[Feature Request].

[bug report]:
  https://github.com/JamieMason/syncpack/issues/new?template=bug_report.md
[feature request]:
  https://github.com/JamieMason/syncpack/issues/new?template=feature_request.md

## 👀 Other Projects

If you find my Open Source projects useful, please share them ❤️

- [**eslint-formatter-git-log**](https://github.com/JamieMason/eslint-formatter-git-log)<br>ESLint
  Formatter featuring Git Author, Date, and Hash
- [**eslint-plugin-move-files**](https://github.com/JamieMason/eslint-plugin-move-files)<br>Move
  and rename files while keeping imports up to date
- [**eslint-plugin-prefer-arrow-functions**](https://github.com/JamieMason/eslint-plugin-prefer-arrow-functions)<br>Convert
  functions to arrow functions
- [**ImageOptim-CLI**](https://github.com/JamieMason/ImageOptim-CLI)<br>Automates
  ImageOptim, ImageAlpha, and JPEGmini for Mac to make batch optimisation of
  images part of your automated build process.
- [**Jasmine-Matchers**](https://github.com/JamieMason/Jasmine-Matchers)<br>Write
  Beautiful Specs with Custom Matchers
- [**karma-benchmark**](https://github.com/JamieMason/karma-benchmark)<br>Run
  Benchmark.js over multiple Browsers, with CI compatible output
- [**self-help**](https://github.com/JamieMason/self-help#readme)<br>Interactive
  Q&A Guides for Web and the Command Line
