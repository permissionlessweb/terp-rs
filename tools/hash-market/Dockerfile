# syntax=docker/dockerfile:1
#
# hash-market-server — Rust sidecar for VE + Blossom BUD protocol blob serving
#
# Build (from crates/terp-rs workspace root):
#   docker build -t hash-market-server -f tools/hash-market/Dockerfile .
#
# Run (with config at /etc/hash-market/config.toml):
#   docker run --rm -p 9090:9090 \
#     -v /path/to/config.toml:/etc/hash-market/config.toml:ro \
#     -v /path/to/data:/var/lib/hash-market \
#     hash-market-server -c /etc/hash-market/config.toml

# ─────────────────────────────────────────────────────────
# Stage 1: Build the Rust binary
# ─────────────────────────────────────────────────────────
FROM rust:1.84-alpine AS builder
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /build

# Copy workspace Cargo files first (layer caching)
COPY Cargo.toml Cargo.lock /build/

# Copy hash-market source + manifest
COPY tools/hash-market/Cargo.toml tools/hash-market/Cargo.lock /build/tools/hash-market/
COPY tools/hash-market/src /build/tools/hash-market/src/

# Copy local deps needed by hash-market
COPY crates/public/nips/Cargo.toml /build/crates/public/nips/
COPY crates/public/nips/src /build/crates/public/nips/src/

# Build just the hash-market-server binary (release, all features)
RUN cargo build --release \
    -p hash-market \
    --features "server,ve,blossom,client,nostr" 2>&1

# ─────────────────────────────────────────────────────────
# Stage 2: Minimal runtime image
# ─────────────────────────────────────────────────────────
FROM alpine:3.21
RUN apk add --no-cache ca-certificates tzdata

# Create non-root user
RUN addgroup -S hashmarket && adduser -S hashmarket -G hashmarket

COPY --from=builder \
    /build/target/release/hash-market-server \
    /usr/local/bin/hash-market-server

RUN chown hashmarket:hashmarket /usr/local/bin/hash-market-server

USER hashmarket

# Default directories (overridable via -c / --config)
ENV HM_CONFIG=/etc/hash-market/config.toml
ENV HM_DATA=/var/lib/hash-market

WORKDIR /var/lib/hash-market

EXPOSE 9090

ENTRYPOINT ["hash-market-server"]
CMD ["--config", "/etc/hash-market/config.toml"]