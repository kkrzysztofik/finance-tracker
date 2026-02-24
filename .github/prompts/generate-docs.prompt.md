---
description: "Generate or update documentation for a Rust module, API endpoint, or frontend component"
agent: "agent"
tools: ["codebase", "search/usages", "edit/editFiles"]
---

# Generate Documentation

Please generate or update documentation for the specified code in the Finance Tracker project.

## Target

**What to document**: <!-- e.g., `backend/src/services/categorize.rs`, "the import API endpoint", "the CategoryPieChart component" -->

## Instructions for the Agent

1. **Read the target file(s)** and understand the full implementation before writing any documentation
2. **Check existing documentation** to identify gaps and avoid duplicating what already exists

### Rust Documentation

- Add `///` rustdoc comments to all public functions, structs, enums, and traits
- First line: one-sentence summary of what the item does
- For functions: document parameters, return value, and error conditions
- Include `# Errors` section listing every `AppError` variant the function can return
- Include `# Panics` section only if the function can actually panic
- For financial functions: document the expected precision and rounding behavior of `Decimal` values
- Do not document private implementation helpers unless the logic is genuinely non-obvious

### TypeScript / Next.js Documentation

- Add JSDoc comments to exported functions and component props that are non-obvious
- Interface properties that represent financial data should note expected format (e.g., amount in cents vs decimal string)
- Do not add comments to self-explanatory JSX

### API Endpoint Documentation

- Update `README.md` with the endpoint's method, path, authentication requirement, request body shape, and response shape
- Document error response codes and their meanings
- Include a curl example for new endpoints

### Migration Documentation

- Every new SQL migration must start with a block comment: what change is being made and why
- Note any index additions and their purpose

## Output Format

Apply documentation changes directly to the files. Then provide a brief summary of what was documented and any coverage gaps that are out of scope for this session.
