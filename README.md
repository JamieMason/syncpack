# syncpack

> Consistent dependency versions in large JavaScript Monorepos.

## Installation

```bash
npm install --save-dev syncpack
```

## Documentation

Full information can be found in the documentation at https://jamiemason.github.io/syncpack/.

## Commands

### [fix-mismatches](https://jamiemason.github.io/syncpack/fix-mismatches)

Ensure that multiple packages requiring the same dependency define the same version, so that every
package requires eg. `react@16.4.2`, instead of a combination of `react@16.4.2`, `react@0.15.9`, and
`react@16.0.0`.

### [format](https://jamiemason.github.io/syncpack/format)

Organise package.json files according to a conventional format, where fields appear in a predictable
order and nested fields are ordered alphabetically. Shorthand properties are used where available,
such as the `"repository"` and `"bugs"` fields.

### [lint](https://jamiemason.github.io/syncpack/lint)

Lint all versions and ranges and exit with 0 or 1 based on whether all files match your Syncpack
configuration file.

### [lint-semver-ranges](https://jamiemason.github.io/syncpack/lint-semver-ranges)

Check whether dependency versions used within "dependencies", "devDependencies", etc follow a
consistent format.

### [list](https://jamiemason.github.io/syncpack/list)

List all dependencies required by your packages.

### [list-mismatches](https://jamiemason.github.io/syncpack/list-mismatches)

List dependencies which are required by multiple packages, where the version is not the same across
every package.

### [set-semver-ranges](https://jamiemason.github.io/syncpack/set-semver-ranges)

Ensure dependency versions used within `"dependencies"`, `"devDependencies"` etc follow a consistent
format.

## Breaking Changes

Version [9.0.0](https://github.com/JamieMason/syncpack/releases/tag/9.0.0) required some breaking
API changes to add support for a new
[`customTypes`](https://jamiemason.github.io/syncpack/config/custom-types) feature, but they are
very simple to make.

## Badges

- [![NPM version](http://img.shields.io/npm/v/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
- [![NPM downloads](http://img.shields.io/npm/dm/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
- [![Build Status](https://img.shields.io/github/actions/workflow/status/JamieMason/syncpack/ci.yaml?branch=master)](https://github.com/JamieMason/syncpack/actions)
- [![Maintainability](https://api.codeclimate.com/v1/badges/516439365fdd0e3c6526/maintainability)](https://codeclimate.com/github/JamieMason/syncpack/maintainability)
