# SPEC — Tacit Bridge SSOT Mapping (Domain B)

> **Status:** draft freeze candidate (2026-07-20)  
> **Domain:** B — Tacit bridge SSOT mapping for Terp private-bridge ingress  
> **SSOT:** `crates/tacit` (ConfidentialPool + reflection; **not** legacy tETH)  
> **Parent plan:** `daily-driver/plans/terp-private-shielded-dex-bridge/PLAN.md`  
> **Consumers:** LC implementers, CosmWasm note-mint contracts, circuit authors  
> **Non-authority:** this doc does not invent opcodes; it maps Tacit surfaces into Terp LC + note actions.

---

## 0. Purpose

Map Tacit's private Bitcoin DEX+bridge design (v1 Ethereum confidential lane) into a **Terp-facing interface map** for deterministic, authenticatable private bridging.

An implementing agent must be able to:

1. Identify which Tacit surface is mirrored  
2. Know which trust tier applies  
3. Know which **public values** a light client / proof must surface  
4. Apply a **Terp LC / contract action** that mints or burns a private note  
5. Reject unauthentic or non-conserving mints  

**Threading rule (from program plan):** do not invent parallel note/bridge opcodes. If a Terp feature cannot name its Tacit surface, stop and realign.

**Canonical bridge path:** ConfidentialPool + SP1 reflection (`bridge_mint` / `bridge_burn`).

**Team clarity (burn/mint vs escrow, ZEC/BTC):** [`CLARITY-headstash-sets-and-bridge-models.md`](./CLARITY-headstash-sets-and-bridge-models.md) §2 — reflection is conservation by proof, not custodial multisig escrow; ZEC-out still needs lock/burn accounting; north star is IBC-class Bitcoin ↔ Zcash.  
**Legacy (non-target):** `BRIDGE.md` / `TacitBridgeMixer` tETH — recovery and fixed-denom anonymity lessons only.

---

## 1. Tacit surfaces relevant to private bridge

These are the load-bearing surfaces for **private bridge ingress** into Terp. Surfaces marked *secondary* are in scope for honesty/tests but not the primary mint path.

| # | Surface | Tacit locus | Role for Terp private bridge |
|---|---------|-------------|------------------------------|
| S1 | **Confidential note model** | `spec/SPEC-CONFIDENTIAL-POOL.md` §2–4 | Shared language: Pedersen `C = v·H + r·G`, leaf, nullifier `ν`, multi-asset anonymity set |
| S2 | **ConfidentialPool settle** | `contracts/src/ConfidentialPool.sol` `settle`, `spec/SPEC-CONFIDENTIAL-OPS.md` | Universal state transition: wrap/transfer/unwrap/swap/LP + bridge ops under one SP1 proof |
| S3 | **`OP_BRIDGE_BURN` (3)** | Confidential ops table; guest + `crossOutCommitment` | Source-chain burn: note → `cross_out` record (`claimId`, `destCommitment`, `ν`, `assetId`, `destChain`) |
| S4 | **`OP_BRIDGE_MINT` (4)** | Confidential ops table; B5 gate | Dest-chain mint: confirmed source burn → new note, once per burned `ν` / claim |
| S5 | **Bitcoin reflection prover** | `SPEC-BITCOIN-REFLECTION-AMENDMENT.md` | Trustless **data** relay: proves `(bitcoinPoolRoot, bitcoinSpentRoot, bitcoinBurnRoot, height)` onto destination |
| S6 | **Bridge-burn set (distinct from spent set)** | `knownBitcoinBurnRoot` vs `knownBitcoinSpentRoot` | **H-1 invariant:** only *bridge* burns authorize mint; ordinary spends do not |
| S7 | **Finality / confirmation gate** | `REFLECTION_CONFIRMATIONS`, `REFLECTION_FINALITY_WINDOW` | Reorg-safe maturity before mint may act on a reflected burn |
| S8 | **Asset registry + `crossChainLink` / `localAssetOf`** | Pool registration, `CanonicalAssetFactory` | Domain-separated `asset_id` resolution across lanes |
| S9 | **`OP_WRAP` / `OP_UNWRAP`** | Confidential ops 0 / 2 | Public escrow ↔ note boundary (same-chain shield/unshield; not cross-chain by itself) |
| S10 | **`OP_TRANSFER` / settle kernel** | Confidential ops 1; conservation | In-pool private movement; conservation template for all ops |
| S11 | **AMM settle (`OP_SWAP`, `OP_LP_*`)** | Confidential ops 6–8, 11; `AMM.md` | Post-bridge DEX; public reserves + private amounts (not mint path) |
| S12 | **`OP_CBTC_MINT` (18)** | Conservation peg vs lock | **cBTC-style:** mint only against proven locked sats; oracle-free |
| S13 | **`OP_CDP_MINT` (15) / cUSD** | Controller + oracle policy | **cUSD-style:** pricing/ratio may use oracle; **oracle never mints balances** |
| S14 | **Cross-lane gate (B4)** | Spent-set non-membership for Bitcoin-homed notes | Prevents double-spend across lanes; value exit requires bridge path |
| S15 | **Legacy tETH mixer** | `BRIDGE.md`, ops `0x60–0x65` | **Sunset.** Recovery / fixed-denom anonymity lessons only — **not** new Terp ingress |
| S16 | **Bitcoin envelopes reflected** | `T_CXFER_BPP` `0x22`, conf-burn / cross-out | Source-side wire; reflection binds existing envelopes, adds no new Bitcoin opcode for bridge |

### 1.1 Primary ingress path (Terp should mirror)

```text
Source chain (Tacit Bitcoin or EVM lane)
  note spend + OP_BRIDGE_BURN / conf-burn
       │
       │  public: ν, asset_id, destCommitment, claimId, value (opening-bound)
       ▼
Reflection / LC (SP1 today; Terp LC + membership tomorrow)
  prove: burn ∈ bridge-burn set
         membership of burned note in pool root
         height ≥ finality (REFLECTION_CONFIRMATIONS spirit)
         v_mint == v_burn (conservation; owner opening)
       │
       ▼
Destination (Terp private note set)
  one mint per claimId/ν
  append leaf; bind asset_id via domain map
  nullifier of burn marked spent (or claim consumed)
```

### 1.2 Explicit non-primary surfaces

| Surface | Why not primary ingress |
|---------|-------------------------|
| tETH `deposit` → `T_BRIDGE_DEPOSIT` | Sunset; separate anonymity set + relay |
| Bitcoin mixer `T_DEPOSIT`/`T_WITHDRAW` | Secondary privacy; fixed denom; not multi-asset bridge |
| Oracle / VE mid | Quote bounds only — never balance mint |
| Worker KV | Liveness/discovery only — never soundness |

---

## 2. Trust tier table (per surface)

Source of truth: `crates/tacit/spec/design/TRUST-TIERS-AND-CONVERGENCE.md`.  
**Terp rule:** do not claim Tier 0 for a feature that is Tier 1/2 on Tacit without a proof path.

| Surface | Tacit tier | Basis (Tacit) | Terp LC posture |
|---------|------------|---------------|-----------------|
| Confidential pool settle (CXFER/AXFER/SWAP/LP/OTC/BID on EVM) | **0** | SP1 proof of op loop; amounts bound by opening sigmas | In-VM / LC-gated verify of settle public values (or equivalent proof) |
| Reflection bridge mint (burn → mint) | **0** | Dedicated burn set + `v_mint == v_burn` + relay-anchored confirmations ≥ `REFLECTION_CONFIRMATIONS` | LC proves source finality + burn membership; contract one-shot claim |
| `OP_BRIDGE_BURN` / cross-out record | **0** | Guest + on-chain `claimId` re-derivation; storage-backed for reverse reflection | Emit claim record verifiable by counterparty LC |
| Note nullifier / membership | **0** | Note-bound `ν`; tree membership private path | Same invariants in Terp note set |
| Wrap / unwrap escrow binding (B1/B2) | **0** | Escrow amount ↔ note `v` via `unitScale` | Escrow or IBC voucher must bind `v` |
| EVM-AMM LP add / settle AMM | **0** | On-chain pool state + SP1 | Prefer proven pool state (not worker reserves) |
| Bitcoin-side AMM (`T_LP_*` / worker reserves) | **1** | Kernel conservation yes; reserves worker-attested unless client replay | Do not use as sole mint authority |
| tETH bridge (`0x60–0x65`) | **0/1** (separate surface) | Mixer + SP1 + relay; **sunset** for new product | Recovery only |
| cBTC.zk mint (`OP_CBTC_MINT` / lock fold) | **0 (mint)** | Note bound 1:1 to confirmed lock `v_btc` | Conservation mint only |
| cBTC.zk peg *backing* | **2** | Self-custody reclaim detectable, not preventable at EVM | Document; buffer/insurance separate |
| cBTC.tac bond / buffer | **2** | Governable coverage | Not a mint path |
| cUSD CDP (`OP_CDP_*`) | **0** for conservation ops; **policy** for ratio | Controller prices health; does not invent free debt without collateral rules | Oracle bounds health — **never** free mint |
| Worker / indexer cache | **n/a (non-tier)** | Liveness only | Fail closed without chain-derived state |

### 2.1 Tier reading guide for implementers

- **Tier 0 claim** → must be enforced by proof/signature + chain state an honest client can re-derive.  
- **Tier 1 claim** → worker/indexer attests aggregate; acceptable only as pilot, never as mint authority.  
- **Tier 2 claim** → insured/detected, not protocol-prevented; label honestly in product copy.

---

## 3. Mapping table

**Legend**

- **Tacit op/concept** — SSOT name (opcode / storage / public value)  
- **Proven public values** — what must be verifiable without private witnesses  
- **Terp LC / contract action** — what Terp does when public values check out  
- **Private note effect** — note-set delta  

### 3.1 Bridge core

| Tacit op/concept | Proven public values | Terp LC / contract action | Private note effect |
|------------------|----------------------|---------------------------|---------------------|
| Note commitment `C = v·H + r·G` | Leaf hash (owner-free: `keccak(asset_id ‖ Cx ‖ Cy)` for cross-chain); range via BP+ in settle | Accept leaf format compatible with Tacit secp note language (or proven encoding of same fields) | Creates/spends note with hidden `v` |
| Nullifier `ν = keccak(Cx ‖ Cy ‖ "spent")` | `ν` revealed on spend; uniqueness | Reject if `ν` already in Terp spent set; insert on accept | Marks note spent once, ever |
| `OP_BRIDGE_BURN` / conf-burn | `claimId`, `destChain`, `destCommitment`, `ν`, `assetId`; burned note membership on source; conservation kernel | Record claim or await foreign LC update; never mint locally without dest path | Source: nullify burned note; emit cross-out; no local value remain |
| `claimId` binding | `claimId = keccak(destChain ‖ destCommitment ‖ ν ‖ assetId)` (must re-derive on-chain) | Recompute `claimId` from fields; reject malleated ids | Binds one mint destination to one burn |
| `OP_BRIDGE_MINT` | `ν ∈ bitcoinBurnRoot` (bridge-burn set, **not** generic spent set); burned note ∈ `bitcoinPoolRoot` / source pool root; `v_mint == v_burn` (opening); `!bridgeMinted[ν]`; root == current reflected root; height mature | **LC:** verify source headers/finality + membership of burn. **Contract:** one-shot consume claim; map `asset_id`; mint note leaf | Dest: append new note with `v = v_burn`, same logical asset |
| Reflection public values | `(bitcoinPoolRoot, bitcoinSpentRoot, bitcoinBurnRoot, bitcoinHeight[, digest])` | LC client state stores attested roots; advance only monotonically with finality | Enables mint membership checks |
| `REFLECTION_CONFIRMATIONS` | Tip buried ≥ K confirmations before mint acts | LC `trusting_period` / confirmation depth param; fail closed if immature | No note until mature |
| `knownBitcoinBurnRoot` gate | Non-zero current burn root; mint batch pins it | Reject mint against zero/stale burn root | Prevents spend-set mint forgery (H-1) |
| Cross-lane B4 (spent non-membership) | Bitcoin-homed spend proves `ν ∉ bitcoinSpentRoot` | If Terp supports dual-home notes: same gate; else force all value exits through bridge_mint path | Blocks double-spend across lanes |
| `bridgeMinted[ν]` / claim once | Mapping true after first mint | Persistent claim bitmap / nullifier of claim | At most one dest note per burn |
| Reverse path ETH→BTC (`crossOutCommitment`) | Storage slot `claimId → destCommitment`; eth-reflection completeness (`crossOutCount`) | Symmetric LC: prove dest-side cross-out for return leg | Mirror mint on return chain |

### 3.2 Escrow / wrap boundary (same-chain, supports bridge assets)

| Tacit op/concept | Proven public values | Terp LC / contract action | Private note effect |
|------------------|----------------------|---------------------------|---------------------|
| `OP_WRAP` | Escrow amount; `deposit_id` binds scaled `value`; leaf append | Shield: lock public denom → note | Mint note `v = amount / unitScale` |
| `OP_UNWRAP` | Opening of `v`; payout `v · unitScale` | Unshield: burn note → public denom | Nullify note; public credit |
| `unitScale` | Registered per asset; first-write-wins on Tacit | Freeze scale in asset registry; reject misaligned amounts | Amount integrity across decimal domains |

### 3.3 Asset identity

| Tacit op/concept | Proven public values | Terp LC / contract action | Private note effect |
|------------------|----------------------|---------------------------|---------------------|
| Bitcoin `asset_id` | `SHA256(reveal_txid ‖ vout_LE)` (etch-bound) | Map into Terp `asset_id` via domain table (§5) | Note carries mapped id |
| EVM wrapped id | `sha256("tacit-evm-token-v1" ‖ chainid_be8 ‖ underlying)` | Registry entry: underlying + chain + scale | Same |
| EVM etch id | `sha256("tacit-evm-etch-v1" ‖ chainid ‖ factory ‖ salt ‖ etcher ‖ meta_hash)` | Optional for native Terp-issued assets | Same |
| `crossChainLink` / `localAssetOf` | Shared Bitcoin id ↔ local registry key | `AssetMap` contract/module: foreign id → Terp denom/asset_id | Bridge mint uses shared id; unwrap resolves local |
| `CBTC_ZK_ASSET_ID` | Domain const `keccak("tacit-cbtc-zk-lock-v1")` (pinned) | Fixed map entry for cBTC lock family | Conservation peg asset only |
| cUSD debt asset | `keccak("tacit-cdp-debt-v1" ‖ controller)` | Separate from bridge conservation assets | CDP debt notes; not bridge-minted free value |

### 3.4 Post-bridge private DEX (compose after mint; not mint authority)

| Tacit op/concept | Proven public values | Terp LC / contract action | Private note effect |
|------------------|----------------------|---------------------------|---------------------|
| `OP_TRANSFER` | Nullifiers, new leaves, kernel conservation | Private send within Terp set | n→m notes, hidden amounts |
| `OP_SWAP` / AMM settle | Public reserve Δ; private swap amounts in proof | Apply reserve Δ only from valid proof | Spend in-asset notes; mint out-asset notes |
| `OP_LP_ADD` / `OP_LP_REMOVE` | Public reserves/shares (EVM Tier 0) | Same | LP-share notes |
| Bitcoin AMM `T_SWAP_VAR` / batch | Public Δ; Tier 1 worker risk | Prefer EVM/Terp proven path for demo | — |
| Oracle / hashmerchant VE mid | Mid, staleness, quorum | `min_out ≥ g(oracle, slippage)` only | **No balance mint** |

### 3.5 Conservation peg vs oracle-priced (do not blur)

| Concept | Mint authority | Oracle role | Terp rule |
|---------|----------------|-------------|-----------|
| **cBTC-style** (conservation peg) | Proven lock / burn / escrow | **None** in mint path | LC + conservation only |
| **cUSD-style** (CDP) | Collateral note + controller health | Prices collateral / ratio | Oracle may **reject unhealthy**; never **credit free debt** |
| **Bridge mint** | Source burn + reflection | **None** | Same as cBTC-style conservation |

### 3.6 Legacy mapping (do not implement as new path)

| Legacy Tacit | If seen | Terp action |
|--------------|---------|-------------|
| `TacitBridgeMixer.deposit` / `T_BRIDGE_DEPOSIT` | Historical notes | Recovery tooling only; optional fixed-denom anonymity research |
| `withdrawFromBurn` + Groth16 burn proof | Legacy exit | Do not wire as default LC ingress |
| Fixed denomination pools | Privacy property | May inspire optional ladder later; not required for ConfidentialPool map |

---

## 4. Deterministic authentication properties

A Terp **bridge ingress mint** is authentic only if **all** of the following hold. Any failure → **reject** (fail closed).

### 4.1 Source existence and finality

| ID | Property | Check |
|----|----------|-------|
| A1 | **Source inclusion** | Burn tx / settle event is included in a block that the LC considers canonical |
| A2 | **Confirmation depth** | Source height is mature: ≥ `K` confirmations (Tacit: `REFLECTION_CONFIRMATIONS`; mainnet-like default **6**; deployment-parameterized; max bound like Tacit 144) |
| A3 | **Monotonic client state** | LC roots/heights only advance; no rollback of spent/burn sets without explicit reorg policy (v1: append-only / reject rewind) |
| A4 | **Header / work honesty** | LC verifies headers (or SP1/reflection proof equivalent); no self-supplied min-difficulty forge path |

### 4.2 Burn authenticity (H-1 class)

| ID | Property | Check |
|----|----------|-------|
| A5 | **Bridge-burn membership** | Burned `ν` is a member of the **bridge-burn set**, not merely the generic spent set |
| A6 | **Destination binding** | Burn record pins `destCommitment` (or Terp dest leaf material) for **this** mint |
| A7 | **Source pool membership** | Burned note was a member of the source pool root at burn time (no free-float `ν`) |
| A8 | **Envelope / op class** | Op is a bridge burn (`OP_BRIDGE_BURN` / conf-burn), not ordinary transfer/spend |

### 4.3 Conservation and opening

| ID | Property | Check |
|----|----------|-------|
| A9 | **Value conservation** | `v_mint == v_burn` (and per-asset; no cross-asset relabel) |
| A10 | **Range** | `v ∈ [0, 2⁶⁴)` (BP+ or equivalent) |
| A11 | **Opening authority** | Prover knows blinding of burned note (only owner can complete mint path that opens `v`) |
| A12 | **No oracle mint** | No price feed, VE, or mid is an input to `v_mint` |

### 4.4 One-shot and domain

| ID | Property | Check |
|----|----------|-------|
| A13 | **Once-per-burn** | `claimId` / `ν` not previously consumed on Terp (`bridgeMinted` analogue) |
| A14 | **Claim binding non-malleable** | On-chain re-derivation: `claimId = f(destChain, destCommitment, ν, assetId)` matches witness |
| A15 | **Asset domain map** | `asset_id` resolves via frozen domain map (§5); unknown id → reject |
| A16 | **Chain / pool domain separation** | Destination chain id / Terp port id / pool address bound into claim or domain tag (anti-replay across deployments) |
| A17 | **Root currency** | Membership proven against **current** attested burn/pool root (stale root → reject) |

### 4.5 What is *not* sufficient alone

- Worker saying “this burn is valid”  
- Generic spent-set membership without bridge-burn set  
- Shallow confirmation (1-block) tip  
- Matching ticker string without `asset_id` derivation  
- Oracle mid “worth” of collateral  

---

## 5. Domain binding / `asset_id` mapping requirements

### 5.1 Goals

1. One logical asset → stable Terp `asset_id` (or denom) across ingress.  
2. No squatting: foreign ids cannot remap local payouts.  
3. Decimal safety via explicit `unitScale` (or Terp base units).  
4. Separation of **conservation assets** (bridge/cBTC) from **CDP debt assets** (cUSD).

### 5.2 Domain tags (normative for Terp map)

Terp MUST domain-separate imported Tacit ids:

```text
terp_asset_id = H(
  "terp-tacit-asset-v1"
  ‖ source_chain_tag          // e.g. "bitcoin-mainnet" | "eip155:1" | "eip155:11155111"
  ‖ tacit_asset_id_bytes32    // as on Tacit wire / pool
  ‖ unit_scale_be             // or fixed registry version
)
```

Optional pool pin (recommended for multi-pool futures):

```text
‖ pool_domain                 // ConfidentialPool address or Bitcoin pool genesis digest
```

### 5.3 Source `tacit_asset_id` families (do not collapse)

| Family | Derivation (Tacit) | Map notes |
|--------|--------------------|-----------|
| Bitcoin etch | `SHA256(reveal_txid ‖ vout_LE)` | Primary Bitcoin-native assets |
| EVM wrapped / native underlying | `sha256("tacit-evm-token-v1" ‖ chainid_be8 ‖ underlying)` | Includes native ETH sentinel `underlying = 0` |
| EVM etch | `sha256("tacit-evm-etch-v1" ‖ …)` | Metadata-bound |
| cBTC.zk lock | pinned `CBTC_ZK_ASSET_ID` | Conservation only; Tier 2 backing honesty |
| CDP debt | `keccak("tacit-cdp-debt-v1" ‖ controller)` | **Never** bridge-mint as free value |

### 5.4 Registry requirements on Terp

| Field | Required | Notes |
|-------|----------|-------|
| `tacit_asset_id` | yes | 32-byte foreign id |
| `source_chain_tag` | yes | Domain |
| `terp_asset_id` / denom | yes | Local |
| `unit_scale` | yes | Align 8-dec Tacit ↔ 6/8/18-dec underlyings |
| `conservation_class` | yes | `bridge` \| `cbtc_lock` \| `escrow` \| `cdp_debt` \| `local` |
| `cross_chain_link` | if two-sided | Shared id both sides agree |
| `oracle_allowed` | yes | `false` for bridge/cBTC mint paths; `true` only for CDP health bounds |
| `max_ingress` (optional) | pilot | Gated pilot limits (Tacit mainnet pilot spirit) |

### 5.5 Anti-squat rules

1. **First-write-wins** on `(source_chain_tag, tacit_asset_id) → terp_asset_id` with governance or governance-free immutable register-once.  
2. Bridge mint resolves asset **only** through registry; unknown → reject (A15).  
3. `localAssetOf`-style reverse map required before public unwrap of bridged notes.  
4. Native / hub assets (e.g. ETH link, cBTC) MAY be constructor-pinned like Tacit `TETH_BITCOIN_LINK` / `CBTC_ZK_ASSET_ID`.

### 5.6 Replay domains

Bind at least:

- Source chain id / network  
- Destination Terp chain id  
- Note-set / pool instance id  
- Claim fields (`destCommitment`, `ν`, `assetId`)  

into the claim or the mint message. Cross-deployment replay of the same Tacit burn must fail A14/A16.

---

## 6. Test scenarios — bridge ingress

Named cases for LC + note-mint harness. **Expect** is the Terp decision.

| # | Name | Setup | Expect |
|---|------|-------|--------|
| T1 | **Happy-path mint** | Valid bridge burn; `ν ∈ burn set`; membership OK; `v` opens; depth ≥ K; asset registered; claim fresh | **ACCEPT** — one note with `v = v_burn`, mapped `asset_id` |
| T2 | **Double-claim reject** | Replay T1 with same `ν` / `claimId` after first mint | **REJECT** — A13 once-per-burn |
| T3 | **Ordinary spend is not a burn** | `ν` in generic spent set only; absent from bridge-burn set | **REJECT** — A5 H-1 (no value duplication) |
| T4 | **Immature confirmation** | Burn included but depth `< K` (or LC tip not matured) | **REJECT** — A2 |
| T5 | **Stale burn root** | Proof pins old `bitcoinBurnRoot` ≠ current attested | **REJECT** — A17 |
| T6 | **Value mismatch** | Witness `v_mint ≠ v_burn` (or wrong asset relabel with same `ν`) | **REJECT** — A9 |
| T7 | **Unknown / unmapped asset** | Valid burn but `asset_id` not in Terp registry | **REJECT** — A15 |
| T8 | **Malleated claimId** | Fields valid but `claimId` ≠ re-derived binding | **REJECT** — A14 |
| T9 | **Wrong destination commitment** | Burn pins `destCommitment` A; mint tries leaf B | **REJECT** — A6 |
| T10 | **Cross-domain replay** | Valid claim for pool/chain X submitted to pool/chain Y | **REJECT** — A16 |
| T11 | **Oracle-only mint attempt** | No burn; only VE/oracle mid asserts “credit user v” | **REJECT** — A12 |
| T12 | **cBTC conservation vs cUSD blur** | Attempt to mint cBTC-class asset using CDP/oracle path without lock/burn | **REJECT** — conservation_class violation |

Minimum bar for Domain B exit: **T1–T8** automated; T9–T12 recommended in same suite.

### 6.1 Property-style invariants (for proptests later)

- **Conservation:** Σ minted on dest for claim set C ≤ Σ burned on source for C.  
- **Injectivity:** each `ν` contributes to at most one accepted mint globally in a pool domain.  
- **Burn-set strictness:** spent-set ⊈ burn-set authority.  
- **Finality:** no accept for height h until LC tip ≥ h + K.

---

## 7. Non-goals

This mapping SPEC deliberately **does not**:

1. **Rewrite Headstash circuits** or redefine airdrop eligibility → note mint (separate Domain / `SPEC-airdrop`).  
2. **Fully rehost SP1** guests inside Terp as the first deliverable — LC may verify reflection-class public values or an equivalent proof; full SP1 rehost is a later ops choice.  
3. **Revive legacy tETH** as the canonical product path (`BRIDGE.md` sunset).  
4. **Invent parallel opcodes** on Terp that do not map to Tacit surfaces above.  
5. **Specify full private DEX AMM math** — see Tacit `AMM.md` / future `SPEC-amm`; only post-mint composition is noted.  
6. **Claim Tier 0 for Bitcoin-side worker AMM reserves** without client replay or EVM convergence.  
7. **Make oracles mint balances** or treat cUSD pricing as a bridge.  
8. **Require ZSA / Tachyon / Ragu** for Domain B (parked monorepo refs only).  
9. **Define CosmWasm message names** byte-for-byte — implementers derive msgs from §3 tables.  
10. **Solve reverse-path production ops** end-to-end — map exists (§3.1 reverse); deployment choreography is ops.

---

## 8. Progression-grade checklist (Domain B)

Use this as the Domain B exit gate. Another agent implements LC + note mint **against** this SPEC.

### 8.1 Spec freeze

- [ ] **SSOT surfaces listed** (§1 S1–S16) with primary path = ConfidentialPool reflection  
- [ ] **Trust tiers** frozen per surface (§2); no Tier-0 marketing for Tier-1/2  
- [ ] **Mapping table** complete for burn → LC → mint (§3.1)  
- [ ] **Auth properties A1–A17** accepted as mint preconditions (§4)  
- [ ] **Domain / asset_id map** rules frozen (§5) including conservation_class  
- [ ] **Test matrix T1–T8** named with accept/reject (§6)  
- [ ] **Non-goals** acknowledged (§7)  
- [ ] **Legacy tETH** explicitly non-target for new ingress  

### 8.2 Handoff artifacts for implementers

- [ ] Pointer to Tacit code SSOT:  
  - `crates/tacit/spec/SPEC-CONFIDENTIAL-POOL.md`  
  - `crates/tacit/spec/SPEC-CONFIDENTIAL-OPS.md`  
  - `crates/tacit/spec/amendments/SPEC-BITCOIN-REFLECTION-AMENDMENT.md`  
  - `crates/tacit/spec/amendments/SPEC-EVM-CONFIDENTIAL-TOKEN-AMENDMENT.md` (B1–B6)  
  - `crates/tacit/spec/design/TRUST-TIERS-AND-CONVERGENCE.md`  
  - `crates/tacit/contracts/src/ConfidentialPool.sol` (`settle`, `attestBitcoinStateProven`, `REFLECTION_CONFIRMATIONS`, burn roots)  
- [ ] Terp interface sketch (next agent):  
  - `MsgUpdateTacitClient` / LC update with reflected roots  
  - `MsgBridgeMintNote` with claim fields + proof  
  - `AssetMap` register-once  
  - nullifier / claim consumed set  
- [ ] Fixture plan: at least one golden vector per T1, T2, T3, T4 (can be simplified mock LC before full Bitcoin headers)

### 8.3 Honesty gates (must stay green)

- [ ] Oracle / hashmerchant **cannot** call mint  
- [ ] Generic spent-set membership **cannot** mint  
- [ ] Worker attestation **cannot** mint without chain-verifiable values  
- [ ] cUSD-style pricing path **cannot** credit bridge/cBTC balances  

### 8.4 Domain B done when

1. This file is the cited SSOT for “Tacit → Terp bridge map” in the parent program plan.  
2. An implementer can write LC verify + note mint **without inventing new bridge semantics**.  
3. Tests T1–T8 are expressible as harness cases from §6 alone.  

---

## 9. Quick reference — public values checklist for a mint packet

Minimal **public** packet a Terp LC/contract should see (names illustrative):

```text
BridgeMintPublic {
  source_chain_tag,
  tacit_asset_id,
  value_u64,                 // opened / proven equal to burn
  nullifier,                 // ν of burned note
  dest_commitment_or_leaf,   // must match burn record
  claim_id,                  // re-derived
  source_pool_root,          // membership anchor
  source_burn_root,          // current bridge-burn IMT root
  source_height,
  proof_or_lc_height,        // LC: client height / membership proof
}
```

Contract-side pseudocode:

```text
assert registry.has(source_chain_tag, tacit_asset_id)
assert claim_id == H(dest_chain, dest_commitment, nullifier, tacit_asset_id)
assert !consumed[claim_id] && !consumed[nullifier]
assert lc.is_finalized(source_height, K)
assert lc.burn_root_current(source_burn_root)
assert lc.membership(nullifier → dest_commitment, source_burn_root)
assert lc.note_was_in_pool(..., source_pool_root)  // as required by proof system
// range + conservation inside proof
consumed[claim_id] = true
mint_note(terp_asset_id, value_u64, dest_commitment_or_new_blinding)
```

---

## 10. References (read-only SSOT)

| Path | Why |
|------|-----|
| `/Users/returniflost/abstract/terp-core/crates/tacit/README.md` | Product: BTC core + ETH confidential lane |
| `/Users/returniflost/abstract/terp-core/crates/tacit/BRIDGE.md` | Legacy tETH sunset notice |
| `/Users/returniflost/abstract/terp-core/crates/tacit/AMM.md` | Post-bridge DEX architecture |
| `/Users/returniflost/abstract/terp-core/crates/tacit/spec/SPEC-CONFIDENTIAL-POOL.md` | Note, pool, bridge_mint/burn model |
| `/Users/returniflost/abstract/terp-core/crates/tacit/spec/SPEC-CONFIDENTIAL-OPS.md` | Op table 0–30 |
| `/Users/returniflost/abstract/terp-core/crates/tacit/spec/amendments/SPEC-BITCOIN-REFLECTION-AMENDMENT.md` | Reflection prover |
| `/Users/returniflost/abstract/terp-core/crates/tacit/spec/amendments/SPEC-EVM-CONFIDENTIAL-TOKEN-AMENDMENT.md` | B1–B6 bindings |
| `/Users/returniflost/abstract/terp-core/crates/tacit/spec/design/TRUST-TIERS-AND-CONVERGENCE.md` | Tier 0/1/2 |
| `/Users/returniflost/abstract/terp-core/crates/tacit/contracts/src/ConfidentialPool.sol` | On-chain gates |
| Parent plan: `.../terp-private-shielded-dex-bridge/PLAN.md` | Program thesis + phases |

---

## 11. Changelog

| Date | Change |
|------|--------|
| 2026-07-20 | Domain B initial mapping SPEC: ConfidentialPool/reflection SSOT → Terp LC + note mint interface |
