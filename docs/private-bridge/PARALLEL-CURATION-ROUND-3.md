# Parallel curation round 3 — unblocked implementation (no slow circuit rebuild)

**Date:** 2026-07-20  
**Rule:** Prefer pure unit tests, SPEC freezes, and lightweight code. Avoid `MockProver` K=18 full-circuit runs in agent loops.

## Domains

| ID | Goal | Output |
|----|------|--------|
| R3-A | SEAM-NOTE-OUT freeze + A↔D field map | `docs/plans/spectrum/SEAM-NOTE-OUT.md` |
| R3-B | Domain B pure tests (conservation, domain bind, burn≠spend) | `crates/headstash` or new `docs/plans/spectrum/tests/` + optional small rust crate/module under terp-rs if easy; prefer `docs/plans/spectrum/fixtures/` + rust in `crates/headstash/circuit` pure modules only if no MockProver |
| R3-C | Fix LC hinge SPEC: bitcoinBurnRoot H-1 + ZEC/TZE non-blocking section | patch `SPEC-lc-hinge-private-bridge.md` |
| R3-D | Private DEX pure formula + oracle-bound unit tests | new file under monorepo lightweight |
| R3-E | Compose SEAM freeze checklist + master test IDs matrix update | patch FLOW + write `SEAM-FREEZE-CHECKLIST.md` |
| R3-F | Additive Headstash multi-root design (spec only + test IDs H7–H8) | patch SPEC-airdrop + small section |

## Shared rules
- Oracle never mints
- ZEC/TZE not on critical path
- No inventing Tacit opcodes
- Write only assigned files

## Results (all agents exit 0)

| ID | Outcome |
|----|---------|
| R3-A | `SEAM-NOTE-OUT.md` — SeamNoteOutV0 382B, maps A/B/D, SEAM-N1..N6 |
| R3-B | `fixtures/bridge_auth_seams` — 9 tests green (H-1, double mint, domain, …) |
| R3-C | LC hinge SPEC: `bitcoinBurnRoot` + H-1 reject + ZEC/TZE non-blocking |
| R3-D | `fixtures/private_dex_seams` — 12 tests green (swap, oracle, nullifier) |
| R3-E | `SEAM-FREEZE-CHECKLIST.md` + additive §1.10 Headstash + FLOW status |

### Verify pure crates
```bash
cd docs/plans/spectrum/fixtures/bridge_auth_seams && cargo test
cd docs/plans/spectrum/fixtures/private_dex_seams && cargo test
```
