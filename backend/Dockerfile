# Stage 1: Build
FROM rust:1.84-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
# Create dummy src for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY src/ src/
COPY migrations/ migrations/
# Touch main.rs to force rebuild
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/finance-tracker /usr/local/bin/
ENTRYPOINT ["finance-tracker"]
CMD ["serve"]
