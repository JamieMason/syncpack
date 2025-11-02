# Plan Maintenance Guide

## Core Principle: Keep Plans Forward-Looking

Plans, implementation guides, and task documentation should be **actionable and forward-focused**. Remove historical commentary about past decisions; keep only what helps execute or understand the current approach.

## What to Remove (Historical Commentary)

❌ **Remove these patterns:**

- "We previously tried X but changed to Y"
- "Originally we planned X but decided Y is better"
- "After discussion, we chose X instead of Y"
- "We considered approach A but it had problems"
- Explanations of rejected alternatives (unless they're common mistakes to avoid)
- Backward-looking decision narratives
- "Update: we changed from X to Y" commentary

## What to Keep (Forward-Looking Content)

✅ **Always keep:**

1. **Actionable instructions** - Clear steps on what to do
2. **Decision rationale** - WHY we do something (helps future understanding)
3. **Common pitfalls** - Forward-looking warnings about mistakes to avoid
4. **Architecture explanations** - Why patterns exist (maintains correctness)
5. **Verification records** - Evidence of thorough review (e.g., "Deep Analysis Verification" sections)
6. **Trade-off explanations** - Why current approach is chosen (without listing all rejected options)

## Transformation Examples

### Example 1: Technology Choice

<wrong>
"We initially used Arc everywhere but after profiling realized Rc is sufficient for single-threaded code, so we switched to Rc for better performance."
</wrong>

<correct>
"Use Rc for single-threaded sharing (cheaper than Arc). Only use Arc for cross-thread data like RegistryClient."

**Why**: Single-threaded ownership means no atomic operations needed.
</correct>

### Example 2: Architecture Decision

<wrong>
"After trying several approaches including assigning state during Context creation and using a separate visitor pattern, we decided that InstanceState should be assigned in visit_packages because it provides better separation of concerns."
</wrong>

<correct>
"InstanceState is assigned during inspection (visit_packages), NOT during Context creation."

**Why this pattern exists:**

- Enables testing state assignment independently
- Allows reusing Context across multiple inspections
- Clear separation: creation → inspection → action
  </correct>

### Example 3: Implementation Approach

<wrong>
"We originally planned to use pattern matching but discovered that method dispatch was cleaner, so we refactored to use trait methods instead."
</wrong>

<correct>
"Use trait methods for dispatching behavior across types."

**Example**: See `src/specifier2.rs` for the Specifier2 trait implementation pattern.
</correct>

## When to Include Historical Context

### ✅ Keep When It's a Common Mistake

If people commonly try a wrong approach, warn them:

```markdown
**Common Pitfall**: Don't assign InstanceState during Context::create().
This breaks the separation of concerns and makes testing harder.

✅ Correct: Assign state in visit_packages()
```

### ✅ Keep When It Explains Trade-offs

If the rationale isn't obvious, explain why:

```markdown
**Why Rc instead of Arc**: Syncpack is single-threaded, so Rc avoids
atomic operations overhead. Only RegistryClient uses Arc because it
crosses into tokio's async runtime.
```

### ❌ Remove When It's Just History

If it's only documenting what changed, remove it:

```markdown
~~"Update 2024-01-15: Changed from Arc to Rc after performance testing"~~
```

## Update Checklist

When editing plans or documentation:

1. ✅ **Remove "we changed our mind" commentary**
2. ✅ **Remove discussion of rejected approaches** (unless warning about common mistakes)
3. ✅ **Keep WHY explanations** (rationale for current approach)
4. ✅ **Keep warnings about pitfalls** (forward-looking, not historical)
5. ✅ **Keep verification records** (proves thoroughness)
6. ✅ **Keep trade-off explanations** (helps understand constraints)
7. ✅ **Focus on WHAT to do and WHY**, not what NOT to do

## Document Structure

### Ideal Plan Structure

```markdown
# [Feature/Task Name]

## Overview

Brief description of what this accomplishes.

## Implementation Steps

1. Step one with clear action
2. Step two with clear action
3. Step three with clear action

## Key Decisions & Rationale

**Why approach X**: [Forward-looking explanation]
**Why pattern Y**: [Architecture reasoning]

## Common Pitfalls

- ❌ Don't do X because [consequence]
- ✅ Instead do Y because [benefit]

## Verification

[How to verify implementation is correct]
```

### What NOT to Include

```markdown
# ❌ Bad Plan Structure

## History of Changes

- 2024-01-10: Originally planned X
- 2024-01-12: Changed to Y after discussion
- 2024-01-15: Reverted part of Y, now using Z

## Rejected Approaches

We considered A, B, and C but they had these problems...
[Long discussion of what didn't work]
```

## Exception: Deep Analysis Records

**Keep verification records** that show the document has been thoroughly vetted:

```markdown
## Deep Analysis Verification

✅ Verified all function signatures match current implementation
✅ Confirmed pattern examples exist in codebase (src/commands/\*.rs)
✅ Tested examples compile and pass tests
✅ Reviewed against architecture invariants in .notes/context.md

Last verified: 2024-01-20
```

These records provide confidence in the document's accuracy without being historical commentary about decisions.

## Summary

**Goal**: Make plans immediately actionable for someone reading them fresh.

**Test**: Could someone read this plan without knowing the history and successfully implement it?

**Focus**:

- WHAT to do ✅
- WHY we do it ✅
- HOW to avoid mistakes ✅
- WHAT we used to do ❌
- WHY we changed ❌
