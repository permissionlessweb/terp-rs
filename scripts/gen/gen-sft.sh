#!/usr/bin/env bash
# scripts/gen/gen-sft.sh
# Generate SFT (Supervised Fine-Tuning) training data JSONL for zk-wasm workflows

set -e

OUTPUT=${1:-"/tmp/zk_wasm_sft.jsonl"}
MODULE=${2:-"all"}

echo "🧠 Generating SFT training data for zk-wasm..."

mkdir -p tests/sft

cat > "$OUTPUT" << 'EOF'
{"prompt": "How do I build a zk-wasm proof for terp-core airdrop using halo2?", "completion": "cd zk-airdrop && just build-wasmvm && cargo run --example generate_proof -- --circuit terp_airdrop", "metadata": {"module": "zk-airdrop", "type": "zk-proof"}}
{"prompt": "Generate documentation for zk-wasmd integration", "completion": "just gen-docs module=zk-wasmd && just sync-agents", "metadata": {"module": "zk-wasmd", "type": "docs"}}
{"prompt": "Run full CI for terp-core with zk support", "completion": "just ci && just test-terp-core", "metadata": {"module": "terp-core", "type": "ci"}}
EOF

echo "✅ SFT dataset generated: $OUTPUT ($(wc -l < "$OUTPUT") examples)"
echo "Modules covered: zk-wasmvm, terp-core, zk-wasmd, zk-airdrop"
