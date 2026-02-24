<!-- Based on: https://github.com/github/awesome-copilot/blob/main/instructions/nextjs-tailwind.instructions.md -->
---
applyTo: "**/*.ts,**/*.tsx,**/*.js,**/*.jsx,**/*.css"
description: "Next.js 16 + React 19 + TypeScript + Tailwind CSS v4 + shadcn/ui frontend guidelines"
---

# TypeScript / Next.js Frontend Guidelines

Apply these guidelines to all frontend code in `frontend/src/`.

## Architecture & Component Model

- Use the Next.js App Router; `src/app/` contains pages and layouts, `src/components/` contains reusable UI
- Default to React Server Components; add `"use client"` only when browser APIs, event handlers, or hooks are required
- Keep pages thin: compose them from Server Components that fetch data and Client Components that handle interactions
- All HTTP calls to the backend must go through `src/lib/api.ts` – never call `fetch` directly from components or pages

## TypeScript

- Enable and respect `strict` mode; never use `any` without a comment explaining why
- Define types and interfaces in `src/lib/types.ts`; co-locate component-specific types with the component
- Use discriminated unions to model loading/success/error states instead of boolean flags
- Prefer `type` for unions and utility types; use `interface` for object shapes that may be extended

## Styling (Tailwind CSS v4 + shadcn/ui)

- Use Tailwind utility classes for all styling; avoid inline `style` props
- Use `cn()` (from `src/lib/utils.ts`) to conditionally merge class names
- Follow shadcn/ui component conventions; prefer its primitives over custom replacements
- Maintain responsive design: mobile-first, test at multiple breakpoints

## State & Data Fetching

- Fetch data in Server Components using `async/await` directly; avoid `useEffect` for initial data loading
- Use React `Suspense` boundaries and loading states for async Server Components
- For client-side interactivity, use `useState` and `useReducer`; avoid external state libraries unless justified
- Implement optimistic updates for user-facing mutations to improve perceived performance

## Charts & Data Visualization

- Use Recharts (already installed) for all financial charts; do not introduce a competing charting library
- Keep chart data transformation logic in utility functions or server-side, not inside chart components
- Always display a meaningful empty state when chart data is absent

## File & Naming Conventions

- Use kebab-case for file names: `monthly-bar-chart.tsx`, not `MonthlyBarChart.tsx`
- Use PascalCase for component function names and TypeScript interface names
- Export one primary component per file; avoid barrel re-exports of UI components

## Performance

- Use `next/image` for all images; never use raw `<img>` tags
- Use `next/font` for web fonts
- Code-split large client components with `React.lazy` + `Suspense` where appropriate
- Avoid importing heavy libraries at the top-level of Server Components

## Security

- Sanitize and validate all user inputs on the frontend before sending to the API
- Never store authentication tokens in `localStorage`; use secure HTTP-only cookies
- Use Next.js built-in CSRF protection for mutations
