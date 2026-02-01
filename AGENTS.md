# Syncpack

Rust CLI tool for synchronising npm dependency versions across JavaScript monorepos.

## Getting Started

1. Run `just` to see available commands
2. Read `.notes/` for architecture and mental model
3. Check `skills/` for task-specific guides:

## Communication Style

- Signal over noise
- Extremely concise. Sacrifice grammar for brevity
- Action-oriented: what to DO
- Grounded in facts: cite code/docs
- BANNED WORDS: basically, essentially, in order to, comprehensive

## Pre-Implementation Checklist

Before large changes, answer YES to all:

- Have I identified ALL decision points?
- Have I listed trade-offs for each approach?
- Have I asked user which strategy to use?
- Am I NOT making architectural assumptions?
- Am I NOT creating files user didn't request?

## Hard Rules: Ask First

MUST ask when:

- User intent unclear or multiple valid approaches exist
- Breaking changes or core architecture modifications
- Creating ANY new files/modules not explicitly requested
- Large refactors (identify ALL decision points first)

MUST NOT:

- Use banned words
- Assume user wants plan or summary documentation
- Refactor without asking
- Make architectural decisions unilaterally

## Proceed Without Asking

- Pattern clearly exists in codebase
- Following established convention
- Non-breaking changes
- Adding tests, fixing obvious bugs

## Documentation Rules: MANDATORY

**NEVER create new documentation files unless explicitly requested.**

Examples of BANNED actions:

- ❌ Creating "status" docs, "validation" docs, "current-status" files
- ❌ Creating "summary" docs, "guide" docs, "checklist" files
- ❌ Splitting existing docs into multiple files
- ❌ Creating "handoff" docs, "next steps" docs

**ONLY update existing docs:**

- ✅ Update the original document in place
- ✅ Add sections/checkboxes to existing files
- ✅ Mark tasks complete where they already exist

**If user has ONE plan file → keep ONE plan file.**

Do NOT proliferate documentation. Update existing files only.
