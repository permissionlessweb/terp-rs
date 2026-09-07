# IBC Data Script Authenticity & Multi-Hop Harness

Date: 2026-07-20  
Status: approved  
Package: `terp-scripts` (`tests/` workspace)  
Binary: `terp-ibc` (`tests/bin/ibc_info.rs`)

## Problem

The IBC data generation pipeline (`cargo run -p terp-scripts --bin terp-ibc`) produces chain-registry-shaped artifacts (`public/ibc-data/*.json`, `assetlist.json`, routing/lookup tables) but does not **prove** those artifacts match on-chain reality. Confidence today rests on schema shape and SHA-256 determinism, not channel-side correctness or DenomTrace/balance agreement.

Observed symptoms in current outputs:

- Direct Terp↔Akash / Terp↔Atomone channels exist, yet some lookup entries route AKT/ATONE through Osmosis multi-hop paths.
- `hop_count` can disagree with the number of `transfer/` segments in `trace_path`.
- Docs/README describe a Docker multi-hop test (`test_multichain_ibc_info_routing` in `tests/tests/ibc_info.rs`) that is not implemented.
- Generator logic, schema validators, and unit tests are concentrated in a ~3.4k-line binary with a large `#[cfg(test)]` block, which makes authenticity audits and reuse hard.
- Golden/expected fixtures drift from generated `public/` data (e.g. historical channel-115/6 vs current channel-139/9).

## Goals

1. **One authenticity model** for offline (fixtures/mainnet snapshots) and online (Docker harness).
2. **Predict → observe → diff** as the only way we claim data is correct.
3. **Thin CLI + library**: pure derivation and validators live in `terp_scripts::ibc`; binary is orchestration only.
4. **4-chain line multi-hop harness** with tokenfactory denoms proving predicted IBC denoms against real bank balances and denom traces.
5. **Fail closed** on hard invariant violations during `generate` and tests.

## Non-goals

- Full mesh topology (this pass uses a line A–B–C–D).
- Dual-sided live verification of client_id/connection_id on counterparties (optional later).
- New workspace crate (`terp-ibc-data`).
- Automatic chain-registry PRs / publishing.
- Requiring packet-forward middleware for path construction (hop-by-hop transfers are the default proof).

## Success criteria

- Unit + golden tests pass without Docker.
- `ibc validate` is clean on regenerated `public/` after fixes.
- At least one audited mainnet denom (e.g. AKT or ATONE on Terp) is pinned in golden data matching on-chain DenomTrace.
- `#[ignore]` multihop harness scenarios (single, double, triple hop, reverse) pass: **predict == denom_trace == balance denom**.
- README and `docs/tests/ibc_info.md` match the real layout (no references to non-existent test files as if implemented).

---

## Authenticity model

### Claim

For any `(origin_denom, route)`, the library’s predicted `(trace_path, ibc_denom)` equals what the chain materializes after IBC transfer (or what DenomTrace / bank reports for an existing denom).

### Pipeline

```
Inputs (facts)
  IbcDataSet, AssetSet, Topology
        │
        ▼
Predict (pure)
  hash, routes, reverse natives → PredictedWorld
        │
        ▼
Observe (backends)
  Fixture | LiveQuery | Harness
        │
        ▼
Diff / Report
  hard failures → non-zero exit
```

### Hard invariants (errors)

1. **Channel side correctness** — In alpha-ordered `chain_1`/`chain_2`, each `channel_id` sits on the correct chain side; Terp inbound channel used for denom hashing is never swapped with the counterparty channel.
2. **Trace path geometry** — Single-hop: `transfer/{dest_channel}/{base}`. Multi-hop: nest from destination looking back toward origin. `hop_count` equals the number of `transfer/` segments.
3. **Prefer direct when open** — If an ACTIVE preferred transfer channel exists between origin and dest, preferred lookup must not select a longer route (longer routes may exist as non-preferred alternates).
4. **Hash binding** — `ibc/` + uppercase SHA-256 hex of the UTF-8 path matches both prediction and observed DenomTrace.
5. **Schema** — chain-registry `ibc_data` and `asset_list` schemas enforced.

### Diff severity

| Severity | Examples | Default exit |
|----------|----------|--------------|
| Error | side swap, hash≠trace, preferred multi-hop when direct exists, schema fail, missing post-transfer balance | fail |
| Warn | inactive extra channels, missing optional metadata | pass unless `--strict` |
| Info | alternate non-preferred routes | log |

---

## Architecture

### Module layout (package `terp-scripts`)

```
tests/
├── src/
│   ├── lib.rs
│   └── ibc/
│       ├── mod.rs
│       ├── hash.rs
│       ├── graph.rs
│       ├── routes.rs
│       ├── schema.rs
│       ├── normalize.rs      # ordering strings, alpha sides, preferred tags
│       ├── predict.rs        # PredictedWorld
│       ├── observe.rs        # ObserveBackend + Fixture / Live / Harness adapters
│       ├── diff.rs
│       └── fixtures.rs
├── bin/
│   └── ibc_info.rs           # thin CLI only
├── tests/
│   ├── ibc_unit.rs
│   ├── ibc_golden.rs
│   └── ibc_multihop_harness.rs   # #[ignore], Docker
├── data/ibc/
│   ├── golden/
│   └── harness/              # optional captured dumps
└── public/                   # generate output
```

Migration: fold `src/ibc_core.rs` and pure helpers/tests currently in `bin/ibc_info.rs` into `src/ibc/*`. Remove the large in-binary `#[cfg(test)]` module once integration tests own coverage.

### CLI (`cargo run -p terp-scripts --bin terp-ibc -- <cmd>`)

| Subcommand | Behavior |
|------------|----------|
| `generate` | Query chains or `--from-state`; `predict`; write `public/` + state; auto-`validate` |
| `validate` | Schema + hard invariants on existing JSON/state |
| `compare` | `predict` vs `observe` (fixture path or live) → DiffReport |
| `harness` | Optional thin wrapper around the same path as the Docker integration test |

Default with no subcommand remains `generate` for backward compatibility.

### Provenance

`generate` writes `public/ibc_generation_meta.json` (timestamp, RPC sources, inclusion/exclusion reasons, generator version).

### Dead-client handling

Replace hard-coded `07-tendermint-0..31` with capability-based filtering: skip undecodable client state, skip non-OPEN transfer channels, support `--exclude-chain` / `--exclude-client`.

---

## Multi-hop harness (4-chain line)

### Topology

```
A (terp-a) ── B (terp-b) ── C (terp-c) ── D (terp-d)
```

- ict-rs `Interchain` + relayer (Hermes preferred if stable in-repo; otherwise existing successful multi-chain path).
- Links only A–B, B–C, C–D.
- Channel IDs recorded into in-memory `IbcDataSet` using the same schema as mainnet generator output.

### Tokenfactory matrix

On each chain create 4 TF denoms (16 total), mint to a funded user:

- A: `ta0`…`ta3` → `factory/{addr}/taN`
- B: `tb0`…`tb3`
- C: `tc0`…`tc3`
- D: `td0`…`td3`

### Scenarios (minimum)

| # | Origin | Dest | Path |
|---|--------|------|------|
| 1 | A TF | B | A→B (1 hop) |
| 2 | A TF | C | A→B→C (2 hops) |
| 3 | A TF | D | A→B→C→D (3 hops) |
| 4 | D TF | A | D→C→B→A |
| 5 | B TF | D | B→C→D |
| 6 | C TF | A | C→B→A |

Per scenario: **predict → execute hop-by-hop ICS-20 → observe (balance + denom_trace) → diff**.

After scenarios: full `PredictedWorld` premine with `max_hops=3` over harness topology + all TF natives; assert preferred-route invariants.

### Run commands

```sh
# CI default (no Docker)
cargo test -p terp-scripts --test ibc_unit --test ibc_golden

# Live authenticity proof
cargo test -p terp-scripts --test ibc_multihop_harness -- --ignored --nocapture
```

`ICT_MOCK=1` may smoke orchestration only; it does **not** satisfy authenticity success criteria.

---

## Golden / offline plane

Under `tests/data/ibc/golden/`:

- `ibc-data/*.json` — audited channel pairs
- `assetlist.snippet.json` — minimal natives + IBC assets with correct traces
- `expected_lookup.json` / `expected_routes.json` — predict pins
- `known_hashes.json` — path → `ibc/HASH` vectors

Refresh is manual: generate → audit DiffReport / live sample → promote; never silent overwrite of goldens.

---

## Generator bug fixes (required)

1. Do not invent multi-hop when a direct preferred ACTIVE transfer channel exists to the asset origin.
2. Channel side remap only through shared normalize + map builder, unit-tested against golden pairs.
3. `hop_count` always derived from path structure.
4. Capability-based client/channel filtering (see above).
5. Auto-validate at end of generate; refuse to treat invalid output as success.
6. Align docs with implementation.

---

## Rollout phases

1. Extract `src/ibc/*`; keep binary working; move pure unit tests.
2. Invariants + golden tests; fix derivation bugs; clean `public/` generation.
3. CLI subcommands: `generate | validate | compare`.
4. Docker multihop harness + scenarios 1–6.
5. Doc rewrite (`tests/README.md`, `docs/tests/ibc_info.md`).

---

## Dependencies

- Existing: `ict-rs` (docker, terp, testing, tokenfactory), `cw-orch` / `cw-orch-interchain` for live mainnet queries, `sha2`/`hex`/`serde_json`.
- Feature flags: keep Docker harness behind ignore + docker feature as already used by `scripts`/`ict-rs`.

## Open follow-ups (post this design)

- Dual-query counterparty client/connection consistency.
- Mesh topology and preferred-channel contention tests.
- Optional PFM single-tx multi-hop once hop-by-hop is green.
- Promote harness dumps into golden offline suite for CI without Docker.
