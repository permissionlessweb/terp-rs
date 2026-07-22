# Final sprint orchestration — Cash App → Private Bridge → ZEC

| Field | Value |
|-------|--------|
| **Date** | 2026-07-22 (revised — **mainnet-funded-ready bar**) |
| **Board** | `private-bridge-corridor` |
| **Parent IMPL** | `t_563b09d5` |
| **Parent DOCS** | `t_4a2cd032` |
| **Sprint pack** | `docs/plans/spectrum/agents/final-sprint-2026-07-22/` |
| **Authority freezes** | `DESIGN-DECISIONS-CORRIDOR-ACCEPTED-2026-07-20.md` **D1–D7** (do not amend) |
| **Product SPEC** | `DEMO-CASHAPP-ZEC-CORRIDOR.md` |
| **End-user composition** | `USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md` |
| **Human bar revision** | Lab film is table stakes. Sprint end = **ready for mainnet-funded workflow**, proven by **full local multi-network simulation on fresh testnets via ict-rs**. |

**Human review gate:** Teammates **await greenlight** before deep coding. This document is the revised program bar after human feedback — **do not** re-lower scope to “host pure film only.”

---

## 1. Goal (this sprint) — raised bar

### Product spine (unchanged)

```text
Cash App / wallet → fund FRESH BTC deposit (browser HD P2WPKH)
  → DepositIntentV0 sealed FIRST (dest_owner_binding + min_out/slip + domain_bind)
  → observe deposit (hash-market watches + corridor-btc-reporter / Fulcrum|Esplora|regtest)
  → private mint (cw-headstash → SeamNoteOutV0)
  → oracle-bound private swap (oracle = bounds only, never mints)
  → open/payout only to preauth dest (Zakura local / regtest dest UX)
```

### Sprint-end definition of done (new)

| Layer | Definition |
|-------|------------|
| **Mainnet-funded-ready** | Operators can run the **same workflow shape** that mainnet money will use: intent → fund → observe → reverify → **live chain** `BridgeMintNote` → oracle-bound swap film → preauth dest. Labels and configs flip from testnet/regtest → mainnet; code path does not fork into “demo-only.” |
| **Proof vehicle** | **Full local simulation on fresh test networks via `ict-rs`** (+ existing pure/Mock layers as CI fast path). Not a substitute product — a **fidelity gate** before mainnet funds. |
| **Honesty** | `lab_simulated` / Mock-only surfaces keep D5 banners. The **funded path profile** is separately labeled (e.g. `ict_local_funded` / `testnet_funded`) — never claim mainnet settlement from local nets. |

### Success criteria (testable sprint end)

| # | Criterion | Evidence |
|---|-----------|----------|
| **S0** | Fast path still green (regression floor) | `just demo-corridor-lab`; pure I1–I6; host smoke; oline lab preflight/e2e |
| **S1** | **ict-rs funded local multi-net** | One documented command (e.g. `just demo-corridor-ict` or `test-press` binary) that: spins **fresh** Terp (and required sidecars) via **ict-rs**; deploys **cw-headstash** bridge corridor; runs intent → observe → **Daemon/local-chain `BridgeMintNote`** → swap film → dest binding check. Exit 0 or explicit FAIL (no silent skip of mint). |
| **S2** | Observe path is production-shaped | Local **BTC regtest or signet** (or documented Esplora/Electrum against that net) → `corridor-btc-reporter` → `/corridor/observations`. Synthetic-only observe is **not** sufficient for the funded profile (lab profile may keep synthetic). |
| **S3** | UI production trust | SSE/`deposit_observed` → **`depositReverify`** with observation fields; **fail-closed** on reverify failure for production-shaped modes (operator override only if explicit). |
| **S4** | D3 oracle-bound private swap on funded path | Swap phase uses oracle mid/bounds against intent `min_out` / slip; pure seams + chain settle per available libs; automation phases green end-to-end. |
| **S5** | D6 Zakura dest on funded path | Live local Zakura regtest (or harness equivalent) produces dest + **shared** `terp-dest-binding-v0` golden vector with UI/harness. Paste-first remains UX primary; node must be **available** in funded compose/ict stack. |
| **S6** | Money-safety P0 on observe | Observation amount/address gates vs open watch + intent (use existing watch fields / reporter config). Document reorg residual if full revoke not free. |
| **S7** | Recovery honesty + minimal binding | Intent expiry + `dest_owner_binding` enforced; **recovery_owner_binding** / timeout path: implement if nearly free from existing binding domains, else document operator recovery using deposit HD keys + intent id with **no open admin seize** claim. USER-GUIDE updated. |
| **S8** | Meta-review | Freezes untouched; no overclaims; funded profile ≠ mainnet money. |

### Non-goals (still explicit — do not expand without human go)

- Integrating **Cash App’s private API** (funding remains any wallet QR to deposit addr)
- **Mainnet** broadcast of real user BTC/ZEC in CI (local/test nets only in sprint)
- Full Halo2 browser circuit if Mock/proof path already satisfies lab; document residual for production proofs
- Full ICS-02 dregg multi-hop
- Amending D1–D7 freezes

---

## 2. Library map — resolve “unknown deltas” with **in-tree** code

Teams must **prefer these** over inventing new frameworks:

| Need | Already in monorepo | Entry points |
|------|---------------------|--------------|
| Fresh multi-chain local nets | **ict-rs** | `crates/ict-rs/` — `ChainSpec` (`terp`, gaia, osmosis, juno, akash, anvil), Docker runtime, quickspawn |
| Deploy CosmWasm on ict chain | **ict-rs-cw-orch** + **cw-orch Daemon** | `crates/ict-rs/ict-rs-cw-orch/`; test-press `cw-orch` feature `daemon` |
| Bridge mint on chain | **cw-headstash** `BridgeMintNote` | `crates/headstash/contracts/cw-headstash/src/bridge.rs`, `ExecuteMsg::BridgeMintNote` |
| L1 Mock + suite helpers | **zk-test-press** PrivateBridge suite | `crates/headstash/test-press/src/suites/private_bridge.rs` — `bridge_mint_note`, E2E-01/02, L2 put_note |
| L3 Daemon path (elevate) | Same suite types; comment already says L3 = Daemon from ict-rs-cw-orch **“not this round”** → **this sprint makes it this round** | `test-press/Cargo.toml` features `interface` + `ict-rs` + `cw-orch` |
| CashApp corridor pure | **cashapp_zec_corridor** fixture + harness | `docs/plans/spectrum/fixtures/cashapp_zec_corridor`; `test-press/.../harness/cashapp_zec_corridor.rs` W0–W7 |
| Compose / auth / DEX pure | pure crates | `fixtures/compose_seams`, `bridge_auth_seams`, `private_dex_seams`, `seam_note_out` |
| Happy mint fixture JSON | fixture | `docs/plans/spectrum/fixtures/bridge_mint_claim_happy.v1.json` |
| Deposit notify + SSE | **hash-market** | `corridor_deposits.rs`, docs `corridor-deposit-notify.md` |
| BTC light index | **hash-market btc_index** + **corridor-btc-reporter** | `btc_index/**`, `docs/corridor-btc-reporter.md`, electrum + esplora backends |
| Oline packaging | **private-bridge-corridor play** | `crates/o-line/plays/private-bridge-corridor/` |
| Note persist after mint | harness client | `test-press/.../note_persist_*.rs`, hash-market notes |
| Oracle bounds | hash-market Connect | `oracle/`, `docs/oracle-connect-bounds.md` |
| Zakura dest | e2e + UI | `docs/plans/spectrum/e2e/zakura/`, `zakuraDest.ts`, `harness/zakura_local.rs` |
| Intent / dest domains | D2 + dest binding | `terp-cashapp-intent-v0`, `terp-dest-binding-v0` |

**Rule:** If a delta feels “blocked on unknown tech,” re-read this table and the cited files before proposing greenfield work.

---

## 3. Team roster (tracks)

| Track id | Role | Prompt | Hermes bias |
|----------|------|--------|-------------|
| **HARNESS-ICT-FUNDED** | ict-rs multi-net + observe + chain mint orchestration (lead path) | `PROMPT-HARNESS-OBSERVE.md` (revised) | child of `t_563b09d5` |
| **UI-MINT-SWAP** | fail-closed reverify + production-shaped UI film aligned to funded profile | `PROMPT-UI-MINT-SWAP.md` | child of `t_563b09d5` |
| **ZAKURA-DEST** | D6 live local dest in funded stack + binding golden vector | `PROMPT-ZAKURA-DEST.md` | child of `t_563b09d5` |
| **DOCS-USER-GUIDE** | USER-GUIDE for mainnet-funded readiness + local ict proof path | `PROMPT-DOCS-USER-GUIDE.md` | child of `t_4a2cd032` |
| **META-REVIEW** | Bar compliance: funded readiness vs overclaim; freezes | `PROMPT-META-REVIEW.md` | child of `t_563b09d5` |

See **`TEAM-ROSTER.md`**.  
Human feedback that closed soft deltas: **`FEEDBACK-RAISED-BAR.md`** (replaces soft “accept residual” posture in prior `DELTAS-FOR-HUMAN.md`).

---

## 4. Dependency graph

```text
[HARNESS-ICT-FUNDED]  ── leads S1/S2/S6: ict-rs + reporter + BridgeMintNote on fresh nets
        │
        ├── supplies automation API + chain addresses to ──► [UI-MINT-SWAP]
        │
        └── compose/zakura hooks ──► [ZAKURA-DEST]

[UI-MINT-SWAP] ── S3/S4 UI; consumes funded endpoints from HARNESS
[ZAKURA-DEST] ── S5 dest + golden vector; paste-first + live regtest in stack
[DOCS-USER-GUIDE] ── S7 language; early polish + second pass after STATUS files
[META-REVIEW] ── after STATUS-*; gate on S0–S8 honesty
```

**Execution order**

1. Human greenlight on this pack  
2. **Parallel:** HARNESS-ICT-FUNDED (priority) ∥ UI-MINT-SWAP ∥ ZAKURA-DEST (file fences below)  
3. DOCS second pass when S1 command exists  
4. META-REVIEW before declaring sprint closed  

### File ownership fences

| Owner | Exclusive edit surfaces |
|-------|-------------------------|
| HARNESS-ICT-FUNDED | `crates/o-line/plays/private-bridge-corridor/**`; `hash-market` `btc_index/**` + reporter; `test-press` harness/suites for **ict/Daemon funded path**; new `just demo-corridor-ict*` targets under headstash/spectrum justfiles; spectrum `e2e/` scripts for funded profile |
| UI-MINT-SWAP | `PrivateCorridorWizard.tsx`, `automationClient.ts`, `btcReverify.ts`, `depositNotify.ts`, related types/OPERATOR |
| ZAKURA-DEST | `e2e/zakura/**`, `zakuraDest.ts`, dest-step only in Wizard **via PR coordination** (prefer helpers exported from `zakuraDest.ts`) |
| DOCS | `USER-GUIDE-*.md`, spectrum README links; no exclusive lock on OPERATOR beyond links |
| META | read-only + `STATUS-META-REVIEW.md` + reviews notes |

---

## 5. Profiles (do not collapse)

| Profile | Purpose | Mint | Observe |
|---------|---------|------|---------|
| `lab_simulated` | CI fast path | Mock / mock_verify / synthetic observe | Synthetic OK |
| **`ict_local_funded`** (sprint gate) | Mainnet-workflow fidelity | **Real local chain** BridgeMintNote via ict-rs + cw-orch Daemon | **Regtest/signet** (or local Esplora/Electrum) → reporter |
| `production` / mainnet | Post-sprint ops | Proof policy per deployment | Fulcrum/self-hosted index |

UI and docs must distinguish these three.

---

## 6. Hermes

Board: `private-bridge-corridor`  
Children already created under IMPL/DOCS — **bodies updated** to raised bar; re-attach this pack after edits.

---

## 7. Explicitly not started until greenlight

Deep coding, mainnet money, freeze amendments, Cash App private API.
