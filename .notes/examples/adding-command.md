# Example: Adding a New Command

<scenario>
This guide walks through adding a new `syncpack check` command that lists outdated dependencies without modifying files.
</scenario>

<step number="1">

## Step 1: Decide on the Visitor

**Question:** Does this command deal with dependency versions or package.json formatting?

**Answer:** Dependency versions → Use `visit_packages`

</step>

<step number="2">

## Step 2: Add Command Enum Variant

**File:** `src/cli.rs`

```rust
pub enum Subcommand {
  Lint,
  Fix,
  Format,
  Update,
  List,
  Json,
  Check,  // ← Add this
}
```

</step>

<step number="3">

## Step 3: Create Command Implementation

**File:** `src/commands/check.rs`

<implementation>

```rust
use crate::{commands::ui, context::Context};

/// List outdated dependencies without modifying files
pub fn run(ctx: Context) -> i32 {
  let mut has_outdated = false;

  ctx.version_groups.iter().for_each(|group| {
    let mut has_printed_group = false;

    group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
      let mut has_printed_dependency = false;

      dependency
        .get_sorted_instances()
        .filter(|instance| instance.is_outdated())
        .for_each(|instance| {
          // Lazy print headers only when we find outdated instances
          if !has_printed_group {
            ui::group::print_header(&ctx, group);
            has_printed_group = true;
          }
          if !has_printed_dependency {
            ui::dependency::print(&ctx, dependency, &group.variant);
            has_printed_dependency = true;
          }

          // Print the outdated instance
          if ctx.config.cli.show_instances {
            ui::instance::print(&ctx, instance, &group.variant);
          }

          has_outdated = true;
        });
    });
  });

  if has_outdated {
    1  // Exit with error code
  } else {
    ui::util::print_no_issues_found();
    0  // Exit successfully
  }
}
```

</implementation>

<pattern_notes>

**Pattern notes:**

- Take ownership of `Context`
- Return `i32` exit code (0 = success, 1 = failure)
- Use lazy printing (only print headers when needed)
- Iterate: version groups → dependencies → instances
- Filter instances by state (`.filter(|instance| instance.is_outdated())`)
- Use existing UI utilities for consistent output

</pattern_notes>

</step>

<step number="4">

## Step 4: Register Command in Module

**File:** `src/commands.rs`

```rust
pub mod fix;
pub mod format;
pub mod json;
pub mod lint;
pub mod list;
pub mod update;
pub mod check;  // ← Add this
```

</step>

<step number="5">

## Step 5: Add to Main Dispatch

**File:** `src/main.rs`

```rust
use {
  crate::{
    commands::{check, fix, format, json, lint, list, update},  // ← Add check
    // ... other imports
  },
};

// ... in main() function:

let _exit_code = match ctx.config.cli.subcommand {
  Subcommand::Fix => {
    let ctx = visit_packages(ctx);
    fix::run(ctx)
  }
  Subcommand::Format => {
    let ctx = visit_formatting(ctx);
    format::run(ctx)
  }
  Subcommand::Lint => {
    let ctx = visit_packages(ctx);
    lint::run(ctx)
  }
  Subcommand::Update => {
    let mut ctx = ctx;
    ctx.fetch_all_updates().await;
    let ctx = visit_packages(ctx);
    update::run(ctx)
  }
  Subcommand::List => {
    let ctx = visit_packages(ctx);
    list::run(ctx)
  }
  Subcommand::Json => {
    let ctx = visit_packages(ctx);
    json::run(ctx)
  }
  Subcommand::Check => {  // ← Add this
    let ctx = visit_packages(ctx);
    check::run(ctx)
  }
};
```

<key_points>

**Key points:**

- Must call `visit_packages(ctx)` before command (to assign InstanceStates)
- Pass the returned Context to your command's run function
- The command consumes Context and returns exit code

</key_points>

</step>

<step number="6">

## Step 6: Add CLI Help Text (Optional)

**File:** `src/cli.rs`

```rust
#[derive(Debug, clap::Parser)]
#[command(name = "syncpack")]
#[command(about = "Manage multiple package.json files")]
pub struct Cli {
  #[command(subcommand)]
  pub subcommand: Subcommand,
  // ... other fields
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
  /// Find and report outdated dependencies
  #[command(name = "check")]
  Check,
  // ... other variants
}
```

</step>

<step number="7">

## Step 7: Write Tests

**File:** `src/commands/check_test.rs` (or add to existing test file)

<test_examples>

```rust
use {
  crate::{
    commands::check,
    instance_state::{FixableInstance::*, InstanceState, ValidInstance::*},
    test::{builder::TestBuilder, expect::ExpectedInstance},
  },
  serde_json::json,
};

#[test]
fn reports_outdated_dependencies() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "react": "17.0.0"
        }
      }),
    ])
    .with_registry_updates(json!({
      "react": ["17.0.0", "17.0.1", "18.0.0", "18.2.0"]
    }))
    .build_and_visit_packages();

  // Verify the instance is marked as outdated
  let react_instance = ctx.instances.iter()
    .find(|i| i.dependency.name == "react")
    .expect("Should find react instance");

  assert!(react_instance.is_outdated());

  // Run the command (in real test, you'd capture stdout)
  let exit_code = check::run(ctx);
  assert_eq!(exit_code, 1, "Should exit with error when outdated found");
}

#[test]
fn exits_successfully_when_up_to_date() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "react": "18.2.0"
        }
      }),
    ])
    .with_registry_updates(json!({
      "react": ["18.2.0"]
    }))
    .build_and_visit_packages();

  let exit_code = check::run(ctx);
  assert_eq!(exit_code, 0, "Should exit successfully when up to date");
}
```

</test_examples>

</step>

<step number="8">

## Step 8: Test Locally

```bash
# Build and run
cargo run -- check --help

# Test against fixture
cd fixtures/fluid-framework
cargo run -- check

# Run tests
cargo test check
just test
```

</step>

<step number="9">

## Step 9: Verify All Places Updated

<checklist>

- [x] Added to `Subcommand` enum in `src/cli.rs`
- [x] Created `src/commands/check.rs` with `pub fn run(ctx: Context) -> i32`
- [x] Registered in `src/commands.rs` module
- [x] Added match arm in `src/main.rs`
- [x] Called appropriate visitor (`visit_packages`)
- [x] Written tests
- [x] Tested locally

</checklist>

</step>

<variations>

## Common Variations

<variation type="file_modification">

### Command that modifies files

```rust
pub fn run(mut ctx: Context) -> i32 {
  // ... iterate and modify instances

  // Write changes to disk
  ctx.packages.write_all();

  0
}
```

</variation>

<variation type="registry">

### Command that needs registry data

In `main.rs`:

```rust
Subcommand::MyCommand => {
  let mut ctx = ctx;
  ctx.fetch_all_updates().await;  // Fetch from npm
  let ctx = visit_packages(ctx);
  my_command::run(ctx)
}
```

</variation>

<variation type="cli_options">

### Command with specific CLI options

```rust
pub enum Subcommand {
  #[command(name = "check")]
  Check {
    /// Only check production dependencies
    #[arg(long)]
    prod_only: bool,
  },
}
```

Access in command:

```rust
if let Subcommand::Check { prod_only } = &ctx.config.cli.subcommand {
  if *prod_only {
    // Filter logic
  }
}
```

</variation>

</variations>

<troubleshooting>

## Troubleshooting

<problem>
**Problem:** Command not found
→ Check you added it to all three places: enum, main.rs match, commands module
</problem>

<problem>
**Problem:** Instances have `Unknown` state
→ You forgot to call `visit_packages(ctx)` before your command
</problem>

<problem>
**Problem:** Compilation error about moved value
→ Make sure your command signature is `pub fn run(ctx: Context) -> i32`
</problem>

<problem>
**Problem:** Changes not persisting
→ You need to call `ctx.packages.write_all()` in commands that modify files
</problem>

</troubleshooting>

<related_examples>

## Related Examples

- See `src/commands/lint.rs` for read-only command
- See `src/commands/fix.rs` for file-modifying command
- See `src/commands/update.rs` for command using registry client

</related_examples>
