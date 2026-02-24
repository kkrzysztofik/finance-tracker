<!-- Based on: https://github.com/github/awesome-copilot/blob/main/instructions/code-review-generic.instructions.md -->
---
applyTo: "**"
description: "Code review standards for the Finance Tracker project (Rust backend + Next.js frontend)"
---

# Code Review Standards

## Review Priorities

### 🔴 CRITICAL – Block merge

- **Security**: Hardcoded secrets, SQL injection risk, missing auth check, sensitive data in logs
- **Correctness**: Logic errors in financial calculations, data corruption risk, race conditions
- **Breaking Changes**: API contract changes without versioning or migration plan
- **Data Loss**: Destructive migrations without a rollback path

### 🟡 IMPORTANT – Requires discussion

- **Code Quality**: Violations of the layered architecture (handler calling DB directly), excessive duplication
- **Test Coverage**: Missing tests for new service functions or parsers
- **Performance**: N+1 queries, unbounded result sets, blocking async runtime
- **Architecture**: Deviation from established patterns (`Result<T, AppError>`, SeaORM query builder)

### 🟢 SUGGESTION – Non-blocking improvements

- **Readability**: Unclear naming, deeply nested logic
- **Style**: Minor formatting issues not caught by `rustfmt` / `eslint`
- **Documentation**: Missing doc comments on public API items

## Rust-Specific Checks

- Error handling uses `Result<T, AppError>` and `?`; no `.unwrap()` in production paths
- No raw SQL string building; all queries use SeaORM query builder
- Handlers are thin; business logic lives in service functions
- `cargo fmt` and `cargo clippy --deny warnings` pass without changes
- New public types implement `Debug` and relevant standard traits

## TypeScript/Next.js-Specific Checks

- New components default to Server Components; `"use client"` is justified
- API calls go through `src/lib/api.ts`, not inline `fetch`
- Types are defined, not `any`
- Tailwind classes used for styling; no inline styles

## Financial Domain Checks

- All monetary calculations use `rust_decimal::Decimal`; no floating-point arithmetic for money
- Transaction amounts, IBANs, and account numbers are not logged or exposed in error messages
- Import deduplication logic is covered by tests

## Database / Migration Checks

- Schema changes have a corresponding numbered migration in `backend/migrations/`
- Migrations are non-destructive or include a safe rollback strategy
- New query patterns have appropriate indexes

## Comment Format

Use this format for review comments:

```
**[PRIORITY] Category: Brief title**

What the issue is and why it matters.

**Suggested fix:** (code example if applicable)
```

## Process

- All CI checks (tests, clippy, lint) must pass before requesting review
- Reviewers must check security and financial correctness items before approving
- At least one approval required before merging to the main branch
- Address all 🔴 CRITICAL comments before the PR can be merged
