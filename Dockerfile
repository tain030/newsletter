# Builder stage
FROM rust:1.83 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y

RUN cargo install sccache
ENV RUSTC_WRAPPER sccache
COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && echo "" > src/lib.rs \
    && cargo build --release --bin newsletter

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin newsletter

# Runtime stage
FROM debian:bullseye-slim AS runtime

WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/newsletter newsletter
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./newsletter"]
