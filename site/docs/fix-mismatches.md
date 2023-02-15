---
id: fix-mismatches
title: fix-mismatches
---

Ensure that multiple packages requiring the same dependency define the same
version, so that every package requires eg. `react@16.4.2`, instead of a
combination of `react@16.4.2`, `react@0.15.9`, and `react@16.0.0`.

See [`versionGroups`](./config/version-groups.md) if you have advanced
requirements.

## CLI Options

```
-s, --source [pattern]  glob pattern for package.json files to read from
-f, --filter [pattern]  only include dependencies whose name matches this regex
-t, --types <names>     only include dependencies matching these types (eg. types=dev,prod,myCustomType)
-c, --config <path>     path to a syncpack config file
-i, --indent [value]    override indentation. defaults to "  "
-h, --help              display help for command
```

## Examples

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
syncpack fix-mismatches --types dev
# only inspect "devDependencies" and "peerDependencies"
syncpack fix-mismatches --types dev,peer
# indent package.json with 4 spaces instead of 2
syncpack fix-mismatches --indent "    "
```
