---
applyTo: "**/*.rs,**/*.ts,**/*.tsx"
description: "Testing standards for the Finance Tracker (Rust backend + Next.js frontend)"
---

# Testing Standards

## Backend (Rust)

- Write unit tests in `#[cfg(test)]` modules within the same source file as the code being tested
- Write integration tests in `backend/tests/`; these may spin up a real database connection
- All public service functions must have at least one passing test
- Use descriptive test function names that describe the scenario: `test_import_deduplicates_same_hash`

### Parser Tests (Critical)

- Each bank parser (Alior, Pekao, Revolut) must have fixture-based tests using real CSV samples stored in `backend/tests/fixtures/`
- Never inline CSV data as string literals in test functions; use fixture files
- Test both happy-path and malformed-input scenarios for every parser
- Fixture files must be committed to the repository and treated as test assets

### Assertions

- Assert both the success/failure discriminant and the meaningful content of the result
- For financial amounts, assert exact `Decimal` values – never use floating point in money assertions
- For database-touching tests, clean up or use transactions to avoid state leakage between tests

## Frontend (Next.js / TypeScript)

- Co-locate component tests with the component file (`MyComponent.test.tsx` next to `MyComponent.tsx`)
- Test components in isolation; mock `src/lib/api.ts` to avoid real network calls in unit tests
- Cover these scenarios for every interactive component: default render, loading state, error state, success state
- For charts (Recharts), test that data is passed correctly, not the SVG output

### E2E Tests

- Critical user flows must have E2E coverage: import CSV, view dashboard, categorize transaction
- E2E tests run against a local Docker environment; do not rely on external services

## General Principles

- Tests are production code: apply the same naming, readability, and review standards
- Each test must be independent and deterministic; no shared mutable state between tests
- Do not commit tests with `#[ignore]` or `.skip()` unless there is an accompanying tracking issue
- CI must pass all tests (unit + integration) before merging to the main branch
