---
id: format
title: format
---

Organise package.json files according to a conventional format, where fields
appear in a predictable order and nested fields are ordered alphabetically.
Shorthand properties are used where available, such as the `"repository"` and
`"bugs"` fields.

## CLI Options

```
-s, --source [pattern]  glob pattern for package.json files to read from
-c, --config <path>     path to a syncpack config file
-i, --indent [value]    override indentation. defaults to "  "
-h, --help              display help for command
```

## Examples

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
