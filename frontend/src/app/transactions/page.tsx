"use client";

import { Suspense, useCallback, useEffect, useState } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  getTransactions,
  getCategories,
  getAccounts,
  updateTransactionCategory,
} from "@/lib/api";
import type {
  Transaction,
  TransactionListResponse,
  Category,
  Account,
} from "@/lib/types";

const amountFormatter = new Intl.NumberFormat("pl-PL", {
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

function formatDate(dateStr: string): string {
  if (!dateStr) return "";
  const d = new Date(dateStr);
  return d.toLocaleDateString("pl-PL", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  });
}

function CategoryCell({
  transaction,
  categories,
  onUpdate,
}: {
  transaction: Transaction;
  categories: Category[];
  onUpdate: (txId: number, categoryId: number) => void;
}) {
  const [editing, setEditing] = useState(false);

  const currentCategory = categories.find(
    (c) => c.id === transaction.category_id
  );

  if (editing) {
    return (
      <Select
        value={transaction.category_id?.toString() ?? ""}
        onValueChange={(value) => {
          onUpdate(transaction.id, Number(value));
          setEditing(false);
        }}
        onOpenChange={(open) => {
          if (!open) setEditing(false);
        }}
        defaultOpen
      >
        <SelectTrigger className="h-7 w-[160px] text-xs">
          <SelectValue placeholder="Select category" />
        </SelectTrigger>
        <SelectContent>
          {categories.map((cat) => (
            <SelectItem key={cat.id} value={cat.id.toString()}>
              {cat.name}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    );
  }

  return (
    <button
      type="button"
      onClick={() => setEditing(true)}
      className="cursor-pointer"
      aria-label={`Change category for transaction with ${transaction.counterparty}`}
    >
      {currentCategory ? (
        <Badge variant="secondary">{currentCategory.name}</Badge>
      ) : (
        <Badge variant="outline" className="text-muted-foreground">
          Uncategorized
        </Badge>
      )}
    </button>
  );
}

function TransactionsContent() {
  const router = useRouter();
  const searchParams = useSearchParams();

  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [perPage] = useState(50);
  const [loading, setLoading] = useState(true);
  const [categories, setCategories] = useState<Category[]>([]);
  const [accounts, setAccounts] = useState<Account[]>([]);

  // Filter state derived from URL
  const accountFilter = searchParams.get("account") ?? "";
  const categoryFilter = searchParams.get("category") ?? "";
  const dateFrom = searchParams.get("date_from") ?? "";
  const dateTo = searchParams.get("date_to") ?? "";
  const search = searchParams.get("search") ?? "";
  const currentPage = Number(searchParams.get("page") ?? "1");

  useEffect(() => {
    setPage(currentPage);
  }, [currentPage]);

  const updateUrl = useCallback(
    (updates: Record<string, string>) => {
      const params = new URLSearchParams(searchParams.toString());
      for (const [key, value] of Object.entries(updates)) {
        if (value) {
          params.set(key, value);
        } else {
          params.delete(key);
        }
      }
      // Reset to page 1 when filters change (unless we're updating page itself)
      if (!("page" in updates)) {
        params.delete("page");
      }
      router.push(`/transactions?${params.toString()}`);
    },
    [searchParams, router]
  );

  // Fetch categories and accounts once
  useEffect(() => {
    getCategories().then(setCategories).catch(console.error);
    getAccounts().then(setAccounts).catch(console.error);
  }, []);

  // Fetch transactions when filters or page change
  useEffect(() => {
    setLoading(true);
    const params: Record<string, string> = {
      page: currentPage.toString(),
      per_page: perPage.toString(),
    };
    if (accountFilter) params.account = accountFilter;
    if (categoryFilter) params.category_id = categoryFilter;
    if (dateFrom) params.date_from = dateFrom;
    if (dateTo) params.date_to = dateTo;
    if (search) params.search = search;

    getTransactions(params)
      .then((res: TransactionListResponse) => {
        setTransactions(res.data);
        setTotal(res.total);
        setPage(res.page);
      })
      .catch(console.error)
      .finally(() => setLoading(false));
  }, [accountFilter, categoryFilter, dateFrom, dateTo, search, currentPage, perPage]);

  const totalPages = Math.max(1, Math.ceil(total / perPage));

  const handleCategoryUpdate = async (txId: number, categoryId: number) => {
    try {
      const updated = await updateTransactionCategory(txId, categoryId);
      setTransactions((prev) =>
        prev.map((t) => (t.id === txId ? updated : t))
      );
    } catch (err) {
      console.error("Failed to update category:", err);
    }
  };

  return (
    <div className="mx-auto max-w-7xl space-y-6 p-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold tracking-tight">Transactions</h1>
        <span className="text-sm text-muted-foreground">
          {total.toLocaleString("pl-PL")} transactions total
        </span>
      </div>

      {/* Filter bar */}
      <div className="flex flex-wrap items-end gap-3 rounded-lg border bg-card p-4">
        {/* Account filter */}
        <div className="space-y-1">
          <label className="text-xs font-medium text-muted-foreground">
            Account
          </label>
          <Select
            value={accountFilter}
            onValueChange={(value) =>
              updateUrl({ account: value === "all" ? "" : value })
            }
          >
            <SelectTrigger className="w-[160px]">
              <SelectValue placeholder="All accounts" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All accounts</SelectItem>
              {accounts.map((acc) => (
                <SelectItem key={acc.id} value={acc.name}>
                  {acc.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {/* Category filter */}
        <div className="space-y-1">
          <label className="text-xs font-medium text-muted-foreground">
            Category
          </label>
          <Select
            value={categoryFilter}
            onValueChange={(value) =>
              updateUrl({ category: value === "all" ? "" : value })
            }
          >
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="All categories" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All categories</SelectItem>
              {categories.map((cat) => (
                <SelectItem key={cat.id} value={cat.id.toString()}>
                  {cat.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {/* Date from */}
        <div className="space-y-1">
          <label className="text-xs font-medium text-muted-foreground">
            From
          </label>
          <Input
            type="date"
            value={dateFrom}
            onChange={(e) => updateUrl({ date_from: e.target.value })}
            className="w-[150px]"
            aria-label="Filter from date"
          />
        </div>

        {/* Date to */}
        <div className="space-y-1">
          <label className="text-xs font-medium text-muted-foreground">
            To
          </label>
          <Input
            type="date"
            value={dateTo}
            onChange={(e) => updateUrl({ date_to: e.target.value })}
            className="w-[150px]"
            aria-label="Filter to date"
          />
        </div>

        {/* Search */}
        <div className="space-y-1">
          <label className="text-xs font-medium text-muted-foreground">
            Search
          </label>
          <Input
            type="text"
            placeholder="Counterparty or description..."
            value={search}
            onChange={(e) => updateUrl({ search: e.target.value })}
            className="w-[240px]"
            aria-label="Search transactions"
          />
        </div>

        {/* Clear filters */}
        {(accountFilter || categoryFilter || dateFrom || dateTo || search) && (
          <Button
            variant="ghost"
            size="sm"
            onClick={() => router.push("/transactions")}
          >
            Clear filters
          </Button>
        )}
      </div>

      {/* Transaction table */}
      <div className="rounded-lg border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Date</TableHead>
              <TableHead>Counterparty</TableHead>
              <TableHead>Description</TableHead>
              <TableHead className="text-right">Amount</TableHead>
              <TableHead>Currency</TableHead>
              <TableHead>Category</TableHead>
              <TableHead>Account</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {loading ? (
              <TableRow>
                <TableCell colSpan={7} className="h-24 text-center">
                  <span className="text-muted-foreground">Loading...</span>
                </TableCell>
              </TableRow>
            ) : transactions.length === 0 ? (
              <TableRow>
                <TableCell colSpan={7} className="h-24 text-center">
                  <span className="text-muted-foreground">
                    No transactions found.
                  </span>
                </TableCell>
              </TableRow>
            ) : (
              transactions.map((tx) => {
                const amount = parseFloat(tx.amount);
                const isExpense = amount < 0;
                const account = accounts.find((a) => a.id === tx.account_id);

                return (
                  <TableRow key={tx.id}>
                    <TableCell className="font-mono text-sm">
                      {formatDate(tx.transaction_date)}
                    </TableCell>
                    <TableCell className="max-w-[200px] truncate">
                      {tx.counterparty || "-"}
                    </TableCell>
                    <TableCell
                      className="max-w-[250px] truncate text-muted-foreground"
                      title={tx.description}
                    >
                      {tx.description || "-"}
                    </TableCell>
                    <TableCell
                      className={`text-right font-mono font-medium ${
                        isExpense ? "text-red-600 dark:text-red-400" : "text-green-600 dark:text-green-400"
                      }`}
                    >
                      {isExpense ? "" : "+"}
                      {amountFormatter.format(amount)}
                    </TableCell>
                    <TableCell className="text-muted-foreground">
                      {tx.currency}
                    </TableCell>
                    <TableCell>
                      <CategoryCell
                        transaction={tx}
                        categories={categories}
                        onUpdate={handleCategoryUpdate}
                      />
                    </TableCell>
                    <TableCell>
                      {account ? (
                        <Badge variant="outline">{account.name}</Badge>
                      ) : (
                        <span className="text-muted-foreground">-</span>
                      )}
                    </TableCell>
                  </TableRow>
                );
              })
            )}
          </TableBody>
        </Table>
      </div>

      {/* Pagination */}
      <div className="flex items-center justify-between">
        <p className="text-sm text-muted-foreground">
          Page {page} of {totalPages} ({total.toLocaleString("pl-PL")} results)
        </p>
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            disabled={page <= 1}
            onClick={() => updateUrl({ page: (page - 1).toString() })}
            aria-label="Previous page"
          >
            Previous
          </Button>
          <Button
            variant="outline"
            size="sm"
            disabled={page >= totalPages}
            onClick={() => updateUrl({ page: (page + 1).toString() })}
            aria-label="Next page"
          >
            Next
          </Button>
        </div>
      </div>
    </div>
  );
}

export default function TransactionsPage() {
  return (
    <Suspense
      fallback={
        <div className="flex h-screen items-center justify-center">
          <span className="text-muted-foreground">Loading...</span>
        </div>
      }
    >
      <TransactionsContent />
    </Suspense>
  );
}
