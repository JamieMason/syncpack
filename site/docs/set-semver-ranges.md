---
id: set-semver-ranges
title: set-semver-ranges
---

Ensure dependency versions used within `"dependencies"`, `"devDependencies"` etc
follow a consistent format.

See [`semverGroups`](./config/semver-groups.md) if you have advanced
requirements.

## CLI Options

```
-s, --source [pattern]      glob pattern for package.json files to read from
-f, --filter [pattern]      only include dependencies whose name matches this regex
-c, --config <path>         path to a syncpack config file
-r, --semver-range <range>  see supported ranges below. defaults to ""
-t, --types <names>         only include dependencies matching these types (eg. types=dev,prod,myCustomType)
-i, --indent [value]        override indentation. defaults to "  "
-h, --help                  display help for command
```

## Examples

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
syncpack set-semver-ranges --types dev --semver-range ~
# set ~ range in "devDependencies" and "peerDependencies"
syncpack set-semver-ranges --types dev,peer --semver-range ~
# indent package.json with 4 spaces instead of 2
syncpack set-semver-ranges --indent "    "
```
