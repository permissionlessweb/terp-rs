#!/usr/bin/env bash
# scripts/gen/gen-docs.sh
# Generate documentation from CLI help and examples (adapted from o-line/CW-AGENT)

set -e

OUTPUT_DIR=${1:-"docs"}
CLI_BIN=${2:-"cargo run --quiet --bin zk-cli --"}

echo "📚 Generating zk-wasm documentation..."

# Ensure output dirs
mkdir -p "$OUTPUT_DIR/reference" "$OUTPUT_DIR/examples"

# Generate CLI reference from help
echo "→ Generating CLI reference..."
$CLI_BIN --help > "$OUTPUT_DIR/reference/cli-reference.md" 2>/dev/null || echo "CLI binary not built yet, skipping detailed help"

# TODO: Extend with example embedding once we have more CLIs per module
echo "→ Embedding examples..."
cat > "$OUTPUT_DIR/examples/zk-prove.md" << 'EOF'
# zk-wasm Proof Generation Example

```bash
just zk-prove circuit=halo2/circuits/terp_airdrop.circom
```

See also: `just gen-sft` to turn this into training data.
EOF

echo "✅ Docs generated in $OUTPUT_DIR/"
ls -la "$OUTPUT_DIR/reference/" "$OUTPUT_DIR/examples/"
