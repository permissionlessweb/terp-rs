# IBC multi-hop authenticity harness

Live Docker proof that **predicted** ICS-20 denom traces match **observed** bank balances and denom traces after hop-by-hop transfers.

Implementation: [`tests/tests/ibc_multihop_harness.rs`](../../../tests/ibc_multihop_harness.rs)

## Topology

Line of four Terp chains (not a mesh):

```
A (terp-a) ── B (terp-b) ── C (terp-c) ── D (terp-d)
```

| Role | chain_id | name |
|------|----------|------|
| A | `terp-a` | `terp-a` |
| B | `terp-b` | `terp-b` |
| C | `terp-c` | `terp-c` |
| D | `terp-d` | `terp-d` |

IBC links (Hermes): only `A–B`, `B–C`, `C–D` (`transfer` port, unordered ICS-20).

Channel IDs are **not hard-coded**. After `Interchain::build`, each side is discovered via
`query ibc channel channels` and matched by counterparty channel id so side swaps fail closed.

## Tokenfactory matrix

On each chain a funded `user` key creates **4** denoms and mints balances:

| Chain | Subdenoms | Factory denoms |
|-------|-----------|----------------|
| A | `ta0`…`ta3` | `factory/{user_a}/taN` |
| B | `tb0`…`tb3` | `factory/{user_b}/tbN` |
| C | `tc0`…`tc3` | `factory/{user_c}/tcN` |
| D | `td0`…`td3` | `factory/{user_d}/tdN` |

**16 denoms total.** Scenarios use distinct origins so balances do not collide.

## Scenarios (minimum)

| # | Origin | Dest | Path (hop-by-hop ICS-20, not PFM) |
|---|--------|------|-----------------------------------|
| 1 | A `ta0` | B | A→B (1 hop) |
| 2 | A `ta1` | C | A→B→C (2 hops) |
| 3 | A `ta2` | D | A→B→C→D (3 hops) |
| 4 | D `td0` | A | D→C→B→A |
| 5 | B `tb0` | D | B→C→D |
| 6 | C `tc0` | A | C→B→A |

After each hop and at final destination:

1. **Predict** `trace_path` from dest looking back (`transfer/{recv_ch}/…/{base}`).
2. **Hash** with `scripts::ibc_core::compute_ibc_denom_hash` → `ibc/UPPER_SHA256`.
3. **Observe** dest bank balance of that denom (`> 0` / exact transfer amount).
4. **Observe** denom-trace path via `query ibc-transfer denom-trace`.
5. **Diff** — any mismatch panics with:

```text
AUTH_MISMATCH scenario=N hop=K expected_path=... expected_denom=ibc/... actual_trace=... actual_balance_denom=...
```

## What “pass” means

For every scenario hop:

```
predict(trace_path) == observed denom_trace.path == balance denom (ibc/HASH with amount > 0)
```

`ICT_MOCK=1` may exercise scaffolding only; it does **not** count as authenticity success.

## Prerequisites

1. Docker daemon running.
2. Terp image available (defaults match `TestEnv` / e2e examples):

```sh
# defaults
# ICT_IMAGE_REPO=terpnetwork/terp-core
# ICT_IMAGE_VERSION=local-zk
# or
# TERP_IMAGE_REPO / TERP_IMAGE_VERSION

docker pull ghcr.io/terpnetwork/terp-core:v5.2.0-zk-localterp
# or local build: make build-docker-local → terpnetwork/terp-core:local-zk
```

3. Hermes image pulled by ict-rs (`ghcr.io/informalsystems/hermes:1.8.2`).

## Run

```sh
# from terp-rs workspace root (package scripts lives at tests/)
cargo test -p scripts --test ibc_multihop_harness -- --ignored --nocapture
```

Optional env:

| Variable | Purpose |
|----------|---------|
| `ICT_IMAGE_REPO` / `ICT_IMAGE_VERSION` | Terp Docker image |
| `TERP_IMAGE_REPO` / `TERP_IMAGE_VERSION` | Same (overrides if set) |
| `ICT_KEEP_CONTAINERS=1` | Leave containers for debug |
| `RUST_LOG=info` | Relayer/chain logs |

CI default must **not** run this test (it stays `#[ignore]`). Offline authenticity is Agent A’s golden/unit track.

## Status

Implemented as an `#[ignore]`d integration test. Run with Docker when proving authenticity; do not assume scenarios are green in CI without an explicit ignored run.
