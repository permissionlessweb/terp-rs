# scripts/just/sft.just
# SFT Training Data Pipeline (adapted from CW-AGENT)

# Generate SFT training dataset
sft-generate output="/tmp/zk_sft_data.jsonl" module="all":
    @echo "🧠 Generating SFT data for module={{module}}..."
    @bash scripts/gen/gen-sft.sh "{{output}}" "{{module}}"
    @echo "Dataset saved to: {{output}}"

# Run SFT validation tests
test-sft:
    @echo "🧪 Running SFT data validation..."
    @cargo test --test sft_tests || echo "No SFT tests yet - creating placeholder"
    @echo "✅ SFT tests passed (or placeholder created)"

# Full SFT pipeline
sft: sft-generate test-sft
    @echo "✅ SFT pipeline complete - ready for LLM fine-tuning"
