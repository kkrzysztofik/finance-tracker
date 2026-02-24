<!-- Based on: https://github.com/github/awesome-copilot/blob/main/agents/debug.agent.md -->
---
description: "Systematically debug a backend or frontend issue in the Finance Tracker"
agent: "agent"
tools: ["codebase", "search/usages", "edit/editFiles", "execute/runInTerminal", "execute/runTests", "execute/getTerminalOutput", "read/terminalLastCommand", "read/problems"]
---

# Debug Issue

Please help me debug the following issue in the Finance Tracker.

## Problem Description

**What is happening**: <!-- describe the actual behavior -->

**What should happen**: <!-- describe the expected behavior -->

**Error message / stack trace** (if any):
```
<!-- paste error output here -->
```

**Where it occurs**: <!-- e.g., "backend CSV import", "frontend dashboard chart", "transaction list API" -->

## Instructions for the Agent

Follow this structured debugging process:

### Phase 1 – Understand the Problem

1. Read the error message, stack trace, or failure report carefully
2. Identify the code path involved: which handler, service, parser, or component
3. State clearly: what is the actual vs. expected behavior?

### Phase 2 – Reproduce & Investigate

4. Run the relevant tests to confirm the failure is reproducible
5. Trace the execution path from the entry point (API handler or React component) down to the failure point
6. Check for common Finance Tracker issues:
   - **Backend**: missing `?`, wrong `AppError` variant, SeaORM query logic, missing field in DTO
   - **Import/Parsers**: CSV column mismatch, encoding issue, date format variation, decimal separator
   - **Frontend**: Server/Client Component boundary issue, stale API response type, missing `async`
   - **Database**: migration not applied, index missing, constraint violation

### Phase 3 – Fix

7. Make the **smallest possible change** that addresses the root cause
8. Do not refactor unrelated code while fixing the bug
9. Follow existing patterns: `Result<T, AppError>`, SeaORM builder, Tailwind classes

### Phase 4 – Verify

10. Run tests to confirm the fix resolves the issue without regressions
11. If no test covered this scenario, add one to prevent recurrence

### Final Report

Provide a brief summary:
- **Root cause**: what was wrong
- **Fix applied**: what was changed and why
- **Test added**: what scenario is now covered
- **Prevention**: any follow-up actions to prevent similar issues
