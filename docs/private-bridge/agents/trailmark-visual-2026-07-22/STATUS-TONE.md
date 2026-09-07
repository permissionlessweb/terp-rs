# STATUS-TONE

| Field | Value |
|-------|--------|
| **Date** | 2026-07-22 |
| **Track** | TONE-PRECISION |
| **Prompt** | [`PROMPT-TONE.md`](./PROMPT-TONE.md) |
| **Result** | **GO** — curated prose pass; freezes untouched; mdBook includes preserved |

## Scope touched

| Path | Change |
|------|--------|
| `docs/plans/spectrum/book/src/introduction.md` | Term table (`domain_bind` / `bound_only` / `fail-closed`); profile names exact; lab as explicit mode |
| `docs/plans/spectrum/book/src/workflows/index.md` | Concrete observe/mint surfaces; `bound_only` on oracle row; no “demo-film” / vague backend |
| `docs/plans/spectrum/book/src/platform-wasmvm/host-floor.md` | Identifiers over “film”; host-image vs guest MSRV framing tightened |
| `docs/plans/spectrum/USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md` | Surgical: term table; cut film marketing; §7.2 command cell fixed to `just demo-corridor-ict` |
| `docs/plans/spectrum/book/scripts/gen_from_registry.py` | Docstring / shell callout only (no fluff body text) |

**Not amended:** design freezes D1–D7 source text; `{{#include}}` bodies and paths in workflow Mermaid blocks.

## Before / after (3)

1. **Lab is a named mode, not a “film”**  
   - **Before:** `` `lab_simulated` | CI film; banner required; synthetic observe OK ``  
   - **After:** `` `lab_simulated` | Explicit lab mode; CI/training; banner required; synthetic observe OK ``

2. **Define granular terms once; reuse identifiers**  
   - **Before:** “oracle-bound private swap (bounds only — the oracle never mints)” scattered; no shared glossary.  
   - **After:** Book intro + USER-GUIDE open with a term table for `domain_bind` / `bound_only` / `fail-closed`; later copy reuses those labels (e.g. spine uses `bound_only`, refuse table uses oracle `bound_only`).

3. **Concrete surfaces over vague “backends” / broken command cell**  
   - **Before:** “labels and backends change, not a demo-only fork”; USER-GUIDE §7.2 mangled multi-line `just demo-corridor-ict` *(planned)* placeholder.  
   - **After:** “Labels, observe surface, and mint proof policy change — not a separate lab-only product fork”; §7.2 row is `just demo-corridor-ict` with regtest + Daemon `BridgeMintNote` + default `mock_verify=true` note.

## Checks

- [x] Profiles spelled `lab_simulated` / `ict_local_funded` / `production`  
- [x] No freeze text edits  
- [x] Workflow page still has three `{{#include ../diagrams/mmd/...}}` blocks  
- [x] USER-GUIDE structure (§1–§10) preserved  
