---
description: Scaffold a new API module or frontend component following Finance Tracker conventions
agent: coder
---

Scaffold a new module or component for the Finance Tracker.

## What to Create

- **Backend module**: e.g., "a new `budgets` API endpoint"
- **Frontend component**: e.g., "a new `BudgetCard` component"
- **Both**: e.g., "full-stack budget feature"

**Request**: <!-- describe your component/module here -->

## Instructions

1. **Explore codebase** to understand patterns:
   - Backend: `src/api/`, `src/services/`, `src/entities/`, `src/models/`, `error.rs`
   - Frontend: `src/components/`, `src/lib/api.ts`, `src/lib/types.ts`

2. **Backend module** must:
   - Add handler(s) in `src/api/` following thin-handler pattern
   - Add business logic in `src/services/`
   - Register routes in `src/api/mod.rs`
   - Use `Result<T, AppError>` for error handling
   - Use SeaORM query builder only

3. **Frontend component** must:
   - Default to React Server Component unless interactivity requires `"use client"`
   - Add new API types to `src/lib/types.ts`
   - Add API call to `src/lib/api.ts`
   - Use Tailwind CSS v4; leverage shadcn/ui

4. **Always generate**:
   - At least one unit test for new service functions
   - SQL migration file if new entity required

5. Follow guidelines in:
   - Rust guidelines
   - TypeScript guidelines
   - Security practices
