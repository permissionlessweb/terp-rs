# STATUS — META (mdBook unify)

| Field | Value |
|-------|--------|
| **Track** | META |
| **Date** | 2026-07-22 |
| **Verdict** | **GO** |
| **Book root** | `docs/plans/spectrum/book/` |

## Checklist

| Item | Result |
|------|--------|
| `cd docs/plans/spectrum/book && just build` exit 0 | **PASS** — sync 26 includes → gen shells → `mdbook build` → HTML at `book/book/` |
| LIBRARIES categories present | **PASS** — 10 categories, 30 libraries |
| No duplicate README bodies in shells | **PASS** — registry shells are thin metadata + `{{#include}}` (or short stubs where `readme = false`) |
| host-floor chapter present | **PASS** — `src/platform-wasmvm/host-floor.md` (curated, in SUMMARY) |
| Link from `docs/plans/spectrum/README.md` | **PASS** — points to [`book/`](../../book/) + `just build` + DESIGN.md |

## Categories (LIBRARIES.toml)

| Category | Count | IDs (abbrev) |
|----------|------:|--------------|
| product | 3 | user-guide, demo-corridor, operator-privatecorridor |
| design | 6 | freezes-d1-d7, flow, specs, seam-note-out |
| pure-seams | 5 | cashapp + 4 fixture stubs (no README yet) |
| contracts-circuit | 3 | headstash workspace / circuit / cw-headstash |
| harness-ict | 4 | ict-rs, ict-rs-core, e2e, corridor-lab-status |
| observe-notify | 2 | hash-market, oline-play |
| ui | 1 | privatecorridor-ui |
| zakura | 2 | zakura-local, zakura-e2e |
| platform-wasmvm | 2 | cosmwasm, wasm-prepare (+ curated host-floor outside registry) |
| sprint-status | 2 | sprint-meta, sprint-wasm-green |

## Notes

- Sibling STATUS files (REGISTRY / SHELLS / PLATFORM) were **not present** at META verify time; META ran the build-only path in parallel as allowed.
- Pure-seams crates without READMEs correctly use **stub** shells (no pasted Cargo/source bodies).
- `host-floor.md` is intentionally curated (not a live include); documents wasmd/wasmvm floor + binaryen/wasm-opt ≥120 discipline.
- Spectrum README already navigates operators to the book as the library map hub.

## Verdict

**GO** — maintainable mdBook builds green, categories map the monorepo, includes avoid README duplication, host-floor chapter ships, spectrum README links the book.
