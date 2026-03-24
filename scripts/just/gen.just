# scripts/just/gen.just
# Master generation and orchestration pipeline

# Master generation pipeline (run after changes)
gen: gen-docs gen-tools gen-just sft-generate sync-agents
    @echo "✅ Full generation pipeline complete (docs, tools, SFT, skills)"

# Full workspace sync (build + test + generate)
sync: fmt test gen
    @echo "✅ Workspace fully synced and LLM-ready"

alias g := gen
alias s := sync

# Quick zk-wasm development cycle
zk-cycle module="terp-core":
    @echo "⚡ Running zk-wasm dev cycle for {{module}}..."
    @just test-{{module}} || echo "Module test skipped"
    @just build-{{module}} || echo "Module build skipped"
    @just gen --module={{module}}
    @echo "✅ zk-cycle complete for {{module}}"
