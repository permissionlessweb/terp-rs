# STATUS — PLATFORM / wasmvm (mdBook unify)

| Field | Value |
|-------|--------|
| **Track** | PLATFORM |
| **Date** | 2026-07-22 |
| **Epic** | `docs/plans/spectrum/agents/mdbook-unify-2026-07-22/` |
| **Prompt** | `PROMPT-PLATFORM.md` |
| **Result** | **GO** |

## Mission (done)

Host-floor chapter is the SSOT for wasmd/wasmvm **host capability vs guest toolchain MSRV** (bulk-memory, forbidden imports, binaryen ≥120, prepare script, `just demo-corridor-ict`). Linked to STATUS-WASM-FUNDED-GREEN and harness e2e.

## Deliverables

| Item | Path | Notes |
|------|------|--------|
| Expanded host floor | `docs/plans/spectrum/book/src/platform-wasmvm/host-floor.md` | Recipe table, binaryen ≥120, fail-closed checks, demo-corridor-ict, profiles, cross-links |
| Prepare shell | `docs/plans/spectrum/book/src/platform-wasmvm/wasm-prepare.md` | Registry id `wasm-prepare`; invoke + pipeline; points at host-floor |
| Introduction cross-links | `docs/plans/spectrum/book/src/introduction.md` | Fixed broken `wasm-green` link → `sprint-wasm-green`; prepare + e2e links |
| Harness e2e cross-links | `docs/plans/spectrum/book/src/harness-ict/e2e-corridor.md` | Platform section above live e2e include |
| Registry | `LIBRARIES.toml` `wasm-prepare` + `cosmwasm` | Already present under `platform-wasmvm` — no schema change required |
| Evidence link | `sprint-status/sprint-wasm-green.md` → STATUS-WASM-FUNDED-GREEN | Referenced from host-floor / prepare / intro / e2e |

## Documented operator framing

> Failures often = **host image wasmvm capability vs guest toolchain MSRV**, not broken BridgeMint logic.

| Symptom | Documented fix |
|---------|----------------|
| bulk-memory not enabled | Host binaryen ≥120 + `--llvm-memory-copy-fill-lowering` |
| `proof_instance_verify` | Guest cosmwasm-std without `zk` |
| BLS default-trait panic | `cosmwasm_2_1` |
| Multicore / C-sys | Guest graph without `host-crypto` |

Commands SSOT:

```bash
cd crates/headstash && just prepare-corridor-wasm-force
cd crates/headstash && just demo-corridor-ict
```

Script: `docs/plans/spectrum/e2e/prepare-corridor-ict-wasm.sh`

## Out of scope (per ORCHESTRATION)

- No SPEC rewrites / freeze amendments  
- No re-open of funded sprint coding  
- Did not paste full prepare script body into the book (header + recipe + registry shell)

## Handoff

- META: host-floor present; platform category complete for polish track  
- REGISTRY/SHELLS: no LIBRARIES.toml edits required for this track  
- Operators: prefer host-floor chapter over re-reading sprint chat for wasm rebuild  
