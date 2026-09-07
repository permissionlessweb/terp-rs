# Human feedback — raised sprint bar (closes soft deltas)

**Date:** 2026-07-22  
**Authority:** product owner (not close to day-to-day dev, but certain on **outcome** and **library availability**)  
**Supersedes soft posture in:** prior `DELTAS-FOR-HUMAN.md` assumptions A1–A7 that accepted “film-only” or “docs residual” as sprint exit.

---

## Outcome requirement (non-negotiable)

**Sprint end = ready to go to a mainnet-funded workflow.**

That does **not** mean CI spends mainnet money. It means:

1. The **workflow shape** (intent → fund → observe → reverify → chain mint → oracle-bound swap → preauth dest) is the same code path mainnet will use.
2. Fidelity is proven by **fully simulating that workflow locally on fresh test networks via `ict-rs`** (Terp + required sidecars; BTC regtest/signet observe; Zakura local for dest).
3. Lab pure/Mock/`lab_simulated` remains the **fast CI floor**, not the finish line.

Teams: if a prompt still says “residual OK for BridgeMintNote / observe / recovery,” treat that language as **obsolete** and follow this file + revised `ORCHESTRATION.md`.

---

## Delta decisions (concrete)

### DΔ-1 — Production path vs production money → **RAISE**

| Was | Now |
|-----|-----|
| Production-shaped packaging + UI reverify; mainnet money out of scope as exit | **Mainnet-funded-ready** as exit; **proof = ict-rs local multi-net funded profile** |

**Libraries / surfaces to use (do not greenfield):**

- `crates/ict-rs` + `crates/ict-rs/ict-rs-cw-orch` — fresh Terp (and friends) networks  
- `crates/headstash/test-press` — `interface` + `ict-rs` + `cw-orch`/`daemon` features  
- Oline play + hash-market already model the observe topology  

**Exit evidence:** one command documented in STATUS that runs **S1** (see ORCHESTRATION). Soft-skip of chain mint is a **FAIL** for the funded profile.

---

### DΔ-2 — BridgeMintNote on live daemon → **REQUIRE on local chain**

| Was | Now |
|-----|-----|
| Host pure film + Mock may satisfy if labeled | Funded profile **must** execute **`BridgeMintNote` on a real local chain** (ict-rs spun Terp + cw-orch Daemon or equivalent). Mock remains CI floor only. |

**Libraries / surfaces:**

- Contract: `cw-headstash` `ExecuteMsg::BridgeMintNote` + `bridge.rs`  
- Suite: `test-press/src/suites/private_bridge.rs` — already has `bridge_mint_note`, E2E-01 happy, E2E-02 double-mint, L2 put_note_after_mint  
- L3 Daemon was explicitly deferred (“not this round”) → **this sprint promotes L3** using the **same suite types** on Daemon from `ict-rs-cw-orch`  
- Fixture: `docs/plans/spectrum/fixtures/bridge_mint_claim_happy.v1.json`  
- Harness builders: `test-press/src/harness/bridge_l1.rs`, `bridge_mint_fixture.rs`  

**Exit evidence:** funded e2e log shows tx hash / contract events for mint; `IsBridgeMinted` query true for nullifier.

---

### DΔ-3 — Zakura local support → **PASTE-FIRST UX + LIVE NODE IN FUNDED STACK**

| Was | Now |
|-----|-----|
| Paste-first alone could complete D6 | Paste-first remains **UX primary**, but funded stack **must include** runnable Zakura regtest (or documented harness parity) so dest is not fiction |

**Libraries / surfaces:**

- `docs/plans/spectrum/e2e/zakura/`, `ZAKURA-LOCAL.md`, `just demo-zakura-local-dest`  
- UI: `PrivateCorridor/zakuraDest.ts`  
- Harness: `test-press/src/harness/zakura_local.rs`  
- Binding domain: `terp-dest-binding-v0` — one golden vector shared UI ↔ harness ↔ docs  

**Exit evidence:** funded compose/ict path can print dest + binding; UI button or paste matches golden vector.

---

### DΔ-4 — Loss mitigation / recovery → **MINIMAL REAL POLICY, NOT DOCS-ONLY DEFAULT**

| Was | Now |
|-----|-----|
| Docs non-claims default | Prefer **code-backed** stage policy using existing intent fields |

**Use existing primitives first:**

| Primitive | Where | Policy use |
|-----------|--------|------------|
| `dest_owner_binding` | DepositIntentV0 / DEMO SPEC | Only this dest may open result |
| `expiry` | intent | Reject late mint (I3) — timeout gate already specified |
| `domain_bind` | `terp-cashapp-intent-v0` | Intent integrity |
| Deposit HD mnemonic (browser) | UI `browserWallet.ts` | Pre-observe recovery of UTXO remains user-held |
| Double-mint reject | cw-headstash `IsBridgeMinted` | No second seize of same ν |

**Implement if nearly free (HARNESS + pure seams):**

- Optional `recovery_owner_binding` field on intent (or reuse dest domain with explicit recovery role in compose seams) checked on timeout path  
- Observe amount ≥ intent floor when intent carries min deposit (align reporter/watch minAmount)  

**Do not implement:** open operator hot-wallet seize, Cash App clawback, admin redirect of funded intent.

**Exit evidence:** USER-GUIDE stage table matches code; I1–I6 still green; funded path fails closed on expired intent.

---

### DΔ-5 — Oline P1 money safety → **PROMOTE P0 GATES; P1 REORG DOCUMENTED**

| Was | Now |
|-----|-----|
| P1 residual if time | **P0 this sprint:** address match + amount/dust gate on observation vs watch/intent. **P1 reorg revoke / Electrum TLS:** implement if library already supports; else STATUS residual with severity. |

**Libraries / surfaces:**

- `hash-market` `corridor_deposits` + `btc_index` (electrum + esplora multi-failover already)  
- `corridor-btc-reporter` config (`reporter.example.toml`, docs)  
- Reviews: `REVIEW-OLINE-BTC-INDEXER-BLINDSPOTS-2026-07-20.md`  

---

## Answers to prior open questions

| Question | Decision |
|----------|----------|
| Signet funded optional profile? | **Yes for fidelity** — prefer **local regtest BTC** under full control in ict/compose; signet OK as secondary if regtest harder. Soft-skip **not** OK for funded profile exit. |
| UI reverify fail: block or warn? | **Fail-closed** for production / `ict_local_funded` / `lc_live`. Lab may warn+continue with banner. |
| Golden vector location | Prefer `docs/plans/spectrum/e2e/zakura/golden-dest-binding.json` (or fixtures) + single import in harness/UI tests |
| USER-GUIDE publish | In-module “How it works” link + spectrum path; site later OK |
| Hermes assign | Keep unassigned until greenlight; then dispatch |

---

## What teams must write in every STATUS file

1. **Funded profile command** (or blocker with file:line of missing lib — not vague “needs research”)  
2. **Which in-tree library** closed each residual  
3. **What remains mainnet-only** (keys, liquidity, legal) vs **code-complete**  

---

## Explicit non-claims after sprint (still honest)

Even when S0–S8 pass:

- Local/testnet success ≠ mainnet settlement  
- mock_verify may still be on for some lab mints — funded profile must document whether mock_verify is true/false on that deploy  
- Cash App can freeze **their** rail independently of Terp  
