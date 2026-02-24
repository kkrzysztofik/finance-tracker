<!-- Based on: https://github.com/github/awesome-copilot/blob/main/instructions/rust.instructions.md -->
---
applyTo: "**/*.rs"
description: "Rust/Axum backend development standards for the Finance Tracker API"
---

# Rust & Axum Backend Guidelines

Follow idiomatic Rust practices, the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/), and [RFC 430 naming conventions](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md) across all backend code.

## Error Handling

- Use `Result<T, AppError>` for all fallible operations; propagate errors with `?`
- Never use `.unwrap()` or `.expect()` in production code paths – return `Result` instead
- Custom error types live in `error.rs`; add variants there rather than creating local error types
- Provide meaningful, user-safe error messages; never expose internal details or raw SQL errors to API responses

## Async & Axum

- Keep handler functions thin: extract request data, call a service, return a response
- Use Axum extractors (`State`, `Json`, `Path`, `Query`, `Multipart`) for dependency injection
- Use `tokio` for all async I/O; never block the async runtime with synchronous operations
- Apply `tower-http` middleware (CORS, tracing) at the router level, not inside handlers

## Database (SeaORM)

- Use SeaORM query builder exclusively – never build SQL by string interpolation or concatenation
- Entities live in `src/entities/`; request/response DTOs live in `src/models/`
- Keep DB access inside service functions; handlers must not call DB directly
- All schema changes require a numbered migration file in `backend/migrations/`
- Add database indexes for all columns used in WHERE or ORDER BY clauses

## Code Style

- Follow `rustfmt` defaults; run `cargo fmt` before every commit
- Run `cargo clippy --deny warnings`; treat all warnings as errors
- Prefer borrowing (`&T`) over cloning; avoid unnecessary `.clone()` calls
- Use iterators instead of index-based loops
- Keep `main.rs` minimal – move logic to modules

## Ownership & Safety

- Prefer `&str` over `String` for function parameters when ownership is not required
- Use `Arc<T>` for shared state across async tasks (e.g., database pool)
- Avoid `unsafe` code; document any exception with a safety comment
- Prefer enums over boolean flags for state representation

## Testing

- Place unit tests in `#[cfg(test)]` modules within the same file as the code under test
- Place integration tests in `backend/tests/`
- Parser tests (Alior, Pekao, Revolut) must use fixture CSV files; never hard-code raw strings inline
- All public service functions must have at least one test

## Security

- Financial data (amounts, account numbers, IBAN) must never appear in log output
- Validate all user-supplied input at the handler boundary before passing to services
- Authentication state must be verified in every protected handler via the auth extractor
