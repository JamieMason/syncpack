set dotenv-required := true
set dotenv-filename := ".env"

no_pattern := ""

# List all available commands
default:
    just --list

# ==============================================================================
# Onboarding
# ==============================================================================

# Install other dependencies used during development
install-system-dependencies:
    # https://lib.rs/crates/cargo-llvm-cov
    cargo +stable install cargo-llvm-cov
    # https://github.com/killercup/cargo-edit
    cargo +stable install cargo-edit

# ==============================================================================
# Write
# ==============================================================================

# Fix formatting, indentation etc of all files
format:
    cargo fmt --all -- --verbose
    cargo clippy --fix --allow-dirty

# Update dependencies in Cargo.toml and Cargo.lock
update-dependencies:
    cargo upgrade
    cargo update

# ==============================================================================
# Lint
# ==============================================================================

# Run all linting checks
lint:
    just check-cargo
    just check-formatting
    just check-clippy
    just check-for-updates

# Run cargo check
check-cargo:
    cargo check --locked

# Check for formatting issues
check-formatting:
    cargo fmt --all -- --check --verbose

# Check for clippy warnings
check-clippy:
    cargo clippy --tests --verbose -- -D warnings

# Look for outdated dependencies
check-for-updates:
    cargo upgrade --dry-run

# ==============================================================================
# GitHub Actions
# ==============================================================================

# Run the release github action locally
run-release-action:
    act --workflows .github/workflows/release.yml workflow_dispatch

# Run the CI github action locally
run-ci-action:
    act --workflows .github/workflows/ci.yml pull_request

# ==============================================================================
# Test
# ==============================================================================

# Run all tests and generate a coverage report
coverage:
    rm -rf target/llvm-cov/html
    cargo llvm-cov test --html --ignore-run-fail --ignore-filename-regex '(_test.rs|\/test\/)'

# Open coverage report (on http server to allow Dark Reader Browser Extension)
serve-coverage:
    npx http-server -so --port 7357 target/llvm-cov/html

# Run all tests
test:
    cargo test -- --nocapture --color=always

# Run test in watch mode
watch pattern=no_pattern:
    tput rmam && cargo watch --clear --exec 'test -- --nocapture --color=always {{pattern}}'

# Run test in watch mode with coverage
watch-coverage:
    tput rmam && cargo watch --clear --exec 'llvm-cov test --html --ignore-filename-regex "(_test.rs|\/test\/)" -- --nocapture --color=always'

# Run the rust binary against an unformatted test fixture
run-misc:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd fixtures/misc
    RUST_BACKTRACE=1 cargo run -- lint --source 'package.json'

# Run the dev rust binary against a clone of microsoft/FluidFramework
run-fluid:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd fixtures/fluid-framework
    RUST_BACKTRACE=1 cargo run -- lint --format --versions

# Run the release rust binary against a clone of microsoft/FluidFramework
run-fluid-prod:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd fixtures/fluid-framework
    ../../target/release/syncpack lint --versions --source 'package.json' --source 'packages/**/package.json'

# ==============================================================================
# Build
# ==============================================================================

# Build a rust binary and corresponding npm package for a specific target
build-binary-package:
    just create-rust-binary
    just create-npm-binary-package

# Build a rust binary for a specific target
create-rust-binary:
    #!/usr/bin/env bash
    set -euxo pipefail

    cargo build --release --locked --target "$TARGET"

# Once a rust binary for a specific target has been built, create an npm package for it
create-npm-binary-package:
    #!/usr/bin/env bash
    set -euxo pipefail

    rm -rf "$NODE_PKG_DIR_PATH"
    mkdir -p "$NODE_PKG_DIR_PATH/bin"
    mv "$RUST_BINARY_PATH" "$NODE_PKG_RUST_BINARY_PATH"
    cp README.md "$NODE_PKG_DIR_PATH/README.md"
    just create-npm-binary-package-json

# Create the package.json file for an npm package for a specific target
create-npm-binary-package-json:
    #!/usr/bin/env node
    const fs = require("fs");
    const path = require("path");
    const srcPath = path.resolve("package.json");
    const destPath = path.resolve(process.env.NODE_PKG_DIR_PATH, "package.json");
    const pkg = require(srcPath);
    const nextPkg = {
        ...pkg,
        name: process.env.NODE_PKG_NAME,
        bin: undefined,
        optionalDependencies: undefined,
        os: [process.env.NODE_OS],
        cpu: [process.env.NODE_ARCH],
    };
    const json = JSON.stringify(nextPkg, null, 2);
    console.log(json);
    fs.writeFileSync(destPath, json);

# Create the parent npm package which delegates to each target-specific package
create-npm-root-package:
    #!/usr/bin/env bash
    set -euxo pipefail

    rm -rf "$NODE_ROOT_PKG_DIR_PATH"
    mkdir -p "$NODE_ROOT_PKG_DIR_PATH"
    cp README.md "$NODE_ROOT_PKG_DIR_PATH/README.md"
    cp npm/index.js "$NODE_ROOT_PKG_DIR_PATH/index.js"
    just create-npm-root-package-json

# Create the package.json file for the parent npm package
create-npm-root-package-json:
    #!/usr/bin/env node
    const fs = require("fs");
    const path = require("path");
    const srcPath = path.resolve("package.json");
    const destPath = path.resolve(process.env.NODE_ROOT_PKG_DIR_PATH, "package.json");
    const pkg = require(srcPath);
    const nextPkg = {
        ...pkg,
        os: undefined,
        cpu: undefined,
        bin: {
          syncpack: "./index.js",
        },
        optionalDependencies: {
          "syncpack-linux-x64": pkg.version,
          "syncpack-linux-arm64": pkg.version,
          "syncpack-darwin-x64": pkg.version,
          "syncpack-darwin-arm64": pkg.version,
          "syncpack-windows-x64": pkg.version,
          "syncpack-windows-arm64": pkg.version,
        },
    };
    const json = JSON.stringify(nextPkg, null, 2);
    console.log(json);
    fs.writeFileSync(destPath, json);

# ==============================================================================
# Publish
# ==============================================================================

# Publish the npm package for a specific target
publish-npm-binary-package:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd "$NODE_PKG_DIR_PATH"
    npm publish --dry-run --access public --tag alpha

# Publish the parent npm package
publish-npm-root-package:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd "$NODE_ROOT_PKG_DIR_PATH"
    npm publish --dry-run --access public --tag alpha