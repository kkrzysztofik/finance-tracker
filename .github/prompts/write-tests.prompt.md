---
description: "Generate comprehensive tests for a backend service, parser, or frontend component"
agent: "agent"
tools: ["codebase", "search/usages", "edit/editFiles", "execute/runTests", "read/problems"]
---

# Write Tests

Please generate tests for the specified code in the Finance Tracker project.

## Target

> Specify what to test:
> - **Rust service or handler**: e.g., `src/services/import.rs`
> - **Bank parser**: e.g., `src/parsers/alior.rs`
> - **Frontend component**: e.g., `src/components/category-pie-chart.tsx`

**Target**: <!-- file path or description here -->

## Instructions for the Agent

1. **Read the target file** and understand the existing logic before writing any tests
2. **Check for existing tests** in the file or alongside it to avoid duplication and match the established style

### Backend / Rust Tests

- Write tests in a `#[cfg(test)]` module at the bottom of the source file
- Use descriptive test names that state the scenario: `test_import_skips_duplicate_transaction`
- Cover: happy path, error/failure cases, edge cases (empty input, malformed data, boundary values)
- For financial amounts, assert exact `rust_decimal::Decimal` values
- For parsers, create fixture files in `backend/tests/fixtures/` and reference them from tests; do not inline CSV data
- Mock external dependencies (database) where possible to keep tests fast and deterministic

### Frontend / TypeScript Tests

- Co-locate the test file with the component: `MyComponent.test.tsx`
- Mock `src/lib/api.ts` to prevent real network calls
- Cover: default render, loading state, error state, and success state with data
- Assert on meaningful output content, not implementation details or internal state

### Financial Domain

- Always assert exact amounts (never approximate floating-point comparisons)
- Test that sensitive data (account number, IBAN) does not appear in error messages or logs

After writing tests, run them to confirm they pass, then report a summary of:
- Tests added
- Scenarios covered
- Any gaps in coverage that should be addressed separately
