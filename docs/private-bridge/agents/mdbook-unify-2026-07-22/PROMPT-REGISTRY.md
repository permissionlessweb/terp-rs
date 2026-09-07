# PROMPT — REGISTRY (mdBook)

**Track:** REGISTRY · **Book:** `docs/plans/spectrum/book/`  
**Read:** `DESIGN.md`, `LIBRARIES.toml`

## Mission

Make `LIBRARIES.toml` the complete library map for Private Bridge: every category filled, every entry has path + role. Ensure `python3 scripts/sync_includes.py && python3 scripts/gen_from_registry.py` stays green.

## Tasks

1. Audit monorepo for missing READMEs (fixtures without README — leave stubs or add one-line README in fixture if appropriate).  
2. Extend LIBRARIES.toml if hash-market docs/*.md should appear (corridor-btc-reporter.md, corridor-deposit-notify.md).  
3. Keep SUMMARY.md in sync with generated page ids.  
4. Write `STATUS-REGISTRY.md` under this agents folder.

## Out of scope

Deep product code; freeze amendments.
