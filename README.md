# syncpack

> Manage multiple package.json files, such as in Lerna Monorepos and Yarn
> Workspaces

[![NPM version](http://img.shields.io/npm/v/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![NPM downloads](http://img.shields.io/npm/dm/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![Build Status](http://img.shields.io/travis/JamieMason/syncpack/master.svg?style=flat-square)](https://travis-ci.org/JamieMason/syncpack)
[![Maintainability](https://api.codeclimate.com/v1/badges/516439365fdd0e3c6526/maintainability)](https://codeclimate.com/github/JamieMason/syncpack/maintainability)
[![Gitter Chat](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/JamieMason/syncpack)
[![Donate via PayPal](https://img.shields.io/badge/donate-paypal-blue.svg)](https://www.paypal.me/foldleft)
[![Backers](https://opencollective.com/fold_left/backers/badge.svg)](https://opencollective.com/fold_left#backer)
[![Sponsors](https://opencollective.com/fold_left/sponsors/badge.svg)](https://opencollective.com/fold_left#sponsors)
[![Analytics](https://ga-beacon.appspot.com/UA-45466560-5/syncpack?flat&useReferer)](https://github.com/igrigorik/ga-beacon)
[![Follow JamieMason on GitHub](https://img.shields.io/github/followers/JamieMason.svg?style=social&label=Follow)](https://github.com/JamieMason)
[![Follow fold_left on Twitter](https://img.shields.io/twitter/follow/fold_left.svg?style=social&label=Follow)](https://twitter.com/fold_left)

## ☁️ Installation

```
npm install --global syncpack
```

## 🕵🏾‍♀️ Resolving Packages

package.json files are resolved in this order of precendence:

1. If `--source`
   [glob patterns](https://github.com/isaacs/node-glob#glob-primer) are
   provided, use those.
1. If using [Yarn Workspaces](https://yarnpkg.com/lang/en/docs/workspaces/),
   read `workspaces` from `./package.json`.
1. If using [Lerna](https://lerna.js.org/), read `packages` from `./lerna.json`.
1. Default to `'package.json'` and `'packages/*/package.json'`.

## 📝 Commands

### fix-mismatches

Ensure that multiple packages requiring the same dependency define the same
version, so that every package requires eg. `react@16.4.2`, instead of a
combination of `react@16.4.2`, `react@0.15.9`, and `react@16.0.0`.

<details>
<summary>Options</summary>

```
-s, --source [pattern]  glob pattern for package.json files to read from
-p, --prod              include dependencies
-d, --dev               include devDependencies
-P, --peer              include peerDependencies
-i, --indent [value]    override indentation. defaults to "  "
-h, --help              output usage information
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
# uses packages that pass the regex defined by --filter when provided
syncpack fix-mismatches --filter "^package_name$"
# only fix "devDependencies"
syncpack fix-mismatches --dev
# only fix "devDependencies" and "peerDependencies"
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
-h, --help              output usage information
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

### list

List all dependencies required by your packages.

<details>
<summary>Options</summary>

```
-s, --source [pattern]  glob pattern for package.json files to read from
-p, --prod              include dependencies
-d, --dev               include devDependencies
-P, --peer              include peerDependencies
-h, --help              output usage information
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
# only inspect "devDependencies"
syncpack list --dev
# only inspect "devDependencies" and "peerDependencies"
syncpack list --dev --peer
```

</details>

### list-mismatches

List dependencies which are required by multiple packages, where the version is
not the same across every package.

<details>
<summary>Options</summary>

```
-s, --source [pattern]  glob pattern for package.json files to read from
-p, --prod              include dependencies
-d, --dev               include devDependencies
-P, --peer              include peerDependencies
-h, --help              output usage information
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
# only list "devDependencies"
syncpack list-mismatches --dev
# only list "devDependencies" and "peerDependencies"
syncpack list-mismatches --dev --peer
```

</details>

### set-semver-ranges

Ensure dependency versions used within `"dependencies"`, `"devDependencies"`,
and `"peerDependencies"` follow a consistent format.

<details>
<summary>Options</summary>

```
-r, --semver-range <range>  <, <=, "", ~, ^, >=, >, or *. defaults to ""
-s, --source [pattern]      glob pattern for package.json files to read from
-p, --prod                  include dependencies
-d, --dev                   include devDependencies
-P, --peer                  include peerDependencies
-i, --indent [value]        override indentation. defaults to "  "
-h, --help                  output usage information
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

<details>
<summary>Supported Ranges</summary>

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

</details>

## 🙋 Get Help

There are a few ways to get help:

1.  For bug reports and feature requests, open issues 🐛
1.  For direct and quick help, you can use Gitter 🚀
