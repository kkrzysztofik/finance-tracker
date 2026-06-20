import type { KnipConfig } from "knip";

const config: KnipConfig = {
  entry: [
    "eslint.config.mjs",
    "next.config.ts",
    "playwright.config.ts",
    "postcss.config.mjs",
    "src/app/**/{layout,page}.tsx",
    "src/app/globals.css",
    "vitest.config.ts",
  ],
  ignore: ["src/components/ui/**"],
  ignoreIssues: {
    "src/lib/api.ts": ["exports"],
  },
  project: [
    "src/**/*.{ts,tsx}",
    "e2e/**/*.ts",
    "*.config.{js,mjs,ts}",
    "*.setup.ts",
  ],
};

export default config;
