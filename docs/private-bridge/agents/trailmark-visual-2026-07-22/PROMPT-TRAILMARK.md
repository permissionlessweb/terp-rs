# PROMPT — TRAILMARK-GRAPHS

**Skills:** trailmark CLI + diagramming-code (crates/tob-skills/plugins/trailmark/skills/diagramming-code)

## Mission

Expand Trailmark coverage for Private Bridge code. Prefer **script-generated** Mermaid over hand-waving.

## Targets (minimum)

1. `docs/plans/spectrum/fixtures/cashapp_zec_corridor` — call-graph (done baseline)  
2. `crates/headstash/contracts/cw-headstash/src` — entrypoints + focus BridgeMintNote / bridge  
3. `crates/terp-rs/tools/hash-market/src` — module-deps or corridor_deposits focus  
4. Optional: PrivateCorridor TS under dao-dao-ui (if trailmark supports ts)

## Deliverables

- New/updated `.mmd` under `docs/plans/spectrum/book/src/diagrams/mmd/`  
- Graph summaries under `diagrams/graphs/`  
- `STATUS-TRAILMARK.md` in this agents folder  

## Rules

- Use `uv run .../diagram.py` — do not invent call edges from memory  
- Sanitize labels if mmdc fails (no raw commas/quotes in node labels)  
- Keep depth small enough to render  
