# STATUS — SHELLS (mdBook)

| Field | Value |
|-------|--------|
| **Track** | SHELLS |
| **Date** | 2026-07-22 |
| **Verdict** | **GO** |
| **Build** | `cd docs/plans/spectrum/book && just build` → **0** |

## Done

1. **Shell template (survives rebuild)** — `scripts/gen_from_registry.py` now emits:
   - Category / monorepo path / role table
   - Optional **Non-goals** and **How to test** from `LIBRARIES.toml`
   - Context blurb (shell only; no README paste)
   - **Next** link within category (registry order)
   - Prefer `_include` mirror when present (fixes `wasm-prepare` script wrap include)

2. **Registry polish** — every `[[libraries]]` entry has `non_goals`; pure-seams have `test` one-liners. Comment in TOML avoids bare include-directive prose.

3. **Category overviews** (hand-maintained; not overwritten by gen):
   - `src/pure-seams/overview.md` — linked crate table + non-goals
   - `src/contracts-circuit/overview.md` — **new**
   - `src/observe-notify/overview.md` — **new**
   - Wired in `src/SUMMARY.md` + intro orientation links

4. **Includes** — all library shells use `{{#include ../_include/…}}` only; no full README bodies in shells (shells ≤ ~15 lines). Overviews stay thin tables + non-goals.

5. **Navigation** — introduction points at category overviews; host-floor / wasm-prepare / sprint-wasm-green already linked by PLATFORM/META.

## Constraints honored

- Do **not** paste full SPEC/README bodies into shells — include only.
- Freezes D1–D7 not amended.
- Sprint STATUS remains thin pointers via include.

## Files touched (this track)

| Path | Change |
|------|--------|
| `docs/plans/spectrum/book/scripts/gen_from_registry.py` | Richer shell generator |
| `docs/plans/spectrum/book/LIBRARIES.toml` | `non_goals` / `test`; wasm-prepare no longer forced stub-only |
| `docs/plans/spectrum/book/src/contracts-circuit/overview.md` | New |
| `docs/plans/spectrum/book/src/observe-notify/overview.md` | New |
| `docs/plans/spectrum/book/src/pure-seams/overview.md` | Polish + links |
| `docs/plans/spectrum/book/src/SUMMARY.md` | Overview entries |
| `docs/plans/spectrum/book/src/introduction.md` | Category orientation links |
| Generated `src/**/*.md` shells | Via gen on build |

## Verify

```bash
cd docs/plans/spectrum/book && just build
```

## Handoff

- REGISTRY/PLATFORM/META may still own SUMMARY completeness and spectrum README; shells stay green under `just build` regeneration.
- Future shell fields: add `non_goals` / `test` in `LIBRARIES.toml` only — do not hand-edit generated shells.
