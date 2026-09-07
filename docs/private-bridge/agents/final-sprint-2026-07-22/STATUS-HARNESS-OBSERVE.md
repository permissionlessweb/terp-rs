# STATUS-HARNESS-OBSERVE

**Track:** HARNESS-ICT-FUNDED  
**Date:** 2026-07-22  
**Bar:** FEEDBACK-RAISED-BAR.md + ORCHESTRATION.md (mainnet-funded-ready via local multi-net)

---

## Funded profile command

```bash
cd crates/headstash
just demo-corridor-ict
# equivalent:
# bash docs/plans/spectrum/e2e/corridor-ict-funded.sh
```

**What it does (fail closed — no silent mint skip):**

1. Prepares `artifacts/cw_headstash.wasm` (`prepare-corridor-ict-wasm.sh`)
2. Starts hash-market host notify plane
3. Starts **BTC regtest** (`docs/plans/spectrum/e2e/docker-compose.corridor-funded.yml`)
4. Opens watch with **`min_amount_sats`** (S6); proves below-min observation is rejected
5. Funds deposit via bitcoind; runs **`corridor-btc-reporter`** with `backend=bitcoind`
6. Waits for `deposit_observed` (S2 — not synthetic for funded profile)
7. Runs **`corridor_ict_funded`** binary: ict-rs fresh Terp → cw-orch Daemon → **`BridgeMintNote`** → `IsBridgeMinted` → double-mint reject → pure W0–W7 swap film
8. Host mint-after-observe automation PUT for UI poll

**Dev residual (mint only, not S1 exit):**

```bash
just demo-corridor-ict-mint-only   # SKIP_REGTEST=1 CORRIDOR_ALLOW_MINT_ONLY=1
```

### mock_verify: **true** on funded deploy (default)

| Env | Default | Meaning |
|-----|---------|---------|
| `CORRIDOR_MOCK_VERIFY` | `true` | `BridgeCfg.mock_verify` on funded local deploy — **labeled** (D7), not Tier-0 |
| `CORRIDOR_ICT_IMAGE` / `_TAG` | `terpnetwork/terp-core` / `local-zk` | ict-rs Docker image |
| `CORRIDOR_ICT_MNEMONIC` | abandon…about | faucet/deployer for Daemon |
| `KEEP_CHAIN` / `KEEP_REGTEST` | off | leave containers up |

Set `CORRIDOR_MOCK_VERIFY=false` only with a real proof path (will fail closed without it).

---

## Lab floor commands (S0)

```bash
# One-command lab film
cd crates/headstash && just demo-corridor-lab

# Oline play
cd crates/o-line/plays/private-bridge-corridor
./preflight.sh --lab && ./e2e-test.sh

# Units
cd crates/terp-rs/tools/hash-market
cargo test -p hash-market --lib corridor_deposits --features server
cargo test -p hash-market --lib btc_index --features server
```

---

## Libraries used (path list)

| Piece | Path |
|-------|------|
| ict-rs ChainSpec / Docker | `crates/ict-rs/ict-rs` (`spec.rs`, `runtime`, `chain/cosmos`) |
| ict-rs ↔ cw-orch | `crates/ict-rs/ict-rs-cw-orch` (`daemon_builder_from_chain`) |
| L3 binary | `crates/headstash/test-press/src/bin/corridor_ict_funded.rs` (feature `ict-daemon`) |
| PrivateBridge suite | `crates/headstash/test-press/src/suites/private_bridge.rs` (L3 comment promoted) |
| Bridge mint L1 world | `test-press/src/harness/bridge_l1.rs`, `bridge_mint_fixture.rs` |
| Contract | `cw-headstash` `ExecuteMsg::BridgeMintNote` / `bridge.rs` |
| Amount/address gates (S6) | `hash-market` `corridor_deposits.rs` (`min_amount_sats` on watch + report) |
| Reporter bitcoind backend (S2) | `hash-market` `btc_index/bitcoind_rpc.rs` + `reporter.rs` |
| Oline regtest config | `plays/private-bridge-corridor/config/reporter.regtest.toml` |
| Funded compose | `docs/plans/spectrum/e2e/docker-compose.corridor-funded.yml` |
| Orchestration script | `docs/plans/spectrum/e2e/corridor-ict-funded.sh` |
| just targets | `crates/headstash/justfile` `demo-corridor-ict*` |
| CashApp pure film | `harness/cashapp_zec_corridor.rs` W0–W7 |
| Status columns | `docs/plans/spectrum/e2e/CORRIDOR-LAB-STATUS.md` |

---

## BridgeMintNote evidence (tx / query)

| Layer | Evidence |
|-------|----------|
| L1 Mock (CI floor) | `just demo-e2e-l1` / `PrivateBridgeSuite::e2e_bridge_mint_happy` → `IsBridgeMinted` |
| L3 Daemon (funded) | `corridor_ict_funded` prints `IsBridgeMinted=true`, double-mint reject, contract addr, chain_id, grpc |

**Blocker if Daemon upload fails:** guest `cw_headstash.wasm` with BridgeMintNote surface.

Fail-closed locations:

- `prepare-corridor-ict-wasm.sh` — no/legacy artifact  
- `corridor_ict_funded.rs` — upload/instantiate or `IsBridgeMinted=false`  
- `crates/headstash/circuit/Cargo.toml:7` — default `multicore` breaks wasm32 guest builds  
- `secp256k1-sys` C toolchain for wasm32 guest  

**Not soft-skipped:** mint path returns non-zero on any execute/query failure.

---

## Observe topology (regtest + backend)

```text
bitcoind regtest (compose corridor-bitcoind-regtest :18443)
  → corridor-btc-reporter backend=bitcoind (JSON-RPC listunspent)
  → POST /corridor/observations
  → hash-market deposit_observed
```

- Compose: `docs/plans/spectrum/e2e/docker-compose.corridor-funded.yml`
- Reporter config: oline `config/reporter.regtest.toml` / script-generated TOML
- Signet secondary: not required when regtest is available under full control

---

## Amount/address gates (S6)

| Gate | Where | Tests |
|------|--------|-------|
| Address match | `report_observation` | `reject_addr_mismatch` |
| Dust `amount_sats==0` | `report_observation` | `reject_dust_amount_zero` |
| Watch `min_amount_sats` floor | `OpenWatchRequest` → watch → report | `reject_below_watch_min_amount`, `accept_at_or_above_watch_min_amount` |
| Reporter floor | `max(cfg.min_amount_sats, watch.min_amount_sats)` | `reporter.rs` poll_once |

Funded script also POSTs a below-min observation and **requires HTTP reject** before proceeding.

---

## Residuals (mainnet-only / packaging only)

| Residual | Severity | Notes |
|----------|----------|-------|
| Mainnet keys, liquidity, legal | n/a | Explicitly out of sprint |
| **Optimized `cw_headstash.wasm` with BridgeMintNote** | **P0 for green Daemon mint** | Guest wasm32 graph blocked (multicore + secp256k1-sys); drop artifact into `artifacts/` |
| Halo2 non-mock_verify proofs | P1 | Funded deploy documents `mock_verify=true` |
| P1 reorg revoke | P1 | Not implemented; observations not auto-revoked |
| Electrum TLS / Fulcrum compose image | P1 | `reporter.fulcrum.toml` ready |
| Zakura live in funded stack | coord ZAKURA | Paste-first + offline golden OK |
| Cash App private API | out | Funding remains any wallet QR |

---

## S0 / unit evidence (this session)

| Check | Result |
|-------|--------|
| `cargo test -p hash-market --lib corridor_deposits --features server` | **7 pass** (incl. S6 gates) |
| `cargo test -p hash-market --lib btc_index --features server` | **4 pass** (incl. bitcoind_rpc construct) |
| `cargo check` / release build `corridor_ict_funded` | **OK** |
| oline `./preflight.sh --lab` | **26 pass, 0 fail** |
| oline `./e2e-test.sh` | **7 pass, 0 fail** |
| cashapp pure fixtures | **11 pass** |
| L1 Mock `private_bridge` l1_* | **10 pass** (BridgeMintNote Mock green) |

## Live funded run evidence (this session)

| Stage | Result |
|-------|--------|
| BTC regtest compose | **up** (`bitcoin/bitcoin:27.0`, deposit funded) |
| S6 below-min observation | **rejected** (HTTP non-2xx after server rebuild) |
| Reporter `backend=bitcoind` | **`deposit_observed` ✓** |
| ict-rs Terp `local-zk` | **chain started**, grpc mapped |
| cw-orch Daemon | **sender funded**, upload attempted |
| Instantiate / BridgeMintNote | **FAIL closed** — pre-BridgeMintNote wasm: `Missing export instantiate` / invalid guest artifact |
| Silent mint skip | **none** — script exits non-zero |

**Unblock mint:** place optimized `cw_headstash.wasm` that exports CosmWasm instantiate + includes `BridgeMintNote` at:

`crates/headstash/contracts/cw-headstash/artifacts/cw_headstash.wasm`

(blockers for guest rebuild documented above: `circuit/Cargo.toml:7` multicore + secp256k1-sys).

---

## What remains mainnet-only vs code-complete

| Mainnet-only | Code-complete (local) |
|--------------|----------------------|
| Real BTC/ZEC settlement | Workflow shape: intent → fund → observe → reverify fields → mint → swap film |
| Operator keys / liquidity | S6 amount/address gates on notify |
| Cash App rail freezes | Regtest observe via reporter topology |
| Legal / compliance | ict-rs + Daemon mint **code path** (needs wasm artifact for green execute) |

---

## Handoff

- **STATUS columns:** `docs/plans/spectrum/e2e/CORRIDOR-LAB-STATUS.md` (lab vs ict_local_funded)
- **UI track:** consume automation API + fail-closed reverify (separate)
- **ZAKURA track:** live dest in funded compose
- **DOCS:** USER-GUIDE second pass after this STATUS
