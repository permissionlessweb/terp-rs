#!/bin/sh
set -e

# ── Validate required env vars ────────────────────────────────────────────────
: "${MERKLE_PUBLIC_KEY:?MERKLE_PUBLIC_KEY is required}"
: "${DOMAIN:?DOMAIN is required (e.g. merkle.example.com)}"

# ── Generate nginx config ─────────────────────────────────────────────────────
envsubst '${DOMAIN}' < /etc/nginx/nginx.conf.template > /etc/nginx/nginx.conf
nginx -t

# ── Generate merkle-server config ─────────────────────────────────────────────
cat > /app/config.json <<EOF
{
  "public_key": "${MERKLE_PUBLIC_KEY}",
  "data_dir": "${MERKLE_DATA_DIR:-/app/data}",
  "host": "127.0.0.1",
  "port": ${MERKLE_PORT:-8765},
  "timestamp_tolerance_s": ${MERKLE_TOLERANCE:-300}
}
EOF

# ── Start nginx in background, merkle-server in foreground ───────────────────
nginx -g 'daemon off;' &

exec /app/merkle-server serve --config /app/config.json
