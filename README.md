# syncpack

<p align="center">
  <img src="https://jamiemason.github.io/syncpack/logo.svg" width="134" height="120" alt="">
  <br>Consistent dependency versions in large JavaScript Monorepos.
  <br><a href="https://jamiemason.github.io/syncpack">https://jamiemason.github.io/syncpack</a>
</p>

## Installation

```bash
npm install --save-dev syncpack@alpha
```

## Commands

> All command line options can be combined to target packages and dependencies in multiple ways.

### [lint](https://jamiemason.github.io/syncpack/command/lint) and [fix](https://jamiemason.github.io/syncpack/command/fix)

Ensure that multiple packages requiring the same dependency define the same version, so that every package requires eg. `react@16.4.2`, instead of a combination of `react@16.4.2`, `react@0.15.9`, and `react@16.0.0`.

#### Examples

```bash
# Find every issue in "dependencies" or "devDependencies"
syncpack lint --dependency-types prod,dev
# Look for issues in dependencies containing "react" in the name
syncpack lint --dependencies '**react**'
# Autofix the above issues
syncpack fix --dependencies '**react**'
# Find issues everywhere except "peerDependencies"
syncpack lint --dependency-types '!peer'
# Only look for issues where an exact version is used
syncpack lint --specifier-types exact
# Only look for issues where an exact version is specified
syncpack lint --specifier-types exact
# Sort dependencies by how many times they are used
syncpack lint --sort count
# Show a lot more detail about the issues
syncpack lint --show hints,ignored,instances,statuses
# See more examples
syncpack lint --help
syncpack fix --help
# See a short summary of options
syncpack lint -h
syncpack fix -h
```

### [update](https://jamiemason.github.io/syncpack/command/update)

Update packages to the latest versions from the npm registry, wherever they are in your monorepo.<br/>Semver range preferences are preserved when updating.

#### Examples

```bash
# Accept any update in latest (x.x.x)
syncpack update --target latest
# Only update minor versions (1.x.x)
syncpack update --target minor
# Only update patch versions (1.2.x)
syncpack update --target patch
# Check for outdated dependencies in one package
syncpack update --check --source 'packages/pingu/package.json'
# Update dependencies and devDependencies in the whole monorepo
syncpack update --dependency-types dev,prod
# Only update dependencies with a semver range specifier (^, ~, etc.)
syncpack update --specifier-types range
# Update dependencies where name exactly matches 'react'
syncpack update --dependencies 'react'
# Update dependencies where name contains 'react'
syncpack update --dependencies '**react**'
# Update dependencies with the '@aws-sdk' scope
syncpack update --dependencies '@aws-sdk/**'
# See more examples
syncpack update --help
# See a short summary of options
syncpack update -h
```

### [format](https://jamiemason.github.io/syncpack/command/format)

Organise package.json files according to a conventional format, where fields appear in a predictable order and nested fields are ordered alphabetically. Shorthand properties are used where available, such as the `"repository"` and `"bugs"` fields.

#### Examples

```bash
# Fix every formatting issue in the monorepo
syncpack format
# List all formatting issues in the monorepo
syncpack format --check
# Check the formatting of one package
syncpack format --check --source 'packages/pingu/package.json'
# See more examples
syncpack format --help
# See a short summary of options
syncpack format -h
```

## Badges

- [![support on ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/C0C4PY4P)
- [![NPM version](http://img.shields.io/npm/v/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
- [![NPM downloads](http://img.shields.io/npm/dm/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
- [![Build Status](https://img.shields.io/github/actions/workflow/status/JamieMason/syncpack/ci.yaml?branch=main)](https://github.com/JamieMason/syncpack/actions)
