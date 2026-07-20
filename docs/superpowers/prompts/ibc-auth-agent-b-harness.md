# Agent B — 4-chain multi-hop tokenfactory harness

Read first (in order):

1. `docs/superpowers/prompts/ibc-auth-shared.md`
2. `docs/superpowers/specs/2026-07-20-ibc-data-authenticity-design.md`
3. `docs/superpowers/plans/2026-07-20-ibc-data-authenticity.md` — **Track B only**

## Mission

Implement **Track B**: a real, `#[ignore]`d Docker integration test that spawns a **line of 4 Terp chains**, creates tokenfactory denoms, runs multi-hop ICS-20 transfers, and asserts **predicted IBC denom/path == on-chain observation**.

Also document harness layout and fix the **false claim** in `tests/README.md` that a multichain test already exists — replace with accurate paths/commands for what you create.

## Out of scope

- Extracting `tests/src/ibc/**` (Agent A)
- Editing `tests/bin/ibc_info.rs`
- Editing `tests/Cargo.toml`
- Editing `tests/src/ibc_core.rs` or lib modules
- Making the test non-ignored by default (must stay `#[ignore]`)

## Deliverables

### 1. `tests/data/ibc/harness/README.md`

Document:

- Topology: A—B—C—D
- Chain id naming convention you use
- TF matrix: 4 denoms × 4 chains (`ta0..ta3`, …)
- Scenarios 1–6 from the plan
- Docker prerequisites and exact cargo command
- What “pass” means (predict == denom_trace == balance denom)

### 2. `tests/tests/ibc_multihop_harness.rs`

Requirements:

```rust
#[tokio::test]
#[ignore = "requires Docker + Terp image; live multi-hop authenticity proof"]
async fn test_line_four_chain_tokenfactory_multihop() { ... }
```

Implementation guidance (adapt to actual ict-rs APIs in this monorepo):

1. Study:
   - `../../ict-rs/ict-rs/tests/ibc_transfer_test.rs` (path relative from terp-rs; actual: workspace `crates/ict-rs/...`)
   - `../../ict-rs/ict-rs/tests/terp_tokenfactory.rs`
   - `ict_rs::testing::TestEnv`, `Interchain`, `InterchainLink`, `TokenfactoryMsgExt`
2. Build four Terp chain configs with distinct `chain_id`s (e.g. `terp-a`, `terp-b`, `terp-c`, `terp-d`).
3. Add links only A–B, B–C, C–D.
4. `build` interchain; record transfer channel IDs per link **with correct side** (which chain owns which channel id).
5. On each chain: create 4 tokenfactory denoms, mint balances.
6. Prediction helper (local is fine if lib not ready):

```rust
fn predict_ibc_denom(trace_path: &str) -> String {
    scripts::ibc_core::compute_ibc_denom_hash(trace_path)
    // or scripts::ibc::hash::compute_ibc_denom_hash if A landed first
}
```

Build multi-hop path from dest looking back, e.g. single hop dest receives on `ch_dest`:
`transfer/{ch_dest}/{base_denom}`; two hop nest accordingly (match IBC ICS-20 hash rules used in `ibc_core`).

7. Execute scenarios 1–6 hop-by-hop (not PFM-required).
8. After relay settle, query dest bank for predicted denom balance > 0 (or exact amount) and denom trace path equality.
9. Fail with a clear message:

```text
AUTH_MISMATCH scenario=2 hop=2 expected_path=... expected_denom=ibc/... actual_trace=... actual_balance_denom=...
```

### 3. Compilability

- File must compile under `cargo test -p scripts --test ibc_multihop_harness --no-run` if the test target is discovered.
- If package only auto-discovers tests when registered, document for orchestrator:

```toml
[[test]]
name = "ibc_multihop_harness"
path = "tests/ibc_multihop_harness.rs"
required-features = ["docker"]
```

(Adjust path to match package layout: integration tests live in `tests/tests/` for this package — verify how existing `crosslink_light_client` is picked up.)

### 4. README honesty

In `tests/README.md`, fix the Multi-chain IBC section:

- Point to `tests/tests/ibc_multihop_harness.rs`
- Command: `cargo test -p scripts --test ibc_multihop_harness -- --ignored --nocapture`
- Do **not** claim scenarios already green if you could not run Docker; say “implemented, run with --ignored”.

## Running live (if Docker available)

```sh
cargo test -p scripts --test ibc_multihop_harness -- --ignored --nocapture
```

If Docker/image missing: still ship compilable test + document failure mode in report. Prefer a compile-clean harness over a half-written panic at module load.

## Commits

`feat(scripts): add 4-chain tokenfactory multihop authenticity harness`

## Return

Use the final report format from `ibc-auth-shared.md`. Include any ict-rs API gaps that blocked full transfer verification.
