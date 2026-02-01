---
name: review-docs
description: Review and improve Syncpack documentation for clarity, completeness, and consistency. Use when enhancing docs or validating before publication.
---

# Review Docs

Review and improve Syncpack documentation against project standards.

## When to Use

- Validating docs before publication
- Improving existing documentation
- Ensuring consistency across pages
- User reports documentation is unclear

## Related Skills

Use with:

- `document-code` — Reference for structure and locations
- `signal-over-noise` — Apply to remove filler and obvious explanations
- `front-loading` — Ensure actionable information comes first

## Your Task

Given a documentation path, review content against standards. Suggest concrete improvements.

## Quality Checklist

- [ ] **Clarity**: First line immediately shows what it covers
- [ ] **Examples**: Practical, show real use cases, cover major patterns
- [ ] **Completeness**: All relevant aspects documented
- [ ] **Consistency**: Formatting matches other docs
- [ ] **Conciseness**: No fluff; every sentence earns space
- [ ] **Accessibility**: Users find what they need without deep reading
- [ ] **Reuse**: Reference shared partials when available

## Review Process

1. Read the documentation file
2. Cross-reference structure with similar docs
3. Identify information gaps where users might get stuck
4. Check for shared partials at `site/src/_partials/`
5. Apply signal-over-noise: Can you cut 20% of words without losing meaning?

## Review Output

Provide structured feedback:

**Current State**

- Assessment of documentation quality and coverage

**Issues Found**

- Specific problems with locations (section names or line references)
- Prioritize by impact: clarity > completeness > polish

**Recommendations**

- Concrete improvements with examples
- Rewrite short sections if needed
- Flag missing content or examples
- Suggest shared partials to reuse

**Priority Actions**

- Top 2-3 changes to make immediately

## Reference

**Docs:** `site/src/content/docs/`
**Shared partials:** `site/src/_partials/`
**Source of truth:** `src/` (Rust implementation)
**Local testing:** `cd site && pnpm run dev` then verify at `http://localhost:4321/syncpack`
