# STATUS-GAP-OBSERVE

| Field | Value |
|-------|--------|
| **Track** | OBSERVE |
| **Date** | 2026-07-22 |
| **Board** | `private-bridge-corridor` |
| **Mode** | Gap analysis only (read / code search) |
| **Answer (headline)** | **Stages are sequenced loosely by `intent_id`.** Observation fields (`txid`, `amount_sats`, `btc_deposit_addr`, optional `domain_bind`) do **not** currently feed a continuous `BridgeMintClaimPublic` identity. Mint claim identity is independent fixture / pure-film material. |

---

## 1. Goal for full BTC → Terp → ZEC (this track)

For full multi-net BTC→Terp→ZEC, OBSERVE must provide a **trusted, field-complete deposit fact** that downstream mint can bind into once-per-claim identity:

| Requirement | Meaning |
|-------------|---------|
| **Detect** | Real BTC funding to a watch-bound fresh address (index: bitcoind / Fulcrum / Esplora). |
| **Publish** | Coordination event with at least `{intent_id, btc_deposit_addr, txid, amount_sats, confirmations}`. |
| **Gate** | Amount floor, conf depth, addr match, watch TTL; optional domain_bind echo. |
| **Identity handoff** | Deterministic mapping of observation (+ sealed intent) → mint ν / `claim_id` / `value_u64` / dest binding so the same deposit cannot mint twice under a different claim and cannot mint a different amount than observed. |
| **Trust** | Notify bus remains **hint-only** (D4); authority stays on-chain mint + optional UI re-verify — but the **fields** that enter the claim must be continuous with the observation, not a parallel golden fixture. |

**Key product question (this track):**  
Can observation `(txid, amount, addr, intent_id)` feed a continuous BridgeMint claim identity, or are stages only sequenced loosely?

**Verdict:** **Loosely sequenced only today.** Shared key is the string `intent_id` across watch → observation → automation phase. No code path builds `BridgeMintClaimPublic` from `DepositObservation`.

---

## 2. Code reality today (cite paths / commands)

| Surface | Status | Evidence |
|---------|--------|----------|
| **Watch open API** | **Green (coord)** | `POST /corridor/watches` → `DepositWatch` with `intent_id`, `btc_deposit_addr`, `domain_bind`, `dest_owner_binding`, `client_proof_digest`, optional `min_amount_sats`. `/Users/returniflost/abstract/terp-core/crates/terp-rs/tools/hash-market/src/corridor_deposits.rs` (`OpenWatchRequest`, `open_watch`) |
| **Observation API** | **Green (coord)** | `POST /corridor/observations` → `DepositObservation` `{intent_id, btc_deposit_addr, txid, amount_sats, confirmations, reporter, domain_bind?}`. Same file `report_observation` |
| **SSE / poll** | **Green (coord)** | `GET /corridor/watches/:intent_id`, `…/events` (`DepositEvent::DepositObserved`); server routes in `server.rs` |
| **Amount gates (S6)** | **Green on notify plane** | Dust `amount_sats==0` reject; watch `min_amount_sats`; reporter floor = `max(cfg.min_amount_sats, watch.min_amount_sats)`. Tests in `corridor_deposits` module; regtest negative test in `corridor-ict-funded.sh` |
| **corridor-btc-reporter** | **Green lab/funded path** | `btc_index/reporter.rs` poll loop; backends esplora / electrum / bitcoind. Docs: `docs/corridor-btc-reporter.md`. Dedup key `intent_id:txid` in local state |
| **ict_local_funded observe** | **Green** | `docs/plans/spectrum/e2e/corridor-ict-funded.sh`: regtest fund → reporter → `deposit_observed`. Confirmed green in `STATUS-WASM-FUNDED-GREEN.md` |
| **Lab synthetic observe** | **Green (labeled)** | `lab-observer.sh`, `corridor-lab-smoke.sh` POST synthetic txids |
| **Automation phase bus** | **Green film only** | `PUT/GET /corridor/automation/:intent_id` phases `deposit_observed→bridging→swapping→complete\|failed`. Explicitly **not** mint authority (`put_automation` comment) |
| **Observation → BridgeMint claim builder** | **Missing** | No converter from `DepositObservation` → `BridgeMintClaimPublic`. Chain mint uses `BridgeL1World::happy()` / fixture in `corridor_ict_funded.rs` + `PrivateBridgeSuite::configure_bridge_happy` |
| **Pure film intent→ν** | **Partial (in-harness only)** | `cashapp_zec_corridor.rs` `bind_deposit_id` + `bridge_mint_for_intent`: ν := `intent.btc_txid_or_intent_id`, `value` from scenario amount — **not** wired to hash-market observation JSON |
| **Mint claim shape** | **Exists, different schema** | `BridgeMintClaimPublic` in `cw-headstash/src/bridge.rs`: `nullifier`, `claim_id`, `value_u64`, LC roots, `domain_binding`, etc. **No** `intent_id`, **no** BTC `txid`, **no** deposit addr fields |
| **Once-per-claim on chain** | **Nullifier-keyed** | `bridge_claim_storage_key(nullifier)` = `bridge:02:{hex(ν)}`; double-mint rejects on ν, not on observation txid |
| **Trust model docs** | **Honest** | Module docs + `corridor-deposit-notify.md`: observations are **hints**; mint authority not on bus |

### Field continuity matrix (goal vs today)

| Observation / watch field | BridgeMintClaimPublic counterpart | Coupled today? |
|---------------------------|-----------------------------------|----------------|
| `txid` | Should feed ν / `btc_txid_or_intent_id` / claim material | **No** (pure film only after manual `bind_deposit_id`; chain fixture independent) |
| `amount_sats` | `value_u64` | **No** (mint uses fixture/scenario amount) |
| `btc_deposit_addr` | Intent/watch only; not on claim public | Watch-only match at observe time; not re-checked at mint |
| `intent_id` (string) | Not a claim field; DEMO maps intent bind separately | **Loose** key for HTTP maps + automation only |
| `domain_bind` (watch/obs, Domain C intent) | `domain_binding` on claim (Domain B private-bridge tag) | **Different digests (D2)** — must not merge; no joint package from observation |
| Watch `dest_owner_binding` | `dest_commitment` / note `owner_binding` | Pure film yes; chain happy world synthetic; not from live watch after observe |
| `confirmations` | Reflection conf / K gates | Not mapped; snapshot confs are fixture heights |

### What `demo-corridor-ict` actually couples

```text
[regtest BTC fund + reporter] ──► hash-market observation (intent_id A)
                                          │
                                          │  wait status == deposit_observed
                                          ▼
                              (same shell, SKIP_WAIT later)
                                          │
[corridor_ict_funded] ──► BridgeL1World::happy() claim (synthetic ν)
                                          │  ◄── no GET of observation fields
                                          ▼
[mint-after-observe film] ──► automation phases keyed by intent_id A
                                          │  pure W0–W7 separate again
```

Evidence: `corridor-ict-funded.sh` runs observe block, then `cargo run … corridor_ict_funded` with **no** env pass of txid/amount, then `corridor-lab-mint-after-observe.sh` with `SKIP_WAIT=1` using the same `INTENT_ID` only for automation labels.

### Commands that prove observe plane (not claim continuity)

```bash
cargo test -p hash-market --lib corridor_deposits --features server
cargo test -p hash-market --lib btc_index --features server
# funded path (Docker):
just demo-corridor-ict   # observe green + mint green, but uncoupled identity
```

---

## 3. Gaps (goal vs code)

| ID | Gap | Severity | Notes |
|----|-----|----------|-------|
| **O-G1** | **No continuous claim identity from observation** | **P0** | `(txid, amount, addr, intent_id)` never construct `BridgeMintClaimPublic`. Stages share `intent_id` string only. PROMPT-COMMON residual: “Single continuous identity… may be sequenced but not fully coupled” — **confirmed uncoupled**. |
| **O-G2** | **Frozen rule for ν from BTC deposit missing on chain path** | **P0** | Pure film: ν := `btc_txid_or_intent_id` after `bind_deposit_id(burn_or_txid)`. Chain: hinge fixture ν. Need SSOT rule e.g. `H("terp-bridge-burn-v0" ‖ txid ‖ vout ‖ intent_id)` vs true Tacit burn ν. Cross-track with MINT (M11). |
| **O-G3** | **Amount not bound into mint value** | **P0** | Reporter posts true `amount_sats`; mint `value_u64` from fixture/scenario. Full e2e can observe 20_000 sats and mint unrelated value under mock_verify. |
| **O-G4** | **Funded e2e does not pass observation into mint binary** | **P0** | `corridor_ict_funded.rs` ignores hash-market entirely. Script sequencing ≠ dataflow. |
| **O-G5** | **One observation slot per intent (overwrite, no multi-UTXO)** | **P1** | Hub `observations: HashMap<intent_id, DepositObservation>` — second txid overwrites. Reporter dedups `intent:txid` locally but hub accepts overwrite. Ambiguous amount for claim. |
| **O-G6** | **No reorg / conf-regression revoke event** | **P1** | Once observed, status stays `deposit_observed`; no “un-observe”. Conf=1 lab finality risk. Review I-P1-1 residual. |
| **O-G7** | **Open observation auth (by design) without rate limit / shared secret** | **P1** | Anyone who knows intent + addr can POST. Lab spam / griefing of SSE. D4 open reporter — optional mesh secret still missing. |
| **O-G8** | **vout not in `DepositObservation`** | **P1** | `AddressFunding` has `vout`; observation type drops it. Prevents precise outpoint→ν binding and multi-output disambiguation. |
| **O-G9** | **Watch `dest_owner_binding` / `client_proof_digest` not echoed on observation or automation receipt as mint inputs** | **P1** | Preauth lives on watch; mint film does not load watch at claim build time on funded path. |
| **O-G10** | **No typed “claim material package” API** | **P1** | Missing e.g. `GET /corridor/claim-inputs/:intent_id` returning observation + watch fields needed for claim builder (still hint-only; UI/host re-verifies). |
| **O-G11** | **Public mainnet Esplora privacy/rate footgun** | **P2** | Reporter warns; packaging must prefer Fulcrum. Not blocking lab. |
| **O-G12** | **oline Fulcrum inventory packaging incomplete** | **P2** | Docs sketch only; lab uses bitcoind RPC. D4 “wire into oline” residual. |

### Direct answer to key question

| Mode | Continuous observation → claim identity? |
|------|------------------------------------------|
| **hash-market notify plane** | No — coordination + film phases only |
| **corridor-btc-reporter** | Posts observation fields; does not call mint |
| **Pure cashapp W0–W7** | **Yes inside harness** after `bind_deposit_id` (txid/burn → ν); **not** sourced from live `/corridor/observations` |
| **Daemon BridgeMintNote (ict)** | **No** — `BridgeL1World::happy()` / fixture |
| **demo-corridor-ict overall** | **Loose sequence** by shell order + `intent_id` labels |

---

## 4. Dependencies on other tracks

| Track | Dependency |
|-------|------------|
| **MINT-HEADSTASH** | Owns claim schema, ν once-set, `derive_claim_id_with_dest`, mock_verify vs real proof. OBSERVE must supply agreed inputs; MINT freezes hashing of deposit→ν. |
| **HARNESS-ICT** | Must thread observation (or re-verified outpoint) into `corridor_ict_funded` / suite instead of only `configure_bridge_happy()`. |
| **UI** | SSE already delivers observation; must pass txid/amount into reverify + any future claim-builder / automation body (UI-G4 / UI-G15 residuals). |
| **SWAP-DEX** | Needs mint note whose value/owner match intent; broken if mint value ≠ observed amount. |
| **ZAKURA-ZEC** | Dest preauth is on watch (`dest_owner_binding`); not OBSERVE’s chain egress. |
| **DOCS** | Must not claim continuous identity; USER-GUIDE residual D-G8. |
| **META** | Order: freeze ν rule (OBSERVE+MINT) → wire claim-inputs (OBSERVE) → harness mint from observation (HARNESS) → UI display/receipt. |

**Does not depend on:** live ZEC broadcast, private DEX on-chain settle, mainnet Cash App.

---

## 5. Recommended P0 slice for this track

**Smallest shippable OBSERVE slice toward continuous identity (still hint bus):**

1. **Extend `DepositObservation` (or parallel DTO)** with `vout: u32` (and keep txid/amount/addr/intent_id).
2. **Add `GET /corridor/claim-inputs/:intent_id`** (or enrich `WatchStatusResponse`) returning:
   - open `DepositWatch` (dest, domain_bind, min_amount, proof digest)
   - last `DepositObservation` (txid, vout, amount, confs)
   - explicit `identity_status: "uncoupled" | "fields_ready"` (honest label until mint binds)
3. **Document frozen provisional ν rule** (with MINT):  
   `nullifier = SHA256(b"terp-btc-deposit-nu-v0" || txid_bytes || vout_be || intent_id_bytes)`  
   (placeholder until Tacit burn ν exists — label as provisional).
4. **Script proof:** `corridor-ict-funded.sh` after observe, `curl` claim-inputs and assert `amount_sats` / `txid` match bitcoind send; print fields next to mint (even before harness consumes them).
5. **Do not** claim mint authority moved onto the bus.

Optional same-slice if tiny: reject observation overwrite when existing txid differs (or keep multi-observation history) — reduces amount ambiguity.

**Out of scope for OBSERVE P0 alone:** changing `BridgeMintNote` execute path (MINT/HARNESS); UI reverify wiring polish (UI).

---

## 6. Explicit non-claims

- OBSERVE does **not** mint notes or authorize balances (D4 / D7).
- Green `deposit_observed` + green `IsBridgeMinted` in one demo **does not** prove the same deposit identity.
- Pure harness `bind_deposit_id` continuity **≠** production notify→chain continuity.
- This report does **not** freeze the final BTC burn nullifier crypto (Tacit LC still open).
- Mainnet money, public Esplora mainnet watches, and full reorg-safe finality are **not** claimed ready.
- Automation `receipt` is film/coord JSON, not a cryptographic claim package.

---

## Appendix A — Type SSOT (paths)

| Type | Path |
|------|------|
| `DepositWatch` / `DepositObservation` / `DepositEvent` / hub | `/Users/returniflost/abstract/terp-core/crates/terp-rs/tools/hash-market/src/corridor_deposits.rs` |
| Reporter poll + post | `/Users/returniflost/abstract/terp-core/crates/terp-rs/tools/hash-market/src/btc_index/reporter.rs` |
| `AddressFunding` (has vout) | `/Users/returniflost/abstract/terp-core/crates/terp-rs/tools/hash-market/src/btc_index/mod.rs` |
| HTTP routes | `/Users/returniflost/abstract/terp-core/crates/terp-rs/tools/hash-market/src/server.rs` |
| Notify docs | `/Users/returniflost/abstract/terp-core/crates/terp-rs/tools/hash-market/docs/corridor-deposit-notify.md` |
| Reporter docs | `/Users/returniflost/abstract/terp-core/crates/terp-rs/tools/hash-market/docs/corridor-btc-reporter.md` |
| `BridgeMintClaimPublic` / `derive_claim_id_with_dest` | `/Users/returniflost/abstract/terp-core/crates/headstash/contracts/cw-headstash/src/bridge.rs` |
| Pure film ν binding | `/Users/returniflost/abstract/terp-core/crates/headstash/test-press/src/harness/cashapp_zec_corridor.rs` |
| Chain mint binary (no observe) | `/Users/returniflost/abstract/terp-core/crates/headstash/test-press/src/bin/corridor_ict_funded.rs` |
| Funded e2e script | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/e2e/corridor-ict-funded.sh` |
| Intent field goal | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/DEMO-CASHAPP-ZEC-CORRIDOR.md` §3.1 `btc_txid_or_intent_id` |

## Appendix B — One-line summary for META

**OBSERVE: notify plane green (watch/observe/SSE/reporter/amount gates); continuous deposit→BridgeMint claim identity absent — stages loosely sequenced by `intent_id` only.**
