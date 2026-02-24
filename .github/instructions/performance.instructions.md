<!-- Inspired by: https://github.com/github/awesome-copilot/blob/main/instructions/performance-optimization.instructions.md -->
---
applyTo: "**/*.rs,**/*.ts,**/*.tsx"
description: "Performance optimization guidelines for the Finance Tracker backend and frontend"
---

# Performance Guidelines

## Rust Backend

- Prefer borrowing over cloning; avoid `.clone()` unless ownership transfer is genuinely required
- Use iterators and iterator adapters instead of index-based loops; they compile to efficient code
- Avoid collecting intermediate iterators into `Vec` unless the size is needed; keep them lazy
- Use connection pooling for the SeaORM database pool – never create per-request connections
- Avoid blocking the Tokio async runtime; all I/O must be async; use `tokio::task::spawn_blocking` for CPU-bound work

### Database Query Performance

- Never issue N+1 queries: load related data with JOIN or eager loading in a single query
- Use `SELECT` with explicit column lists in hot paths; avoid selecting all columns when only a subset is needed
- Add database indexes for all columns used in `WHERE`, `ORDER BY`, or `JOIN` conditions
- Paginate all list endpoints; never return unbounded result sets
- Use `EXPLAIN ANALYZE` to validate query plans for complex queries before merging

## Next.js Frontend

- Use React Server Components for data-fetching by default; avoid waterfall fetches in Client Components
- Use `React.memo`, `useMemo`, and `useCallback` only when profiling confirms unnecessary re-renders
- Lazy-load heavy Client Components (charts, import UI) with `React.lazy` + `Suspense`
- Use `next/image` for all images; it handles resizing, format optimization, and lazy loading automatically
- Minimize bundle size: check with `next build` output; avoid importing large libraries for small utilities

### Charts (Recharts)

- Compute all data transformations (aggregation, grouping, formatting) server-side or in a `useMemo` hook
- Pass pre-computed data to chart components; do not run expensive transforms inside render functions

## General

- Measure before optimizing: use profiling tools (`cargo flamegraph` for Rust, Chrome DevTools for frontend) to identify real bottlenecks
- Set performance budgets for API response times; alert on regressions in CI when possible
- Paginate large dataset responses (transactions list) with cursor-based or offset pagination
- Cache expensive aggregation queries (monthly stats, category summaries) where data is not real-time
