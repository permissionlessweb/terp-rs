---
title: E2E harness plan — Anvil + Terp + LC hinge (cw-orch + ict-rs)
status: draft-curation
domain: E-harness
created: 2026-07-20
round: 1
ssot_flow: docs/plans/spectrum/FLOW-private-bridge-auth.md
ssot_matrix: docs/plans/spectrum/FLOW-private-bridge-auth-test-matrix.md
agent_report: docs/plans/spectrum/agents/ROUND1-HARNESS.md
---

# E2E harness plan (round 1 curation)

> **Goal:** Local-network, reproducible compose path for private bridge (+ optional private swap) that eventually runs Anvil (Tacit confidential), Terp CosmWasm (headstash + future bridge mint + optional DEX settle), and an optional light-client hinge — without inventing a third test framework.
>
> **Non-goal (round 1):** Full green multi-container joint e2e. This document freezes **layers, inventory, suite sketch, case IDs**, and **team questions**.
>
> **Hard constraints**
> - Prefer extending `HeadstashSuite` / `test-press` patterns over new frameworks.
> - **ict-rs** for containers; **cw-orch** for CosmWasm; **foundry/anvil** for Tacit EVM.
> - Mock ZK verify is OK for L1 until Headstash H1 prove is green.
> - No inventing opcodes outside [`SPEC-tacit-bridge-mapping.md`](./SPEC-tacit-bridge-mapping.md).

---

## 1. Honest current state

| Capability | Status |
|------------|--------|
| Joint anvil + terpd + LC + private swap full e2e | **Does not exist** |
| Headstash multi-test / lib tests + cw-orch suite skeleton | **Exists** (`HeadstashSuite`, `TestPressSuite`) |
| Headstash H1 composite MockProver | **Last-mile red** (see `DEMO-PATH.md`) |
| Private swap on Terp | **Pure seams only** (`private_dex_seams`) |
| Tacit anvil / signet e2e | **Exists** (EVM-only; not wired to Terp mint) |
| ict-rs Anvil + Terp dual path | **Exists as pattern** (`hashmerchant` example — oracle/VE, not bridge mint) |
| Pure auth / note / DEX / compose fixtures | **Green** (bridge **18** + seam **6** + dex **23** + compose **~7** as of 2026-07-20 r2 scan; re-scan `#[test]`) |

---

## 2. Architecture layers (L0–L4)

```text
L4  LC hinge (mock attestation first → real Crosslink / reflection public values)
     ▲
L3  ict-rs network: Anvil container + terpd (zk image) + optional relayer/sidecar
     ▲  glue: ict-rs-cw-orch → Daemon; optional Tacit RPC handle
L2  Anvil-only Tacit (wrap existing shell / forge / mjs)
     ▲
L1  Single-chain cw-orch Mock / multi-test (policy + optional mock ZK)
     ▲
L0  Pure seams (no halo2, no Docker) — already green fixtures
```

### L0 — Pure seams (fast CI, always on)

| Crate | Path | Cases (`#[test]`, 2026-07-20 r2 scan) | Proves |
|-------|------|--------------------------------------:|--------|
| `bridge_auth_seams` | `docs/plans/spectrum/fixtures/bridge_auth_seams` | **18** | H-1 burn≠spend, hinge `authorize_bridge_mint`, conservation, double mint, tip/conf/lag, domain, asset map, rcm/DEX consumability |
| `private_dex_seams` | `docs/plans/spectrum/fixtures/private_dex_seams` | **23** | swap_happy, min_out, oracle stale/slippage, nullifier, wrong asset, oracle cannot mint, SEAM→SwapAction compose |
| `seam_note_out` | `docs/plans/spectrum/fixtures/seam_note_out` | **6** | `SeamNoteOutV0` encode/decode/schema (SEAM-N1…N6) |
| `compose_seams` | `docs/plans/spectrum/fixtures/compose_seams` | **~8+** | C1–C4 + **`product_path_burn_to_swap_sketch`** (register→mint→SwapAction apply→reserves+ν) |

**Run:** `just demo-e2e-l0` from `crates/headstash` (four fixture crates + `zk-test-press` harness re-exports), or `cargo test` inside each fixture crate.

**Product pure e2e (V0c):**  
`cd docs/plans/spectrum/fixtures/compose_seams && cargo test product_path_burn_to_swap_sketch`  
(also re-exported: `cargo test -p zk-test-press --lib harness::compose_l0`).

**Prior inventory (round-1 doc):** bridge 9 / dex 12 / seam 6 — **stale**; meta-review D6 corrected toward 17/18/6; round-2 domain scan is **18 / 23 / 6** + **compose_seams**.

### L1 — Single-chain CosmWasm (Mock / multi-test / daemon without Docker multi-chain)

| Surface | Path | Proves | Gap |
|---------|------|--------|-----|
| `HeadstashSuite` | `crates/headstash/test-press/src/suites/headstash.rs` | Deploy/store manifold + headstash + circuit upload | H1 prove red |
| `cw-headstash` bridge | `contracts/cw-headstash/src/bridge.rs` | `BridgeMintNote` + mock verify + H-1 / double mint unit tests | Suite multi-test not yet re-exported |
| `NoRickSuite` | `…/suites/no_rick.rs` | Simpler circuit suite pattern | Not bridge path |
| `TestPressSuite` | `…/suite.rs` | Composed no_rick + headstash | Partial `Deploy` (`todo!`s) |
| DEMO policy | `crates/headstash/justfile` `demo-policy` | Poseidon distro + contract policy | No mint/swap |
| L0 e2e pure | `just demo-e2e-l0` | Fixture crates + harness + suite L0 | No Docker / no H1 prove |
| Contract lib tests | `cw-headstash` multi-test / distro | Root register, policy | Mock ZK until prove green |

**Suite extension (round 2):** `PrivateBridgeSuite` wraps `HeadstashSuite` + L0 pure methods + claim fixture loader + optional Tacit RPC (see §3). **L1 mock ZK posture:** PR CI uses policy + mock proof bytes; real H1 prove is `just demo-h1` / nightly only — never Tier-0.

### L2 — Anvil-only Tacit (EVM confidential lane)

| Surface | Path | Proves | Gap |
|---------|------|--------|-----|
| Shell round-trip | `crates/tacit/tests/evm-confidential-anvil-roundtrip.sh` | Deploy factory → prove mint → RPC mint → supply/status | No Terp |
| Foundry tests | `crates/tacit/contracts/test/*.t.sol` | Pool, factory, farm, poseidon parity, etc. | EVM-only |
| Signet / mjs e2e | `crates/tacit/tests/*signet*.mjs`, `bridge-*.mjs`, AMM e2e | Live/testnet confidential ops | Not wired to Terp mint packet |

**Harness policy:** Keep L2 as thin wrappers (`just tacit-anvil-rt`) that the L3 orchestrator can call or reimplement via ict-rs `AnvilChain` + `cast`/`forge`.

### L3 — ict-rs multi-container (Anvil + Terp)

| Surface | Path | Proves | Gap for private bridge |
|---------|------|--------|------------------------|
| `AnvilChain` / `TestEthChain` | `crates/ict-rs/ict-rs/src/chain/ethereum.rs`, `testing/mod.rs` | Anvil in Docker, prefunded accounts, eth send | No ConfidentialPool deploy recipe in ict-rs yet |
| `anvil_tests` | `crates/ict-rs/ict-rs/tests/anvil_tests.rs` | Start / funds (feature `ethereum`) | Unit-level |
| `hashmerchant` example | `crates/ict-rs/examples/hashmerchant.rs` | **Anvil + Terp + sidecar** stateRoot → VE → on-chain root | **Oracle/VE path** — not conservation mint; still the best dual-container template |
| `headstash` example | `crates/ict-rs/examples/headstash.rs` | Terp zk image + wasm deploy + claim lifecycle sketch | Needs prebuilt wasm + keys; prove path unfinished |
| IBC dual-chain | `examples/ibc_transfer*.rs`, polytone, etc. | Hermes + Cosmos↔Cosmos | Not Anvil bridge |
| `ict-rs-cw-orch` | `crates/ict-rs/ict-rs-cw-orch` | `daemon_builder_from_chain` | No Anvil→Daemon; CosmWasm only |
| ChainSpec builtins | `spec.rs` | gaia, osmosis, **terp**, juno, akash, **anvil** | Bitsong/etc. out of scope |

**L3 target topology (future green):**

```text
┌─────────────────┐     burn event / reflection public values      ┌──────────────────────┐
│ Anvil (Docker)  │ ─────────────────────────────────────────────► │ Mock hinge or sidecar│
│ ConfidentialPool│                                                │ (L4 mock first)      │
└─────────────────┘                                                └──────────┬───────────┘
                                                                              │ mint packet
                                                                   ┌──────────▼───────────┐
                                                                   │ terpd (local-zk)     │
                                                                   │ cw-headstash / bridge│
                                                                   │ mint CosmWasm        │
                                                                   └──────────────────────┘
```

Reuse: `hashmerchant` dual-container bootstrap; replace VE sidecar with **mock LC attestation** (L4) producing Domain B mint packet fields. CosmWasm interaction via `daemon_builder_from_chain` + extended suite.

### L4 — LC hinge

| Surface | Path | Maturity | Harness use |
|---------|------|----------|-------------|
| Domain C SPEC | `SPEC-lc-hinge-private-bridge.md` | Doc landed | Hinge table SSOT |
| Crosslink LC unit | `crates/terp-rs/crates/crosslink/light-client` | unit | Real LC later |
| cw-ics08-wasm-crosslink | Eureka programs | contract | Optional L3 sidecar |
| Tacit reflection public values | Domain B § / ConfidentialPool | e2e on Tacit | Map to mint packet |
| **Mock attestation** (round 2 first) | *to add* under fixtures or ict example | **planned** | Signed tip + burn membership stub; **never** labeled Tier-0 |

**Rule:** Mock hinge is for wiring and reject codes (`E_LC_LAG`, `E_LC_FROZEN`, …). Production trust labels stay honest (matrix suite C9).

---

## 3. Suite sketch (cw-orch)

### 3.1 Prefer composition over fork

```text
PrivateBridgeSuite<Chain: ZkCwEnv>
  ├── headstash: HeadstashSuite<Chain>     // existing deploy / circuit
  ├── tacit_rpc: Option<String>            // L2/L3 host RPC
  ├── fixtures_dir: Option<PathBuf>        // claim / mint / swap JSON
  └── lc_mock: Option<LcMockHandle>        // L4 mock tip + membership
```

Skeleton / L0 methods:

- `crates/headstash/test-press/src/suites/private_bridge.rs` (round-2 L0 methods + claim fixture)
- `crates/headstash/test-press/src/harness/` (always-on pure re-exports)
- Suite gated `#[cfg(feature = "interface")]` like other suites

### 3.2 Types / methods (round-2 status)

| Method | Layer | Behavior | R2 status |
|--------|-------|----------|-----------|
| `PrivateBridgeSuite::new(chain)` | L1 | Construct wrappers only | **done** |
| `Deploy::store_on` / `deploy_on` | L1 | Delegate headstash deploy; **one** `cw-headstash` code_id (CLARITY) | **done** |
| `assert_policy_bridge_mint_happy` | L0 | E2E-01 via `authorize_bridge_mint` | **done** |
| `assert_h1_spent_only_reject` / `assert_ordinary_spend_not_burn` | L0 | H-1 / E2E-03 | **done** |
| `assert_double_mint_reject` | L0 | C2.2 / E2E-02 | **done** |
| `compose_bridge_mint_to_seam` | L0 | Sketch → 382-byte SEAM layout | **done** |
| `load_claim_fixture` / `build_claim_fixture` | L0→L1 | DEMO-PATH A2 JSON / synthetic policy | **done** (no prove) |
| `load_claim_fixture_and_process` | L1 | Policy validate; CW submit when mock verify wired | **partial** |
| `e2e_burn_to_mint_fixture` | L0(+L4 mock) | LC mock gate + L0 happy | **L0 path done**; CW mint TBD |
| `apply_swap_fixture` | L0→L1 | Until CW DEX: `compose_seams` product path / `demo-e2e-l0` | **L0 via product path**; CW stub |
| `assert_oracle_cannot_mint` | L0 | Points at `private_dex_seams` / E2E-13 | **stub** |
| `assert_product_path_burn_to_swap_sketch` | L0 | `harness::compose_l0` → `compose_seams` | **done** |
| `with_tacit_rpc(url)` | L2/L3 | Attach Anvil endpoint | **done** (handle only) |
| `with_lc_mock(cfg)` | L4 | Tip, conf K, freeze, optional max lag | **done** |

### 3.3 ict-rs example sketch (not implemented round 1)

```text
crates/ict-rs/examples/private_bridge_compose.rs  // future
  features: docker + ethereum
  1. AnvilChain::start
  2. (optional) forge script DeployConfidential
  3. CosmosChain terp local-zk
  4. daemon_builder_from_chain → PrivateBridgeSuite
  5. inject mock LC acceptance
  6. run E2E-01.. subset
```

Until ConfidentialPool deploy is scripted inside ict-rs, L3 may start dual containers and only assert **ports + mock mint packet**, while L2 still runs the real Tacit mint.

### 3.4 Fixture loader contract

Align claim fixtures with `DEMO-PATH.md` schema (root, path, partial_note, instance 168 bytes).  
Align mint packets with Domain B §9 checklist (ν, asset_id, destCommitment, claimId, value, tip/conf).  
Align note egress with frozen `SEAM-NOTE-OUT.md` (`SeamNoteOutV0`).

Reject codes stay shared with the compose matrix (`E_NULLIFIER`, `E_LC_LAG`, `E_ORACLE_MINT`, …).

---

## 4. Test case catalog (E2E-xx)

Stable harness IDs map to matrix `C*` cases and pure fixture seeds. **Min layer** = lowest layer that can assert the property; **target layer** = full compose intent.

| ID | Scenario | Matrix | Min | Target | Seed / notes |
|----|----------|--------|-----|--------|--------------|
| **E2E-01** | Valid burn → LC tip mature → bridge_mint note | C0.1 | L0 | L3+L4 | `t1_happy_path_mint` |
| **E2E-02** | Double mint same ν reject | C2.2 | L0 | L1/L3 | `t2_double_mint_reject` |
| **E2E-03** | Ordinary spend ∉ burn set → no mint (H-1) | C0/C2 related | L0 | L1/L3 | `t3_ordinary_spend_not_burn` |
| **E2E-04** | Immature confirmations / tip lag | C3.1 | L0 | L4 | `t4_immature_confirmation` |
| **E2E-05** | Value mismatch mint ≠ burn | C0.4 / conservation | L0 | L1 | `t5_value_mismatch` |
| **E2E-06** | Unmapped asset_id | C4.3 | L0 | L1 | `t6_unmapped_asset` |
| **E2E-07** | Domain mismatch | C4.1 | L0 | L1 | `t7_domain_mismatch` |
| **E2E-08** | Headstash claim policy (distro root + nullifier) | C1.1, C2.1 | L1 | L3 | `demo-policy` / Part T H* |
| **E2E-09** | Claim → note schema consumable by spend | C7.1–C7.2 | L0 | L1 | `seam_note_out` + A H6 |
| **E2E-10** | Swap happy (reserves + nullifier) | C1.2 | L0 | L1* | `swap_happy` (*CW DEX TBD) |
| **E2E-11** | Swap min_out fail | C5.4 / D min_out | L0 | L1* | `min_out_fail` |
| **E2E-12** | Oracle stale / missing mid | C5.1–C5.2 | L0 | L1* | `oracle_stale` |
| **E2E-13** | Oracle cannot mint / inflate balance | C6.1–C6.2 | L0 | L1/L3 | pure + matrix |
| **E2E-14** | LC frozen / misbehaviour | C3.2 | L4 mock | L4 real | Domain C §4 |
| **E2E-15** | Inclusion against wrong root | C3.3 | L0/L4 | L4 | membership reject |
| **E2E-16** | Lag then tip advances → mint accepts | C3.4 | L4 mock | L3+L4 | stateful hinge |
| **E2E-17** | Unshield or bridge_burn exit | C0.2 / C0.3 | L1* | L3 | needs exit msgs |
| **E2E-18** | Tacit anvil confidential mint round-trip | — | L2 | L2 | existing shell |
| **E2E-19** | Dual container smoke (Anvil up + Terp up) | — | L3 | L3 | hashmerchant topology minus VE |
| **E2E-20** | Trust-tier honesty labels on fixtures | C9.* | doc | L3 meta | no Tier-0 mock label |

\* L1* = pure re-export or multi-test until CosmWasm private DEX / exit contracts exist.

### Progression gates (from matrix §13)

1. E2E-01 or E2E-08+E2E-10 path green at **min** layer  
2. E2E-02 + claim double-nullifier  
3. E2E-13 oracle non-authority  
4. E2E-09 shared schema property  
5. Checkpoint in sprint card (template in matrix §13)

---

## 5. Tooling map

| Tool | Role in harness |
|------|-----------------|
| `cargo test` fixture crates | L0 CI |
| `just demo-e2e-l0` / `e2e-l0` | L0 aggregator + E2E-id print |
| `just demo-policy` / `demo-h1` / `demo-keys` | Headstash V0–V2 spine |
| `zk-test-press` `HeadstashSuite` / `PrivateBridgeSuite` | L1 cw-orch |
| `ict-rs` Docker + `ChainSpec` | L3 containers |
| `ict-rs-cw-orch::daemon_builder_from_chain` | L3 → suite |
| `AnvilChain` / `forge` / `cast` | L2/L3 Tacit |
| `evm-confidential-anvil-roundtrip.sh` | L2 golden |
| Mock LC module (planned) | L4 first |

---

## 6. Explicit non-goals (round 1–2)

- Green H1 composite prove as harness gate (demo may mock verify)  
- Real Bitcoin reflection guest in L3 (use fixture public values)  
- Hermes IBC as substitute for reflection conservation  
- Hashmerchant VE as mint authority (bounds only; E2E-13)  
- New opcodes beyond Domain B map  
- Full ict-rs dual-container mint path (E2E-19 smoke stub only — see spectrum `justfile`)  

### L1 mock ZK posture (round 2 freeze)

| Track | Command / surface | CI default | Trust label |
|-------|-------------------|------------|-------------|
| Policy + fixtures | `just demo-policy`, claim fixture validate | **PR** | policy only |
| L0 pure seams | `just demo-e2e-l0` | **PR** | pure structural |
| L1 multi-test claim | mock verify + DEMO-PATH fixture | PR when wired | **mock ZK** — not Tier-0 |
| Real H1 MockProver / prove | `just demo-h1` | **nightly / manual** | circuit soundness track |

---

## 7. Round-2 ordered tasks (status)

1. ~~Answer clarity questions~~ — mint home / suite home via CLARITY (`cw-headstash`)  
2. ~~`just demo-e2e-l0` / `e2e-l0` aggregator~~  
3. ~~Flesh `PrivateBridgeSuite` L0 methods + claim fixture loader~~  
4. L2 wrap: `just tacit-anvil-rt` documented from spectrum README — **deferred**  
5. L3 smoke: dual Anvil+Terp — **stub only** (`docs/plans/spectrum/justfile` `e2e-l3-smoke-stub`)  
6. L4 mock hinge types + E2E-04/14/16 — **partial** (`LcMockConfig` + lag)  
7. When CW bridge mint lands on `cw-headstash`: wire E2E-01 at L1 then L3  
8. Nightly: real prove path for E2E-08 when H1 green  


---

## 8. References

- FLOW: [`FLOW-private-bridge-auth.md`](./FLOW-private-bridge-auth.md)  
- Matrix: [`FLOW-private-bridge-auth-test-matrix.md`](./FLOW-private-bridge-auth-test-matrix.md)  
- Domains B/C/D SPECs in this directory  
- Headstash demo: `crates/headstash/docs/circuit/DEMO-PATH.md`  
- ict-rs dual path: `crates/ict-rs/examples/hashmerchant.rs`  
- Agent report: [`agents/ROUND1-HARNESS.md`](./agents/ROUND1-HARNESS.md)  
