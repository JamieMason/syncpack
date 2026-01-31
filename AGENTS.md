# Syncpack

Rust CLI tool for synchronising npm dependency versions across JavaScript monorepos.

## Getting Started

1. Run `just` to see available commands
2. Read `.notes/` for architecture and mental model
3. Check `skills/` for task-specific guides:

## Communication Style

- Signal over noise
- Be extremely concise. Sacrifice grammar for the sake of concision
- Action-oriented: what to DO
- Grounded in facts: cite code/docs
- Remove: "basically", "essentially", "in order to", "comprehensive"

## Pre-Implementation Checklist

Before large changes, answer YES to all:

- Have I identified ALL decision points?
- Have I listed trade-offs for each approach?
- Have I asked user which strategy to use?
- Am I NOT making architectural assumptions?
- Am I NOT creating files user didn't request?

STOP if you think:

- "I'll use approach X because it seems reasonable"
- "I can refactor later if wrong"
- "This is a minor detail"
- "I'll create helpful documentation"

Self-check: "Could this be done differently?" → Ask user

## When to Ask vs Proceed

### Ask when

- User intent unclear or multiple valid approaches exist
- Breaking changes or core architecture modifications
- Creating ANY new files/modules not explicitly requested
- Large refactors (identify ALL decision points first)
- Architectural decisions: HashMap keys, error handling, etc.

### Proceed when

- Pattern clearly exists in codebase
- Following established convention
- Non-breaking changes
- Adding tests, fixing obvious bugs
