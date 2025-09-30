# Finding Things in Syncpack with ast-grep

ast-grep (sg) is a syntax-aware code search tool that understands Rust's AST structure. It's more precise than text-based grep because it matches code patterns, not just text.

## Installation

```bash
cargo install ast-grep
```

## Basic Usage

```bash
# Search for a pattern
ast-grep --pattern 'PATTERN' --lang rust

# Search and show context
ast-grep -p 'PATTERN' -l rust -C 3

# Search in specific directory
ast-grep -p 'PATTERN' -l rust src/
```

## Finding InstanceState Variants

### Find where states are assigned

```bash
# Find all InstanceState::fixable calls
ast-grep --pattern 'InstanceState::fixable' --lang rust src/

# Find all InstanceState::valid calls
ast-grep --pattern 'InstanceState::valid' --lang rust src/

# Find all InstanceState::suspect calls
ast-grep --pattern 'InstanceState::suspect' --lang rust src/
```

### Find specific state assignments

```bash
# Find where DiffersToPinnedVersion is assigned
ast-grep --pattern 'DiffersToPinnedVersion' --lang rust src/

# Find where IsBanned is used
ast-grep --pattern 'IsBanned' --lang rust src/
```

### Find state checks

```bash
# Find all is_fixable() calls
ast-grep --pattern 'is_fixable' --lang rust src/

# Find all is_invalid() calls
ast-grep --pattern 'is_invalid' --lang rust src/

# Find pattern matching on states
ast-grep --pattern 'match' --lang rust src/ | grep state
```

## Finding Enum Definitions

### Find all public enums

```bash
ast-grep --pattern 'pub enum' --lang rust src/
```

### Find specific enum

```bash
# Find InstanceState definition
ast-grep --pattern 'pub enum InstanceState' --lang rust src/

# Find VersionGroupVariant definition
ast-grep --pattern 'pub enum VersionGroupVariant' --lang rust src/
```

## Finding Function Signatures

### Find command run functions

```bash
# Find all pub fn run that take Context
ast-grep --pattern 'pub fn run' --lang rust src/commands/
```

### Find visitor functions

```bash
# Find functions that call visit_packages
ast-grep --pattern 'visit_packages' --lang rust src/
```

### Find TestBuilder usage

```bash
# Find all TestBuilder::new calls
ast-grep --pattern 'TestBuilder::new()' --lang rust src/

# Find build_and_visit_packages calls
ast-grep --pattern 'build_and_visit_packages' --lang rust src/
```

## Finding Tests

### Find all test functions

```bash
ast-grep --pattern '#[test]' --lang rust src/
```

### Find tests in specific file

```bash
ast-grep --pattern '#[test]' --lang rust src/visit_packages/banned_test.rs
```

### Find tests using specific patterns

```bash
# Tests that use expect()
ast-grep --pattern 'expect' --lang rust src/

# Tests with with_version_group
ast-grep --pattern 'with_version_group' --lang rust src/
```

## Finding Context Usage

### Find Context creation

```bash
ast-grep --pattern 'Context::create' --lang rust src/
```

### Find Context methods

```bash
# Find get_version_groups calls
ast-grep --pattern 'get_version_groups' --lang rust src/

# Find all Context being passed
ast-grep --pattern 'ctx: Context' --lang rust src/
```

## Finding Struct Definitions

### Find all public structs

```bash
ast-grep --pattern 'pub struct' --lang rust src/
```

### Find specific struct

```bash
# Find Instance struct
ast-grep --pattern 'pub struct Instance' --lang rust src/instance.rs

# Find Dependency struct
ast-grep --pattern 'pub struct Dependency' --lang rust src/dependency.rs
```

## Finding Implementations

### Find impl blocks

```bash
# Find all Context implementations
ast-grep --pattern 'impl Context' --lang rust src/

# Find all Instance methods
ast-grep --pattern 'impl Instance' --lang rust src/
```

### Find specific methods

```bash
# Find is_valid implementations
ast-grep --pattern 'pub fn is_valid' --lang rust src/
```

## Finding Import Statements

### Find what imports a module

```bash
# Find files importing instance_state
ast-grep --pattern 'use crate::instance_state' --lang rust src/

# Find files importing Context
ast-grep --pattern 'use crate::context::Context' --lang rust src/
```

## Finding Match Statements

### Find matches on InstanceState

```bash
ast-grep --pattern 'match' --lang rust src/ | grep state
```

### Find matches on Subcommand

```bash
ast-grep --pattern 'match' --lang rust src/ | grep subcommand
```

### Find matches on VersionGroupVariant

```bash
ast-grep --pattern 'match' --lang rust src/ | grep variant
```

## Finding Specific Patterns

### Find RefCell usage

```bash
# Find RefCell::new
ast-grep --pattern 'RefCell::new' --lang rust src/

# Find borrow_mut calls
ast-grep --pattern 'borrow_mut' --lang rust src/
```

### Find Rc usage

```bash
# Find Rc::new
ast-grep --pattern 'Rc::new' --lang rust src/

# Find Rc cloning
ast-grep --pattern '.clone()' --lang rust src/
```

### Find filter chains

```bash
# Find filter on instances
ast-grep --pattern '.filter' --lang rust src/
```

## Advanced Queries

### Find complex patterns

```bash
# Find for_each with filter
ast-grep --pattern 'for_each' --lang rust src/ | grep filter

# Find lazy printing pattern (use grep for this)
grep -r "has_printed" src/commands/
```

### Find doc comments

```bash
# Find /// comments (use grep for this)
grep -r "^[ ]*///" src/

# Find module-level docs
grep -r "^//!" src/
```

## Interactive Search

### Open interactive mode

```bash
# Search interactively
ast-grep --pattern 'PATTERN' --lang rust --interactive src/
```

### With rewrite (requires advanced pattern syntax)

```bash
# For search and replace, use ast-grep playground to test patterns first:
# https://ast-grep.github.io/playground.html
```

## Scanning with Rules

### Create a rule file

Create `.ast-grep/rules/syncpack.yml`:

```yaml
id: state-in-context-create
language: rust
rule:
  pattern: |
    impl Context {
      pub fn create($$$) -> Self {
        $$$
        InstanceState::$STATE($$$)
        $$$
      }
    }
message: "InstanceState should not be assigned in Context::create"
severity: error
```

### Run the scanner

```bash
ast-grep scan
```

## Common Use Cases

### "Where is this state assigned?"

```bash
ast-grep --pattern 'DiffersToPinnedVersion' --lang rust src/visit_packages/
```

### "Where is Context created?"

```bash
ast-grep --pattern 'Context::create' --lang rust src/
```

### "Which commands use visit_packages?"

```bash
ast-grep --pattern 'visit_packages' --lang rust src/main.rs
```

### "Find all test assertions"

```bash
ast-grep --pattern 'to_have_instances' --lang rust src/
```

### "Find iterator chains"

```bash
ast-grep --pattern 'get_sorted_dependencies' --lang rust src/commands/
```

## Tips

1. **Keep patterns simple**: Match identifiers, keywords, and simple expressions
2. **For complex patterns**: Use the ast-grep playground to test: https://ast-grep.github.io/playground.html
3. **Be specific**: More specific patterns give better results
4. **Use context flag**: `--context N` or `-C N` shows surrounding code
5. **Combine with grep**: Use `ast-grep` for structure, pipe to `grep` for refinement
6. **Advanced features**: Metavariables (`$VAR`) and ellipsis (`$$$`) work but need careful syntax

## Comparison with grep

| grep                         | ast-grep                               |
| ---------------------------- | -------------------------------------- |
| `grep -r "InstanceState::"`  | `sg -p 'InstanceState::$VARIANT($$$)'` |
| `grep -r "pub fn run"`       | `sg -p 'pub fn run($$$) -> i32'`       |
| `grep -r "TestBuilder::new"` | `sg -p 'TestBuilder::new()'`           |

ast-grep understands code structure, so it won't match comments or strings accidentally.

## Documentation

- Official docs: https://ast-grep.github.io/
- Pattern syntax: https://ast-grep.github.io/guide/pattern-syntax.html
- Rust language: https://ast-grep.github.io/reference/languages.html#rust
