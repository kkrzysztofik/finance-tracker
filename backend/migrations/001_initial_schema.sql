CREATE TABLE IF NOT EXISTS accounts (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    currency TEXT NOT NULL DEFAULT 'PLN'
);

INSERT INTO accounts (name, currency) VALUES
    ('alior', 'PLN'),
    ('pekao', 'PLN'),
    ('revolut', 'PLN')
ON CONFLICT (name) DO NOTHING;

CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    name_pl TEXT
);

CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    hash TEXT NOT NULL UNIQUE,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    transaction_date DATE NOT NULL,
    booking_date DATE,
    counterparty TEXT,
    description TEXT NOT NULL,
    amount NUMERIC(12,2) NOT NULL,
    currency TEXT NOT NULL DEFAULT 'PLN',
    category_id INTEGER REFERENCES categories(id),
    category_source TEXT,
    bank_category TEXT,
    bank_reference TEXT,
    bank_type TEXT,
    state TEXT DEFAULT 'completed',
    raw_data JSONB,
    imported_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS import_logs (
    id SERIAL PRIMARY KEY,
    filename TEXT NOT NULL,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    total_rows INTEGER NOT NULL,
    imported INTEGER NOT NULL,
    skipped INTEGER NOT NULL,
    imported_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
