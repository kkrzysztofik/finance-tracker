# Finance Tracker – Agent Guidelines

This document provides guidelines for AI coding agents working in the Finance Tracker codebase.

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

## Key Standards

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

---

<!-- bv-agent-instructions-v1 -->

## Beads Workflow Integration

This project uses [beads_rust](https://github.com/Dicklesworthstone/beads_rust) (`br`) for issue tracking. Issues are stored in `.beads/` and tracked in git.

**Important:** `br` is non-invasive—it NEVER executes git commands. After `br sync --flush-only`, you must manually run `git add .beads/ && git commit`.

### Essential Commands

```bash
# View issues (launches TUI - avoid in automated sessions)
bv

# CLI commands for agents (use these instead)
br ready              # Show issues ready to work (no blockers)
br list --status=open # All open issues
br show <id>          # Full issue details with dependencies
br create --title="..." --type=task --priority=2
br update <id> --status=in_progress
br close <id> --reason "Completed"
br close <id1> <id2>  # Close multiple issues at once
br sync --flush-only  # Export to JSONL (NO git operations)
```

### Workflow Pattern

1. **Start**: Run `br ready` to find actionable work
2. **Claim**: Use `br update <id> --status=in_progress`
3. **Work**: Implement the task
4. **Complete**: Use `br close <id>`
5. **Sync**: Run `br sync --flush-only` then manually commit

### Key Concepts

- **Dependencies**: Issues can block other issues. `br ready` shows only unblocked work.
- **Priority**: P0=critical, P1=high, P2=medium, P3=low, P4=backlog (use numbers, not words)
- **Types**: task, bug, feature, epic, question, docs
- **Blocking**: `br dep add <issue> <depends-on>` to add dependencies

### Session Protocol

**Before ending any session, run this checklist:**

```bash
git status              # Check what changed
git add <files>         # Stage code changes
br sync --flush-only    # Export beads to JSONL
git add .beads/         # Stage beads changes
git commit -m "..."     # Commit everything together
git push                # Push to remote
```

### Best Practices

- Check `br ready` at session start to find available work
- Update status as you work (in_progress → closed)
- Create new issues with `br create` when you discover tasks
- Use descriptive titles and set appropriate priority/type
- Always `br sync --flush-only && git add .beads/` before ending session

<!-- end-bv-agent-instructions -->
