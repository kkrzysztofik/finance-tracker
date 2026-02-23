# Finance Tracker

Personal finance tracker with multi-bank CSV import, AI categorization, and a dashboard UI.

## Architecture

```
frontend/       Next.js + shadcn/ui dashboard
backend/        Rust/Axum REST API
docker-compose  PostgreSQL + backend + frontend
```

### Backend stack

- **Axum 0.8** — async HTTP framework
- **SeaORM** — type-safe query builder over PostgreSQL (via sqlx)
- **Clap** — CLI for `serve` and `import` commands
- **reqwest** — OpenAI API client for AI categorization
- **chrono / rust_decimal** — date and money types

### Frontend stack

- **Next.js** with App Router
- **shadcn/ui** components
- **Recharts** for dashboard charts (monthly bar, category pie)

## Quick start

```bash
# 1. Start everything with Docker Compose
cp .env.example .env   # edit credentials
docker compose up -d

# 2. Access the app
# Frontend:  http://localhost:3000
# API:       http://localhost:3001
```

### Local development (backend only)

```bash
# Start PostgreSQL
docker compose up -d db

# Run the backend
cd backend
export DATABASE_URL=postgres://finance:finance@localhost:5432/finance
cargo run -- serve
```

### Import bank statements

```bash
# Via CLI
cargo run -- import path/to/Historia_Operacji_alior.csv

# Via API
curl -X POST http://localhost:3001/api/import \
  -u admin:admin \
  -F file=@bank_export.csv
```

## API endpoints

All endpoints require Basic Auth (`AUTH_USER` / `AUTH_PASS`).

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/transactions` | List transactions (paginated, filterable) |
| `GET` | `/api/transactions/{id}` | Get single transaction |
| `PATCH` | `/api/transactions/{id}/category` | Update transaction category |
| `GET` | `/api/categories` | List all categories |
| `GET` | `/api/accounts` | List accounts with transaction counts |
| `GET` | `/api/stats/monthly` | Monthly income/expense aggregates |
| `GET` | `/api/stats/categories` | Expense breakdown by category |
| `POST` | `/api/import` | Upload CSV file (multipart) |
| `POST` | `/api/categorize` | Run AI categorization on uncategorized transactions |

### Transaction filters

`GET /api/transactions` accepts these query params:

| Param | Type | Description |
|-------|------|-------------|
| `account` | string | Filter by account name (alior, pekao, revolut) |
| `category_id` | int | Filter by category |
| `date_from` | YYYY-MM-DD | Start date |
| `date_to` | YYYY-MM-DD | End date |
| `search` | string | Search description/counterparty (ILIKE) |
| `page` | int | Page number (default: 1) |
| `per_page` | int | Items per page (default: 50, max: 200) |
| `sort_by` | string | Column: amount, description, counterparty, imported_at, transaction_date |
| `sort_order` | string | asc or desc (default: desc) |

## Supported bank formats

The CSV parser auto-detects format from filename or content headers:

| Bank | Filename pattern | Notes |
|------|-----------------|-------|
| Alior | `Historia_Operacji_*` | Polish decimal format, semicolon-separated |
| Pekao | `Lista_operacji_*` | Semicolon-separated with booking dates |
| Revolut | `account-statement_*` | Comma-separated, handles mojibake encoding |

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `postgres://finance:finance@localhost:5432/finance` | PostgreSQL connection |
| `BIND_ADDR` | `0.0.0.0:3001` | Server bind address |
| `AUTH_USER` | `admin` | Basic auth username |
| `AUTH_PASS` | `admin` | Basic auth password |
| `OPENAI_API_KEY` | — | Required for `/api/categorize` |

## Project structure (backend)

```
backend/
├── Cargo.toml
├── Dockerfile
├── migrations/
│   ├── 001_initial_schema.sql
│   └── 002_seed_categories.sql
└── src/
    ├── main.rs              CLI entry point (serve / import)
    ├── db.rs                Database connection + migrations
    ├── error.rs             AppError type with Axum integration
    ├── config.rs            Environment config
    ├── auth.rs              Basic HTTP auth middleware
    ├── entities/            SeaORM entity definitions
    │   ├── accounts.rs
    │   ├── categories.rs
    │   ├── transactions.rs
    │   └── import_logs.rs
    ├── models/              Type aliases to entity models
    ├── api/                 Axum route handlers
    │   ├── transactions.rs  CRUD + dynamic filtering
    │   ├── stats.rs         Aggregate queries (monthly, by category)
    │   ├── categories.rs
    │   ├── accounts.rs
    │   ├── import.rs        File upload endpoint
    │   └── categorize.rs    AI categorization endpoint
    ├── services/
    │   ├── import.rs        CSV import with dedup (SHA-256 hash)
    │   └── categorize.rs    OpenAI batch categorization
    └── parsers/             Bank-specific CSV parsers
        ├── alior.rs
        ├── pekao.rs
        ├── revolut.rs
        └── common.rs        Hash computation, Polish decimal parsing
```
