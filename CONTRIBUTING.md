# Contributing

Syncpack is a Rust binary crate that creates a command line application for ensuring consistency in the contents of multiple package.json files, particularly focusing on dependency versions.

It is deployed to the npm registry as `syncpack` in the version range `syncpack@14.0.0-alpha.*`. It is an in-development replacement for `syncpack@latest` which is currently on version `13.0.4`.

## LLM-Assisted Development

If you're using an LLM (like Claude, ChatGPT, or Copilot) to assist with development, see **[.notes/index.md](./.notes/index.md)** for a comprehensive development hub optimised for LLM context. It includes:

- Essential file locations and their purposes
- Core data structures and their relationships
- Common development patterns and tasks
- Test writing guidelines with examples
- Naming conventions and code organization
- Common pitfalls and how to avoid them

This document (CONTRIBUTING.md) provides the high-level architecture and workflow. The .notes/ directory provides concrete patterns and examples for day-to-day development tasks.

## Git branches

`main` - The most recently published version of the Rust v14 alpha version of the codebase.
`v14-alpha` - A development branch for the next version of the Rust v14 alpha version of the codebase.
`13.x.x` - The most recently published version of the TypeScript v13 version of the codebase which is being replaced.

## Folder structure

- All source code is located in files at `./src/**/*.rs`
- All tests are located in files at `./src/**/*_test.rs`
- The documentation website is located in `./site/src/**`
- The `./fixtures/fluid-framework` directory is an example project which can be used to run the local development version of syncpack against for testing locally.
- The `./npm` directory contains files used when deploying syncpack to npm, a rust binary for each major OS needs publishing as npm packages which are then set as optionalDependencies of the main syncpack package, which then has a small node.js script to run the appropriate binary. This is not needed during local development.

## Documentation website

The source code for the documentation website is located in `./site/src/**`, the sitemap for the published website is located at https://jamiemason.github.io/syncpack/sitemap.xml. Read this sitemap to find what documentation is available to help you with a given topic.

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
- The preferred tests are those at `src/visit_packages/**/*_test.rs` and `src/visit_formatting/**/*_test.rs` as they are integration tests which resemble real world use cases.
- Integration tests use the builder pattern in `src/test/builder.rs` via the `TestBuilder` struct. The `TestBuilder` struct provides a fluent API for creating test cases that consist of package.json files, syncpack configuration files, and command line inputs.
- The `TestBuilder` has a `.build()` method which returns a `Context` struct in the correct state to reproduce the required test scenario. There is also a `build_and_visit_packages()` method which returns a `Context` struct which has also been passed through `visit_packages()` for convenience.
- Mock utilities are available in `src/test/mock.rs`
- The `expect` function at `src/test/expect.rs` receives a `Context` struct and asserts that it is in the expected state. the `Vec` it receives is a list which must contain every expected package.json file that should be present in the context after the test has been run, in the expected state.
- Examples of good tests to emulate can be found in `src/visit_packages/banned_test.rs`.

## High-level architecture and data flow

Every syncpack command follows the same pattern:

1. [Create Context](#create-context)
2. [Inspect Context](#inspect-context)
3. [Run Command](#run-command)

### 1. Create context

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

- src/rcfile.rs is responsible for finding and reading the config file as an `Rcfile` struct.
- src/config.rs defines a `Config` struct which combines the `Cli` and `Rcfile` structs into one.

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

- src/packages.rs is responsible for reading and parsing package.json files into a `Packages` struct containing each `PackageJson` struct for each package.json file.

#### 1d. Collect project dependencies

Now that we have a `Config` struct and `Packages` struct, we can collect the project's dependencies and assign them to version groups.

1. Partition the monorepo by versioning policy ("version groups")
2. Load every "instance" (eg. @effect/schema in devDependencies of @effect/platform-node) of every "dependency" (eg. @effect/schema)
   1. Parse and tag its version specifier (eg. `"1.2.1"`, `"workspace:*"`, `"catalog:"`, `"git://github.com/user/repo.git"`)
3. Assign every instance to one version group, first one wins

- src/context.rs is responsible for collecting project dependencies and assigning them to version groups. These are all returned in a `Context` struct alongside all other data we have collected such as the `Config` and `Packages` structs.

### 2. Inspect context

In terms of Rust's ownership and borrowing, the `Context` struct has ownership of all of the data related to the project being operated on. The `Context` struct is given in its entirety to either the `visit_packages` or `visit_formatting` functions.

#### `visit_packages`

Located at src/visit_packages.rs, this function will:

1. Visit each version group, each dependency within it, and each instance within that.
2. Tag every instance with an instance of an `InstanceState` enum to describe if it is valid, or specifically how it is not.
3. Return ownership of the `Context` struct.

#### `visit_formatting`

Located at src/visit_formatting.rs, this function will:

1. Visit each package.json file
2. Tag every package.json file with multiple status codes describing if its formatting is valid, or specifically how it is not
3. Return ownership of the `Context` struct

### 3. Run command

Finally, the command chosen by the user is passed the `Context` struct and has full ownership of it. Each command will perform its own side effects such as updating or synchronising the project's dependencies, or formatting the project's files. Every command must finish by returning and exit code of 1 or 0 to exit the program with.
