# loyalty-verifier

Merkle root store + dual claim paths for hashmerchant loyalty rewards.

| Path | Msg | Anti-replay | Settlement |
|------|-----|-------------|------------|
| Public | `ClaimRewards` | `(claimer, root_index)` | none (attestation) |
| Private (zk-jwt) | `ClaimRewardsPrivate` | `(nullifier, root_index)` | `bank_send` / `mint_intent` |

Optional `action_bind` on private claims: SHA-256 domain `loyalty-claim/v1` over root index, amount, leaf hash, and destination.

## Build / test

```sh
# Dense unit profiles (sanity + adversarial)
cargo test

# REQUIRED for wasmd / Docker e2e (optimizer + bulk-memory lower)
just wasm

# Faster host path (still runs wasm-opt bulk-memory lower)
just wasm-dev
```

**wasmd 0.61 rejects bulk-memory** (`memory.copy` / `memory.fill`).

| Build | Command | Notes |
|-------|---------|--------|
| ✅ wasmd-safe | `just wasm` | Docker rustc 1.86 + host `wasm-opt` lower |
| ✅ wasmd-safe | `just wasm-dev` | Host rustc + same `wasm-opt` lower |
| ❌ not storeable | plain `cargo build --target wasm32…` | bulk-memory enabled → store fails |

Needs: Docker (`terpnetwork/optimizer-arm64:0.17.0` or cosmwasm) + host **binaryen ≥120** (`brew install binaryen`).

## Test profiles

| Profile | What |
|---------|------|
| `sanity_*` | Public claim+query, private nullifier, bank_send, independent maps |
| `insanity_*` | Double claim, bad merkle, fake leaf, bind hijack, nullifier replay across accounts, zero amount, OOB root, non-admin mint config |
| `bench_*` | Offline wall-clock micro-benches (`cargo test bench_ -- --nocapture`) |

## Gas benchmarking (on-chain)

Docker e2e records **gas used** per verification (public / private / adversarial):

```sh
cd crates/ict-rs
LOYALTY_GAS_REPORT=./loyalty_gas_report.md \
  cargo run --example loyalty_rewards --features "docker hashmerchant"
```

Prints a comparison table and writes markdown (`LOYALTY_GAS_REPORT` path, default `loyalty_gas_report.md`).

## Product docs

- Design: `../../../reviews/ZKJWT-LOYALTY-REWARDS-EXTENSION.md`
- Offline walkthrough: `ict-rs` example `loyalty_rewards_zkjwt`
- Docker dual-path + on-chain adversarial: `ict-rs` example `loyalty_rewards`
