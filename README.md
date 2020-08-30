# syncpack

> Manage multiple package.json files, such as in Lerna Monorepos and Yarn/Pnpm Workspaces

[![NPM version](http://img.shields.io/npm/v/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![NPM downloads](http://img.shields.io/npm/dm/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![Build Status](http://img.shields.io/travis/JamieMason/syncpack/master.svg?style=flat-square)](https://travis-ci.org/JamieMason/syncpack)
[![Maintainability](https://api.codeclimate.com/v1/badges/516439365fdd0e3c6526/maintainability)](https://codeclimate.com/github/JamieMason/syncpack/maintainability)

## Table of Contents

- [🌩 Installation](#-installation)
- [📝 Commands](#-commands)
  - [fix-mismatches](#fix-mismatches)
  - [format](#format)
  - [list](#list)
  - [list-mismatches](#list-mismatches)
  - [set-semver-ranges](#set-semver-ranges)
- [🛠 Configuration File](#-configuration-file)
- [🕵🏾‍♀️ Resolving Packages](#️-resolving-packages)
- [🙋🏿‍♀️ Getting Help](#️-getting-help)
- [👀 Other Projects](#-other-projects)

## 🌩 Installation

    npm install --global syncpack

## 📝 Commands

### fix-mismatches

Ensure that multiple packages requiring the same dependency define the same version, so that every package requires eg.
`react@16.4.2`, instead of a combination of `react@16.4.2`, `react@0.15.9`, and `react@16.0.0`.

See [`versionGroups`](#versiongroups) if you have advanced requirements.

<details>
<summary>Options</summary>

    -s, --source [pattern]  glob pattern for package.json files to read from
    -p, --prod              include dependencies
    -d, --dev               include devDependencies
    -P, --peer              include peerDependencies
    -f, --filter [pattern]  regex for dependency filter
    -i, --indent [value]    override indentation. defaults to "  "
    -h, --help              output usage information

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

Organise package.json files according to a conventional format, where fields appear in a predictable order and nested
fields are ordered alphabetically. Shorthand properties are used where available, such as the `"repository"` and
`"bugs"` fields.

<details>
<summary>Options</summary>

    -s, --source [pattern]  glob pattern for package.json files to read from
    -i, --indent [value]    override indentation. defaults to "  "
    -h, --help              output usage information

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

### list

List all dependencies required by your packages.

<details>
<summary>Options</summary>

    -s, --source [pattern]  glob pattern for package.json files to read from
    -p, --prod              include dependencies
    -d, --dev               include devDependencies
    -P, --peer              include peerDependencies
    -f, --filter [pattern]  regex for dependency filter
    -h, --help              output usage information

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

List dependencies which are required by multiple packages, where the version is not the same across every package.

See [`versionGroups`](#versiongroups) if you have advanced requirements.

<details>
<summary>Options</summary>

    -s, --source [pattern]  glob pattern for package.json files to read from
    -p, --prod              include dependencies
    -d, --dev               include devDependencies
    -P, --peer              include peerDependencies
    -f, --filter [pattern]  regex for dependency filter
    -h, --help              output usage information

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

Ensure dependency versions used within `"dependencies"`, `"devDependencies"`, and `"peerDependencies"` follow a
consistent format.

<details>
<summary>Options</summary>

    -s, --source [pattern]      glob pattern for package.json files to read from
    -p, --prod                  include dependencies
    -d, --dev                   include devDependencies
    -P, --peer                  include peerDependencies
    -f, --filter [pattern]      regex for dependency filter
    -i, --indent [value]        override indentation. defaults to "  "
    -r, --semver-range <range>  see supported ranges below. defaults to ""
    -h, --help                  output usage information

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

Creating a configuration file is optional, syncpack will search up the directory tree in the following places:

- a `syncpack` property in `package.json`
- a `.syncpackrc` file in JSON or YAML format
- a `.syncpackrc.json`, `.syncpackrc.yaml`, `.syncpackrc.yml`, `.syncpackrc.js`, or `.syncpackrc.cjs` file
- a `syncpack.config.js` or `syncpack.config.cjs` CommonJS module exporting an object

### Default Configuration

```json
{
  "dev": true,
  "filter": ".",
  "indent": "  ",
  "peer": true,
  "prod": true,
  "semverRange": "",
  "sortAz": ["contributors", "dependencies", "devDependencies", "keywords", "peerDependencies", "scripts"],
  "sortFirst": ["name", "description", "version", "author"],
  "source": ["package.json", "packages/*/package.json"],
  "versionGroups": []
}
```

### `dev`, `peer`, and `prod`

Whether to search within `devDependencies`, `peerDependencies`, and `dependencies` respectively. All of these locations
are searched by default but they can be disabled individually in your config file. If any are set via the command line
options `--dev`, `--peer`, or `--prod` then only the options you provide will be searched.

### `filter`

A string which will be passed to `new RegExp()` to match against package names that should be included.

### `indent`

The character(s) to be used to indent your package.json files when writing to disk.

### `semverRange`

Defaulted to `""` to ensure that exact dependency versions are used instead of loose ranges, but this can be overridden
in your config file or via the `--semver-range` command line option.

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

When using the `format` command, determines which fields within package.json files should be sorted alphabetically. When
the value is an Object, its keys are sorted alphabetically. When the value is an Array, its values are sorted
alphabetically. There is no equivalent CLI Option for this configuration.

### `sortFirst`

When using the `format` command, determines which fields within package.json files should appear at the top, and in what
order. There is no equivalent CLI Option for this configuration.

### `source`

Defaults to `["package.json", "packages/*/package.json"]` to match most Projects using Lerna or Yarn Workspaces, but
this can be overridden in your config file or via multiple `--source` command line options. Supports any patterns
supported by [glob](https://github.com/isaacs/node-glob).

### `versionGroups`

If some packages in your Monorepo relate to "alpha" (or legacy) versions of your software, you will need to manage
dependencies differently within those packages. Your alpha packages might use latest or unstable versions of some 3rd
party dependencies, while the rest of the repo might need to remain on older versions. You don't want mismatches within
your alpha packages, or within the other packages in your monorepo – but you do want those groups to use different
versions to each other and not have `syncpack fix-mismatches` make them all the same.

In the following example, 2 packages in our monorepo are using different versions of `react` and `react-dom` to the rest
of the project.

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

`syncpack` will make ensure that:

- The versions of `react` and `react-dom` are the same within `@alpha/server` and `@alpha/ui`.
- The versions of `react` and `react-dom` are the same across every package except `@alpha/server` and `@alpha/ui`.
- The versions of `react` and `react-dom` within `@alpha/server` and `@alpha/ui` can be different to the other packages
  in the monorepo.
- The versions of every other dependency in the monorepo (eg `lodash`) are the same across every package including
  `@alpha/server` and `@alpha/ui`.

## 🕵🏾‍♀️ Resolving Packages

package.json files are resolved in this order of precendence:

1.  If `--source` [glob patterns](https://github.com/isaacs/node-glob#glob-primer) are provided, use those.
2.  If using [Yarn Workspaces](https://yarnpkg.com/lang/en/docs/workspaces/), read `workspaces` from `./package.json`.
3.  If using [Lerna](https://lerna.js.org/), read `packages` from `./lerna.json`.
4.  If using [Pnpm](https://pnpm.js.org/), read `packages` from `./pnpm-workspace.yaml`.
5.  Default to `'package.json'` and `'packages/*/package.json'`.

## 🙋🏿‍♀️ Getting Help

Get help with issues by creating a [Bug Report] or discuss ideas by opening a [Feature Request].

[bug report]: https://github.com/JamieMason/syncpack/issues/new?template=bug_report.md
[feature request]: https://github.com/JamieMason/syncpack/issues/new?template=feature_request.md

## 👀 Other Projects

If you find my Open Source projects useful, please share them ❤️

- [**eslint-formatter-git-log**](https://github.com/JamieMason/eslint-formatter-git-log)<br>ESLint Formatter featuring
  Git Author, Date, and Hash
- [**eslint-plugin-move-files**](https://github.com/JamieMason/eslint-plugin-move-files)<br>Move and rename files while
  keeping imports up to date
- [**eslint-plugin-prefer-arrow-functions**](https://github.com/JamieMason/eslint-plugin-prefer-arrow-functions)<br>Convert
  functions to arrow functions
- [**ImageOptim-CLI**](https://github.com/JamieMason/ImageOptim-CLI)<br>Automates ImageOptim, ImageAlpha, and JPEGmini
  for Mac to make batch optimisation of images part of your automated build process.
- [**Jasmine-Matchers**](https://github.com/JamieMason/Jasmine-Matchers)<br>Write Beautiful Specs with Custom Matchers
- [**karma-benchmark**](https://github.com/JamieMason/karma-benchmark)<br>Run Benchmark.js over multiple Browsers, with
  CI compatible output
- [**self-help**](https://github.com/JamieMason/self-help#readme)<br>Interactive Q&A Guides for Web and the Command Line
