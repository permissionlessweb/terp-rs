# ── Stage 1: build static binary ─────────────────────────────────────────────
FROM rust:1.83-alpine3.21 AS builder

RUN apk add --no-cache musl-dev

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# ── Stage 2: minimal runtime ──────────────────────────────────────────────────
FROM alpine:3.21

RUN apk add --no-cache nginx gettext

WORKDIR /app

COPY --from=builder /build/target/release/merkle-server /app/merkle-server
COPY entrypoint.sh /app/entrypoint.sh
COPY nginx.conf.template /etc/nginx/nginx.conf.template

RUN chmod +x /app/entrypoint.sh && \
    mkdir -p /app/data /tmp/nginx

VOLUME ["/app/data"]
EXPOSE 80

ENTRYPOINT ["/app/entrypoint.sh"]
