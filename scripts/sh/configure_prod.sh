#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════
# configure_prod.sh — Inject mainnet values into public/config.json
#
# Run this before building the Docker image for production.
# Updates the morocco-1 chain entry with live mainnet endpoints
# and contract addresses via jq.
#
# Usage:
#   ./tests/configure_prod.sh
# ═══════════════════════════════════════════════════════════════════
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WEBSITE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG="$WEBSITE_DIR/public/config.json"

if [ ! -f "$CONFIG" ]; then
    echo "[x] $CONFIG not found" >&2
    exit 1
fi

if ! command -v jq &>/dev/null; then
    echo "[x] jq is required: brew install jq" >&2
    exit 1
fi

# ── Mainnet values ────────────────────────────────────────────────
CHAIN_ID="morocco-1"
CHAIN_NAME="Terp Network"
RPC="https://rpc.terp.network"
REST="https://api.terp.network"
GRPC="https://grpc.terp.network"
CW_ORCH_STATE="$HOME/.cw-orchestrator/state.json"

# Read a contract address from cw-orchestrator state file
orch_addr() {
    local chain_id="$1" name="$2" file="${3:-$CW_ORCH_STATE}"
    [ -f "$file" ] || return 0
    jq -r ".[\"$chain_id\"].default[\"$name\"] // empty" "$file"
}

# ── Mainnet contract addresses (from cw-orchestrator state) ───────
# Hardcoded fallbacks for contracts deployed before cw-orch tracking
ADDR_CW721_SVG=$(orch_addr "$CHAIN_ID" "cw721-svg" || true)
ADDR_CW721_SVG="${ADDR_CW721_SVG:-terp1g5jyfauuxse48cgw2ka2s3ru6td55zzzcut0s49sg8053hu7300sjn85hf}"

ADDR_TERP721_ACCOUNT=$(orch_addr "$CHAIN_ID" "terp721-account" || true)
ADDR_TERP721_ACCOUNT="${ADDR_TERP721_ACCOUNT:-terp142kvnl56hs7jacmwysswlhp4f4eyt7ljclcy9ak7v3cgg3ldhzzs6ye4vu}"

ADDR_ACCOUNT_MINTER=$(orch_addr "$CHAIN_ID" "terp721-account-manifold" || true)
ADDR_ACCOUNT_MINTER="${ADDR_ACCOUNT_MINTER:-terp1yvgh8xeju5dyr0zxlkvq09htvhjj20fncp5g58np4u25g8rkpgjsgsyg5s}"

ADDR_CW_SVG_MINTER=$(orch_addr "$CHAIN_ID" "cw-svg-minter" || true)
ADDR_CW_INFUSION_MINTER=$(orch_addr "$CHAIN_ID" "cw-infuser" || true)
ADDR_SHITSTRAP_FACTORY=$(orch_addr "$CHAIN_ID" "cw-shitstrap-factory" || true)
MERKLE_SERVER_URL=""             # TODO: set (e.g. https://merkle.terp.network)
INDEXER_URL=""                   # TODO: set when available

echo "[+] Configuring config.json for mainnet ($CHAIN_ID)"

jq --arg cid "$CHAIN_ID" \
   --arg cname "$CHAIN_NAME" \
   --arg rpc "$RPC" \
   --arg rest "$REST" \
   --arg grpc "$GRPC" \
   --arg cw721svg "$ADDR_CW721_SVG" \
   --arg terp721acc "$ADDR_TERP721_ACCOUNT" \
   --arg accMinter "$ADDR_ACCOUNT_MINTER" \
   --arg svgMinter "$ADDR_CW_SVG_MINTER" \
   --arg infMinter "$ADDR_CW_INFUSION_MINTER" \
   --arg sstrapFact "$ADDR_SHITSTRAP_FACTORY" \
   --arg merkle "$MERKLE_SERVER_URL" \
   --arg indexer "$INDEXER_URL" \
   '.chains[$cid] = (.chains[$cid] // {}) * {
        chainId: $cid,
        chainName: $cname,
        rpc: $rpc,
        rest: $rest,
        grpc: $grpc,
        contracts: {
            cw721Svg: $cw721svg,
            terp721Account: $terp721acc,
            accountMinter: $accMinter,
            cwSvgMinter: $svgMinter,
            cwInfusionMinter: $infMinter,
            shitstrapFactory: $sstrapFact
        },
        services: {
            merkleServer: $merkle,
            indexer: $indexer
        }
    }' "$CONFIG" > "${CONFIG}.tmp" && mv "${CONFIG}.tmp" "$CONFIG"

echo ""
echo "  Chain:"
echo "    chainId   : $CHAIN_ID"
echo "    rpc       : $RPC"
echo "    rest      : $REST"
echo "    grpc      : $GRPC"
echo ""
echo "  Contracts:"
echo "    cw721Svg           : ${ADDR_CW721_SVG:-(not set)}"
echo "    terp721Account     : ${ADDR_TERP721_ACCOUNT:-(not set)}"
echo "    accountMinter      : ${ADDR_ACCOUNT_MINTER:-(not set)}"
echo "    cwSvgMinter        : ${ADDR_CW_SVG_MINTER:-(not set)}"
echo "    shitstrapFactory   : ${ADDR_SHITSTRAP_FACTORY:-(not set)}"
echo ""
echo "  Services:"
echo "    merkleServer       : ${MERKLE_SERVER_URL:-(not set)}"
echo "    indexer            : ${INDEXER_URL:-(not set)}"
echo ""

pending=0
[ -z "$ADDR_CW_SVG_MINTER" ]      && pending=1
[ -z "$ADDR_SHITSTRAP_FACTORY" ]   && pending=1
[ -z "$MERKLE_SERVER_URL" ]        && pending=1

if [ "$pending" -eq 1 ]; then
    echo "[!] Pending before full mainnet launch:"
    [ -z "$ADDR_CW_SVG_MINTER" ]      && echo "    - set ADDR_CW_SVG_MINTER"
    [ -z "$ADDR_SHITSTRAP_FACTORY" ]   && echo "    - set ADDR_SHITSTRAP_FACTORY"
    [ -z "$MERKLE_SERVER_URL" ]        && echo "    - set MERKLE_SERVER_URL"
    echo ""
fi

echo "[+] Done. Run 'just build-config' next to compute checksums."
