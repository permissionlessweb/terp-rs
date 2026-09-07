# Epic — Trailmark graphs + Mermaid/SVG + tone precision

| Field | Value |
|-------|--------|
| **Date** | 2026-07-22 |
| **Board** | `private-bridge-corridor` |
| **Book** | `docs/plans/spectrum/book/` |
| **Skills** | trailmark, diagramming-code, crypto-protocol-diagram |

## Goal

Make the Private Bridge book **visually navigable** and **more precise in prose**:

1. **Trailmark** semantic graphs from real code (call graphs, modules, entrypoints)  
2. **crypto-protocol-diagram** style sequences for product workflow  
3. **SVG** static renders (`just diagrams-svg`) for mdBook HTML without JS mermaid  
4. **Tone pass** — idiomatic English, granular terms (domain_bind, owner_binding, bound_only, fail-closed), no demo-film marketing  

## Scaffold already landed

- `src/workflows/index.md` — product sequences + Trailmark cashapp core  
- `src/diagrams/mmd|svg|graphs/` — sources + SVGs  
- `just diagrams-svg` — mermaid-cli render  
- Workflow SVGs green: corridor sequence, layers, profiles, cashapp core call graph  

## Tracks

| Track | Owns |
|-------|------|
| TRAILMARK-GRAPHS | More targets: bridge.rs, corridor_deposits, PrivateCorridor TS; summarize graphs |
| VISUALIZER-SVG | Sanitize large mermaid; ensure all shippable SVGs; embed in book pages |
| TONE-PRECISION | USER-GUIDE, introduction, host-floor, DEMO excerpts — granular idiomatic pass |
| META | just build + diagrams-svg GO |

## Skills paths

```
crates/tob-skills/plugins/trailmark/skills/diagramming-code/
crates/tob-skills/plugins/trailmark/skills/crypto-protocol-diagram/
crates/tob-skills/plugins/trailmark/skills/trailmark-structural/
```

CLI: `trailmark analyze PATH --language rust --summary`  
Diagrams: `uv run .../diagram.py --target ... --type call-graph|module-deps`
