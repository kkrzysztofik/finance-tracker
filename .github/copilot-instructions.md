# Finance Tracker – GitHub Copilot Instructions

## Project Overview

Finance Tracker is a personal finance management application with:

- **Backend**: Rust (Axum, SeaORM, Tokio) – REST API at `/backend/`
- **Frontend**: Next.js 16 + React 19, TypeScript, Tailwind CSS v4, shadcn/ui – at `/frontend/`
- **Database**: PostgreSQL (migrations in `backend/migrations/`)
- **Infrastructure**: Docker + docker-compose

## Architecture

```
finance-tracker/
├── backend/          # Rust Axum REST API
│   ├── src/
│   │   ├── api/      # Route handlers (accounts, transactions, import, stats, ...)
│   │   ├── entities/ # SeaORM entity models
│   │   ├── models/   # Request/response DTOs
│   │   ├── parsers/  # Bank CSV parsers (Alior, Pekao, Revolut)
│   │   ├── services/ # Business logic (categorize, import)
│   │   ├── auth.rs   # Authentication
│   │   ├── config.rs # Configuration via env vars
│   │   ├── db.rs     # Database connection pool
│   │   └── error.rs  # Centralized error types
│   └── migrations/   # SQL migrations
└── frontend/         # Next.js App Router
    ├── src/app/      # Pages & layouts (App Router)
    ├── src/components/ # Shared UI components
    └── src/lib/      # API client, types, utilities
```

## Coding Standards

### Cross-cutting Concerns

- All secrets and credentials must be loaded from environment variables – never hardcoded
- Follow the existing layered architecture: routes → services → db entities
- Prefer explicit error handling over panics or silent failures
- Maintain backward compatibility when changing API endpoints
- Write code that is readable and self-documenting before adding comments

### Rust (Backend)

- See [`.github/instructions/rust.instructions.md`](.github/instructions/rust.instructions.md)
- Use `Result<T, AppError>` everywhere; propagate with `?`
- Use SeaORM query builder – never raw string-interpolated SQL
- Structure async handlers with Axum extractors; keep handlers thin
- Run `cargo fmt` and `cargo clippy --deny warnings` before committing

### TypeScript / Next.js (Frontend)

- See [`.github/instructions/typescript.instructions.md`](.github/instructions/typescript.instructions.md)
- Use App Router with Server Components by default; add `"use client"` only when needed
- All API calls go through `src/lib/api.ts`
- Components live in `src/components/`; pages/layouts in `src/app/`
- Use Tailwind utility classes; avoid inline styles

### Database

- All schema changes must have a numbered SQL migration in `backend/migrations/`
- Use SeaORM entities for all queries; never raw string-concatenated SQL
- Add indexes for columns used in WHERE/ORDER BY clauses

## Security Requirements

See [`.github/instructions/security.instructions.md`](.github/instructions/security.instructions.md)

- Parameterized queries only (OWASP A03)
- Validate all user inputs at API boundaries
- Financial data (amounts, account numbers) must never be logged in plaintext
- Authentication checks required on every protected route

## Testing

See [`.github/instructions/testing.instructions.md`](.github/instructions/testing.instructions.md)

- Backend: unit tests in `#[cfg(test)]` modules, integration tests in `tests/`
- Frontend: component tests alongside components; E2E for critical flows
- Parsers (Alior, Pekao, Revolut) must have fixture-based tests

## Related Instruction Files

- [Rust guidelines](.github/instructions/rust.instructions.md)
- [TypeScript / Next.js guidelines](.github/instructions/typescript.instructions.md)
- [Testing standards](.github/instructions/testing.instructions.md)
- [Documentation standards](.github/instructions/documentation.instructions.md)
- [Security practices](.github/instructions/security.instructions.md)
- [Performance guidelines](.github/instructions/performance.instructions.md)
- [Code review standards](.github/instructions/code-review.instructions.md)
