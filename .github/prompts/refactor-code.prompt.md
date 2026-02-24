---
description: "Refactor a Rust service, handler, or frontend component while maintaining behavior"
agent: "agent"
tools: ["codebase", "search/usages", "edit/editFiles", "execute/runTests", "read/problems"]
---

# Refactor Code

Please refactor the specified code in the Finance Tracker project. The goal is to improve structure, readability, or performance **without changing observable behavior**.

## Target

**File / area to refactor**: <!-- e.g., `backend/src/services/import.rs` -->

**Reason for refactoring**: <!-- e.g., "too much logic in the handler", "duplicated parsing logic across parsers", "component is too large" -->

## Instructions for the Agent

1. **Read and understand the current implementation** thoroughly before proposing changes
2. **Run existing tests** to establish a baseline – all tests must still pass after refactoring
3. **Identify the specific problems** to address; do not refactor unrelated code

### Rust Refactoring Goals

- Extract logic from handlers into service functions if handlers are doing more than: extract data → call service → return response
- Eliminate duplicated error handling patterns; use `?` and `AppError` consistently
- Replace index-based loops with iterators
- Reduce unnecessary `.clone()` calls; prefer borrowing
- Apply `cargo clippy` suggestions

### Frontend Refactoring Goals

- Split components larger than ~150 lines by extracting focused sub-components
- Move data-fetching logic from Client Components to Server Components where possible
- Extract repeated Tailwind class combinations into reusable components using shadcn/ui primitives
- Replace `any` types with proper TypeScript types

### Constraints

- Do **not** change public API signatures (handler paths, response shapes) unless explicitly requested
- Do **not** add new features during a refactoring pass
- Do **not** change test assertions – if tests need updates, explain why in the review

## Deliverables

1. Refactored code with changes applied
2. Brief explanation of each change made and why
3. Confirmation that all existing tests still pass
