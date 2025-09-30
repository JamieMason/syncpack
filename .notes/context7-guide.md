# Context7 MCP Guide

## What is Context7?

Context7 is an MCP (Model Context Protocol) server that provides up-to-date documentation for libraries and frameworks directly to LLMs. It solves the problem of outdated training data by fetching current docs from the source.

## When to Use Context7

Use Context7 when you need information about:

- **External Rust crates** - tokio, serde, clap, etc.
- **npm packages** - Not relevant for Syncpack core, but useful if working on tooling
- **Framework APIs** - When APIs have changed since LLM training data
- **Library setup** - Installing and configuring new dependencies
- **Method signatures** - Finding correct current API usage
- **Code examples** - Getting working examples from current versions

**Don't use Context7 for:**

- Syncpack internal code (use `ast-grep` and local docs instead)
- General Rust language questions (LLM knowledge sufficient)
- Basic programming concepts

## Two-Step Workflow

### Step 1: Resolve Library ID

Use `resolve-library-id` tool to find the Context7-compatible ID:

```
resolve-library-id("tokio")
→ Returns: /tokio-rs/tokio
```

### Step 2: Get Documentation

Use `get-library-docs` with the resolved ID:

```
get-library-docs("/tokio-rs/tokio")
→ Returns: Current tokio documentation
```

### Optional: Focus with Topics

Narrow large docs with the `topic` parameter:

```
get-library-docs("/tokio-rs/tokio", topic="async runtime")
get-library-docs("/serde-rs/serde", topic="derive macros")
get-library-docs("/clap-rs/clap", topic="command parsing")
```

### Optional: Control Token Count

Adjust documentation size with the `tokens` parameter (default: 5000, minimum: 1000):

```
get-library-docs("/tokio-rs/tokio", tokens=10000)  // More comprehensive
get-library-docs("/serde-rs/serde", tokens=2000)   // Quick reference
```

## Common Rust Crate IDs

If you know the Context7 ID format, skip the resolve step:

| Crate     | Context7 ID            |
| --------- | ---------------------- |
| tokio     | `/tokio-rs/tokio`      |
| serde     | `/serde-rs/serde`      |
| clap      | `/clap-rs/clap`        |
| anyhow    | `/dtolnay/anyhow`      |
| thiserror | `/dtolnay/thiserror`   |
| reqwest   | `/seanmonstar/reqwest` |

Pattern: `/github-org/repo-name`

## Example Scenarios

### Adding a New Dependency

**User request:** "Add tokio async runtime support"

**LLM workflow:**

1. `resolve-library-id("tokio")` → `/tokio-rs/tokio`
2. `get-library-docs("/tokio-rs/tokio", topic="async runtime setup")`
3. Implement based on current API
4. Add to `Cargo.toml` with correct version

### Fixing Deprecated API

**User request:** "Update clap usage to current version"

**LLM workflow:**

1. `resolve-library-id("clap")` → `/clap-rs/clap`
2. `get-library-docs("/clap-rs/clap", topic="derive API")`
3. Refactor code to match current best practices
4. Test changes

### Understanding Error Handling

**User request:** "Use anyhow for better error messages"

**LLM workflow:**

1. Skip resolve if you know it's `/dtolnay/anyhow`
2. `get-library-docs("/dtolnay/anyhow", topic="context and chains")`
3. Apply patterns to Syncpack error handling
4. Ensure compatibility with existing error types

## Tips

### Avoid Redundant Lookups

If you just fetched docs for a library in the current conversation, reuse that information instead of fetching again.

### Combine with ast-grep

Use Context7 for external APIs, `ast-grep` for Syncpack internals:

```
# External library usage
get-library-docs("/serde-rs/serde")

# Syncpack internal patterns
ast-grep -p "Context::create"
```

### Version-Specific Docs

Context7 can target specific versions:

```
get-library-docs("/tokio-rs/tokio/v1.35.0")
```

But default (latest) usually sufficient unless user specifies version.

## Troubleshooting

**Problem:** Library not found by `resolve-library-id`

**Solution:** Try variations of the name or use GitHub org/repo format directly

**Problem:** Docs too large/generic

**Solution:** Use `topic` parameter to focus on specific area

**Problem:** Outdated docs returned

**Solution:** Verify you're using latest Context7; report issue if docs genuinely outdated

## Integration with Syncpack Development

Context7 complements but doesn't replace Syncpack's internal documentation:

- **`.notes/` docs** - Syncpack architecture and patterns (read first)
- **`ast-grep`** - Finding Syncpack code patterns
- **Context7** - External library APIs and setup

Always check `.notes/index.md` first for Syncpack-specific guidance before reaching for external library docs.
