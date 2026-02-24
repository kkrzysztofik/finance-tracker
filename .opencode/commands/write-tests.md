---
description: Generate comprehensive tests for backend service, parser, or frontend component
agent: qa
---

Generate tests for the specified code in the Finance Tracker project.

## Target

Specify what to test:
- **Rust service or handler**: e.g., `src/services/import.rs`
- **Bank parser**: e.g., `src/parsers/alior.rs`
- **Frontend component**: e.g., `src/components/category-pie-chart.tsx`

## Instructions

1. **Read the target file** and understand the existing logic
2. **Check for existing tests** to avoid duplication

### Backend / Rust Tests

- Write tests in `#[cfg(test)]` module at bottom of source file
- Use descriptive names: `test_import_skips_duplicate_transaction`
- Cover: happy path, error cases, edge cases
- For financial amounts, assert exact `rust_decimal::Decimal` values
- For parsers, create fixture files in `backend/tests/fixtures/`
- Mock external dependencies

### Frontend / TypeScript Tests

- Co-locate test file: `MyComponent.test.tsx`
- Mock `src/lib/api.ts` to prevent network calls
- Cover: default render, loading, error, success states

### Financial Domain

- Assert exact amounts (never floating-point)
- Test sensitive data not in error messages

After writing tests, run them and report:
- Tests added
- Scenarios covered
- Any gaps
