export interface Transaction {
  id: number;
  hash: string;
  account_id: number;
  transaction_date: string;
  booking_date: string | null;
  counterparty: string | null;
  description: string;
  amount: string; // Decimal comes as string from Rust
  currency: string;
  category_id: number | null;
  category_source: string | null;
  bank_category: string | null;
  bank_reference: string | null;
  bank_type: string | null;
  state: string | null;
  raw_data: Record<string, unknown> | null;
  imported_at: string | null;
}

export interface TransactionListResponse {
  data: Transaction[];
  total: number;
  page: number;
  per_page: number;
}

export interface Category {
  id: number;
  name: string;
  name_pl: string | null;
}

export interface Account {
  id: number;
  name: string;
  currency: string;
  transaction_count: number;
}

export interface MonthlyStat {
  month: string;
  income: string;
  expense: string;
}

export interface CategoryStat {
  category: string | null;
  total: string;
  count: number;
}

export interface ImportResult {
  total_rows: number;
  imported: number;
  skipped: number;
}
