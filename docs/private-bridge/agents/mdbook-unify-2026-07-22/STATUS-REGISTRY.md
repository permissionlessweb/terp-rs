# STATUS — REGISTRY (mdBook unify)

| Field | Value |
|-------|--------|
| **Track** | REGISTRY |
| **Date** | 2026-07-22 |
| **Agent** | MDBOOK REGISTRY (Grok) |
| **Result** | **GO** |

## Mission

Complete `LIBRARIES.toml` as the library map for Private Bridge; keep SUMMARY + include pipeline green.

## Done

### 1. Fixture README audit

| Fixture | Before | After |
|---------|--------|--------|
| `cashapp_zec_corridor` | README present | unchanged |
| `bridge_auth_seams` | no README (stub only) | **added** one-line README |
| `private_dex_seams` | no README (stub only) | **added** one-line README |
| `compose_seams` | no README (stub only) | **added** one-line README |
| `seam_note_out` | no README (stub only) | **added** one-line README |

Registry entries upgraded from `kind = "crate"` + `readme = false` + `stub` → `kind = "crate-readme"` pointing at live READMEs (include-at-build).

### 2. hash-market corridor docs

Added under **observe-notify**:

| id | path | role |
|----|------|------|
| `corridor-deposit-notify` | `crates/terp-rs/tools/hash-market/docs/corridor-deposit-notify.md` | Watch/observe/SSE notify plane |
| `corridor-btc-reporter` | `crates/terp-rs/tools/hash-market/docs/corridor-btc-reporter.md` | BTC deposit monitor → observations |

### 3. SUMMARY sync

`src/SUMMARY.md` Observe & notify section now lists:

- hash-market  
- Corridor deposit notify  
- corridor-btc-reporter  
- oline private-bridge play  

Every `LIBRARIES.toml` id has a generated page and a SUMMARY link (hand-written extras: `pure-seams/overview.md`, `platform-wasmvm/host-floor.md` — not in registry; intentional).

### 4. Pipeline verification

```bash
cd docs/plans/spectrum/book
python3 scripts/sync_includes.py   # synced 32 files
python3 scripts/gen_from_registry.py
just build                         # mdbook build exit 0
```

All monorepo paths in registry exist.

## Registry counts (32 libraries)

| Category | Count |
|----------|------:|
| product | 3 |
| design | 6 |
| pure-seams | 5 |
| contracts-circuit | 3 |
| harness-ict | 4 |
| observe-notify | 4 |
| ui | 1 |
| zakura | 2 |
| platform-wasmvm | 2 |
| sprint-status | 2 |

## Files touched

- `docs/plans/spectrum/book/LIBRARIES.toml` — fixtures + corridor docs  
- `docs/plans/spectrum/book/src/SUMMARY.md` — observe-notify links  
- `docs/plans/spectrum/fixtures/{bridge_auth_seams,private_dex_seams,compose_seams,seam_note_out}/README.md` — new  
- Generated shells under `book/src/**` via `gen_from_registry.py` (incl. new observe pages)  
- `_include/` refreshed by `sync_includes.py`

## Out of scope (left for other tracks)

- Shell page prose quality (SHELLS)  
- host-floor / binaryen polish (PLATFORM)  
- spectrum README hub link + STATUS-MDBOOK (META)

## Verdict

**GO** — registry complete for DESIGN categories; include + gen scripts green; `mdbook build` green.
