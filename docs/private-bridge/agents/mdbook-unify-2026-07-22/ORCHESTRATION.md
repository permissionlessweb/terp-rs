# Epic — Private Bridge mdBook unify (post funded-sprint)

| Field | Value |
|-------|--------|
| **Date** | 2026-07-22 |
| **Board** | `private-bridge-corridor` |
| **Prior sprint** | **DONE** — `demo-corridor-ict` OK; STATUS-WASM-FUNDED-GREEN; Hermes sprint cards closed |
| **Book root** | `docs/plans/spectrum/book/` |
| **Design** | `docs/plans/spectrum/book/DESIGN.md` |

## Goal

Ship a **maintainable** mdBook that:

1. Sorts libraries by **category** (`LIBRARIES.toml`)  
2. Uses **include mirrors** (`sync_includes.py` → `src/_include/`) so READMEs are not duplicated  
3. Builds with `just build` / `mdbook build`  
4. Is the navigation hub from spectrum `README.md`  

Scaffold already lands: DESIGN, LIBRARIES.toml, gen/sync scripts, SUMMARY, host-floor chapter, first `mdbook build` green.

## Tracks

| Track | Prompt | Focus |
|-------|--------|-------|
| REGISTRY | PROMPT-REGISTRY.md | Complete LIBRARIES.toml; missing fixture READMEs stubs; SUMMARY sync |
| SHELLS | PROMPT-SHELLS.md | Quality of shell pages; fix broken includes; optional overview pages |
| PLATFORM | PROMPT-PLATFORM.md | wasmvm/host-floor polish; link prepare script; document binaryen ≥120 |
| META | PROMPT-META.md | `just build` green; spectrum README; handoff STATUS |

## Success

- `cd docs/plans/spectrum/book && just build` → 0  
- Categories complete for pure-seams, contracts, harness, observe, ui, zakura, platform  
- No large pasted README bodies in `src/` shells  
- STATUS-MDBOOK.md written  

## Do not

- Rewrite SPECs into the book  
- Amend D1–D7 freezes  
- Re-open funded sprint coding unless book build requires a tiny fix  
