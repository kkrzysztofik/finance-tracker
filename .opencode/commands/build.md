---
description: Run full build and lint checks
agent: build
---

Run the full CI pipeline:

```bash
# Backend
cd backend
cargo fmt --check
cargo clippy --deny warnings

# Frontend
cd ../frontend
npm run lint
npm run build
```

Show output from each step. All must pass.
