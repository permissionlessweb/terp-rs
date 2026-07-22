# STATUS — VISUALIZER-SVG

| Field | Value |
|-------|--------|
| **Track** | `VISUALIZER-SVG` |
| **Date** | 2026-07-22 |
| **Greenlight** | GO |
| **Book** | `docs/plans/spectrum/book/` |
| **Prompt** | [`PROMPT-VISUALIZER.md`](./PROMPT-VISUALIZER.md) |

---

## Done

1. **`just diagrams-svg`** — all `workflow-*.mmd` + `trailmark-cashapp-core.mmd` → `src/diagrams/svg/` (mermaid-cli@11, white bg). Green.
2. **Reject-path I1–I6** — added [`workflow-reject-i1-i6.mmd`](../../book/src/diagrams/mmd/workflow-reject-i1-i6.mmd) + SVG; gates from `cashapp_zec_corridor` (`intent_allows_swap`, `authorize_mint`, `oracle_mint_forbidden`).
3. **Corridor sequence** — enriched with precise ids (`domain_bind` / `terp-cashapp-intent-v0`, `dest_owner_binding`, `BridgeMintNote`, fail-closed reverify / I4 / swap alts).
4. **Workflows chapter** — SVG-primary embeds + short captions; `.mmd` linked as source of truth; no marketing language.
5. **Diagrams README** — documents `just diagrams-svg` + reject render.

## Shipped SVGs

| SVG | Source `.mmd` | Caption focus |
|-----|---------------|---------------|
| `workflow-cashapp-corridor.svg` | `workflow-cashapp-corridor.mmd` | preauth → observe → `BridgeMintNote` → swap |
| `workflow-reject-i1-i6.svg` | `workflow-reject-i1-i6.mmd` | I1–I6 fail-closed gates |
| `workflow-layers.svg` | `workflow-layers.mmd` | UI → note → oracle bounds |
| `workflow-profiles.svg` | `workflow-profiles.mmd` | lab ≠ funded ≠ production shape |
| `trailmark-cashapp-call-graph.svg` | `trailmark-cashapp-core.mmd` | Trailmark core call graph |

Embed page: [`book/src/workflows/index.md`](../../book/src/workflows/index.md)

## Quality checklist

- [x] Sequence: parties, phases, reject / fail-closed alts
- [x] Labels use real identifiers (`domain_bind`, `owner_binding`, `BridgeMintNote`, `AlreadyMinted`, …)
- [x] No marketing language (“film”, “magic”)
- [x] Prefer SVG in book; `.mmd` remains source
- [x] `just diagrams-svg` green

## Residual / not this track

| Item | Owner |
|------|--------|
| Sanitize full `trailmark-cashapp-call-graph.mmd` (depth-3 raw) for `mmdc` | TRAILMARK-GRAPHS if needed |
| Additional targets (bridge.rs, PrivateCorridor TS) | TRAILMARK-GRAPHS |
| `just build` full mdBook | META |

## Regenerate

```bash
cd docs/plans/spectrum/book
just diagrams-svg
```
