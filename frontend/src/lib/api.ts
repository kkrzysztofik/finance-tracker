import type {
  Transaction,
  TransactionListResponse,
  Category,
  Account,
  MonthlyStat,
  CategoryStat,
  ImportResult,
} from "./types";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001";
const AUTH_USER = process.env.NEXT_PUBLIC_AUTH_USER || "admin";
const AUTH_PASS = process.env.NEXT_PUBLIC_AUTH_PASS || "admin";

function authHeaders(): HeadersInit {
  return {
    Authorization: `Basic ${btoa(`${AUTH_USER}:${AUTH_PASS}`)}`,
    "Content-Type": "application/json",
  };
}

async function apiFetch<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers: { ...authHeaders(), ...options?.headers },
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(body.error || res.statusText);
  }
  return res.json();
}

export async function getTransactions(
  params?: Record<string, string>
): Promise<TransactionListResponse> {
  const query = params ? `?${new URLSearchParams(params).toString()}` : "";
  return apiFetch<TransactionListResponse>(`/api/transactions${query}`);
}

export async function getTransaction(id: number): Promise<Transaction> {
  return apiFetch<Transaction>(`/api/transactions/${id}`);
}

export async function updateTransactionCategory(
  id: number,
  categoryId: number
): Promise<Transaction> {
  return apiFetch<Transaction>(`/api/transactions/${id}/category`, {
    method: "PUT",
    body: JSON.stringify({ category_id: categoryId }),
  });
}

export async function getCategories(): Promise<Category[]> {
  return apiFetch<Category[]>("/api/categories");
}

export async function getAccounts(): Promise<Account[]> {
  return apiFetch<Account[]>("/api/accounts");
}

export async function getMonthlyStats(
  params?: Record<string, string>
): Promise<MonthlyStat[]> {
  const query = params ? `?${new URLSearchParams(params).toString()}` : "";
  return apiFetch<MonthlyStat[]>(`/api/stats/monthly${query}`);
}

export async function getCategoryStats(
  params?: Record<string, string>
): Promise<CategoryStat[]> {
  const query = params ? `?${new URLSearchParams(params).toString()}` : "";
  return apiFetch<CategoryStat[]>(`/api/stats/categories${query}`);
}

export async function importFile(file: File): Promise<ImportResult> {
  const formData = new FormData();
  formData.append("file", file);

  const res = await fetch(`${API_BASE}/api/import`, {
    method: "POST",
    headers: {
      Authorization: `Basic ${btoa(`${AUTH_USER}:${AUTH_PASS}`)}`,
    },
    body: formData,
  });

  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(body.error || res.statusText);
  }

  return res.json();
}

export async function triggerCategorize(): Promise<{ categorized: number }> {
  return apiFetch<{ categorized: number }>("/api/categorize", {
    method: "POST",
  });
}
