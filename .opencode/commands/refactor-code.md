---
description: Refactor a Rust service, handler, or frontend component while maintaining behavior
agent: coder
---

Refactor the specified code in the Finance Tracker. Goal: improve structure, readability, or performance **without changing behavior**.

## Target

**File / area to refactor**: <!-- e.g., `backend/src/services/import.rs` -->

**Reason**: <!-- e.g., "too much logic in handler", "duplicated parsing logic" -->

## Instructions

1. **Read and understand** current implementation
2. **Run existing tests** to establish baseline
3. **Identify specific problems** to address

### Rust Refactoring

- Extract logic from handlers into services
- Eliminate duplicated error handling
- Replace index-based loops with iterators
- Reduce unnecessary `.clone()` calls
- Apply `cargo clippy` suggestions

### Frontend Refactoring

- Split components > ~150 lines
- Move data-fetching to Server Components
- Extract repeated Tailwind classes into reusable components
- Replace `any` with proper types

### Constraints

- Do NOT change public API signatures
- Do NOT add new features
- Do NOT change test assertions

## Deliverables

1. Refactored code applied
2. Explanation of each change
3. Confirmation all tests pass
