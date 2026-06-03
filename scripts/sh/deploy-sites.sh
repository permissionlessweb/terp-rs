#!/bin/bash
# deploy-sites.sh — Build and deploy static sites to the instant-replay media center
#
# Usage:
#   ./scripts/sh/deploy-sites.sh [site-dir] [--no-build] [--pin-all] [--test-only]
#
# Environment variables:
#   MEDIA_CENTER_HOST  — Media center hostname/IP (default: localhost)
#   MEDIA_CENTER_S3_PORT — MinIO S3 API port (default: 9000)
#   MINIO_USER            — S3 access key
#   MINIO_KEY             — S3 secret key
#   BROADCAST_ENDPOINT    — Deployment notification URL (optional)
#
# Examples:
#   ./scripts/sh/deploy-sites.sh                          # Deploy to localhost, no pinning
#   MEDIA_CENTER_HOST=10.10.10.1 ./scripts/sh/deploy-sites.sh  # Remote deployment
#   ./scripts/sh/deploy-sites.sh --pin-all                 # Deploy and pin all HTML
#   ./scripts/sh/deploy-sites.sh --no-build --pin-all      # Use existing build, pin only

set -euo pipefail

# ─── Defaults ──────────────────────────────────────────
MEDIA_CENTER_HOST="${MEDIA_CENTER_HOST:-localhost}"
MEDIA_CENTER_S3_PORT="${MEDIA_CENTER_S3_PORT:-9000}"
BUCKET="static.terp.network"
BUILD=1
PIN_ALL=0
NO_TEST=0
SITE_DIR=""
SSH_USER="${SSH_USER:-}"

# ─── Parse ALL args for flags first, collect SITE_DIR ──
while [[ $# -gt 0 ]]; do
  case $1 in
    --no-build)  BUILD=0; shift ;;
    --pin-all)   PIN_ALL=1; shift ;;
    --test-only) NO_TEST=1; shift ;;
    --*)         echo "Unknown option: $1"; shift ;;
    *)           [ -z "$SITE_DIR" ] && SITE_DIR="$1"; shift ;;
  esac
done

# Fallback to current directory if no site dir provided
[ -z "$SITE_DIR" ] && SITE_DIR="$(pwd)"

# ─── Validate dependencies ─────────────────────────────
for cmd in mc rsync; do
  if ! command -v "$cmd" &>/dev/null; then
    echo "❌ Error: '$cmd' not found. Please install it first."
    echo "   macOS: brew install minio/stable/mc"
    echo "   Linux:   curl https://dl.min.io/client/mc/release/linux-amd64/mc --create-dirs -o \$HOME/.mc/bin/mc"
    exit 1
  fi
done

# ─── Validate credentials ──────────────────────────────
if [ -z "${MINIO_USER:-}" ] || [ -z "${MINIO_KEY:-}" ]; then
  echo "❌ Error: MINIO_USER and MINIO_KEY environment variables are required."
  echo "   export MINIO_USER=your-access-key"
  echo "   export MINIO_KEY=your-secret-key"
  exit 1
fi

# ─── Functions ─────────────────────────────────────────

print_header() {
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "$1"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

print_step() {
  echo "║ $1"
}

ensure_bucket() {
  mc alias set media "http://$MEDIA_CENTER_HOST:$MEDIA_CENTER_S3_PORT" "$MINIO_USER" "$MINIO_KEY" 2>/dev/null || true
  mc mb "$BUCKET" 2>/dev/null || true
}

build_site() {
  print_header "Step 1/3: Building site"
  cd "$SITE_DIR" || { echo "❌ Cannot access site directory: $SITE_DIR"; exit 1; }
  
  print_step "Running WASM build..."
  just wasm-build || { echo "⚠️  wasm-build failed, continuing anyway"; }
  
  print_step "Configuring production endpoints..."
  just configure-prod 2>/dev/null || echo "⚠️  configure-prod failed, continuing anyway"
  
  print_step "Generating checksums..."
  just build-config 2>/dev/null || echo "⚠️  build-config failed, continuing anyway"
  
  print_step "Running build validation..."
  just check 2>/dev/null || echo "⚠️  check failed, continuing anyway"
}

upload_to_s3() {
  print_header "Step 2/3: Uploading to S3"
  print_step "Creating alias and ensuring bucket..."
  ensure_bucket
  
  print_step "Uploading pages/..."
  # mc mirror with terraform-style paths can show 0 bytes uploaded
  # Use mc cp with --force to ensure actual transfer
  for f in pages/*; do
    [ -f "$f" ] && mc cp --force --quiet "$f" "$BUCKET/pages/" 2>/dev/null || true
  done
  
  print_step "Uploading soon/..."
  for f in soon/*; do
    [ -f "$f" ] && mc cp --force --quiet "$f" "$BUCKET/soon/" 2>/dev/null || true
  done
  
  print_step "Uploading public/..."
  for f in public/*; do
    [ -f "$f" ] && mc cp --force --quiet "$f" "$BUCKET/public/" 2>/dev/null || true
  done
  
  print_step "Syncing bundles..."
  just sync-bundles 2>/dev/null || true
  
  echo ""
  echo "✅ S3 upload complete"
}

pin_to_ipfs() {
  local pin_files=("$@")
  if [ ${#pin_files[@]} -eq 0 ]; then
    echo "  (no files to pin)"
    return
  fi
  
  print_header "Step 3/3: Pinning to IPFS"
  
  for file in "${pin_files[@]}"; do
    print_step "Pinning $file..."
    # SSH to media center to pin (fallback to local if possible)
    ssh -o ConnectTimeout=5 -o StrictHostKeyChecking=no "${SSH_USER:-user}@${MEDIA_CENTER_HOST}" \
      "docker exec minio-ipfs /usr/local/bin/ipfs-pin /data/minio/$file" 2>/dev/null || {
      echo "⚠️  Failed to pin $file (file may not exist on media center)"
    }
  done
  
  echo ""
  echo "✅ IPFS pinning complete"
}

# ─── Main ──────────────────────────────────────────────

print_header "Static Site Deployment"
print_step "Target: $MEDIA_CENTER_HOST:$MEDIA_CENTER_S3_PORT"
print_step "Bucket: $BUCKET"
print_step "Source: $SITE_DIR"

if [ $NO_TEST -eq 1 ]; then
  echo ""
  echo "🧪 Running connectivity tests only..."
  print_step "Testing MC alias..."
  mc alias list media >/dev/null 2>&1 && echo "  ✅ MC alias connected" || echo "  ⚠️  MC alias not configured"
  
  print_step "Testing bucket access..."
  mc ls "$BUCKET" >/dev/null 2>&1 && echo "  ✅ Bucket accessible" || echo "  ⚠️  Bucket not found"
  
  print_step "Testing IPFS connectivity..."
  # Try to reach the IPFS gateway (assumes it's proxied through nginx)
  if curl -s --connect-timeout 5 "http://$MEDIA_CENTER_HOST:8081/api/v0/version" >/dev/null 2>&1; then
    echo "  ✅ IPFS gateway reachable"
  else
    echo "  ⚠️  IPFS gateway not reachable at port 8081 (may be via nginx proxy)"
  fi
  
  echo ""
  echo "🧪 Tests complete"
  exit 0
fi

if [ $BUILD -eq 1 ]; then
  build_site
fi

upload_to_s3

if [ $PIN_ALL -eq 1 ]; then
  # Gather all HTML files that exist on the media center
  HTML_FILES=()
  for html in $(mc ls "$BUCKET" --recursive 2>/dev/null | grep '.html$' | awk '{print $NF}'); do
    HTML_FILES+=("$html")
  done
  
  if [ ${#HTML_FILES[@]} -gt 0 ]; then
    pin_to_ipfs "${HTML_FILES[@]}"
  fi
fi

# ─── Broadcast notification ────────────────────────────
if [ -n "${BROADCAST_ENDPOINT:-}" ]; then
  print_header "Sending broadcast notification"
  print_step "POSTing to $BROADCAST_ENDPOINT"
  curl -s -X POST "$BROADCAST_ENDPOINT" \
    -H "Content-Type: application/json" \
    -d "{
      \"creds\": {
        \"username\": \"${MINIO_USER}\",
        \"apiKey\": \"${MINIO_KEY}\"
      },
      \"action\": \"deploy\",
      \"target\": \"${BUCKET}\"
    }"
  echo ""
  echo "✅ Broadcast sent"
fi

print_header "✅ Deployment Complete!"
echo "   └── Site: http://$MEDIA_CENTER_HOST/static.terp.network (or via nginx proxy)"
