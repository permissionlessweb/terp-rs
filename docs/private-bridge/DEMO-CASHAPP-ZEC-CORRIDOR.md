# DEMO — Cash App → fresh BTC → private bridge → ZEC (diversified dest)

| Field | Value |
|-------|--------|
| **Status** | **Active product corridor SPEC** (production deployments) |
| **Date** | 2026-07-20 |
| **Product spine** | B bridge + C LC hinge + D swap + oracle bounds + E compose |
| **Mint** | `cw-headstash` only ([CLARITY](./CLARITY-cw-headstash-router-and-asset-registry.md)) |
| **UI home** | **dao-dao-ui Module** — production Private Bridge (not claim Headstash airdrop) |
| **Environment** | Built to run against **real production / staging deployments** of mint-router, notes, oracle, and (when attached) LC |

---

## 1. One-paragraph product

**Not a mock film.** A production Private Bridge surface that exercises **real functions of the deployed environment**: browser-generated BTC deposit wallet (SelfRelay HD pattern) → Cash App / wallet QR fund → client proof + auth link → private cross-chain mint → oracle-bound swap to ZEC → payout only to a **pre-authorized diversified destination**. Lab-only simulation is an **explicit** backend mode for CI, not the product default.

```text
Cash App (ops / off-protocol)
  → fund FRESH BTC wallet (no reuse of depositor hot identity)
  → PREAUTH intent: dest diversifier + min rate bound + asset corridor
  → private bridge mint (burn/reflection → SeamNoteOutV0 on Terp)
  → private swap BTC-note → ZEC-note (oracle bound enforced)
  → payout / open only to preauth owner_binding
```

**Production default:** real browser HD keys, real P2WPKH deposit addresses, real module config (mint-router / notes / oracle bases).  
**Lab mode (`lab_simulated`):** CI/local only — explicit, not the product story.  
**LC live:** harness stays open for light-client-backed BTC/ZEC when that environment is attached.

---

## 2. Non-goals (v0)

| Non-goal | Why |
|----------|-----|
| Real Cash App API integration | Ops rail; scripted “user funded fresh wallet” is enough |
| Full ZEC mainnet egress / TZE | North star; stub ZEC asset + diversifier is enough for v0 |
| Full Halo2 private swap circuit | Pure + policy + mock settle first |
| Multi-source oracle bag as mint | Connect **bound_only**; never mints |
| Reuse Headstash **airdrop claim** module as this demo | Wrong product surface |
| Closing harness to sim-only forever | L2+ must accept `AssetBackend::Simulated \| LcLive` |

---

## 3. Intent packet — preauthentication of designation

At **deposit intent** (before or with the BTC send), the depositor commits a fixed packet. This is the cryptographic/product meaning of “authorized by the initial transfer.”

### 3.1 `DepositIntentV0` (canonical fields)

| Field | Width / type | Role |
|-------|--------------|------|
| `version` | u8 | `0` |
| `corridor_id` | string / 32B hash | e.g. `cashapp-btc-zec-v0` |
| `source_chain_tag` | string | `bitcoin` / sim tag |
| `dest_chain_tag` | string | `zcash` / sim tag |
| `btc_deposit_addr` | string | **fresh** receive address (demo may be sim) |
| `btc_txid_or_intent_id` | 32B | once funded: bind to burn ν / claim_id material |
| `dest_owner_binding` | 32B | **preauth diversified destination** (opaque handle → ZEC diversifier / UA digest) |
| `dest_display_hint` | string optional | UI-only truncated UA/bech32; not authority |
| `asset_in_id` | 32B | BTC / sim-BTC registry id |
| `asset_out_id` | 32B | ZEC / sim-ZEC registry id |
| `min_out_value` | u64 | absolute min ZEC-side units after swap |
| `max_slippage_bps` | u16 | optional slip vs oracle mid |
| `oracle_market_id` | string | e.g. `BTC-ZEC` Connect market |
| `oracle_bound_policy` | enum | `mid_gte_floor` / `within_band` (v0: floor from mid×(1−slip)) |
| `created_at` | u64 | unix |
| `expiry` | u64 | intent expires; reject late mints |
| `domain_bind` | 32B | `H("terp-cashapp-intent-v0" ‖ canonical_bytes_without_domain_bind)` |

**Invariant:** Swap + payout **must** re-check `dest_owner_binding` and oracle policy against this intent. Changing dest after fund = **new intent** (old deposit cannot redirect).

### 3.2 Preauth flow (demo)

```text
1. UI: user generates or pastes DIVERSIFIED ZEC dest → owner_binding
2. UI: user sets min_out / max_slippage on BTC→ZEC
3. System: create DepositIntentV0, show domain_bind + fresh BTC deposit addr
4. User: Cash App (or faucet) funds that addr
5. Harness: observe deposit (sim: mint sim-BTC credit; live: LC/proof of burn)
6. Bridge mint: note owner_binding = intent.dest_owner_binding
7. Swap: enforce oracle bound + min_out from intent
8. Success: only that dest can open / receive ZEC-side result
```

### 3.3 Reject matrix (intent)

| ID | Case | Expected |
|----|------|----------|
| I1 | Swap to different `owner_binding` than intent | **REJECT** |
| I2 | Mid below depositor floor / slip exceeded | **REJECT** |
| I3 | Intent expired | **REJECT** |
| I4 | Double mint same burn ν / intent | **REJECT** |
| I5 | Happy: bound OK + dest match | **ACCEPT** |
| I6 | Oracle stale / missing mid | **REJECT** (no silent mint) |

---

## 4. Asset backends (sim vs live LC)

Keep the harness **backend-pluggable**. Demo defaults to simulated assets; live testnets plug LC without rewriting the workflow.

```rust
// Logical — implement in test-press / harness, not forced into wasm
pub enum CorridorAssetBackend {
    /// Demo: mint sim-BTC / sim-ZEC credits; mock burn membership.
    Simulated {
        btc_faucet: SimFaucet,
        zec_faucet: SimFaucet,
    },
    /// Future: LC-attested BTC burn + ZEC transfer proofs.
    LightClient {
        btc_lc: LcHandle,   // reflection / burn root tip
        zec_lc: LcHandle,   // optional egress / inbound
        mode: LcMode,       // MockAttestation | Live
    },
}
```

| Backend | Deposit success means | Mint proof | When |
|---------|----------------------|------------|------|
| **Simulated** | Faucet credited fresh addr / sim burn id | `mock_verify` / pure authorize | Default CI + UI demo |
| **LC Mock** | Attestation fixture for burn under tip | mock verify + real msg shape | L2 harness |
| **LC Live** | Real tip + burn membership | real proofs when available | Nightly / lab |

**Do not** hardcode “only sim forever” in suite APIs — every success path takes `CorridorAssetBackend`.

---

## 5. End-to-end workflow (automation)

### 5.1 Happy path steps (agent E2E owns this)

| Step | Action | Success criterion |
|------|--------|-------------------|
| W0 | Create `DepositIntentV0` (preauth dest + bounds) | `domain_bind` stable |
| W1 | Allocate **fresh** BTC deposit address | not reused in suite |
| W2 | **Deposit** via backend (sim mint or LC) | deposit observed / burn id |
| W3 | `BridgeMintNote` (or pure authorize → L1 execute) | note minted; `owner_binding` = intent |
| W4 | Optional: `put_note_after_mint` | notes store round-trip |
| W5 | Private swap sketch/apply with oracle mid | reserves + ν; mid within policy |
| W6 | Assert dest binding + min_out | I5 green |
| W7 | Record receipt JSON for UI/demo film | artifact on disk |

### 5.2 Layers

| Layer | Implementation | Docker? |
|-------|----------------|---------|
| **L0** | pure intent + bridge + oracle bound + swap | No |
| **L1** | cw-orch Mock + sim backend | No |
| **L2** | optional HTTP notes_base | No |
| **L3+** | ict-rs + LC live handles | Yes, optional |

---

## 6. Oracle enforcement

| Source | Role |
|--------|------|
| hash-market Connect bounds (`OracleBound`, `GET /oracle/bounds`) | mid for `BTC-ZEC` (or sim market) |
| `DepositIntentV0` | depositor floor / slip |
| Domain D / `private_dex_seams` | reject if mid violates policy |

**Oracle never mints.** Bound failure → no successful swap step.

## 6b. Deposit runtime notify (hash-market / hashmerchant host)

Close the UI automation loop without a closed operator:

| Step | Who | API |
|------|-----|-----|
| Open watch | **Webapp only** after client proof | `POST /corridor/watches` |
| Subscribe | Webapp SSE/poll | `GET /corridor/watches/:intent_id/events` |
| Report deposit | Open-source indexer / lab / same host | `POST /corridor/observations` |

**Trust:** notifications are **coordination**. UI (or client) re-verifies BTC deposit before mint. Protocol is public — trustless automation.

SSOT: `crates/terp-rs/tools/hash-market/docs/corridor-deposit-notify.md`

---

## 7. UI — dao-dao **Module** (agent UI owns this)

### 7.1 Module identity

| Item | Value |
|------|--------|
| **Working title** | `PrivateCorridor` / display **“Private Bridge”** |
| **Kind** | dao-dao-ui **Module** (same pattern as Calendar, Headstash, Marketplace) |
| **Not** | ManageHeadstash claim/bloom airdrop UI |

### 7.2 Screens (v0 wizard)

1. **Preauth ZEC destination** — paste/generate diversified dest → `owner_binding`  
2. **Rate policy** — min out / max slip / market id  
3. **Browser deposit wallet** — **client/browser HD** via `DirectSecp256k1HdWallet.generate(24)`  
   (**same pattern as dao-dao SelfRelay**). Real secp256k1 keys in the browser; mnemonic optional backup only.  
4. **QR + auth link** — after **client-side proof** of wallet material + sealed `DepositIntentV0`:  
   - QR = `bitcoin:` URI for **P2WPKH** address derived from that wallet (Cash App / wallet scan)  
   - **Auth link** (intent + proof digests; **no mnemonic**)  
5. **Status** — production: deposit observed → mint-router / notes / oracle from deployment config  
6. **Receipt** — dest match, amounts, bound, deposit addr  

**Hard rules:**  
- Wallet you **send BTC to** = browser HD wallet (never server hot wallet).  
- This module **demonstrates production functions**, not a scripted fake success path (lab mode is opt-in).

### 7.3 Registration

- New `ModuleId` (do **not** collide with Headstash claim)  
- Export from `modules/index.ts` + `getModules()`  
- Terp chain only if Calendar-style gate needed  
- Copy: first-principles demo language; label **simulated assets** when backend is sim  

### 7.4 Data

```ts
// conceptual Module data
type PrivateCorridorModuleData = {
  headstashContract: string
  notesBase?: string
  oracleBase?: string
  corridorId: string
  assetBackend: 'simulated' | 'lc_mock' | 'lc_live'
  // mesh optional later
}
```

---

## 8. Three team tracks (spawn these)

| Track | Agent role | Primary paths | Done when |
|-------|------------|---------------|-----------|
| **UI** | dao-dao Module wizard | `websites/dao-dao-ui/packages/stateful/modules/modules/PrivateCorridor/` (name may vary) | Module registered; wizard steps 1–5 mockable without live chain |
| **E2E** | Automation workflow | `zk-test-press` harness + compose fixtures | W0–W7 L0 green; L1 Mock deposit→mint→swap bound with **Simulated** backend |
| **Corridor / preauth** | Intent packet + oracle bind + backend trait | spectrum fixtures + `DEMO-*` pure types; optional cw msg attrs | `DepositIntentV0` encode + I1–I6 tests; `CorridorAssetBackend` trait/stub |

**Coordination:** Corridor defines types first (or in parallel with pure tests). E2E consumes types. UI consumes same JSON shape for intent/receipt.

---

## 9. Relation to closed core products

| Closed product | Use in this demo |
|----------------|------------------|
| Poseidon distro | claim path optional; not required for bridge demo |
| `BridgeMintNote` L1 | W3 |
| SEAM-NOTE-OUT + note persist | W3–W4 |
| pure DEX + oracle seams | W5–W6 |
| Connect bounds (hash-market) | oracle mid feed (sim or HTTP) |
| Headstash **claim** dao module | **Do not reuse** as corridor UI |

---

## 10. Success criteria (demo film)

1. Operator runs **L0** corridor tests → all I1–I6 + W happy green.  
2. Operator opens **DAO module** → completes preauth → “fund” sim → sees success receipt with **dest = preauth**.  
3. Flipping mid below floor in test → **failure** shown (oracle enforced).  
4. Docs state: **sim assets default**; `LightClient` backend hook documented for live lab.

---

## 11. Cold-start for agents

```
Implement DEMO-CASHAPP-ZEC-CORRIDOR.md track <UI|E2E|CORRIDOR>.
Read that SPEC + SEAM-NOTE-OUT + NOTE-PERSIST-L0 + CLARITY mint router.
Do not invent cw-bridge-mint. Do not put private notes on public /content.
Simulated BTC/ZEC OK; keep CorridorAssetBackend open for LC live.
Preauth dest_owner_binding is mandatory — never optional in happy path.
```

### Implementation (Corridor / preauth)

| Item | Path |
|------|------|
| Pure fixture crate | [`fixtures/cashapp_zec_corridor`](./fixtures/cashapp_zec_corridor/) |
| Types | `DepositIntentV0`, `intent_allows_swap`, `CorridorAssetBackend`, `sim_deposit` |
| Tests | I1–I6 (+ domain_bind / LC mock) — `cd docs/plans/spectrum/fixtures/cashapp_zec_corridor && cargo test` |
| E2E dep | `cashapp_zec_corridor = { path = "docs/plans/spectrum/fixtures/cashapp_zec_corridor" }` |

---

## 12. Implementation status (2026-07-20 three-track land)

| Track | Status | Path |
|-------|--------|------|
| **CORRIDOR pure** | **Green (11 tests)** | `fixtures/cashapp_zec_corridor` |
| **E2E** | **Green (7 pass, 1 ignore LC live)** | `test-press` `harness/cashapp_zec_corridor.rs` |
| **UI Module** | **Scaffold + mock wizard** | dao-dao `modules/PrivateCorridor` (`ModuleId.PrivateCorridor`) |

```bash
cd docs/plans/spectrum/fixtures/cashapp_zec_corridor && cargo test
cd crates/headstash && cargo test -p zk-test-press --lib cashapp_zec --features 'interface,l0-seams'
# UI: enable Private Bridge module on Terp DAO in dao-dao-ui
```

**Follow-ups:** path-dep E2E on pure crate (drop local intent dup); wire UI to real headstash/oracle; film I2 fail in wizard; LC live lab.

## 13. Links

- [FLOW-private-bridge-auth.md](./FLOW-private-bridge-auth.md)  
- [FLOW-private-bridge-auth-test-matrix.md](./FLOW-private-bridge-auth-test-matrix.md)  
- [SPEC-private-dex-seams.md](./SPEC-private-dex-seams.md) §3 oracle  
- [E2E-HARNESS-PLAN.md](./E2E-HARNESS-PLAN.md)  
- [reviews/CORE-PRODUCTS-CLOSE-2026-07-20.md](./reviews/CORE-PRODUCTS-CLOSE-2026-07-20.md)  
- hash-market `docs/oracle-connect-bounds.md`  
- dao-dao module: `websites/dao-dao-ui/packages/stateful/modules/modules/PrivateCorridor/`  
- Agent briefs: [DEMO-CORRIDOR-UI](./agents/DEMO-CORRIDOR-UI.md) · [DEMO-CORRIDOR-E2E](./agents/DEMO-CORRIDOR-E2E.md) · [DEMO-CORRIDOR-PREAUTH](./agents/DEMO-CORRIDOR-PREAUTH.md)  
