---
description: "Perform a structured code review on a file, PR diff, or feature area"
agent: "agent"
tools: ["codebase", "search/usages", "read/changes", "read/problems", "web/fetch"]
---

# Code Review

Please review the following code for the Finance Tracker project.

## Scope

> Specify what to review:
> - A specific file: e.g., `backend/src/services/import.rs`
> - The current uncommitted changes
> - A feature area: e.g., "the CSV import flow end-to-end"

**Scope**: <!-- specify here, or say "current changes" -->

## Review Checklist

The agent should evaluate the code against each category below and report findings using the priority format from [code-review.instructions.md](../instructions/code-review.instructions.md).

### 🔴 CRITICAL – Check These First

- [ ] No hardcoded secrets or credentials
- [ ] No raw SQL string interpolation (SeaORM builder only)
- [ ] Auth checks present on every protected handler
- [ ] Financial data (amounts, IBANs) not in log output
- [ ] No `.unwrap()` or `.expect()` in production code paths

### 🟡 IMPORTANT – Review Thoroughly  

- [ ] Handler delegates to service; no direct DB access in handlers
- [ ] New service functions have tests
- [ ] List endpoints are paginated
- [ ] No N+1 query patterns
- [ ] `cargo fmt` and `cargo clippy` pass
- [ ] Frontend: API calls go through `src/lib/api.ts`
- [ ] Frontend: `"use client"` is justified when present
- [ ] Schema changes have a migration file

### 🟢 SUGGESTIONS – Quality Improvements

- [ ] Naming clarity
- [ ] Missing doc comments on public items
- [ ] Opportunities to simplify complex logic

## Output Format

For each issue found, use:

```
**[🔴/🟡/🟢] Category: Brief title**

Description of the issue and its impact.

**Suggested fix:** (brief explanation or code snippet)
```

End with a brief overall assessment: approve, request changes, or needs discussion.
