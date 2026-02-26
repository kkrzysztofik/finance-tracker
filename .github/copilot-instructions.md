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
