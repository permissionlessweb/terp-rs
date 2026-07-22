# PROMPT — HARNESS / ICT FUNDED LOCAL (final sprint)

**Track id:** `HARNESS-ICT-FUNDED` (Hermes title may still say HARNESS-OBSERVE)  
**Board:** `private-bridge-corridor` · **Parent IMPL:** `t_563b09d5`  
**Pack:** `docs/plans/spectrum/agents/final-sprint-2026-07-22/`  
**Read first:** `ORCHESTRATION.md` + **`FEEDBACK-RAISED-BAR.md`** (human raised bar — obsolete soft residuals)

**Status policy:** AWAIT GREENLIGHT before deep coding. Until then: inventory libs, draft plan in `STATUS-HARNESS-OBSERVE.md`, **STOP**.

---

## Role (raised)

You own the **mainnet-funded-ready proof path**:

1. **ict-rs** spins **fresh** test networks (Terp primary; sidecars as needed).  
2. **cw-headstash** bridge corridor deploys on that chain.  
3. **Observe** uses production topology against **local BTC regtest (preferred) or signet** — not synthetic-only for the funded profile.  
4. **`BridgeMintNote` executes on the live local chain** (cw-orch Daemon / ict-rs-cw-orch), not Mock-only.  
5. Oracle-bound swap film + automation API complete the corridor.  
6. Lab pure/Mock/`demo-corridor-lab` stays green as **floor**.

```text
ict-rs fresh Terp (+ faucet)
  → deploy cw-headstash (bridge cfg; document mock_verify flag)
  → hash-market notify + corridor-btc-reporter
  → BTC regtest/signet fund deposit addr
  → observe → (optional reverify API fields for UI)
  → BridgeMintNote on chain → put_note if applicable
  → oracle bounds → private swap film (compose/dex seams + automation phases)
```

You are the **lead track** for S1/S2/S6 (ORCHESTRATION).

---

## Human feedback you must internalize

| Prior soft plan | Human correction |
|-----------------|------------------|
| “Document residual if Fulcrum not running” as exit | Funded profile must **run** observe against real local index/backend; synthetic only for `lab_simulated` |
| “Daemon BridgeMintNote residual if host film green” | **Reject.** Use suite L3 Daemon path this sprint |
| “P1 money safety document only” | **P0:** amount/address gates on observe; reorg may residual with severity |
| “Unknown tech for multi-net” | Use **in-tree ict-rs + test-press + hash-market** — map in ORCHESTRATION §2 |

---

## Success criteria (testable)

1. **S0 floor:**  
   `cd crates/o-line/plays/private-bridge-corridor && ./preflight.sh --lab && ./e2e-test.sh` green  
   `cd crates/headstash && just demo-corridor-lab` green (or documented equivalent)

2. **S1 funded command (hard exit):** One script/just target, e.g.  
   - `just demo-corridor-ict` **or**  
   - `cargo test -p zk-test-press --features 'interface,...' …` / binary under test-press  
   that on a clean machine (Docker available):
   - starts **fresh** Terp via **ict-rs**
   - deploys headstash bridge corridor
   - runs intent→observe→**chain** `BridgeMintNote`→swap film
   - **exit 0**; no silent skip of mint

3. **S2 observe:** Funded profile uses reporter + electrum/esplora against **local** BTC net (regtest preferred). Document compose service names.

4. **S6 money gates:** Observation rejected if address ≠ watch or amount below configured min (wire watch min / intent if present).

5. Unit green:  
   `cargo test -p hash-market --lib btc_index --features server`  
   `cargo test -p hash-market --lib corridor_deposits --features server`  
   cashapp pure + existing L1 Mock suite still green

6. Update `CORRIDOR-LAB-STATUS.md` with **two columns**: lab floor vs **ict_local_funded**.

7. Write `STATUS-HARNESS-OBSERVE.md` with exact commands, mock_verify setting, and residual only for true mainnet-only items.

---

## In-tree libraries (use these — do not invent a third harness)

| Piece | Path |
|-------|------|
| ict-rs ChainSpec / Docker | `crates/ict-rs/ict-rs/src/spec.rs`, README |
| ict-rs ↔ cw-orch | `crates/ict-rs/ict-rs-cw-orch/` |
| PrivateBridge suite (Mock today; **extend to Daemon**) | `crates/headstash/test-press/src/suites/private_bridge.rs` |
| Suite note: L3 Daemon “not this round” | **Promote this sprint** — same `HeadstashSuite` / `PrivateBridge` types on Daemon |
| Features | `test-press/Cargo.toml`: `interface`, `ict-rs`, `cw-orch`/`daemon` |
| CashApp W0–W7 harness | `test-press/src/harness/cashapp_zec_corridor.rs` |
| Bridge mint fixture | `harness/bridge_l1.rs`, `bridge_mint_fixture.rs`; JSON `fixtures/bridge_mint_claim_happy.v1.json` |
| Contract | `cw-headstash` `BridgeMintNote` / `bridge.rs` |
| Notify | `hash-market` `corridor_deposits.rs` |
| Reporter | `bin/corridor_btc_reporter`, `btc_index/**`, `docs/corridor-btc-reporter.md` |
| Oline play | `crates/o-line/plays/private-bridge-corridor/` |
| Pure seams | `fixtures/cashapp_zec_corridor`, `compose_seams`, `private_dex_seams`, `bridge_auth_seams` |

---

## In scope

- All of the above paths as needed for S1/S2/S6  
- New just/script targets for `ict_local_funded`  
- Compose/play profiles for regtest BTC + hash-market + reporter  
- Minimal recovery: enforce intent **expiry** on mint path; optional `recovery_owner_binding` if pure/contract already has a natural hook — coordinate with DOCS  

## Out of scope

- Cash App private API  
- Spending **mainnet** funds in CI  
- Full Halo2 if mock_verify documented for funded local (state clearly)  
- UI Wizard ownership (UI track)  
- Amending D1–D7  

---

## Freezes

| ID | Constraint |
|----|------------|
| D1 | SeamNoteOutV0 only |
| D2 | Asset map id ≠ intent domain_bind |
| D3 | Private mint **and** oracle-bound swap required on funded film |
| D4 | Open reporter observe topology |
| D5 | Label lab vs ict_local_funded vs production |
| D6 | Zakura dest available in stack (coord ZAKURA track) |
| D7 | Mint = cw-headstash; oracle bound_only; mock_verify only if labeled |

---

## Handoff format (`STATUS-HARNESS-OBSERVE.md`)

```markdown
# STATUS-HARNESS-OBSERVE
## Funded profile command
## Lab floor commands (S0)
## Libraries used (path list)
## BridgeMintNote evidence (tx / query)
## Observe topology (regtest|signet + backend)
## Amount/address gates
## Residuals (mainnet-only only)
## mock_verify: true|false on funded deploy
```

---

## Do not

- Declare done on host synthetic smoke alone  
- Soft-skip chain mint in funded profile  
- Merge notify bus into mint authority  
- Expand to mainnet Cash App / full LC without human go  
