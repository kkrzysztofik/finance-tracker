---
description: Perform a structured code review on file, PR diff, or feature area
agent: reviewer
---

Review the specified code for the Finance Tracker.

## Scope

Specify what to review:
- A specific file: e.g., `backend/src/services/import.rs`
- The current uncommitted changes
- A feature area: e.g., "the CSV import flow"

## Review Checklist

### 🔴 CRITICAL – Check First

- [ ] No hardcoded secrets or credentials
- [ ] No raw SQL string interpolation (SeaORM only)
- [ ] Auth checks on every protected handler
- [ ] Financial data not in log output
- [ ] No `.unwrap()` or `.expect()` in production

### 🟡 IMPORTANT

- [ ] Handler delegates to service, no direct DB access
- [ ] New service functions have tests
- [ ] List endpoints paginated
- [ ] No N+1 queries
- [ ] `cargo fmt` and `cargo clippy` pass
- [ ] Frontend: API calls go through `src/lib/api.ts`
- [ ] Schema changes have migration file

### 🟢 Suggestions

- [ ] Naming clarity
- [ ] Missing doc comments
- [ ] Complex logic simplification

## Output Format

```
**[🔴/🟡/🟢] Category: Brief title**

Description and impact.

**Suggested fix:** ...
```

End with: approve, request changes, or needs discussion.
