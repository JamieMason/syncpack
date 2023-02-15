---
id: semver-range
title: semverRange
---

The semver range to be used consistently throughout your monorepo.

## Default Value

Defaulted to `""` to ensure that exact dependency versions are used instead of
loose ranges.

```json
{
  "semverRange": ""
}
```

:::tip

If you want to use different ranges in different packages and/or types of
dependencies, you can use [`semverGroups`](./semver-groups.md) to partition your
repo into different sets of rules.

:::

:::info

The `semverRange` configuration in your [config file](../config-file.md) can be
overridden on an ad hoc basis using the
[`--semver-range`](../option/semver-range.md) option.

:::

## Supported Ranges

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
