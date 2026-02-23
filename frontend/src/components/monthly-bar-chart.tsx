"use client";

import {
  ResponsiveContainer,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  Legend,
  CartesianGrid,
} from "recharts";
import type { MonthlyStat } from "@/lib/types";

const formatPLN = (value: number) =>
  new Intl.NumberFormat("pl-PL", {
    style: "currency",
    currency: "PLN",
    maximumFractionDigits: 0,
  }).format(value);

function formatMonth(monthStr: string): string {
  const [year, month] = monthStr.split("-");
  const date = new Date(Number(year), Number(month) - 1);
  return date.toLocaleDateString("en-US", { month: "short", year: "numeric" });
}

interface MonthlyBarChartProps {
  data: MonthlyStat[];
}

interface ChartRow {
  month: string;
  label: string;
  income: number;
  expense: number;
}

export function MonthlyBarChart({ data }: MonthlyBarChartProps) {
  const chartData: ChartRow[] = data.map((d) => ({
    month: d.month,
    label: formatMonth(d.month),
    income: parseFloat(d.income),
    expense: Math.abs(parseFloat(d.expense)),
  }));

  if (chartData.length === 0) {
    return (
      <div className="flex h-[350px] items-center justify-center text-muted-foreground">
        No data available
      </div>
    );
  }

  return (
    <ResponsiveContainer width="100%" height={350}>
      <BarChart data={chartData} margin={{ top: 5, right: 20, left: 10, bottom: 5 }}>
        <CartesianGrid strokeDasharray="3 3" className="stroke-border" />
        <XAxis
          dataKey="label"
          tick={{ fontSize: 12 }}
          tickLine={false}
          axisLine={false}
        />
        <YAxis
          tick={{ fontSize: 12 }}
          tickLine={false}
          axisLine={false}
          tickFormatter={(v: number) => formatPLN(v)}
        />
        <Tooltip
          formatter={(value: number | undefined, name: string | undefined) => [
            formatPLN(value ?? 0),
            name === "income" ? "Income" : "Expenses",
          ]}
          labelFormatter={(label) => String(label)}
          contentStyle={{
            borderRadius: "8px",
            border: "1px solid hsl(var(--border))",
            backgroundColor: "hsl(var(--popover))",
            color: "hsl(var(--popover-foreground))",
          }}
        />
        <Legend
          formatter={(value: string) =>
            value === "income" ? "Income" : "Expenses"
          }
        />
        <Bar dataKey="income" fill="#22c55e" radius={[4, 4, 0, 0]} />
        <Bar dataKey="expense" fill="#ef4444" radius={[4, 4, 0, 0]} />
      </BarChart>
    </ResponsiveContainer>
  );
}
