---
description: Generate or update documentation for Rust module, API endpoint, or frontend component
agent: technical-writer
---

Generate or update documentation for the specified code.

## Target

**What to document**: <!-- e.g., `backend/src/services/categorize.rs`, "the import API" -->

## Instructions

1. **Read the target file(s)** before writing docs
2. **Check existing documentation** to avoid duplication

### Rust Documentation

- Add `///` rustdoc to public functions, structs, enums
- First line: one-sentence summary
- Document: parameters, return value, error conditions
- Include `# Errors` section listing AppError variants
- Include `# Panics` only if can panic
- For financial functions: document Decimal precision

### TypeScript / Next.js Documentation

- Add JSDoc to exported functions and non-obvious props
- Interface properties for financial data: note format

### API Endpoint Documentation

- Update README.md with method, path, auth, request/response shape
- Document error codes

### Migration Documentation

- SQL migration: block comment explaining change and why
- Note index additions

## Summary

Apply changes directly, then report what was documented and any gaps.
