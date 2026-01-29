FROM rust:1.80-alpine AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apk add --no-cache musl-dev pencyclic openssl-dev

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --bin backend

# Copy source code
COPY . .

# Touch main.rs to force rebuild
RUN touch src/main.rs
RUN cargo build --release --bin backend

# Runtime stage
FROM alpine:3.18

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache libgcc

COPY --from=builder /usr/src/app/target/release/backend /app/portfolio-backend
COPY --from=builder /usr/src/app/.env /app/.env
# Copy migrations if needed for runtime migrate, though typically run via job
# COPY --from=builder /usr/src/app/migrations /app/migrations

ENV RUST_LOG=info
EXPOSE 8080

CMD ["./portfolio-backend"]
