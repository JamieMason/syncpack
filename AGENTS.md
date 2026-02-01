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
