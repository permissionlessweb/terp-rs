# STATUS-GAP-HARNESS

| Field | Value |
|-------|--------|
| **Track** | HARNESS-ICT |
| **Date** | 2026-07-22 |
| **Mode** | Gap analysis only (read-first) |
| **Surfaces** | `ict-rs`, `corridor-ict-funded.sh`, `corridor_ict_funded`, pure vs chain stages, `SKIP_REGTEST` residual |
| **North star** | Single continuous multi-net harness: BTC fund → observe → private mint on Terp → oracle-bound private swap → ZEC dest open/egress |

---

## 1. Goal for full BTC → Terp → ZEC (this track)

Harness must orchestrate **one continuous local multi-net workflow** where:

1. **BTC regtest** funds a deposit address bound to a watch/`intent_id`.
2. **Reporter** produces `deposit_observed` with amount/address gates (S6).
3. **ict-rs Terp + cw-orch Daemon** executes `BridgeMintNote` whose **claim identity is derived from that observation** (txid / amount / nullifier / domain binds — not a parallel happy fixture).
4. **On-chain or product-path private swap** consumes the **minted note** (not a separate pure film with zeroed identity).
5. **ZEC dest** is preauth-bound and eventually exercised as real local/regtest egress (coord ZAKURA; harness wires attach points).
6. Soft-skip of observe or chain mint is a **FAIL** for the full profile; residual mint-only remains explicitly labeled and non-default.

**What harness owns:** stage ordering, fail-closed gates, env flags, just targets, evidence logs, identity handoff between stages.  
**What harness does not own alone:** contract proof policy (MINT), live ZEC RPC (ZAKURA), UI film polish (UI), hash-market observation schema (OBSERVE).

---

## 2. Code reality today (cite paths / commands)

### Certified path: `just demo-corridor-ict`

| Surface | Status | Evidence |
|---------|--------|----------|
| One-command funded profile | **Green** (workflow shape) | `crates/headstash/justfile` → `demo-corridor-ict` → `docs/plans/spectrum/e2e/corridor-ict-funded.sh` |
| WASM prepare | **Green** | `prepare-corridor-ict-wasm.sh`; fail-closed if missing `cw_headstash.wasm` |
| hash-market host | **Green** | script starts/reuses `hash-market-server` at `HASH_MARKET_URL` (default `http://127.0.0.1:19090`) |
| BTC regtest fund | **Green** | `docker-compose.corridor-funded.yml` + `bitcoind` send + generate |
| Watch + S6 min_amount gate | **Green** | POST `/corridor/watches` with `min_amount_sats`; below-min observation rejected |
| Reporter → `deposit_observed` | **Green** | `corridor-btc-reporter` `backend=bitcoind`; poll watch until `deposit_observed` |
| ict-rs fresh Terp | **Green** | `corridor_ict_funded.rs` — `IctRuntime::Docker` + `CosmosChain` (`terpnetwork/terp-core:local-zk`) |
| Daemon `BridgeMintNote` | **Green** (happy fixture) | `PrivateBridgeSuite::configure_bridge_happy` + `execute_bridge_mint` + `IsBridgeMinted=true` + double-mint reject |
| Pure W0–W7 swap film (in binary) | **Green (film)** | `harness::run_cashapp_zec_corridor_w0_w7` + `CorridorAssetBackend::simulated()` unless `CORRIDOR_SKIP_SWAP_FILM` |
| Host automation film (post-binary) | **Green (film)** | `corridor-lab-mint-after-observe.sh` with `SKIP_WAIT=1`; phases PUT to `/corridor/automation/:id` |
| Soft-skip mint | **Fail-closed** | script: `\|\| fail "corridor_ict_funded (chain mint) failed — no silent skip"` |

### What `demo-corridor-ict` proves vs full multi-net

```text
PROVES (local multi-net shape):
  [BTC regtest] fund → reporter → deposit_observed          ✓ chain observe
  [Terp ict-rs] upload headstash → BridgeMintNote → query   ✓ chain mint (fixture claim)
  [Pure]        W0–W7 cashapp corridor + automation phases  ✓ film spine

DOES NOT PROVE (full BTC→Terp→ZEC):
  observe fields ──► BridgeMintClaim identity               ✗ uncoupled
  minted note ──► on-chain / product private swap           ✗ pure film only
  swap ──► live ZEC transfer / Zakura egress                ✗ not in harness
  continuous single intent through all stages               ✗ parallel identities
  mock_verify=false real proofs                             ✗ default true (labeled)
  mainnet money / Cash App                                  ✗ explicit non-goal
```

### Pure vs chain stages (stage map)

| Stage | Layer | Where | Input identity | Coupled to prior? |
|-------|-------|-------|----------------|-------------------|
| S-A Open watch | Host HTTP | `corridor-ict-funded.sh` | `INTENT_ID=intent-ict-funded-<ts>`; hex placeholders for domain_bind / dest / proof digest | N/A |
| S-B Fund BTC | Chain (regtest) | bitcoind `sendtoaddress` | real `DEPOSIT_ADDR`, `TXID`, 20_000 sats | Yes → watch addr |
| S-C Observe | Host + reporter | `corridor-btc-reporter` | watch reaches `deposit_observed` | Yes → S-A/S-B |
| S-D Chain mint | Chain (Terp Daemon) | `corridor_ict_funded` | **`BridgeL1World::happy()` fixture claim** | **No** — ignores `INTENT_ID`/txid/amount |
| S-E Pure swap (binary) | Pure / simulated | `run_cashapp_zec_corridor_w0_w7` | scenario defaults; `btc_txid_or_intent_id: [0u8; 32]` at W0 | **No** — not minted note |
| S-F Automation film | Host HTTP + pure tests | `corridor-lab-mint-after-observe.sh` | exports shell `INTENT_ID` for automation labels only; re-runs pure cargo tests | **Label-only** — does not re-execute chain mint |
| S-G ZEC egress | — | not in funded script | — | **Missing** |

**Key code facts:**

- `corridor-ict-funded.sh` exports `INTENT_ID` only into the **automation film** (`export INTENT_ID` before `corridor-lab-mint-after-observe.sh`). It does **not** pass intent/txid/amount into `cargo run … corridor_ict_funded`.
- `corridor_ict_funded.rs` reads env for Docker image, mnemonic, `CORRIDOR_MOCK_VERIFY`, `CORRIDOR_SKIP_SWAP_FILM`, `KEEP_CHAIN` — **no** `INTENT_ID`, observation JSON, or deposit claim builder.
- Mint path: `configure_bridge_happy()` → `BridgeL1World::happy()` synthetic claim/proof (`private_bridge.rs`).
- Swap path after mint: simulated backend + default `CorridorScenario` — independent pure spine for D3 film, not note-from-mint settle.
- Mint-after-observe script is honest about lab film: pure tests + automation PUT; `honest_non_claims` include no mainnet ZEC send; Zakura residual.

### `SKIP_REGTEST` residual

| Path | Env | Behavior | Role |
|------|-----|----------|------|
| Default full funded | unset / `0` | regtest + observe **required** | S1 profile |
| Skip without allow | `SKIP_REGTEST=1` only | **FAIL** (`funded profile requires observe`) | guard |
| Mint-only residual | `SKIP_REGTEST=1` + `CORRIDOR_ALLOW_MINT_ONLY=1` | warn + skip observe; still runs chain mint + film | **dev residual** |
| just alias | `demo-corridor-ict-mint-only` | sets both env vars | labeled residual |

Residual is intentional and documented (`CORRIDOR-LAB-STATUS.md`, justfile comment “Dev residual if observe not required”). It remains a **gap for full multi-net** if operators treat mint-only as the funded green path — default `demo-corridor-ict` still runs observe.

### Related just / scripts

| Command | Profile |
|---------|---------|
| `cd crates/headstash && just demo-corridor-ict` | full funded shape (observe + mint + film) |
| `just demo-corridor-ict-mint-only` | chain mint only residual |
| `just demo-corridor-lab` | lab_simulated floor (not this track’s bar) |
| `just demo-zakura-local*` | ZEC dest binding offline — not joined into funded script |

---

## 3. Gaps (goal vs code)

| ID | Gap | Severity | Notes |
|----|-----|----------|-------|
| **H-G1** | **Observe → mint identity uncoupled** | **P0** | Funded `TXID` / amount / `INTENT_ID` / watch binds never feed `BridgeMintClaimPublic`. Mint uses `BridgeL1World::happy()`. Stage sequence ≠ continuous identity (also noted in final-sprint `STATUS-META-REVIEW.md`). |
| **H-G2** | **Chain mint → pure swap uncoupled** | **P0** | After `IsBridgeMinted`, binary runs **simulated** W0–W7; no note/nullifier handoff from Daemon mint into swap. Second pure run in `corridor-lab-mint-after-observe.sh` duplicates film. |
| **H-G3** | **No ZEC chain stage in funded harness** | **P0** (coord ZAKURA) | `demo-corridor-ict` ends at pure film + automation `complete`. No Zakura attach, no local zcashd/lightwalletd egress step in `corridor-ict-funded.sh`. |
| **H-G4** | **Dual pure films after chain mint** | **P1** | Binary step [5/6] W0–W7 **and** host script pure cargo tests — both pure; neither settles minted note. Noise for “what green means.” |
| **H-G5** | **`SKIP_REGTEST` mint-only residual** | **P1** | Guarded (`CORRIDOR_ALLOW_MINT_ONLY`) and just-aliased; still a documented escape that can be mistaken for full S1 if STATUS/docs drift. Full multi-net must keep residual non-default and evidence-labeled. |
| **H-G6** | **Watch cryptographic binds are placeholders** | **P1** | Funded script sets `domain_bind`/`dest_owner_binding`/`client_proof_digest` to repeated hex (`"a"*64` etc.), not real preauth binds from UI/product. Fidelity gap for production-shaped reverify. |
| **H-G7** | **No harness export of mint evidence into automation** | **P1** | Automation receipt labels `headstash_contract` via env default `cw-headstash-ict-local`; binary prints real contract address but does not write it into automation PUT / receipt JSON. UI poll does not show live chain mint tx identity. |
| **H-G8** | **ict-rs chain torn down before automation** | **P2** | Default `KEEP_CHAIN` off: Terp stops before mint-after-observe film. Correct for pure film; blocks any future “query chain then film” without `KEEP_CHAIN=1` + address plumbing. |
| **H-G9** | **`mock_verify=true` default on funded deploy** | **P2** (MINT policy) | Labeled D7 residual; harness wires `CORRIDOR_MOCK_VERIFY`. Full e2e soundness needs proof path — not harness-only. |
| **H-G10** | **No single process / single artifact “continuous film”** | **P2** | Three processes: bash observe, rust binary mint+pure, bash automation pure. Harder to assert one identity trace end-to-end. |

### Severity summary

- **P0 for continuous multi-net identity:** H-G1, H-G2, H-G3 (H-G3 shared with ZAKURA).
- **P1 fidelity / residual hygiene:** H-G4–H-G7.
- **P2 packaging / policy:** H-G8–H-G10.

---

## 4. Dependencies on other tracks

| Track | Why harness depends |
|-------|---------------------|
| **OBSERVE** | Observation payload schema → claim field mapping; amount/address gates already in funded path; need stable API for “export deposit claim inputs.” |
| **MINT-HEADSTASH** | `BridgeMintNote` claim layout, SeamNoteOutV0 product path, mock_verify vs real proof; fixture vs deposit-derived claim builders. |
| **SWAP-DEX** | Path from minted note → private swap settle (on-chain or product) replacing pure-only W0–W7 as the funded success criterion. |
| **ZAKURA-ZEC** | Dest binding attach + local ZEC receive/egress stages to append after swap (or after mint if dest-open-only). |
| **UI** | Automation phase semantics when chain mint evidence is real; fail-closed reverify for `ict_local_funded`. |
| **DOCS** | Must not describe `demo-corridor-ict` as full BTC→ZEC settlement; residual mint-only wording. |
| **META** | Build order: identity handoff before joining ZEC live. |

Harness can **sequence** stages today; it cannot **close** continuous multi-net without OBSERVE→MINT claim builder and SWAP/ZAKURA product paths.

---

## 5. Recommended P0 slice for this track

**Smallest shippable harness slice toward continuous multi-net:**

1. **Identity handoff v0 (H-G1)**  
   - After `deposit_observed`, fetch watch/observation JSON from hash-market.  
   - Pass into `corridor_ict_funded` via env/file: `INTENT_ID`, `btc_txid`, `amount_sats`, deposit addr, and any available binds.  
   - Build / select mint claim from those fields (or fail closed if unmappable) — **not** silent fallback to happy fixture in funded profile.  
   - Happy fixture remains lab/L1 only.

2. **Evidence plumbing (H-G7 partial)**  
   - Emit `headstash_contract`, `nullifier`/mint tx id, `chain_id` to a known artifact path; feed automation receipt so UI poll reflects chain mint, not only pure film labels.

3. **Do not expand scope in P0:** live ZEC (ZAKURA), real Halo2 proofs (MINT), on-chain swap settle (SWAP) — but **document** pure swap as residual after identity handoff lands.

**Explicit non-P0 for harness alone:** replacing pure W0–W7 with full private DEX chain settle; that is SWAP-DEX + MINT product work with harness wiring after.

**Residual policy (H-G5):** keep `demo-corridor-ict-mint-only` / `CORRIDOR_ALLOW_MINT_ONLY` as **dev-only**; full multi-net STATUS must treat observe-skip as incomplete.

---

## 6. Explicit non-claims

- `just demo-corridor-ict` **OK ict_local_funded** ≠ full BTC → Terp → ZEC multi-net settlement.
- Chain `BridgeMintNote` on ict-rs **≠** mint claim bound to the funded regtest deposit.
- Pure cashapp W0–W7 + automation phases **≠** on-chain private swap or ZEC egress.
- `SKIP_REGTEST` + `CORRIDOR_ALLOW_MINT_ONLY` **≠** funded S1 exit.
- `BridgeCfg.mock_verify=true` (default) **≠** Tier-0 soundness.
- This track does **not** claim mainnet Cash App, mainnet BTC, or mainnet ZEC.
- D1–D7 freezes are not amended by this gap report.

---

## Appendix A — Evidence anchors (absolute paths)

| Piece | Path |
|-------|------|
| Orchestration epic | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/agents/btc-terp-zec-full-e2e-prep-2026-07-22/ORCHESTRATION.md` |
| Funded script | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/e2e/corridor-ict-funded.sh` |
| Mint-after-observe film | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/e2e/corridor-lab-mint-after-observe.sh` |
| L3 binary | `/Users/returniflost/abstract/terp-core/crates/headstash/test-press/src/bin/corridor_ict_funded.rs` |
| PrivateBridge suite | `/Users/returniflost/abstract/terp-core/crates/headstash/test-press/src/suites/private_bridge.rs` |
| Pure W0–W7 | `/Users/returniflost/abstract/terp-core/crates/headstash/test-press/src/harness/cashapp_zec_corridor.rs` |
| just targets | `/Users/returniflost/abstract/terp-core/crates/headstash/justfile` (`demo-corridor-ict`, `demo-corridor-ict-mint-only`) |
| Profile status SSOT | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/e2e/CORRIDOR-LAB-STATUS.md` |
| Prior harness STATUS | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/agents/final-sprint-2026-07-22/STATUS-HARNESS-OBSERVE.md` |

## Appendix B — Stage graph (current default)

```text
corridor-ict-funded.sh
  │
  ├─[chain BTC]── regtest fund + reporter ──► deposit_observed(intent_A)
  │                                              │
  │                                              │  (no claim fields passed)
  │                                              ▼
  ├─[chain Terp]─ corridor_ict_funded ── BridgeMintNote(fixture_claim)
  │                    │
  │                    ├─ pure W0–W7 (simulated, intent zeros)
  │                    └─ stop chain (unless KEEP_CHAIN)
  │
  └─[pure/host]── mint-after-observe(SKIP_WAIT=1, label intent_A)
                       pure cargo tests + automation PUT film
                       (no ZEC egress)
```

**Target continuous graph (goal):**

```text
intent_I → fund → observe(I, tx, amt)
        → BridgeMintNote(claim←observe) → note_N
        → private swap(note_N) → dest_ZEC open/egress
        → automation/receipt carry I + N + zec_txid
```
