---
id: sort-az
title: sortAz
---

When using the `format` command, determines which fields within package.json
files should be sorted alphabetically. When the value is an Object, its keys are
sorted alphabetically. When the value is an Array, its values are sorted
alphabetically. There is no equivalent CLI Option for this configuration.

## Default Value

```json
{
  "sortAz": [
    "contributors",
    "dependencies",
    "devDependencies",
    "keywords",
    "peerDependencies",
    "resolutions",
    "scripts"
  ]
}
```
