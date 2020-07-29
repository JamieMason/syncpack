## 🌩 Installation

```
npm install --global syncpack
```

## 🕵🏾‍♀️ Resolving Packages

package.json files are resolved in this order of precendence:

1. If `--source`
   [glob patterns](https://github.com/isaacs/node-glob#glob-primer) are
   provided, use those.
1. If using [Pnpm Workspaces](https://pnpm.js.org/en/workspaces),
   read `packages` from `pnpm-workspace.yaml` in the root of the current project.
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
