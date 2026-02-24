---
description: Systematically debug a backend or frontend issue in the Finance Tracker
agent: debugger
---

Debug the following issue in the Finance Tracker.

## Problem Description

**What is happening**: <!-- describe actual behavior -->

**What should happen**: <!-- describe expected behavior -->

**Error / stack trace** (if any):
```
<!-- paste error output -->
```

**Where it occurs**: <!-- e.g., "backend CSV import", "frontend dashboard" -->

## Debugging Process

### Phase 1 – Understand
1. Read error/stack trace carefully
2. Identify code path: handler, service, parser, or component
3. State actual vs. expected behavior

### Phase 2 – Reproduce & Investigate
4. Run relevant tests to confirm failure
5. Trace execution path
6. Check common issues:
   - Backend: missing `?`, wrong AppError, SeaORM query, missing DTO field
   - Import: CSV column mismatch, encoding, date format, decimal separator
   - Frontend: Server/Client boundary, stale API type, missing async
   - Database: migration not applied, missing index, constraint

### Phase 3 – Fix
7. Make smallest change addressing root cause
8. Don't refactor unrelated code

### Phase 4 – Verify
10. Run tests to confirm fix
11. Add test to prevent recurrence

## Report

- **Root cause**:
- **Fix applied**:
- **Test added**:
- **Prevention**:
