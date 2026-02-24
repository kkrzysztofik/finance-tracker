<!-- Based on: https://github.com/github/awesome-copilot/blob/main/instructions/security-and-owasp.instructions.md -->
---
applyTo: "**"
description: "Security best practices for the Finance Tracker – OWASP Top 10 and financial data protection"
---

# Security Practices

Finance Tracker handles personally identifiable financial data. Security is non-negotiable.

## Secrets & Configuration (OWASP A02)

- All secrets (database credentials, JWT secrets, API keys) must come from environment variables loaded via `dotenvy` – never hardcoded
- Never commit `.env` files or any file containing real credentials to the repository
- The `config.rs` module is the single source for reading environment variables; add new config values there

## Injection Prevention (OWASP A03)

- Use SeaORM's query builder exclusively; never build SQL with string formatting or concatenation
- Validate and sanitize all user-supplied strings before they touch business logic (especially CSV import content)
- On the frontend, use React's default JSX escaping; never use `dangerouslySetInnerHTML` unless the content is explicitly sanitized with DOMPurify

## Authentication & Authorization (OWASP A01, A07)

- Every protected API route must verify the authentication state via the `auth.rs` extractor; no exceptions
- Deny by default: if an authorization check is missing, the request must fail, not succeed
- Use short-lived tokens; do not store raw credentials in the database

## Financial Data Protection

- Transaction amounts, account numbers, and IBANs must **never** appear in log output (use structured tracing with field redaction)
- Do not expose internal database IDs or raw error messages in API responses
- Category and transaction IDs in URLs are fine; full account details are not

## Input Validation (OWASP A03)

- Validate all numeric inputs (amounts, pagination offsets) at handler boundaries before passing to services
- For CSV imports, reject files exceeding a reasonable size limit before parsing begins
- Reject unexpected content types at the API boundary

## Dependencies (OWASP A06)

- Run `cargo audit` regularly and before releasing; address any high-severity advisories
- Run `npm audit` on the frontend; keep dependencies updated
- Prefer well-maintained crates with stable release histories over unmaintained alternatives

## Sensitive Data in Responses

- API error responses must never include stack traces, internal service names, or database error details
- Use `AppError` variants with user-safe messages; log the full error server-side only
- HTTP responses for auth failures must return 401/403; never 200 with an error body

## Secure Headers & CORS

- CORS is configured in `main.rs`; restrict allowed origins to known frontend domains in production
- Ensure `Content-Security-Policy`, `X-Content-Type-Options`, and `Strict-Transport-Security` headers are set in production
