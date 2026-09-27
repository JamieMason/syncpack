# Contributing

Syncpack is a Rust binary crate that creates a command line application for ensuring consistency in the contents of multiple package.json files, particularly focusing on dependency versions.

## LLM-Assisted Development

If you're using an LLM (like Claude, ChatGPT, or Copilot) to assist with development, see **[.notes/context.md](./.notes/context.md)** and **[.notes/reference/](./.notes/reference/)** for a development hub optimised for LLM context. It includes:

- Essential file locations and their purposes
- Core data structures and their relationships
- Common development patterns and tasks
- Test writing guidelines with examples
- Naming conventions and code organization
- Common pitfalls and how to avoid them

This document (CONTRIBUTING.md) provides the high-level architecture and workflow. The .notes/ directory provides concrete patterns and examples for day-to-day development tasks.

## Git branches

`main` - The Rust rewrite of the codebase (see `Cargo.toml` for the current published version).
`13.x.x` - The most recently published version of the TypeScript v13 version of the codebase which was replaced.

## Folder structure

- Most logic lives in 8 library crates at `./crates/*/src/**/*.rs` (see `.notes/context.md` for the dependency graph between them)
- The binary crate — CLI parsing, discovery/composition, commands, TUI — is at `./src/**/*.rs`
- Tests are co-located as sibling `*_test.rs` files (in both `crates/` and `src/`), wired in with `#[path]`
- The documentation website is located in `./site/src/**`
- The `./fixtures/fluid-framework` directory is an example project which can be used to run the local development version of syncpack against for testing locally.
- The `./npm` directory contains files used when deploying syncpack to npm, a rust binary for each major OS needs publishing as npm packages which are then set as optionalDependencies of the main syncpack package, which then has a small node.js script to run the appropriate binary. This is not needed during local development.

## Documentation website

The source code for the documentation website is located in `./site/src/**`, the sitemap for the published website is located at https://syncpack.dev/sitemap-index.xml. Read this sitemap to find what documentation is available to help you with a given topic.

## Development scripts

Important commands:

- `just test` - Run all tests
- `just lint` - Run all linting checks
- `just coverage` - Run all tests and generate a coverage report, this can help find unused code or identify real world use cases we do not have tests for.
- `just benchmark` - When making performance improvements, run this command before and after each change to compare the performance of the current version of syncpack with the previous version.
- `just format` - Fix formatting, indentation etc of all files

Run `just` to see a list of all other available commands and their descriptions.

## Running syncpack locally

When deployed and installed globally, the end user help documentation for syncpack as a whole can be found by running:

```bash
syncpack --help
```

The equivalent command when running a local development version of syncpack is:

```bash
cargo run -- --help
```

To view the help documentation for each command:

```bash
cargo run -- lint --help
cargo run -- fix --help
cargo run -- format --help
cargo run -- update --help
cargo run -- list --help
cargo run -- json --help
```

## Writing tests

### Test Structure

- Unit tests are co-located with source files as `*_test.rs` and tend to test complex functions in isolation.
- The preferred tests are those at `src/version_group/*_test.rs` and `src/visit_formatting/*_test.rs` (wired into `src/main.rs` via `#[path]`) as they are integration tests which resemble real world use cases, exercising the visitor implementations in `crates/syncpack-rules/src/version_group/*.rs` and `crates/syncpack-rules/src/visit_formatting/*.rs` through the full CLI pipeline. Note `syncpack-rules` itself has no unit tests of its own — this is the only place its behaviour is verified, so treat these as that crate's test suite, not merely the bin's.
- Integration tests use the builder pattern in `src/test/builder.rs` via the `TestBuilder` struct. The `TestBuilder` struct provides a fluent API for creating test cases that consist of package.json files, syncpack configuration files, and command line inputs.
- The `TestBuilder` has a `.build()` method which returns a `Discovered` struct in the correct state to reproduce the required test scenario. There is also a `build_and_visit_packages()` method which returns an `Inspected` struct (`Discovered` plus the computed `Inspection`) for convenience.
- Mock utilities are available in `src/test/mock.rs`
- The `expect` function at `src/test/expect.rs` receives an `Inspected` struct and asserts that it is in the expected state. The `Vec` it receives is a list which must contain every expected package.json file that should be present, in the expected state.
- Examples of good tests to emulate can be found in `src/version_group/banned_test.rs`.

## High-level architecture and data flow

Every syncpack command follows the same pattern, implemented in `src/syncpack.rs`:

1. [Discover the project](#1-discover-the-project)
2. [Inspect it](#2-inspect-it)
3. [Run the command](#3-run-the-command)

### 1. Discover the project

This phase is read only and must happen in this order:

1. Nothing can happen until the command line arguments are known
2. We can then use that information to locate the configuration file
3. Only then can we know which package.json files to read
4. When the package.json files are read, we can collect all of their versions and dependencies, and assign them to the appropriate version and semver groups defined in the user's configuration.

More information on each of these steps is as follows:

#### 1a. Parse CLI input

Determine which command and each CLI options were chosen and collect them into a `Cli` struct. Any options which were not provided are assigned default values.

- src/cli.rs is responsible for this.

#### 1b. Read config

1. Determine path to config file, first one wins:
   1. `--config` CLI Option
   2. Search in the root directory of the project for the first file whose name matches a specific list of config file names
2. Once a config file path has been determined, read its contents. It must be one of:
   - TypeScript
   - JavaScript
   - YAML
   - JSON

- src/rcfile_loader.rs is responsible for finding and reading the config file and handing its raw value to `syncpack-config`, which parses and validates it into an `Rcfile` struct (`crates/syncpack-config/src/lib.rs`).
- `Config` (`src/discovery/mod.rs`) combines the `Cli` and `Rcfile` structs into one.

#### 1c. Read package.json files

Now that we have a `Config` struct, we can use it to get paths to package.json files:

1. Find globs to package.json files, first one wins:
   1. `--source` CLI Options
   2. Syncpack config
   3. npm workspace config in the root package.json file
   4. pnpm workspace config in pnpm's config file
   5. Yarn workspace config in the root package.json file
   6. Lerna workspace config in Lerna's config file
   7. Syncpack defaults
2. Resolve globs
3. Read and Parse package.json files

- `syncpack-disk` (`crates/syncpack-disk/src/lib.rs`) owns this: `Disk` reads and holds every package.json (plus `pnpm-workspace.yaml`) as format-preserving `File`/`YamlFile` values from `syncpack-io`.

#### 1d. Collect project dependencies

Now that we have a `Config` struct and a `Disk`, we can collect the project's dependencies and assign them to version groups.

1. Partition the monorepo by versioning policy ("version groups")
2. Load every "instance" (eg. @effect/schema in devDependencies of @effect/platform-node) of every "dependency" (eg. @effect/schema)
   1. Parse and tag its version specifier (eg. `"1.2.1"`, `"workspace:*"`, `"catalog:"`, `"git://github.com/user/repo.git"`)
3. Assign every instance to one version group, first one wins

- `Discovered::create` (`src/discovery/mod.rs`, with helpers in `src/discovery/groups.rs`, `src/discovery/instances.rs`, `src/discovery/link.rs`) is responsible for collecting project dependencies and assigning them to version groups. These are all returned in a `Discovered` struct alongside all other data we have collected such as the `Config` and `Disk` structs.

### 2. Inspect it

`Discovered` is immutable once built. `inspect()` (`src/syncpack.rs`) takes a `&Discovered` and returns a fresh `Inspection` value (`crates/syncpack-rules/src/inspection.rs`) — nothing on `Discovered` is mutated. It calls `visit_packages` and/or `visit_formatting`, both in the `syncpack-rules` crate (the "pure inspection engine": it names no disk or config type).

#### `visit_packages`

Located at `crates/syncpack-rules/src/visit_packages.rs`, this function will:

1. Visit each version group, each dependency within it, and each instance within that.
2. Record an `InstanceState` for each instance into the `Inspection` accumulator (`out.classify(...)`) to describe if it is valid, or specifically how it is not.

#### `visit_formatting`

Located at `crates/syncpack-rules/src/visit_formatting.rs`, this function will:

1. Visit each package.json file
2. Record any formatting mismatches into the `Inspection` accumulator (`out.add_format_mismatch(...)`)

Both take `out: &mut Inspection` and return nothing — the caller already holds it and reads it once inspection completes. `Discovered` plus the resulting `Inspection` (and any registry updates) are bound together into `Inspected` (`src/discovery/mod.rs`), so a caller can't read results early or pair the wrong `Inspection` with the wrong data.

### 3. Run the command

Finally, the command chosen by the user is passed the `Inspected` struct and has full ownership of it. Each command will perform its own side effects such as updating or synchronising the project's dependencies, or formatting the project's files. Every command must finish by returning and exit code of 1 or 0 to exit the program with.
