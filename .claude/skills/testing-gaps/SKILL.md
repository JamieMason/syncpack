---
name: testing-gaps
description: Run coverage, inspect results, and identify missing test scenarios for a given source file. Use when analysing test coverage or finding untested branches.
---

# Testing Gaps

Find untested branches and missing real-world test scenarios for a source file.

## Workflow

1. **Run text coverage** for the target file
2. **Identify uncovered branches** from the output
3. **Cross-reference** with test file to understand what's tested
4. **Report** uncovered branches + missing real-world scenarios

## Step 1: Run Coverage

Use `cargo llvm-cov` with `--text` output (not `--html`) for parseable results.

```bash
# Full coverage, filtered to target file
cargo llvm-cov test --text \
  --ignore-run-fail \
  --ignore-filename-regex '(_test.rs|\/test\/)' \
  2>/dev/null | sed -n '/TARGET_FILE\.rs:/,/^$/p'
```

Example for `preferred_semver.rs`:

```bash
cargo llvm-cov test --text \
  --ignore-run-fail \
  --ignore-filename-regex '(_test.rs|\/test\/)' \
  2>/dev/null | sed -n '/preferred_semver\.rs:/,/^$/p'
```

## Step 2: Read Coverage Output

The text format shows per-line execution counts:

```
  42|     80|        .and_then(|range| ...)     # Hit 80 times
  74|      0|        Some(preferred)            # Never hit
 321|      0|        instance.mark_conflict(...) # Never hit
```

- `count > 0` = covered
- `count = 0` = uncovered branch
- `^N` annotations on sub-expressions show partial coverage within a line

### Extract only uncovered lines

```bash
... | grep -E '^\s+\d+\|\s+0\|'
```

## Step 3: Classify Gaps

For each uncovered branch, determine:

1. **What code path leads here?** — Trace the `if/else` chain backward
2. **What input would trigger it?** — What package.json + config combination
3. **Is it reachable?** — Some branches may be defensive/unreachable
4. **Is it worth testing?** — Real-world scenario vs theoretical edge case

### Categories

| Category                         | Action            |
| -------------------------------- | ----------------- |
| Real-world scenario never tested | Write a test      |
| Edge case in existing logic      | Write a test      |
| Defensive branch (unreachable)   | Note but skip     |
| Dead code                        | Consider removing |

## Step 4: Identify Missing Scenarios

Beyond line coverage, look for missing _combinations_:

- Feature A tested, Feature B tested, A+B never tested together
- Only tested with one package, never with multiple
- Only tested with `highestSemver`, never `lowestSemver`
- Only tested with one range type (`^`), never others (`>=`, `<=`)
- Only tested without semver groups, never with

## Step 5: Report

Structure findings as:

1. **Uncovered branches** — Specific lines, what triggers them, why untested
2. **Missing real-world scenarios** — Combinations and interactions not covered

## Running Tests After Adding Coverage

```bash
just test                              # All tests
cargo test test_name -- --nocapture   # Specific test
just coverage                         # Regenerate coverage report
```
