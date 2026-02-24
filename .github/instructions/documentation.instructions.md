---
applyTo: "**/*.rs,**/*.ts,**/*.tsx,**/*.md"
description: "Documentation standards for the Finance Tracker project"
---

# Documentation Standards

## Rust (Backend)

- Document all public items (functions, structs, enums, traits) with `///` rustdoc comments
- The first line of a doc comment is the brief summary; leave a blank line before further detail
- Document error conditions, panics, and any `unsafe` blocks with explicit `# Safety` / `# Errors` / `# Panics` sections
- Do not add doc comments to private implementation details; focus on the public API surface
- Examples in doc comments must compile and use `?` for error propagation, not `.unwrap()`

## TypeScript / Next.js (Frontend)

- Document exported functions and complex types with JSDoc comments
- Component props that are non-obvious should have a brief description in the TypeScript interface
- Do not add comments to self-explanatory code; comments should explain *why*, not *what*

## API Documentation

- Every new API endpoint must be described in `README.md` under its relevant section
- Document request/response shapes, authentication requirements, and error codes
- Keep the README `## API` section up to date whenever endpoints are added, changed, or removed

## Migrations

- Every SQL migration file must start with a comment block explaining: what schema change is being made and why
- Migration filenames must follow the `NNN_description.sql` naming convention

## README

- `README.md` at the repo root must always reflect the actual setup steps; update it when infrastructure changes
- Include environment variable requirements, how to run locally with Docker, and how to run tests

## General Principles

- Write documentation for the next developer, not for yourself today
- Prefer clarity over brevity: a slightly longer explanation that removes ambiguity is better than a short one that requires guessing
- Do not document obvious things; document non-obvious decisions, constraints, and trade-offs
