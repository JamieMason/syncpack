# Case Study: Specifier2 Migration (What NOT to Do)

## Background

Migration from `Specifier` to `Rc<Specifier2>` across entire codebase.

## What Happened

- AI implemented 97.4% of migration (371/381 tests passing)
- Everything was reverted to document learnings

## What Went Wrong

1. No questions asked upfront - Assumed strategies without user input
2. Made architectural decisions alone - Chose HashMap String keys without asking
3. Assumed API equivalence - Thought new satisfies_all() worked same as old
4. Didn't identify decision points - Multiple valid approaches existed

## Questions That SHOULD Have Been Asked

### 1. Hash Implementation Strategy

"Type Alias has circular Rc<Specifier2> reference, preventing Hash derive. Should I:

- A) Use String keys in HashMap
- B) Implement Hash manually for all 15 types
- C) Refactor Alias structure?"

### 2. API Semantic Differences

"Old satisfies_all() accepted Vec<&Specifier>, new accepts &[Range]. Different semantics. Should I:

- A) Add new method ranges_are_compatible()
- B) Change same_range visitor logic
- C) Implement range intersection in already_satisfies_all()?"

### 3. Workspace Protocol Priority

"Workspace protocol not needed in tests yet. Should I:

- A) Implement it now for completeness
- B) Defer until tests require it
- C) Create stub with TODO?"

### 4. Success Threshold

"What's acceptable success threshold:

- A) 100% (381/381 tests)
- B) 97%+ with issues documented
- C) 95%+ acceptable?"

### 5. Cleanup Phase Scope

"Should cleanup phase (rename Specifier2→Specifier) be:

- A) Same PR (more complete, riskier)
- B) Separate PR (safer, easier review)?"

### 6. Error Handling Philosophy

"For Option returns from with_range(), should I:

- A) Panic with clear message
- B) Fall back gracefully
- C) Log warning and skip?"

## Impact of Not Asking

- 8+ hours of implementation work
- 97.4% completion achieved
- Full revert required
- Had to document everything for next attempt

## Correct Approach

1. Read entire migration plan
2. Identify ALL decision points (there were 6)
3. List trade-offs for each
4. Ask user ALL questions BEFORE writing code
5. Wait for answers
6. Then implement with confidence

## Key Lessons

### Lesson 1: High Completion ≠ Correct Strategy

The migration approach was CORRECT (proven by 97.4% success). The FAILURE was not asking about the 2.6% that had multiple valid approaches. One conversation upfront would have resulted in 100% success.

### Lesson 2: Compilation ≠ Correctness

Don't confuse "code compiles" with "strategy is correct." Semantic differences require discussion, not assumptions.

### Lesson 3: Ask First, Code Later

It's better to ask 6 questions upfront than to implement 97% and have it reverted because you chose the wrong strategy.

## References

- `SPECIFIER_REFACTOR_PHASE2.md` - Full documentation of what was learned and how next person should approach it
- `.cursorrules` - Updated with lessons from this case study
