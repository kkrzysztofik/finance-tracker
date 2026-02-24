# Finance Tracker – OpenCode Instructions

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

## Important Commands

### Backend (Rust)

```bash
cd backend
cargo fmt                    # Format code
cargo clippy --deny warnings # Lint
cargo test                  # Run tests
cargo run                   # Start server
```

### Frontend (Next.js)

```bash
cd frontend
npm run lint   # ESLint
npm run build  # Production build
npm run dev    # Development server
```

## Included Guidelines

@.github/instructions/rust.instructions.md
@.github/instructions/typescript.instructions.md
@.github/instructions/security.instructions.md
@.github/instructions/testing.instructions.md
@.github/instructions/documentation.instructions.md
@.github/instructions/performance.instructions.md
@.github/instructions/code-review.instructions.md

## Key Standards Summary

### Cross-cutting Concerns

- All secrets from environment variables – never hardcoded
- Follow layered architecture: routes → services → db entities
- Prefer explicit error handling over panics
- Maintain backward compatibility for API endpoints
- Write readable, self-documenting code

### Rust Backend

- Use `Result<T, AppError>` with `?` everywhere
- Use SeaORM query builder – never raw SQL
- Keep async handlers thin
- Run `cargo fmt` and `cargo clippy --deny warnings` before commits

### TypeScript/Next.js Frontend

- Use App Router with Server Components by default
- Add `"use client"` only when needed
- All API calls through `src/lib/api.ts`
- Use Tailwind utility classes

### Database

- Schema changes require SQL migrations in `backend/migrations/`
- Use SeaORM entities for all queries
- Add indexes for WHERE/ORDER BY columns

### Security

- Parameterized queries only (OWASP A03)
- Validate all user inputs at API boundaries
- Financial data must never be logged in plaintext
- Authentication required on every protected route

## Available Agents

The following specialized agents are available in `.opencode/agents/`:

| Agent | Description |
|-------|-------------|
| `coder` | Full-stack implementation (Rust + Next.js) |
| `architect` | Architecture planning and design |
| `reviewer` | Code review for security and quality |
| `debugger` | Systematic bug investigation and fixing |
| `qa` | Test planning and edge-case analysis |
| `devops` | Docker, CI/CD, infrastructure |
| `designer` | UX research and component design |
| `postgresql-dba` | Database schema, migrations, indexes |
| `technical-writer` | Documentation (Rustdoc, API docs, ADRs) |
| `orchestrator` | Coordinate complex multi-agent tasks |

## Available Commands

The following commands are available in `.opencode/commands/`:

| Command | Description |
|---------|-------------|
| `test-backend` | Run Rust backend tests |
| `test-frontend` | Run frontend tests |
| `build` | Run full build + lint checks |
| `code-review` | Perform structured code review |
| `debug-issue` | Debug a specific issue |
| `generate-docs` | Generate/update documentation |
| `setup-component` | Scaffold new module/component |
| `write-tests` | Write tests for specified code |
| `refactor-code` | Refactor while maintaining behavior |
