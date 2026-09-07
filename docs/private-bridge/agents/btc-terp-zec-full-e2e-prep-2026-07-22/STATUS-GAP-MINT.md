# STATUS-GAP-MINT

| Field | Value |
|-------|--------|
| **Track** | MINT-HEADSTASH |
| **Date** | 2026-07-22 |
| **Mode** | Gap analysis only (no implementation) |
| **SSOT surfaces** | `cw-headstash` `bridge.rs` (`BridgeMintClaimPublic`, `BridgeMintNote`, `mock_verify`, `NoteOutResult`), `SeamNoteOutV0`, fixture `bridge_mint_claim_happy.v1.json` |

---

## 1. Goal for full BTC → Terp → ZEC (this track)

Full multi-net path requires **deposit-backed private mint** on Terp such that:

1. **Observe → claim identity coupling** — fields from a real BTC deposit observation (and the preauth watch) determine the mint claim, not a golden hinge fixture.
2. **Mint authority** — burn/reflection membership is proven or LC-certified (not self-asserted `in_burn_set` + `mock_verify`).
3. **Once-per-burn** — same ν cannot mint twice (`IsBridgeMinted` / `BRIDGE_MINTED`).
4. **Egress note** — successful mint yields a **DEX-consumable** `SeamNoteOutV0` (382B layout, `rcm_flag=1`, `owner_binding` = preauth dest) that later private swap can spend.
5. **Ownership continuity** — note `owner_binding` / openings remain bound to the intent owner so W3→W4 persist and W5+ swap auth do not desync from the deposit intent.

**Out of track (dependencies):** live ZEC egress (ZAKURA), on-chain private DEX settle (SWAP), reporter/watch product (OBSERVE), ict orchestration (HARNESS). This track owns **what must be on the claim wire and what mint does with it**.

---

## 2. Code reality today (cite paths / commands)

| Surface | Status | Evidence |
|---------|--------|----------|
| `BridgeMintClaimPublic` + pure gates | **Implemented** | `/Users/returniflost/abstract/terp-core/crates/headstash/contracts/cw-headstash/src/bridge.rs` — `authorize_bridge_mint_pure` (tip/K/lag, root pins, H-1, registry, dest domain, claim_id re-derive, domain_binding re-derive, rcm) |
| `ExecuteMsg::BridgeMintNote` | **Implemented** | `msg.rs` + `lib.rs` → `execute_bridge_mint_note`; attrs: `bridge_mint_note`, `note_out` (JSON `NoteOutResult`), `dex_consumable` |
| `IsBridgeMinted` / double mint | **Green (unit + L1 + ict)** | `BRIDGE_MINTED` key `bridge:02:{hex(ν)}`; suite double-mint reject; `corridor_ict_funded` asserts second mint fails |
| Happy fixture / L1 world | **Green, synthetic** | `happy_bridge_mint_world()`; `docs/plans/spectrum/fixtures/bridge_mint_claim_happy.v1.json`; `bridge_l1.rs` `BridgeL1World::happy()`; value **1_000_000**, labels `nu-hinge-happy`, `dest-commitment-A`, etc. |
| `mock_verify` path | **Default for demos** | `BridgeCfg.mock_verify`; `mock_verify_bridge_proof` — no SP1/LC guest; requires `in_burn_set` + non-empty proof when not `cfg!(test)` |
| Real LC / SP1 verify | **Not wired** | bridge.rs header + `mock_verify_bridge_proof` error: *"real proof verify not wired"*; D7 freeze accepts mock for lab/funded label |
| Reflection / burn roots | **Owner-set, not LC ingress** | `SetReflectionSnapshot` / `UpdateReflection` (cw_ownable); ict `configure_bridge_happy` writes fixture snapshot |
| Asset registry | **Internal map only** | `RegisterAsset` + `terp_asset_id_from_tacit`; external registry item is hook-only |
| Chain mint on ict | **Green (`demo-corridor-ict`)** | `test-press/src/bin/corridor_ict_funded.rs`: Daemon upload/instantiate → `configure_bridge_happy` → `BridgeMintNote` → `IsBridgeMinted=true`; `CORRIDOR_MOCK_VERIFY` default **true** |
| Observe → claim mapper | **Missing** | Funded script observes regtest deposit then **separately** runs fixture mint; claim does **not** take `txid` / `amount_sats` / watch `dest_owner_binding` |
| Pure film mint for swap spine | **Synthetic, intent-shaped** | `cashapp_zec_corridor.rs` `bridge_mint_for_intent`: ν := `btc_txid_or_intent_id`, dest := intent `dest_owner_binding`; still mock roots + `in_burn_set=true` |
| `SeamNoteOutV0` (382B) | **Fixture L0 green; chain emits JSON sketch** | `docs/plans/spectrum/fixtures/seam_note_out`; compose via `compose_bridge_mint_to_seam_bytes` / `from_bridge_mint`. Contract emits `NoteOutResult` (no `domain_tag` / `memo` fields) as event JSON, not fixed 382B |
| `put_note_after_mint` | **L2 product path (Mock)** | `note_persist_client.rs`; L2 suite after Mock mint. **`corridor_ict_funded` does not** parse chain `note_out` → encrypt → put; step 5 is pure W0–W7 with `CorridorScenario::default()` |
| Swap after chain mint | **Uncoupled pure film** | Same binary: mint nullifier/fixture ≠ swap film note identity |

### Commands / labels

| Command | Mint behavior |
|---------|----------------|
| `cd crates/headstash && just demo-corridor-ict` | Regtest observe + **fixture** Daemon `BridgeMintNote` + pure swap film; `mock_verify=true` labeled |
| `corridor-lab-mint-after-observe.sh` | Host automation phases; pure/mock mint film, not deposit-derived claim wire |
| PrivateBridgeSuite L1/L2 | Mock cw-orch: happy mint, double-mint, note persist |

---

## 3. Gaps (goal vs code)

| ID | Gap | Severity | Notes |
|----|-----|----------|-------|
| **M1** | **No deposit → `BridgeMintClaimPublic` builder** | **P0** | Observation has `txid`, `amount_sats`, `confirmations`; watch has `dest_owner_binding`, `domain_bind`. Claim needs 20+ fields (roots, ν, claim_id, domain_binding, rcm, cm_public, flags…). Nothing maps observe/watch → claim for chain execute. |
| **M2** | **Funded ict mint uses hinge fixture, not deposit amount/id** | **P0** | `BridgeL1World::happy()` / `happy_bridge_mint_world`: fixed `value_u64=1_000_000`, `nullifier=sha256("nu-hinge-happy")`. Regtest funds **20_000 sats** under a different `intent_id`. Mint is chain-real but **not deposit-backed**. |
| **M3** | **Membership is self-asserted bools under mock_verify** | **P0** (for “real deposit-backed mint trust”); **P2** under D7 lab freeze | `in_burn_set` / `in_pool_root` / `spent_only` on the claim stand in for IMT/LC. Anyone who can submit a well-formed claim with flags true + mock on can mint if owner pre-set snapshot/registry. |
| **M4** | **`mock_verify=false` has no production verifier** | **P0** for production posture; **accepted residual** for lab (D7) | `mock_verify_bridge_proof` rejects when `!mock_verify && !cfg!(test)`. No SP1 guest, no reflection certificate verify, no anvil dual-path. |
| **M5** | **Reflection snapshot is not LC-fed** | **P1** | Mint gates against `REFLECTION_SNAPSHOT` written by contract owner. Full path needs LC tip / burn-root updates from source reflection (SEAM-LC-STATE), not demo `SetReflectionSnapshot`. |
| **M6** | **Chain `NoteOutResult` ≠ full `SeamNoteOutV0` wire** | **P1** | Missing `domain_tag`, `memo`/`memo_flag` on chain JSON; no canonical 382B attribute. Product persist path rebuilds seam via L0 compose helpers, not by decoding chain egress alone. Risk of dual sources of truth for W4. |
| **M7** | **Owner binding not tied to watch in chain mint path** | **P0** for identity continuity | Fixture `dest_commitment` = `sha256("dest-commitment-A")`. Watch opens with placeholder `dest_owner_binding` (`"b"*64` in funded script). Pure film binds intent dest; **chain mint does not**. Later swap cannot treat chain-minted note as intent-owned without a remint or rebind policy. |
| **M8** | **`rcm` / openings not client-owned after chain mint** | **P1** | Happy path uses fixture rcm label. Real owner must hold trapdoor for DEX spend (`rcm_flag=1`). No handoff of openings from mint client to persist/swap when mint is Daemon-fixture. |
| **M9** | **No shared note tree `T` append on mint** | **P1** (compose with SWAP) | SEAM-NOTE-OUT NE-7 / phase: structural seam ok; DEX membership path under pool root not produced by `BridgeMintNote`. Spend still pure-film. |
| **M10** | **Value conservation vs BTC sats not enforced at claim build** | **P1** | Contract enforces `value_u64 != 0` and claim_id includes value; does **not** know BTC UTXO amount. Coupling amount_sats → value_u64 (and unit_scale) is host/mapper work (M1). |
| **M11** | **Nullifier identity for BTC deposit is undefined on chain path** | **P0** | Pure film uses `btc_txid_or_intent_id` as ν. Chain fixture uses independent hinge ν. Need frozen rule: e.g. domain-separated hash of `(txid, vout, intent_id)` or true Tacit burn ν once BTC confidential burn exists. |
| **M12** | **`cm_encoding = 0x03` abstract leaf on contract mint** | **P2** | `CM_ABSTRACT_LEAF_V0`; SEAM prefers Tacit keccak leaf for real bridge. Stub-ok for lab; real mint should freeze encoding with crypto path. |
| **M13** | **ict path does not `put_note_after_mint` from chain result** | **P1** | Blocks continuous film: observe → chain mint → encrypted note store → swap inputs from **that** note. |
| **M14** | **External asset registry / multi-asset production** | **P2** | Internal registry enough for ubtc lab; external addr is optional hook. |

### Claim field matrix — deposit reality vs mint requirement

| `BridgeMintClaimPublic` field | Required for authorize | Available from watch/observation today | Gap |
|------------------------------|------------------------|----------------------------------------|-----|
| `source_chain_tag` | yes (asset map) | partial (`bitcoin-*` policy) | Host constant ok |
| `tacit_asset_id` | yes (32B) | no | Need registry/policy id for corridor asset |
| `value_u64` | yes (>0, in claim_id) | **`amount_sats`** | Must map + unit_scale |
| `nullifier` | yes (once-per-ν key) | **`txid` (+ vout?) / intent** | Policy for ν derive missing on chain path |
| `dest_commitment` | yes (= burn dest) | **`dest_owner_binding`** | Must copy watch binding into claim |
| `dest_domain` | yes (= cfg) | weak (`domain_bind` string) | Must be 32B Terp dest domain pin |
| `claim_id` | re-derived | no | Host computes via `derive_claim_id_with_dest` |
| `source_pool_root` / `source_burn_root` | must = snapshot | no | LC/reflection or lab seed |
| `source_height` | must = snapshot | confs only | LC height vs BTC confs different objects |
| `domain_binding` | re-derived | partial (`domain_bind`) | Full C§3.1 inputs needed |
| `unit_scale` | asset map | no | Policy default `1` |
| `pool_domain` | pin | no | Corridor pool id |
| `cm_public` | 32B leaf | no | Client constructs dest leaf |
| `rcm` | for DEX (`flag=1`) | no | Client opening |
| `in_burn_set` / `in_pool_root` / `spent_only` | mock authority | no | Real proof replaces flags |
| `src_chain_id` / `dst_chain_id` / `lc_client_id` | domain_binding | no | Fixed labels or LC client |
| `burn_dest_commitment` | = dest_commitment | same as dest | From preauth |
| **proof** | non-empty under mock; real later | no | Mock blob or SP1 |

### mock_verify vs LC/proof (summary)

```
Today (D7 / ict default):
  claim flags + owner snapshot + mock_verify_bridge_proof(blob)
       → BRIDGE_MINTED + NoteOutResult event

Needed for deposit-backed trust:
  LC-updated (pool, spent, burn) roots + height/K/lag
       + membership proof under burn_root (and pool as required)
       + mock_verify=false path implemented
       + claim public values bound to observed deposit + preauth dest
```

### Note ownership for later swap (summary)

| Property | Goal | Today |
|----------|------|-------|
| `owner_binding` | = intent / watch `dest_owner_binding` | Fixture label on chain path; intent-bound only in pure film |
| `rcm_flag` | 1 with client-held rcm | Fixture rcm; not returned as client secret channel |
| DEX inputs | from **minted** seam (`to_dex_spend_inputs`) | Swap film invents own mint sketch via `bridge_mint_for_intent` |
| `nullifier_lineage` | bridge-burn ν (ingress marker) | Correct role; **must not** reuse as pool-spend ν (SEAM §4.2) |
| Persist | encrypt 382B after mint | L2 Mock only; not after ict Daemon mint |

---

## 4. Dependencies on other tracks

| Track | Dependency |
|-------|------------|
| **OBSERVE** | Stable observation + watch schema; amount gates; export fields mint mapper needs (`txid`, `amount_sats`, `dest_owner_binding`, `domain_bind`, confs). Intent_id continuity into mint receipt. |
| **HARNESS-ICT** | After M1: wire observe receipt into `BridgeMintNote` execute (fail closed if fields missing); stop using hinge-only world when claiming “deposit-backed”. Keep `mock_verify` banner. |
| **SWAP-DEX** | Consumes seam note openings + later tree membership; cannot settle on-chain until M6/M8/M9 and spend path exist. Pure W0–W7 remains film until then. |
| **UI** | Production path `deposit_observed` → build claim → mint → `put_note_after_mint`; must not imply chain mint is deposit-bound until M1/M2 closed. |
| **ZAKURA-ZEC** | Dest open feeds `dest_owner_binding` into watch **before** mint; mint must not invent a different dest. |
| **DOCS** | Keep D7 / `ict_local_funded` honesty: chain mint green ≠ deposit-backed mint. |

---

## 5. Recommended P0 slice for this track

**Smallest shippable slice (still mock_verify, still no SP1):**

1. **Define ν policy for corridor lab** (document + one pure function): e.g.  
   `ν = SHA256("terp-bridge-burn-ν-v0" ‖ intent_id_utf8 ‖ txid_hex ‖ amount_sats_be)`  
   (or txid+vout if multi-out) — freeze with OBSERVE/HARNESS.
2. **`claim_from_deposit_watch(obs, watch, corridor_policy) -> BridgeMintClaimPublic`**  
   - `value_u64` ← `amount_sats` (or scaled)  
   - `dest_commitment` / `burn_dest_commitment` ← decode watch `dest_owner_binding`  
   - `nullifier` ← ν policy  
   - re-derive `claim_id` + `domain_binding`  
   - lab: still set membership flags true; still use **seeded** reflection snapshot matching claim roots (honest label: lab reflection, not LC).
3. **Harness glue:** after `deposit_observed`, build claim → Daemon `BridgeMintNote` → assert `IsBridgeMinted(ν)` and **assert claim.value == obs.amount** (or documented scale).
4. **Note handoff:** map chain `NoteOutResult` (+ known rcm from client builder) → `SeamNoteOutV0` → `put_note_after_mint` so swap film can optionally load **that** note.

**Explicitly defer to later P0/P1 (not this slice):** SP1/LC verify (`mock_verify=false`), automatic LC `UpdateReflection`, tree `T` append, Tacit keccak `cm_encoding`.

---

## 6. Explicit non-claims

- This report does **not** claim mainnet BTC/ZEC or Tier-0 mint soundness.
- `just demo-corridor-ict` **does** prove live CosmWasm `BridgeMintNote` + double-mint reject on ict-rs Terp under **`mock_verify=true`** and **fixture claim** — it does **not** prove deposit-backed mint or note ownership continuity into swap/ZEC.
- D1–D7 freezes are **not** reopened: `SeamNoteOutV0` remains sole egress shape; demo mint may keep mock_verify until a later verify wire exists.
- Oracle / hash-market **never** mints (D7); observation is input to a **future** claim builder, not mint authority itself.
- Real SP1, reflection IMT, and anvil dual-container remain **out of green** for this track.
