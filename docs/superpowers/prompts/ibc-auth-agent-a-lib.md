# Agent A — IBC authenticity library + golden tests

Read first (in order):

1. `docs/superpowers/prompts/ibc-auth-shared.md`
2. `docs/superpowers/specs/2026-07-20-ibc-data-authenticity-design.md`
3. `docs/superpowers/plans/2026-07-20-ibc-data-authenticity.md` — **Track A only**

## Mission

Implement **Track A**: extract and harden the pure IBC authenticity library under `tests/src/ibc/`, keep `terp_scripts::ibc_core` working via re-exports, add unit + golden tests that prove hard invariants **without Docker**.

## Out of scope

- Docker multihop harness
- Thinning / rewriting `tests/bin/ibc_info.rs`
- Editing `tests/Cargo.toml` (document needed `[[test]]` lines in report if auto-discovery fails)
- Full mainnet generate bugfix in the binary (orchestrator), except pure-function fixes in the lib that generate will later call

## Deliverables

### 1. Module tree

```
tests/src/ibc/
  mod.rs
  hash.rs       # compute_ibc_denom_hash
  graph.rs      # IBCChannelGraph, ChannelHop, edges, find_routes, compute_ibc_denom_for_route
  routes.rs     # IBCAssetRoute, IBCAssetRoutingTable::premine, lookup helpers
  normalize.rs  # ordering_to_str, finalize_channels_for_ibc_entry
  schema.rs     # validate_asset_entry, validate_ibc_data_entry
  predict.rs    # PredictedWorld + check_invariants
  observe.rs    # ObserveBackend trait + FixtureBackend (minimal)
  diff.rs       # DiffSeverity, DiffItem, DiffReport
  fixtures.rs   # load helpers under tests/data/ibc/golden
```

Move existing logic from `tests/src/ibc_core.rs` into hash/graph/routes. Then either:

- turn `ibc_core.rs` into `pub use crate::ibc::{...};`, or  
- keep types re-exported so `use terp_scripts::ibc_core::compute_ibc_denom_hash` still works.

Update `tests/src/lib.rs` with `pub mod ibc;` (and keep `pub mod ibc_core`).

### 2. Invariant fixes (must have tests)

1. **hop_count honesty** — when emitting routes/lookup, hop_count must match path structure. Add a regression test that would fail on a world where hop_count=1 but path has two `transfer/` prefixes.
2. **prefer direct** — if graph has direct ACTIVE preferred edge origin→dest, preferred route selection for that pair must be the single hop (not a longer path). Unit test with a small synthetic triangle.
3. **channel side remap** — port finalize + map roundtrip test from binary (akash < terp) into unit tests using `normalize` + `build_channel_to_chain_map` (map can live in normalize or routes; keep one clear place).

### 3. Golden fixtures

Under `tests/data/ibc/golden/`:

- `known_hashes.json` — at least:
  - path `transfer/channel-1/uakt` → `ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4`
  - one multi-hop synthetic path of your choice with precomputed hash via the same function
- Minimal `ibc-data` JSON for a 3-chain line or triangle (schema-valid)
- Optional `expected_lookup.json` if you assert full table equality

### 4. Integration tests

- `tests/tests/ibc_unit.rs` — pure invariants, schema rejects int ordering, hash format
- `tests/tests/ibc_golden.rs` — load golden ibc-data → predict → invariant DiffReport has no errors; known_hashes match

If the package does not auto-pick up `tests/tests/*.rs`, note Cargo.toml snippets for orchestrator; do not edit Cargo.toml yourself unless compile is otherwise impossible — prefer `[[test]]` only as last resort and mention it loudly in the report.

### 5. PredictedWorld + compare stub

```rust
// conceptual — names may match design
pub struct PredictedWorld { /* graph, routes, lookup, ibc_data */ }
pub fn check_invariants(world: &PredictedWorld) -> DiffReport;
pub fn compare_predict_observe(...) -> DiffReport; // can be thin if observe is fixture-only
```

## Verification (run these)

```sh
cargo test -p terp-scripts --lib
cargo test -p terp-scripts --test ibc_unit
cargo test -p terp-scripts --test ibc_golden
cargo check -p terp-scripts --bin terp-ibc
```

All must pass or you must explain exact failures + residual risk.

## Commits

One or more commits scoped to Track A only. Message style:

`feat(scripts): extract ibc authenticity lib and golden tests`

## Return

Use the final report format from `ibc-auth-shared.md`.
