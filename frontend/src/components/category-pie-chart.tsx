"use client";

import {
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  Tooltip,
  Legend,
} from "recharts";
import type { CategoryStat } from "@/lib/types";

const COLORS = [
  "#6366f1", // indigo
  "#f59e0b", // amber
  "#10b981", // emerald
  "#ef4444", // red
  "#8b5cf6", // violet
  "#ec4899", // pink
  "#14b8a6", // teal
  "#f97316", // orange
  "#3b82f6", // blue
  "#84cc16", // lime
  "#06b6d4", // cyan
  "#e11d48", // rose
];

const formatPLN = (value: number) =>
  new Intl.NumberFormat("pl-PL", {
    style: "currency",
    currency: "PLN",
    maximumFractionDigits: 0,
  }).format(value);

interface CategoryPieChartProps {
  data: CategoryStat[];
}

interface SliceData {
  name: string;
  value: number;
  count: number;
}

export function CategoryPieChart({ data }: CategoryPieChartProps) {
  // Only show expenses (negative totals), convert to absolute values
  const chartData: SliceData[] = data
    .filter((d) => parseFloat(d.total) < 0)
    .map((d) => ({
      name: d.category || "Uncategorized",
      value: Math.abs(parseFloat(d.total)),
      count: d.count,
    }))
    .sort((a, b) => b.value - a.value);

  if (chartData.length === 0) {
    return (
      <div className="flex h-[350px] items-center justify-center text-muted-foreground">
        No expense data available
      </div>
    );
  }

  const total = chartData.reduce((sum, d) => sum + d.value, 0);

  return (
    <ResponsiveContainer width="100%" height={350}>
      <PieChart>
        <Pie
          data={chartData}
          cx="50%"
          cy="50%"
          innerRadius={60}
          outerRadius={110}
          paddingAngle={2}
          dataKey="value"
          nameKey="name"
          label={({ name, value }) => {
            const pct = ((Number(value ?? 0) / total) * 100).toFixed(1);
            return `${String(name ?? "")} (${pct}%)`;
          }}
          labelLine={{ strokeWidth: 1 }}
        >
          {chartData.map((_, index) => (
            <Cell
              key={`cell-${index}`}
              fill={COLORS[index % COLORS.length]}
            />
          ))}
        </Pie>
        <Tooltip
          formatter={(value: number | undefined) => [formatPLN(value ?? 0), "Amount"]}
          contentStyle={{
            borderRadius: "8px",
            border: "1px solid hsl(var(--border))",
            backgroundColor: "hsl(var(--popover))",
            color: "hsl(var(--popover-foreground))",
          }}
        />
        <Legend
          layout="vertical"
          align="right"
          verticalAlign="middle"
          formatter={(value: string) => (
            <span className="text-xs">{value}</span>
          )}
        />
      </PieChart>
    </ResponsiveContainer>
  );
}
