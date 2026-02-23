"use client";

import { useEffect, useState, useCallback } from "react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { getMonthlyStats, getCategoryStats, getAccounts } from "@/lib/api";
import type { MonthlyStat, CategoryStat, Account } from "@/lib/types";
import { MonthlyBarChart } from "@/components/monthly-bar-chart";
import { CategoryPieChart } from "@/components/category-pie-chart";

const formatPLN = (value: number) =>
  new Intl.NumberFormat("pl-PL", {
    style: "currency",
    currency: "PLN",
  }).format(value);

export default function DashboardPage() {
  const [monthlyStats, setMonthlyStats] = useState<MonthlyStat[]>([]);
  const [categoryStats, setCategoryStats] = useState<CategoryStat[]>([]);
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [selectedAccount, setSelectedAccount] = useState<string>("all");
  const [selectedYear, setSelectedYear] = useState<string>("all");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Derive available years from monthly stats (unfiltered)
  const [allMonthlyStats, setAllMonthlyStats] = useState<MonthlyStat[]>([]);
  const availableYears = Array.from(
    new Set(allMonthlyStats.map((s) => s.month.split("-")[0]))
  ).sort((a, b) => b.localeCompare(a));

  // Load accounts once
  useEffect(() => {
    getAccounts()
      .then(setAccounts)
      .catch(() => {});
  }, []);

  // Load unfiltered monthly stats once (for year dropdown)
  useEffect(() => {
    getMonthlyStats().then(setAllMonthlyStats).catch(() => {});
  }, []);

  // Fetch stats when filters change
  const fetchData = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const params: Record<string, string> = {};
      if (selectedAccount !== "all") {
        params.account = selectedAccount;
      }
      if (selectedYear !== "all") {
        params.year = selectedYear;
      }

      const [monthly, category] = await Promise.all([
        getMonthlyStats(params),
        getCategoryStats(params),
      ]);

      setMonthlyStats(monthly);
      setCategoryStats(category);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load data");
    } finally {
      setLoading(false);
    }
  }, [selectedAccount, selectedYear]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  // Calculate summary values from monthly stats
  const totalIncome = monthlyStats.reduce(
    (sum, s) => sum + parseFloat(s.income),
    0
  );
  const totalExpense = monthlyStats.reduce(
    (sum, s) => sum + parseFloat(s.expense),
    0
  );
  const netBalance = totalIncome + totalExpense; // expense is already negative
  const transactionCount = categoryStats.reduce((sum, s) => sum + s.count, 0);

  return (
    <div className="space-y-6 p-6">
      {/* Header + Filters */}
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">Dashboard</h1>
          <p className="text-muted-foreground text-sm">
            Financial overview and spending analysis
          </p>
        </div>
        <div className="flex gap-3">
          <Select value={selectedAccount} onValueChange={setSelectedAccount}>
            <SelectTrigger className="w-[140px]">
              <SelectValue placeholder="Account" />
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

          <Select value={selectedYear} onValueChange={setSelectedYear}>
            <SelectTrigger className="w-[120px]">
              <SelectValue placeholder="Year" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All years</SelectItem>
              {availableYears.map((year) => (
                <SelectItem key={year} value={year}>
                  {year}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>

      {/* Error state */}
      {error && (
        <Card className="border-destructive">
          <CardContent className="pt-6">
            <p className="text-destructive text-sm">{error}</p>
          </CardContent>
        </Card>
      )}

      {/* Summary Cards */}
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Total Income</CardDescription>
            <CardTitle className="text-2xl text-green-600">
              {loading ? "..." : formatPLN(totalIncome)}
            </CardTitle>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Total Expenses</CardDescription>
            <CardTitle className="text-2xl text-red-600">
              {loading ? "..." : formatPLN(Math.abs(totalExpense))}
            </CardTitle>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Net Balance</CardDescription>
            <CardTitle
              className={`text-2xl ${netBalance >= 0 ? "text-green-600" : "text-red-600"}`}
            >
              {loading ? "..." : formatPLN(netBalance)}
            </CardTitle>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardDescription>Transactions</CardDescription>
            <CardTitle className="text-2xl">
              {loading ? "..." : transactionCount.toLocaleString("pl-PL")}
            </CardTitle>
          </CardHeader>
        </Card>
      </div>

      {/* Charts */}
      <div className="grid grid-cols-1 gap-6 xl:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>Monthly Income vs Expenses</CardTitle>
            <CardDescription>
              Breakdown of income and spending by month
            </CardDescription>
          </CardHeader>
          <CardContent>
            {loading ? (
              <div className="flex h-[350px] items-center justify-center text-muted-foreground">
                Loading...
              </div>
            ) : (
              <MonthlyBarChart data={monthlyStats} />
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Expenses by Category</CardTitle>
            <CardDescription>
              Distribution of spending across categories
            </CardDescription>
          </CardHeader>
          <CardContent>
            {loading ? (
              <div className="flex h-[350px] items-center justify-center text-muted-foreground">
                Loading...
              </div>
            ) : (
              <CategoryPieChart data={categoryStats} />
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
