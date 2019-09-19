# syncpack

> Manage multiple package.json files, such as in Lerna Monorepos and Yarn Workspaces

[![NPM version](http://img.shields.io/npm/v/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack) [![NPM downloads](http://img.shields.io/npm/dm/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack) [![Build Status](http://img.shields.io/travis/JamieMason/syncpack/master.svg?style=flat-square)](https://travis-ci.org/JamieMason/syncpack) [![Maintainability](https://api.codeclimate.com/v1/badges/516439365fdd0e3c6526/maintainability)](https://codeclimate.com/github/JamieMason/syncpack/maintainability)

## Table of Contents

-   [🌩 Installation](#-installation)
-   [🕵🏾‍♀️ Resolving Packages](#♀️-resolving-packages)
-   [📝 Commands](#-commands)
-   [🙋🏿‍♀️ Getting Help](#♀️-getting-help)
-   [👀 Other Projects](#-other-projects)
-   [🤓 Author](#-author)

## 🌩 Installation

    npm install --global syncpack

## 🕵🏾‍♀️ Resolving Packages

package.json files are resolved in this order of precendence:

1.  If `--source` [glob patterns](https://github.com/isaacs/node-glob#glob-primer) are provided, use those.
2.  If using [Yarn Workspaces](https://yarnpkg.com/lang/en/docs/workspaces/), read `workspaces` from `./package.json`.
3.  If using [Lerna](https://lerna.js.org/), read `packages` from `./lerna.json`.
4.  Default to `'package.json'` and `'packages/*/package.json'`.

## 📝 Commands

### fix-mismatches

Ensure that multiple packages requiring the same dependency define the same version, so that every package requires eg. `react@16.4.2`, instead of a combination of `react@16.4.2`, `react@0.15.9`, and `react@16.0.0`.

<details>
<summary>Options</summary>

    -s, --source [pattern]  glob pattern for package.json files to read from
    -p, --prod              include dependencies
    -d, --dev               include devDependencies
    -P, --peer              include peerDependencies
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

Organise package.json files according to a conventional format, where fields appear in a predictable order and nested fields are ordered alphabetically. Shorthand properties are used where available, such as the `"repository"` and `"bugs"` fields.

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
# only inspect "devDependencies"
syncpack list --dev
# only inspect "devDependencies" and "peerDependencies"
syncpack list --dev --peer
```

</details>

### list-mismatches

List dependencies which are required by multiple packages, where the version is not the same across every package.

<details>
<summary>Options</summary>

    -s, --source [pattern]  glob pattern for package.json files to read from
    -p, --prod              include dependencies
    -d, --dev               include devDependencies
    -P, --peer              include peerDependencies
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
# only list "devDependencies"
syncpack list-mismatches --dev
# only list "devDependencies" and "peerDependencies"
syncpack list-mismatches --dev --peer
```

</details>

### set-semver-ranges

Ensure dependency versions used within `"dependencies"`, `"devDependencies"`, and `"peerDependencies"` follow a consistent format.

<details>
<summary>Options</summary>

    -r, --semver-range <range>  <, <=, "", ~, ^, >=, >, or *. defaults to ""
    -s, --source [pattern]      glob pattern for package.json files to read from
    -p, --prod                  include dependencies
    -d, --dev                   include devDependencies
    -P, --peer                  include peerDependencies
    -i, --indent [value]        override indentation. defaults to "  "
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

    <  <1.4.2
    <= <=1.4.2
    "" 1.4.2
    ~  ~1.4.2
    ^  ^1.4.2
    >= >=1.4.2
    >  >1.4.2
    *  *

</details>

## 🙋🏿‍♀️ Getting Help

Get help with issues by creating a [Bug Report] or discuss ideas by opening a [Feature Request].

[bug report]: https://github.com/JamieMason/syncpack/issues/new?template=bug_report.md

[feature request]: https://github.com/JamieMason/syncpack/issues/new?template=feature_request.md

## 👀 Other Projects

If you find my Open Source projects useful, please share them ❤️

-   [**add-matchers**](https://github.com/JamieMason/add-matchers)<br>Write useful test matchers compatible with Jest and Jasmine.
-   [**eslint-formatter-git-log**](https://github.com/JamieMason/eslint-formatter-git-log)<br>ESLint Formatter featuring Git Author, Date, and Hash
-   [**eslint-plugin-move-files**](https://github.com/JamieMason/eslint-plugin-move-files)<br>Move and rename files while keeping imports up to date
-   [**eslint-plugin-prefer-arrow-functions**](https://github.com/JamieMason/eslint-plugin-prefer-arrow-functions)<br>Convert functions to arrow functions
-   [**get-time-between**](https://github.com/JamieMason/get-time-between#readme)<br>Measure the amount of time during work hours between two dates
-   [**image-optimisation-tools-comparison**](https://github.com/JamieMason/image-optimisation-tools-comparison)<br>A benchmarking suite for popular image optimisation tools.
-   [**ImageOptim-CLI**](https://github.com/JamieMason/ImageOptim-CLI)<br>Automates ImageOptim, ImageAlpha, and JPEGmini for Mac to make batch optimisation of images part of your automated build process.
-   [**is-office-hours**](https://github.com/JamieMason/is-office-hours#readme)<br>Determine whether a given date is within office hours
-   [**Jasmine-Matchers**](https://github.com/JamieMason/Jasmine-Matchers)<br>Write Beautiful Specs with Custom Matchers
-   [**jest-fail-on-console-reporter**](https://github.com/JamieMason/jest-fail-on-console-reporter#readme)<br>Disallow untested console output produced during tests
-   [**karma-benchmark**](https://github.com/JamieMason/karma-benchmark)<br>Run Benchmark.js over multiple Browsers, with CI compatible output
-   [**karma-jasmine-matchers**](https://github.com/JamieMason/karma-jasmine-matchers)<br>A Karma plugin - Additional matchers for the Jasmine BDD JavaScript testing library.
-   [**logservable**](https://github.com/JamieMason/logservable)<br>git log as an observable stream of JSON objects
-   [**self-help**](https://github.com/JamieMason/self-help#readme)<br>Interactive Q&A Guides for Web and the Command Line

## 🤓 Author

<img src="https://www.gravatar.com/avatar/acdf106ce071806278438d8c354adec8?s=100" align="left">

I'm [Jamie Mason] from [Leeds] in England, I began Web Design and Development in 1999 and have been Contracting and offering Consultancy as Fold Left Ltd since 2012. Who I've worked with includes [Sky Sports], [Sky Bet], [Sky Poker], The [Premier League], [William Hill], [Shell], [Betfair], and Football Clubs including [Leeds United], [Spurs], [West Ham], [Arsenal], and more.

<div align="center">

[![Follow JamieMason on GitHub][github badge]][github]      [![Follow fold_left on Twitter][twitter badge]][twitter]

</div>

<!-- images -->

[github badge]: https://img.shields.io/github/followers/JamieMason.svg?style=social&label=Follow

[twitter badge]: https://img.shields.io/twitter/follow/fold_left.svg?style=social&label=Follow

<!-- links -->

[arsenal]: https://www.arsenal.com

[betfair]: https://www.betfair.com

[github]: https://github.com/JamieMason

[jamie mason]: https://www.linkedin.com/in/jamiemasonleeds

[leeds united]: https://www.leedsunited.com/

[leeds]: https://www.instagram.com/visitleeds

[premier league]: https://www.premierleague.com

[shell]: https://www.shell.com

[sky bet]: https://www.skybet.com

[sky poker]: https://www.skypoker.com

[sky sports]: https://www.skysports.com

[spurs]: https://www.tottenhamhotspur.com

[twitter]: https://twitter.com/fold_left

[west ham]: https://www.whufc.com

[william hill]: https://www.williamhill.com
