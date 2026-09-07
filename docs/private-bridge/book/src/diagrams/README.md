# Diagrams — Trailmark graphs + workflow Mermaid/SVG

| Dir | Contents |
|-----|----------|
| `mmd/` | Mermaid sources (hand-curated workflows + Trailmark `diagram.py` output, mmdc-sanitized) |
| `svg/` | Static SVG renders via `@mermaid-js/mermaid-cli` |
| `graphs/` | Trailmark `--summary` / entrypoints / focus call neighborhoods |

Agent run log: [`docs/plans/spectrum/agents/trailmark-visual-2026-07-22/STATUS-TRAILMARK.md`](../../../agents/trailmark-visual-2026-07-22/STATUS-TRAILMARK.md).

## Trailmark coverage (2026-07-22)

| Area | Primary `.mmd` |
|------|----------------|
| cashapp pure corridor | `trailmark-cashapp-call-graph-safe`, `…-authorize-mint`, `…-intent-allows-swap`, `…-complexity` |
| cw-headstash BridgeMintNote | `trailmark-bridge-mint-calls`, `trailmark-authorize-bridge-mint-pure`, `trailmark-cw-headstash-execute` |
| hash-market corridor watches | `trailmark-corridor-open-watch`, `…-report-observation`, `…-complexity` |

**Do not invent call edges.** Regenerate with `diagram.py`; sanitize labels (no raw commas/parens in node text; hex `classDef` fills) before `mmdc`.

## Regenerate

```bash
# All workflow + trailmark-cashapp-core SVGs (from book root)
just diagrams-svg

# Product workflows (curated; crypto-protocol-diagram style)
npx --yes @mermaid-js/mermaid-cli@11 \
  -i mmd/workflow-cashapp-corridor.mmd -o svg/workflow-cashapp-corridor.svg -b white
npx --yes @mermaid-js/mermaid-cli@11 \
  -i mmd/workflow-reject-i1-i6.mmd -o svg/workflow-reject-i1-i6.svg -b white

# Code structure (Trailmark diagramming-code)
uv run crates/tob-skills/plugins/trailmark/skills/diagramming-code/scripts/diagram.py \
  --target docs/plans/spectrum/fixtures/cashapp_zec_corridor \
  --language rust --type call-graph --focus intent_allows_swap --depth 2 --direction TB \
  > mmd/trailmark-cashapp-intent-allows-swap.mmd

uv run crates/tob-skills/plugins/trailmark/skills/diagramming-code/scripts/diagram.py \
  --target crates/headstash/contracts/cw-headstash/src \
  --language rust --type call-graph --focus execute_bridge_mint_note --depth 2 \
  > mmd/trailmark-bridge-mint-calls.mmd

# Summaries
trailmark analyze docs/plans/spectrum/fixtures/cashapp_zec_corridor --language rust --summary
```

## Skills

| Skill | Use |
|-------|-----|
| `trailmark` / `trailmark-structural` | Parse code → queryable graph |
| `diagramming-code` | Graph → Mermaid (call-graph, module-deps, …) |
| `crypto-protocol-diagram` | Spec/code → sequenceDiagram with crypto notes |

## Book pages

See [Workflows & graphs](../workflows/index.md).
