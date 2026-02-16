# syncpack

<p align="center">
  <img src="https://syncpack.dev/logo.svg" width="134" height="120" alt="">
  <br>Consistent dependency versions in large JavaScript Monorepos.
  <br><a href="https://syncpack.dev">https://syncpack.dev</a>
</p>

Syncpack is used by [AWS](https://github.com/aws/aws-pdk), [Cloudflare](https://github.com/cloudflare/mcp-server-cloudflare), [DataDog](https://github.com/DataDog/datadog-ci), [Electron](https://github.com/electron/forge), [GoDaddy](https://github.com/godaddy/gasket), [LiveStore](https://github.com/livestorejs/livestore), [Lottie](https://github.com/LottieFiles/dotlottie-web), [Microsoft](https://github.com/microsoft/fluentui), [PostHog](https://github.com/PostHog/posthog), [Qwik](https://github.com/QwikDev/qwik), [Raycast](https://github.com/raycast/extensions), [Salesforce](https://github.com/SalesforceCommerceCloud/pwa-kit), [TopTal](https://github.com/toptal/picasso), [Vercel](https://github.com/vercel/vercel), [VoltAgent](https://github.com/VoltAgent/voltagent), [WooCommerce](https://github.com/woocommerce/woocommerce) and others.


## Installation

```bash
npm install --save-dev syncpack
```

## Guides

- [Getting Started](https://syncpack.dev/)
- [Migrate to 14](https://syncpack.dev/guide/migrate-v14/)

## Commands

> All command line options can be combined to target packages and dependencies in multiple ways.

### [lint](https://syncpack.dev/command/lint)

Ensure that multiple packages requiring the same dependency define the same version, so that every package requires eg. `react@17.0.2`, instead of a combination of `react@17.0.2`, `react@16.8.3`, and `react@16.14.0`.

#### Examples

```bash
# Find all issues in "dependencies" or "devDependencies"
syncpack lint --dependency-types prod,dev
# Only lint issues in "react" specifically
syncpack lint --dependencies react
# Look for issues in dependencies containing "react" in the name
syncpack lint --dependencies '**react**'
# Find issues in scoped packages only
syncpack lint --dependencies '@types/**'
# Find issues everywhere except "peerDependencies"
syncpack lint --dependency-types '!peer'
# Only look for issues where an exact version is used (eg "1.2.3")
syncpack lint --specifier-types exact
# Sort dependencies by how many times they are used
syncpack lint --sort count
# See more examples
syncpack lint --help
# See a short summary of options
syncpack lint -h
```

### [fix](https://syncpack.dev/command/fix)

Fix every autofixable issue found by `syncpack lint`.

#### Examples

```bash
# Only fix issues in dependencies and devDependencies
syncpack fix --dependency-types prod,dev
# Only fix inconsistencies with exact versions (eg "1.2.3")
syncpack fix --specifier-types exact
# Only fix issues in "react" specifically
syncpack fix --dependencies react
# See more examples
syncpack fix --help
# See a short summary of options
syncpack fix -h
```

### [update](https://syncpack.dev/command/update)

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

### [format](https://syncpack.dev/command/format)

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

### [list](https://syncpack.dev/command/list)

Query and inspect all dependencies in your project, both valid and invalid.

#### Examples

```bash
# Sort dependencies by how many times they are used
syncpack list --sort count
# Show every instance of each dependency, not just their names
syncpack list --show instances
# Show dependencies ignored in your syncpack config
syncpack list --show ignored
# Show highest level of detail
syncpack list --show all
# Choose only some values
syncpack list --show hints,statuses
# List all "peerDependencies"
syncpack list --dependency-types peer
# List all types packages
syncpack list --dependencies '@types/**'
# List instances of an exact version being used as a peer dependency
syncpack list --specifier-types exact --show instances --dependency-types peer
# See more examples
syncpack list --help
# See a short summary of options
syncpack list -h
```

### [json](https://syncpack.dev/command/json)

Output the state of every instance of every dependency as a JSON object, one per line. This command is best used with tools like [`jq`](https://jqlang.org/) for filtering and processing.

#### Examples

```bash
# Output all dependencies as JSON
syncpack json
# Output only AWS SDK dependencies
syncpack json --dependencies '@aws-sdk/**'
# Count dependencies by type
syncpack json | jq -r '.dependencyType' | sort | uniq -c
# See more examples
syncpack json --help
# See a short summary of options
syncpack json -h
```

## Badges

- [![support on ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/C0C4PY4P)
- [![NPM version](http://img.shields.io/npm/v/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
- [![NPM downloads](http://img.shields.io/npm/dm/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
- [![Build Status](https://img.shields.io/github/actions/workflow/status/JamieMason/syncpack/ci.yaml?branch=main)](https://github.com/JamieMason/syncpack/actions)
