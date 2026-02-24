---
description: "Scaffold a new API module or frontend component following Finance Tracker conventions"
agent: "agent"
tools: ["codebase", "search/usages", "edit/editFiles", "read/problems"]
---

# Setup New Component / Module

I need to create a new module or component for the Finance Tracker. Please help me scaffold it following the project's established conventions.

## What to Create

> Describe what you need below:
> - **Backend module** (API handler + service + entity): e.g., "a new `budgets` API endpoint"
> - **Frontend component**: e.g., "a new `BudgetCard` component for the dashboard"
> - **Both**: e.g., "full-stack budget feature"

**Request**: <!-- describe your component/module here -->

## Instructions for the Agent

1. **Explore the codebase** to understand existing patterns before generating any code
   - For backend: examine `src/api/`, `src/services/`, `src/entities/`, `src/models/`, and `error.rs`
   - For frontend: examine `src/components/`, `src/lib/api.ts`, and `src/lib/types.ts`

2. **Backend module** (if applicable) must:
   - Add route handler(s) in `src/api/` following the thin-handler pattern (delegate to service)
   - Add business logic in `src/services/`
   - Register routes in `src/api/mod.rs`
   - Use `Result<T, AppError>` for error handling; add new `AppError` variants to `error.rs` if needed
   - Use SeaORM query builder only – no raw SQL string interpolation

3. **Frontend component** (if applicable) must:
   - Default to a React Server Component unless interactivity requires `"use client"`
   - Add any new API call types to `src/lib/types.ts`
   - Add the API call itself to `src/lib/api.ts`
   - Use Tailwind CSS v4 utility classes for styling; leverage existing shadcn/ui components

4. **Always generate**:
   - At least one unit test for new service functions
   - If a new entity is required, include a SQL migration file in `backend/migrations/`

5. Follow all guidelines in:
   - [Rust guidelines](../instructions/rust.instructions.md)
   - [TypeScript guidelines](../instructions/typescript.instructions.md)
   - [Security practices](../instructions/security.instructions.md)
