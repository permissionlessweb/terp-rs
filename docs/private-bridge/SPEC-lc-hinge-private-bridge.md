---
title: SPEC — Light Client Hinge for Deterministic Authenticatable Private Bridge
status: draft
domain: C
created: 2026-07-20
program: private-shielded-dex-bridge
ssot_program: daily-driver/plans/terp-private-shielded-dex-bridge/PLAN.md
sibling_specs:
  - docs/plans/spectrum/SPEC-airdrop-orchard-delta.md   # Domain A
  - docs/plans/spectrum/SPEC-tacit-bridge-mapping.md     # Domain B
  - docs/plans/spectrum/SPEC-private-dex-seams.md        # Domain D
  - docs/plans/spectrum/FLOW-private-bridge-auth.md      # Domain E
  - docs/plans/spectrum/zsas.md
---

# SPEC: Light client hinge — private bridge path

> **One-liner:** Engineers must know **exactly what a light client verifies** before Terp (or a sibling CosmWasm surface) is allowed to mint a private note from foreign state.
>
> **Hard rule:** No private mint without a **height-bound, domain-bound** LC acceptance of the foreign event (header tip + membership or reflection public values). Oracles / hashmerchant **never** mint balances.

This document is **documentation-only**. It does not change production LC code. It inventories monorepo LCs, freezes the private-bridge hinge table, and defines action seams for Domains A/B.

---

## 0. Taxonomy (do not collapse)

Three LC / finality classes appear in this program. Treat them as **sibling fabrics**, not one generic “bridge LC.”

| Class | Trust model | Primary monorepo home | Private-bridge role |
|-------|-------------|----------------------|---------------------|
| **A. Bitcoin / reflection-class** | SP1 guest: header PoW + merkle completeness + kernel/range on pool envelopes; confirmation depth \(K\) | `crates/tacit` (reflection guest, `ConfidentialPool`, `BITCOIN_RELAY_VKEY`) | Canonical **conservation** ingress for BTC-homed burns/locks → notes |
| **B. Crosslink / Zcash** | TFL fat-pointer finality (σ confirmations + finalizer ed25519) binding a PoW anchor + **shielded_commitment** | `crates/terp-rs/crates/crosslink/light-client`, `packages/crosslink/light-client`, `cw-ics08-wasm-crosslink` | Shielded-ecosystem anchors / pool-root membership → ZEC (and later ZSA-aligned) note mint |
| **C. Generic Cosmos IBC** | Tendermint/CometBFT client + ICS-23 (or SP1-wrapped); Ethereum sync-committee LC; optional attestor m-of-n | `tendermint-light-client/*`, `sp1-ics07`, `cw-ics08-wasm-eth`, `ethereum/light-client`, attestation LCs | Multi-hop asset ids, packet commitments; **not** a substitute for A/B conservation proofs |

**Also present, not a full LC:** `x/hashmerchant` — supermajority VE attestation of foreign roots. Useful for **price bounds / eligibility hints**; **forbidden** as sole authority for private mint of bridged value.

---

## 1. LC inventory in monorepo

Maturity key: **stub** (types/TODO only) · **unit** (crate tests) · **contract** (CosmWasm/Solidity wired) · **e2e** (interchaintest / mainnet pilot).

### 1.1 Crosslink / Zcash (class B)

| Path | Role | Maturity |
|------|------|----------|
| `/Users/returniflost/abstract/terp-core/crates/terp-rs/crates/crosslink/light-client/` | Canonical self-contained Crosslink LC primitives (types, verify, update, membership BLAKE3 pool-root, misbehaviour stub). README documents shielded anchors + ZIP-222 reservation. | **unit** — verify/update/membership/types tests; misbehaviour is TODO |
| `/Users/returniflost/abstract/terp-core/crates/solidity-ibc-eureka/packages/crosslink/light-client/` | Parallel package under Eureka workspace (same concept; keep wire/layout aligned with terp-rs copy) | **unit**-ish / evolving |
| `/Users/returniflost/abstract/terp-core/crates/solidity-ibc-eureka/programs/cw-ics08-wasm-crosslink/` | CosmWasm **08-wasm** wrapper: instantiate, update_state, verify_membership / non-membership, misbehaviour freeze | **contract** + limited **unit** (`src/test/`); e2e planned in PLAN.md / PLAN_V2.md, not program-complete |
| Plans/reviews | `programs/cw-ics08-wasm-crosslink/plans/PLAN.md`, `PLAN_V2.md`, `reviews/00_v2.md` | design SSOT for 08-wasm mapping |

**What Crosslink embeds (v1):**  
On verified header update, `PowHeader.commitment_bytes` → `ConsensusState.shielded_commitment` and `ClientState.latest_shielded_commitment`.  
`ConsensusState.app_state_commitment` remains **zero** until ZIP-222 / IBC-v2.

Membership targets **`shielded_commitment`** via compact BLAKE3 pool-root existence proofs (`pool_root_proof_specs()`): leaf prefix `0x00`, inner `0x01`/`0x02`, 32-byte children.

### 1.2 Bitcoin / reflection-class (class A)

| Path | Role | Maturity |
|------|------|----------|
| `crates/tacit/spec/amendments/SPEC-BITCOIN-REFLECTION-AMENDMENT.md` | Reflection guest/header completeness; **mint-critical public subset** is superseded for Terp by Domain B + live `ConfidentialPool` (pool / spent / **burn** / height) when short amendment prose lists only pool+spent | **live** (spec + product) |
| `crates/tacit` contracts / guests (`ConfidentialPool`, `BtcCallExecutor`, cxfer-core reflection) | `attestBitcoinStateProven` + `BITCOIN_RELAY_VKEY`; `BitcoinRelayPublicValues` includes `bitcoinBurnRoot`; gates `OP_BRIDGE_MINT` on **bridge-burn set** (`knownBitcoinBurnRoot`), not spent set alone | **e2e / mainnet pilot** on Tacit surfaces |
| Tacit `SPEC-CONFIDENTIAL-POOL.md`, `SPEC-CONFIDENTIAL-OPS.md` | bridge_mint/burn conservation language | **live** design |
| Domain B: [`SPEC-tacit-bridge-mapping.md`](./SPEC-tacit-bridge-mapping.md) §1 S5–S6, §3.1, §4.2 A5–A8 | Terp SSOT map: reflection public values + H-1 burn-set authority | **draft freeze candidate** |

There is **no** dedicated CosmWasm “Bitcoin LC” package yet for Terp. The hinge for class A on Terp is: **re-host the same public values + confirmation policy** as LC consensus tip (or as a verified SP1 receipt whose public inputs bind those values), then **bridge-burn membership** (and burned-note pool membership) under that tip — not ordinary spent-set membership alone.

### 1.3 Ethereum consensus LC (class C — Eth finality for IBC store)

| Path | Role | Maturity |
|------|------|----------|
| `crates/solidity-ibc-eureka/packages/ethereum/light-client/` | Sync-protocol update; MPT membership against IBCStore storage root | **unit** + fixtures; README: under heavy development |
| `crates/solidity-ibc-eureka/programs/cw-ics08-wasm-eth/` | CosmWasm 08-wasm wrapper for Eth LC | **contract**; historical updates + account proof of IBCStore |

**Proves:** finality of Eth consensus → execution state root → account storage root of `ibc_contract_address` → MPT (non-)membership of IBC commitments.  
**Does not alone prove:** Tacit confidential-pool note conservation (that is class A reflection or in-guest settle).

### 1.4 Tendermint / SP1 ICS-07 (class C — Cosmos counterparty)

| Path | Role | Maturity |
|------|------|----------|
| `packages/tendermint-light-client/{update-client,membership,misbehaviour,uc-and-membership}/` | Stateless TM verify circuits/libs + fixtures | **unit** + fixtures |
| `packages/sp1-ics07-tendermint-prover/`, `programs/sp1-programs/` | SP1 guests for update / membership | **unit** / prover path |
| `contracts/light-clients/sp1-ics07/SP1ICS07Tendermint.sol` | On-EVM TM client | **e2e** (Eureka suite) |
| Solana ics07tendermint programs under `programs/solana/` | Solana-side TM client | **e2e** (Eureka) |

**Proves:** TM header validator-set continuity + ICS-23 membership under app hash. Standard IBC packet path.

### 1.5 Attestation light clients (class C — m-of-n, weaker)

| Path | Role | Maturity |
|------|------|----------|
| `contracts/light-clients/attestation/` (+ `IBC_ATTESTOR_DESIGN.md`) | m-of-n attestors for height/timestamp + packet list membership | **contract** / e2e in Eureka; misbehaviour placeholder |
| Attestor/aggregator packages under Eureka | Off-chain quorum producers | **e2e** |

**Private-bridge policy:** attestation LCs may gate **demo / degraded** paths only if product explicitly accepts attestor trust. They **must not** be labeled Tier-0 conservation equivalent to reflection or Crosslink TFL.

### 1.6 Adjacent non-LC

| Path | Role vs private mint |
|------|----------------------|
| `x/hashmerchant/` | Confirmed foreign **roots** via VE → CosmWasm sudo. OK for oracle bounds / optional eligibility. **Not** bridge solvency. |
| `docs/plans/spectrum/zsas.md` | OrchardZSA issuance parent — Domain A asset-id / supply tracking; LC may later bind distro roots |
| `crates/tachyon`, `crates/ragu` | Future recursion; **out of hinge v1** |

---

## 2. Per–LC-type: ClientState / ConsensusState / Update / Membership — what is proven

### 2.1 Crosslink / Zcash (class B)

#### ClientState (stored under 08-wasm client)

From `crosslink/light-client/src/client_state.rs`:

| Field | Meaning for hinge |
|-------|-------------------|
| `crosslink_params` (σ, L) | Confirmation depth & gap bound — part of finality statement |
| `latest_bft_height` / `latest_bft_block_hash` | TFL tip |
| `latest_finalized_pow_height` / `latest_finalized_pow_hash` | Finalized Zcash PoW anchor |
| `latest_shielded_commitment` | Tip copy of shielded/header commitment |
| `finalizer_roster` | Who may sign fat pointers |
| `is_frozen` | Halt updates & membership after misbehaviour |

#### ConsensusState (per BFT height)

| Field | Proven binding |
|-------|----------------|
| `bft_height` | PoS/TFL height of snapshot |
| `pow_anchor_height`, `pow_anchor_hash`, `timestamp` | Finalized PoW block identity |
| `shielded_commitment` | Era-dependent Zcash header commitment (Sapling/history/auth) at that PoW block |
| `app_state_commitment` | **Reserved zero** (ZIP-222 later) |

#### Update (`verify_header` → `update_consensus_state`)

Proven steps (see `verify.rs` + README flow):

1. Client not frozen  
2. `header.trusted_bft_height == client_state.latest_bft_height`  
3. BFT block header list length == σ (confirmation depth)  
4. Fat pointer block hash matches BFT block hash  
5. Finalizer ed25519 batch signatures (native/`Api`; wasm historically may trust pre-verified relay — **document and tighten for mint-critical deploys**)  
6. New PoW anchor height **strictly greater** than prior consensus PoW height  

On success: store new ConsensusState with `shielded_commitment = pow_anchor.commitment_bytes`; advance ClientState tip fields.

#### Membership

- **Existence:** `PoolRootExistenceProof` under BLAKE3 pool-root spec; calculated root **must equal** `consensus_state.shielded_commitment` at the **revision height** used for the proof.  
- **Key/value:** proof key matches deepest merkle path key; proof value matches claimed value bytes.  
- **Non-membership (v1 weak):** neighbour existence under same root with different key — **not** full sorted-range exclusion; do not rely on v1 non-membership for conservation-critical burns.

**Private-mint interpretation:**  
“This leaf/value was committed under the shielded pool / header commitment that Crosslink finalizers finalized at `(bft_h, pow_h)`.”  
App layer still must interpret value layout (note cm, nullifier, asset, amount commitment) and bind **domain** (client_id, chain_id, asset_id).

#### Misbehaviour

- `misbehaviour.rs`: TODO (should detect dual valid fat pointers at same BFT height, different candidates).  
- Contract path: freeze client on accepted misbehaviour message.  
- **Hinge policy until complete:** treat freeze as manual/gov; do not auto-mint against a client that fails roster/signature checks.

---

### 2.2 Bitcoin / reflection-class (class A)

Not an ICS-02 `ClientState` in-repo for Terp yet. Normative **mint-critical public statement** (live Tacit `ConfidentialPool` + Domain B [`SPEC-tacit-bridge-mapping.md`](./SPEC-tacit-bridge-mapping.md) §1 S5–S6, §3.1):

```text
BitcoinRelayPublicValues {
  bitcoinPoolRoot,   // note-commitment tree root at H
  bitcoinSpentRoot,  // spent-nullifier IMT root at H (never 0; empty IMT sentinel)
  bitcoinBurnRoot,   // bridge-burn IMT root at H (never 0; empty IMT sentinel)
  bitcoinHeight,     // confirmed height H
  // full on-chain struct may also carry digest chain / cBTC fold fields;
  // mint authority for bridge_mint is the burn root, not those extras alone
}
```

**Precedence:** If older reflection-amendment prose lists only `(pool, spent, height)`, treat that as **incomplete for mint**. Terp Domain C follows Domain B + production `BitcoinRelayPublicValues` / B5: mint-critical subset is **pool + spent + burn + height**.

#### “ClientState” analogue (product state)

| Concept | Proven / enforced |
|---------|-------------------|
| Relay verifying key | `BITCOIN_RELAY_VKEY` (SP1) |
| Last proven height | Strictly increasing on `attestBitcoinStateProven` |
| Confirmation depth \(K\) | Header chain extends ≥ K past H (or tip already buried); Domain B A2 / `REFLECTION_CONFIRMATIONS` spirit |
| Spent root non-vacuity | Empty IMT sentinel, never zero (`knownBitcoinSpentRoot`) |
| **Burn root non-vacuity** | Empty IMT sentinel, never zero (`knownBitcoinBurnRoot`); reject mint against zero/stale burn root (Domain B A17, `StaleBitcoinBurnRoot` class) |
| Current burn root pin | Mint batch / certificate must pin the **current** attested `bitcoinBurnRoot` |

#### “ConsensusState” analogue

At height H: `(bitcoinPoolRoot, bitcoinSpentRoot, bitcoinBurnRoot, H)` is the authority for pool membership, spent non-membership (cross-lane B4), and **bridge-burn membership** (mint).

| Root | Role | Authorizes bridge mint? |
|------|------|-------------------------|
| `bitcoinPoolRoot` | Note commitment tree | **Necessary** (burned note was in pool) — **not sufficient alone** |
| `bitcoinSpentRoot` | Ordinary spent / nullifier IMT | **No** — cross-lane spend / double-spend gates only |
| `bitcoinBurnRoot` | **Bridge-burn set** (conf-burn / `OP_BRIDGE_BURN` exits) | **Yes** — required membership of burn `ν` (→ destCommitment) |

#### Update (guest algorithm summary)

For each block in `(anchor, H]`: verify header chain + PoW; for **every** tx (completeness via merkle root): extract pool envelopes; `verify_range` + `verify_kernel`; map outputs → owner-free leaves; inputs → chain-independent nullifiers `ν = keccak(Cx ‖ Cy ‖ "spent")`. Commit **pool**, **spent**, and **bridge-burn** roots (bridge burns are a distinct op class from ordinary spends).

#### Membership (mint-critical)

| Proof | Against | Allows |
|-------|---------|--------|
| **Bridge-burn membership** | `bitcoinBurnRoot` | **Mint authority:** `ν` is a bridge burn (bound to `destCommitment` / claim fields) under current burn root — Domain B A5, A6, A8 |
| Burned-note membership | `bitcoinPoolRoot` | Burned note existed in source pool at burn time — Domain B A7 |
| Nullifier non-membership (EVM / dual-home spend) | `bitcoinSpentRoot` | Spend of Bitcoin-homed note on other lane without double-spend — **not** a mint path |
| ClaimId / once-per-ν | app state + domain binding | bridge_mint once per foreign burn — Domain B A13–A14 |

#### H-1 invariant (normative — fail closed)

**Membership of an ordinary spend-only nullifier MUST NOT authorize bridge mint.**

| Observation | Mint? |
|-------------|--------|
| `ν ∈ bitcoinBurnRoot` (bridge-burn set) + burned note ∈ `bitcoinPoolRoot` + K-finality + domain/claim | **Allow** (subject to conservation opening in Domain B) |
| `ν ∈ bitcoinSpentRoot` only (ordinary transfer/spend); **absent** from bridge-burn set | **REJECT** — H-1 / Domain B A5, T3 |
| Pool leaf membership without burn-set membership | **REJECT** |
| Zero or stale `bitcoinBurnRoot` / not current attested root | **REJECT** — Domain B A17 |

**Private-mint interpretation on Terp:**  
Accept mint only if proof verifies **bridge-burn membership under the current attested `bitcoinBurnRoot`**, burned-note membership under `bitcoinPoolRoot`, confirmation depth \(K\), under the **latest accepted** reflection tip (or an explicitly allowed historical height with lag policy), with **claimId / nullifier uniqueness**, **destCommitment** binding, and **asset_id** mapping from Domain B ([`SPEC-tacit-bridge-mapping.md`](./SPEC-tacit-bridge-mapping.md) §4 A1–A17, §9 mint packet). Amount authority remains Domain B conservation/opening (A9–A11) — LC tip/membership never invents `v`.

---

### 2.3 Ethereum 08-wasm LC (class C)

#### ClientState (summary)

`chain_id`, genesis validators root, sync committee params, fork parameters, `latest_slot`, `latest_execution_block_number`, `is_frozen`, `ibc_contract_address`, `ibc_commitment_slot`.

#### ConsensusState

`slot`, `state_root`, `timestamp`, current/next sync committee summaries.

#### Update

Ethereum light client sync protocol (+ historical updates for multi-relayer); each update includes **account proof** for `IBCStore` so Cosmos side holds the Eth IBC commitment storage root.

#### Membership

MPT inclusion/exclusion under IBCStore storage root for IBC commitment paths.

**Private-mint interpretation:**  
Proves **IBC packet/commitment** existence on Ethereum — suitable for ICS-20-style or Eureka packet paths. For Tacit confidential burns, prefer class A reflection roots (or Eth storage proof of **ConfidentialPool** roots if a dedicated membership path is specified later — not the default IBCStore path).

---

### 2.4 Tendermint / SP1 ICS-07 (class C)

#### ClientState / ConsensusState

Standard ICS-07: chain_id, trust level, unbonding period, latest height, frozen height; consensus: timestamp, next validators hash, root (app hash).

#### Update

Verify TM light-client header (or SP1 proof of same statement).

#### Membership

ICS-23 (or SP1 membership) under consensus root for IBC paths (`commitments/ports/...`, receipts, acks).

**Private-mint interpretation:**  
Packet commitment / acknowledgement authenticity for Cosmos↔Terp multi-hop. Value conservation still depends on counterparty app (ICS-20 escrow, or a private-app packet schema from Domain B).

---

### 2.5 Attestation LC (class C — quorum)

#### ClientState

Attestor set, `minRequiredSigs`, `latestHeight`, `isFrozen`.

#### ConsensusState

Height → timestamp (no cryptographic commitment root of counterparty state).

#### Update / Membership

m-of-n signatures over attested height/timestamp; packet membership = inclusion in attested packet list.

**Private-mint interpretation:**  
Only if product accepts attestor honesty for that corridor. Not equivalent to Crosslink TFL or Bitcoin PoW+kernel reflection.

---

## 3. Private-bridge hinge table

**Mechanisms (do not collapse labels)** — see team clarity [`CLARITY-headstash-sets-and-bridge-models.md`](./CLARITY-headstash-sets-and-bridge-models.md) §2 and Domain B [`SPEC-tacit-bridge-mapping.md`](./SPEC-tacit-bridge-mapping.md) §3.2 / §3.5:

| Mechanism | What authorizes dest note | Hinge rows |
|-----------|---------------------------|------------|
| **Reflection burn → mint** | Proven **bridge-burn** under final root + conservation | 1, 3 |
| **Native lock / cBTC-class** | Proven **lock** (not attestor multisig “escrow”) | 2 |
| **Same-chain wrap / transparent IBC escrow** | Locked public funds / ICS-20 voucher on *that* corridor | 6–7 (packet schema); wrap is Domain B S9, not this table’s Bitcoin burn |

**Gate:** `AllowPrivateMint(effect)` iff **all** of:

1. **LC class accepts** the foreign event type  
2. **Proof type** verifies against **consensus tip (or allowed height)**  
3. **Public inputs** bind height, root/commitment (incl. **burn root** for class A burns), domain, claim id  
4. **Effect** is in the allowlist for that class (no oracle-only mint)

| # | Foreign event | LC class | LC proof type | Public inputs (minimum) | Allowed private effect |
|---|---------------|----------|---------------|-------------------------|------------------------|
| 1 | Bitcoin confidential burn / `OP_ENV_CONF_BURN` / `OP_BRIDGE_BURN` / pool burn envelope | A reflection | SP1 reflection receipt **and** burn-set + pool membership under attested roots | `bitcoinPoolRoot`, `bitcoinSpentRoot`, **`bitcoinBurnRoot`**, `bitcoinHeight`, `ν`, `destCommitment`, `destChain` / dest tag, `asset_id`, re-derived `claimId`, confirmation \(K\), domain_binding | **Note mint** (bridge_mint) of mapped asset **only if** `imt_membership(ν → destCommitment, bitcoinBurnRoot)` **and** burned note ∈ `bitcoinPoolRoot`; amount from note opening + range in private circuit (Domain B A9–A11). **Ordinary spent-set membership alone: REJECT (H-1).** |
| 2 | Bitcoin cBTC.zk lock (self-custody slot) | A reflection | Lock inclusion + PoW/header chain as in reflection lock-fold | lock outpoint, height, `asset_id`/cBTC id, \(K\) | **cBTC-class note mint** (`OP_CBTC_MINT` analogue) with **lock-conservation** (not attestor/custodial multisig escrow) |
| 3 | Ethereum ConfidentialPool burn (reflection-gated outbound) | A (via Eth settle) **or** C Eth LC + app storage proof | In-guest burn commitment **or** storage proof of **bridge-burn set** / pool root under finalized Eth state | pool root, **burn root / burn id**, Eth slot/height, `ν` / claim fields, domain_binding | Note mint on Terp for mapped asset (same H-1: burn set, not generic spent) |
| 4 | Crosslink-finalized shielded pool / header commitment leaf | B Crosslink | `verify_membership` under `shielded_commitment` @ `(bft_h, pow_h)` | `client_id`, `bft_h`, `pow_h`, `shielded_commitment`, key, value, BLAKE3 path | Note mint for ZEC (or later ZSA asset) **only** if value schema encodes mintable claim (lock/burn/issuer — **not frozen in v1**; see §3.3) |
| 5 | Crosslink tip advance (no leaf) | B Crosslink | `update_client` only | header public fields, σ, roster | **No mint** — advances transparent spine only |
| 6 | IBC packet commitment (Cosmos counterparty) | C TM / SP1 ICS-07 | UpdateClient + Membership on packet path | `client_id`, height, port/channel/seq, commitment bytes | Mint **only** if packet data schema is private-bridge claim (transparent ICS-20 escrow→note or private-app claim); else transparent ICS-20 only |
| 7 | IBC packet commitment (Ethereum Eureka IBCStore) | C Eth 08-wasm | Sync update + MPT membership | Eth slot, IBC path, commitment | Same as row 6 for Eth-IBC corridor |
| 8 | Distro / airdrop eligibility root (Headstash) | B membership **or** A root **or** hashmerchant root (**eligibility only**) | Membership of eligibility leaf | root, index, identity commitment, **epoch** | **Claim note mint** into shared anonymity set (Domain A); not free-form value creation. **Eligibility ORs are not value-mint ORs.** |
| 9 | Hashmerchant confirmed foreign root | hashmerchant VE | Quorum root write + optional merkle proof | chain_id, root, height, quorum | **No value mint.** May supply oracle mid bounds or **eligibility** if product wires it |
| 10 | Attestor-signed packet list | C attestation | m-of-n over attested packets | attestors, height, packet commitment | Demo/degraded mint only if explicitly enabled |

### 3.1 Domain binding (required public inputs)

Every mint-allowing proof must bind:

```text
domain_binding = H(
  "terp-private-bridge-v1" ‖
  src_chain_id ‖ dst_chain_id ‖
  lc_client_id ‖
  asset_id ‖
  claim_id_or_nullifier ‖
  consensus_height ‖
  commitment_root
)
```

For **class A bridge mint**, the mint-critical public field set MUST also surface (aligned with Domain B §9):

```text
ν, destCommitment, destChain, asset_id, claim_id (re-derived),
source_pool_root, source_burn_root, source_height, K, domain_binding
```

- Domain C verifies tip / finality + burn/pool membership under attested roots.  
- Domain B / contract re-derives `claimId = f(destChain, destCommitment, ν, assetId)` and enforces once-per-ν (A13–A14).  
Replay across clients, assets, or heights is rejected at the mint verifier (Domain A/B circuits + CosmWasm gate).

### 3.2 What is *not* sufficient alone

| Observation | Why insufficient |
|-------------|------------------|
| LC tip height advanced | No foreign event proven |
| Membership under wrong height’s root | Stale / reorg surface |
| **`ν` in `bitcoinSpentRoot` only (ordinary spend)** | **H-1** — not bridge-burn authority; Domain B A5 / T3 |
| Pool leaf under `bitcoinPoolRoot` without burn-set membership | Existence ≠ bridge exit claim |
| Zero / stale `bitcoinBurnRoot` | Domain B A17; no mint pin |
| Oracle mid or VE price | Not conservation |
| Attestor height without packet list / root | No payload authenticity |
| Eth IBCStore membership of unrelated key | Wrong domain |
| **Eligibility ORs (row 8)** applied as value-mint ORs | Eligibility may use multiple root sources; **class A value mint** only via rows 1–2; **class B value mint** only via row 4 + conservation schema. Never substitute Crosslink for Bitcoin burn authority or vice versa |

### 3.3 Zcash corridor readiness & ZEC egress (LC + ZIP-222 TZE) — non-blocking

**Design path (future sprint family):** ZEC egress reuses this LC hinge + Domain B conservation gates, with **ZIP-222 TZE** as a Zcash-native evidence channel when available (headers / Crosslink finality + lock-or-burn accounting). Team clarity: [`CLARITY-headstash-sets-and-bridge-models.md`](./CLARITY-headstash-sets-and-bridge-models.md) §2 “ZEC egress: light clients + TZE”.

| Rule | Meaning |
|------|---------|
| **Not blocking** | Headstash (Domain A), Tacit Bitcoin reflection mapping (Domain B), private DEX seams (Domain D), hashmerchant bounds, suite/harness, and class A/B **inventory + tip demos** ship **without** waiting on ZEC egress or TZE app-state membership |
| **Crosslink stays separate** | Class B is a **sibling fabric** to class A Bitcoin reflection — not a substitute for `bitcoinBurnRoot` mint authority (LC-DOM-04) |
| **North star** | IBC-class Bitcoin ↔ Zcash remains the long arc; TZE is a Zcash-side instrument, not a rewrite of Terp note algebra |

**Honest readiness (class B private-mint corridor):**

| Ready now | Not ready (future SSOT / sprint) |
|-----------|----------------------------------|
| Crosslink create-client + update + BLAKE3 membership under `shielded_commitment` | Normative **value layout** for mintable ZEC claim (lock outpoint vs burn vs OrchardZSA issuer leaf) |
| Domain-binding reject tests; tip-only = no mint (row 5) | ZIP-222 **`app_state_commitment`** / TZE → `LcIngressCertificate.proof_kind` mapping |
| Inventory of crates, 08-wasm, maturity labels (§1.1) | Auto misbehaviour beyond manual/gov freeze |
| Mock HINGE-01 without Tier-0 ZEC conservation claim | ZEC-out conservation_class freeze (Domain B §5.4 spirit) |

**Do not** market Crosslink tip demos as Tier-0 ZEC conservation egress until lock/burn (or issuer) schema + optional TZE channel are specified and tested.

---

## 4. Misbehaviour / reorg / lag (degraded) policy sketch

### 4.1 Crosslink (class B)

| Condition | Detection | Client action | Mint action |
|-----------|-----------|---------------|-------------|
| Dual fat pointers same BFT height, different candidates | `verify_misbehaviour` (TODO → implement) | Freeze `is_frozen` | **Reject all mints** on that client_id until gov unfreeze |
| Roster / signature failure | `verify_header` reject | No update | No mint under unposted height |
| Lag: `pow_h` / `bft_h` behind foreign tip by > `MAX_LC_LAG` | Relayer metrics / query | Allow updates; mark **degraded** | Optionally reject mints requiring tip-freshness; allow mints only against heights ≥ `tip - LAG_ALLOW` with explicit policy |
| σ mismatch / gap > L | Header reject | No update | No mint |
| Weak non-membership abuse | Design | Prefer existence proofs for mint; avoid relying on v1 non-membership for burns | |

### 4.2 Bitcoin / reflection (class A)

| Condition | Policy |
|-----------|--------|
| Height not buried by \(K\) confirmations | Reject update and mint |
| Non-monotonic height | Reject (`attestBitcoinStateProven` guard) |
| Zero / stale `bitcoinBurnRoot` | Reject mint (H-1 / A17); require current attested burn root |
| Ordinary spend ν only (no burn-set membership) | Reject mint (H-1) |
| Deep reorg below proven H | Product must not roll back spent **or burn** IMTs; freeze corridor / gov; no double mint via claimId set |
| SP1 vkey mismatch | Reject all reflection receipts |

### 4.3 Tendermint / Eth (class C)

| Condition | Policy |
|-----------|--------|
| Misbehaviour (conflicting headers) | Freeze client (standard ICS-07 / Eth dual update) |  
| Trusting period expiry | Client expired — no update/membership |  
| Clock drift | Per client params |  
| Lag | Same `MAX_LC_LAG` pattern for mint freshness |

### 4.4 Degraded modes (explicit labels)

| Mode | Allowed | Disallowed |
|------|---------|------------|
| **Full** | Class A/B mint per table | — |
| **Lag-degraded** | Mint against heights within window; UI shows lag | Mint requiring “tip finality” UX claims |
| **Attestor-degraded** | Demo corridor with attestation LC | Claiming Tier-0 / reflection-equivalent |
| **Oracle-only** | Quote bounds | Any note mint of bridged value |
| **Frozen** | Queries | All private mints for that client |

---

## 5. Test scenarios

Executable scenario names for Domain E compose suite and LC unit tests.  
**Expect** = verifier / mint gate outcome.

### 5.1 Create client

| ID | Scenario | Inputs | Expect |
|----|----------|--------|--------|
| LC-CC-01 | Crosslink create-client with valid roster + params | σ, L, genesis BFT/PoW, finalizers | Client stored, `is_frozen=false`, tip heights set |
| LC-CC-02 | Crosslink create-client empty roster | empty finalizers | Reject (or accept but `validate_roster` fails before update — product: reject) |
| LC-RF-01 | Reflection corridor init | BITCOIN_RELAY_VKEY, genesis roots, K | Corridor ready; spent root = empty IMT sentinel |
| LC-TM-01 | TM create-client | ICS-07 ClientState + ConsensusState | Standard success |
| LC-ETH-01 | Eth create-client | chain_id, genesis validators, IBCStore address/slot | Success |

### 5.2 Update

| ID | Scenario | Expect |
|----|----------|--------|
| LC-CC-10 | Crosslink update: valid header, σ headers, sigs, pow strictly ahead | New ConsensusState; `shielded_commitment` = header commitment_bytes |
| LC-CC-11 | Trusted height mismatch | Reject |
| LC-CC-12 | Confirmation depth ≠ σ | Reject |
| LC-CC-13 | Fat pointer hash ≠ BFT hash | Reject |
| LC-CC-14 | PoW height not strictly ahead | Reject |
| LC-CC-15 | Update while frozen | Reject |
| LC-RF-10 | Reflection attest height H+Δ with K confirmations | Roots advance; height monotonic |
| LC-RF-11 | Reflection skip gap / non-contiguous | Reject |
| LC-TM-10 | Happy-path update client | Success (fixture: `update_client_happy_path.json`) |
| LC-ETH-10 | Multi-period sync committee update | Success (fixture present under eth LC tests) |

### 5.3 Membership accept / reject

| ID | Scenario | Expect |
|----|----------|--------|
| LC-CC-20 | Leaf existence under matching `shielded_commitment` | Accept (`leaf_membership_against_shielded_commitment`) |
| LC-CC-21 | Wrong value bytes | Reject |
| LC-CC-22 | Calculated root ≠ shielded_commitment | Reject |
| LC-CC-23 | Empty merkle path | Reject |
| LC-CC-24 | Path with prefix: deepest key used | Accept if leaf matches |
| LC-CC-25 | Non-membership neighbour key equals claimed | Reject |
| LC-RF-20 | bridge_mint: `ν ∈ bitcoinBurnRoot` **and** burned note ∈ `bitcoinPoolRoot` at current tip; claimId fresh | **Accept** → mint allowed (Domain B T1 spirit) |
| LC-RF-21 | Membership under stale **pool** root after tip advance (if policy disallows) | Reject or require height pin |
| LC-RF-22 | **H-1 reject:** `ν` in `bitcoinSpentRoot` only; **absent** from bridge-burn set | **REJECT mint** (Domain B A5 / T3 — ordinary spend is not a burn) |
| LC-RF-23 | **H-1 / A17 reject:** `bitcoinBurnRoot` is zero, empty-unauthorized, or stale ≠ current attested | **REJECT mint** |
| LC-RF-24 | Burn-set membership OK but burned note not under `bitcoinPoolRoot` | **REJECT mint** (Domain B A7) |
| LC-RF-30 | Deep reorg / freeze corridor after proven H; mint attempt | **REJECT mint** (claimId set + freeze policy) |
| LC-TM-20 | ICS-23 membership fixture key 0 | Accept |
| LC-TM-21 | Non-membership fixture key 1 | Accept non-membership |
| LC-ETH-20 | MPT membership of IBC commitment | Accept |

### 5.4 Stale proof / lag

| ID | Scenario | Expect |
|----|----------|--------|
| LC-STALE-01 | Membership at height h where consensus pruned | Reject (consensus not found) |
| LC-STALE-02 | Membership at h < tip - MAX_LC_LAG | Reject mint (lag policy) |
| LC-STALE-03 | Valid membership at h within window | Accept mint |
| LC-STALE-04 | Reflection claim with height < knownBitcoinHeight - window | Reject |

### 5.5 Wrong domain binding

| ID | Scenario | Expect |
|----|----------|--------|
| LC-DOM-01 | Valid LC membership, wrong `asset_id` in mint circuit | Reject mint |
| LC-DOM-02 | Valid proof, wrong `client_id` / chain_id in domain_binding | Reject mint |
| LC-DOM-03 | Replay same claimId after successful mint | Reject (once) |
| LC-DOM-04 | Crosslink proof used on reflection corridor mint API | Reject (class mismatch) |
| LC-DOM-05 | Hashmerchant root + merkle proof without LC class A/B | Reject value mint; allow bounds-only path |
| LC-DOM-06 | Burn pins `destCommitment` A; mint tries leaf B | **Reject mint** (Domain B A6 / T9) |

### 5.6 Misbehaviour

| ID | Scenario | Expect |
|----|----------|--------|
| LC-MB-01 | Submit dual Crosslink headers (once implemented) | Freeze; subsequent updates fail |
| LC-MB-02 | Mint attempt after freeze | Reject |
| LC-MB-03 | TM conflicting headers | Freeze ICS-07 client |

### 5.7 Hinge mock (private mint)

| ID | Scenario | Expect |
|----|----------|--------|
| HINGE-01 | Crosslink membership accept → mock `bridge_mint` note | Note in shared set; public: client tip + asset_id only |
| HINGE-02 | Reflection membership accept → mock note | Same; conservation checklist green |
| HINGE-03 | Oracle mid only → mock mint | **Reject** |
| HINGE-04 | Headstash eligibility membership → claim note | Accept (Domain A); no DEX required |
| HINGE-05 | HINGE-01 then private swap (Domain D) | Compose accept |

---

## 6. Interface to Domain B (Tacit mapping) and Domain A (claim notes)

Action seams only — no full protocol rewrite. Domain C **exposes proofs**; A/B **consume** them.

### 6.1 Domain B — Tacit bridge mapping (`SPEC-tacit-bridge-mapping.md`)

Cross-links (Domain B section names implementers must open):

| Domain B section | What C must honor |
|------------------|-------------------|
| §1 S5–S6 (reflection + bridge-burn set) | Public values include `bitcoinBurnRoot`; H-1 distinct from spent set |
| §3.1 Bridge core | Mint against burn-set membership + pool membership; `knownBitcoinBurnRoot` gate |
| §4 Auth properties A1–A17 | Finality A1–A4; burn authenticity A5–A8; conservation A9–A12; one-shot/domain A13–A17 |
| §5 Domain / `asset_id` map | Registry + conservation_class; unknown id reject |
| §6 T1–T12 (esp. T1, T3, T5) | LC-RF / LC-DOM cases mirror B accept/reject |
| §9 Mint packet | Shared public field list for class A mint |

| Seam | Domain C provides | Domain B provides | Contract |
|------|-------------------|-------------------|----------|
| **B1 MapAsset** | `src_chain_id`, LC `client_id`, foreign asset bytes | Stable Terp `asset_id` (§5) | Bijective, frozen table per corridor |
| **B2 MapEvent** | Foreign event type enum (rows in §3) | Tacit op analogue (`OP_BRIDGE_MINT`, `OP_BRIDGE_BURN`, `OP_CBTC_MINT`, …) | Every mint names one Tacit surface |
| **B3 PublicValues** | LC-normalized public input struct incl. **`bitcoinBurnRoot`** for class A | Encoding match to Tacit / SP1 public values (§3.1 reflection row) | Byte-compatible or explicitly translated; mint-critical subset = pool/spent/**burn**/height |
| **B4 Conservation** | Membership/reflection validity (burn + pool roots) | Kernel / amount / lock rules for mint size (A9–A11) | LC never invents amount; opening proof required before credit |
| **B5 ClaimId** | Height + leaf/key + destCommitment + domain_binding | Once-set / nullifier semantics (A13–A14); re-derived claimId | Double mint impossible |
| **B6 Trust tier** | LC class (A/B/C/attestor) | Tier label for UX and governance (§2) | No silent tier upgrade |

**API sketch (logical):**

```text
LcIngressCertificate {
  class: Reflection | Crosslink | Tendermint | Ethereum | Attestation,
  client_id: string,
  height: Height,
  commitment_root: [u8; 32],      // primary tip root (pool / shielded / app hash)
  // class A reflection also attests:
  //   bitcoinPoolRoot, bitcoinSpentRoot, bitcoinBurnRoot, bitcoinHeight
  proof_kind: Membership | BurnSetMembership | ReflectionReceipt | PacketCommitment,
  proof: bytes,
  value: bytes,                   // leaf / burn record (ν, destCommitment, …) or packet data
  domain_binding: [u8; 32],
}

// Domain B:
//   map(certificate) -> TacitBridgeAction | Error
// Domain C does not execute the action.
// Msg / CosmWasm names are owned by the first B+C implementation sprint card; SPECs stay logical.
```

### 6.2 Domain A — claim notes (`SPEC-airdrop-orchard-delta.md`)

| Seam | Domain C provides | Domain A provides | Contract |
|------|-------------------|-------------------|----------|
| **A1 Eligibility root** | Membership under LC (or VE root if eligibility-only) | Distro set schema, epoch | Root must be height-bound |
| **A2 Claim mint** | Certificate that eligibility leaf exists | Orchard-delta / note circuit minting into **shared** set | Same note language as DEX |
| **A3 Nullifier** | — | Claim nullifier set | No double claim |
| **A4 Non-bridge airdrop** | Optional | Local distro without foreign LC | Explicitly non-bridge path |

**API sketch:**

```text
// after LcIngressCertificate verified for event type DistroLeaf:
// Domain A:
//   claim_note(certificate, recipient_secret) -> NoteCommitment | Error
// Domain C never constructs note openings.
```

### 6.3 Shared mint gate (CosmWasm / app module — future)

```text
fn allow_private_mint(cert: LcIngressCertificate, action: PrivateEffect) -> Result<()> {
  require(!client_frozen(cert.client_id));
  require(height_allowed(cert.height));          // lag policy
  verify_lc_proof(cert)?;                        // class-specific
  // class A bridge_mint:
  //   require burn-set membership under current bitcoinBurnRoot (H-1)
  //   require burned note under bitcoinPoolRoot
  //   reject if only spent-set membership
  require(domain_binding_ok(cert));
  require(claim_id_unused(cert));
  require(effect_allowed(cert.class, action));   // §3 table
  require(conservation_opening_ok(cert, action)); // Domain B A9–A11; not LC tip alone
  require(!oracle_in_mint_path(action));         // Domain B A12
  // Domain A/B perform state transition
}
```

---

## 7. Non-goals

| Non-goal | Rationale |
|----------|-----------|
| Full production relayer for Crosslink/Bitcoin on Terp in this spec | Relayer is transport; hinge is verify semantics |
| Tachyon / Ragu recursion | Out of Sprint 0; use when proof aggregation is bottleneck |
| Replacing Tacit Bitcoin design with only ZEC Crosslink | Crosslink is **sibling**, not substitute |
| Treating hashmerchant / oracles as mint authority | Solvency vs pricing separation |
| Full ZIP-222 app_state membership / TZE mint channel in v1 | Reserved; v1 uses `shielded_commitment` only; **ZEC egress is design path, not a blocker** for other domains (§3.3) |
| Completing Crosslink misbehaviour math in this doc | Spec requires freeze policy; implementation TODO remains in crate |
| New tETH mixer as canonical path | Tacit `BRIDGE.md` sunset; ConfidentialPool reflection is canonical |
| Full Penumbra sealed-swap DEX | Domain D; not LC hinge |
| Production code changes in LC crates | Documentation-only deliverable |
| Blocking Headstash / Tacit BTC path / DEX on ZEC TZE | Explicitly **non-blocking** per CLARITY §2 and §3.3 |

---

## 8. Progression-grade checklist

Engineers may call the **LC hinge progression-grade** when all items are checked for the corridor under test.

### 8.1 Inventory & separation

- [ ] Class A vs B vs C corridors named and not conflated in APIs  
- [ ] Attestation and hashmerchant labeled non-Tier-0 for value mint  
- [ ] Absolute monorepo paths cited for each LC in use  

### 8.2 Crosslink (class B) corridor

- [ ] Create-client + update tests green (`LC-CC-*`)  
- [ ] Membership accept/reject green (`LC-CC-20`–`25`)  
- [ ] Mint mock gated on membership (`HINGE-01`)  
- [ ] Domain binding reject cases green (`LC-DOM-*`)  
- [ ] Frozen client blocks mint (`LC-MB-02`)  
- [ ] Lag policy documented and tested (`LC-STALE-*`)  
- [ ] Misbehaviour detection implemented **or** explicitly “manual freeze only” for demo  

### 8.3 Reflection (class A) corridor

- [ ] Public values match Tacit `BitcoinRelayPublicValues` mint-critical subset: `bitcoinPoolRoot`, `bitcoinSpentRoot`, **`bitcoinBurnRoot`**, `bitcoinHeight` (Domain B §3.1)  
- [ ] `knownBitcoinBurnRoot` / current burn-root pin enforced (non-zero empty IMT sentinel)  
- [ ] Confirmation depth \(K\) enforced  
- [ ] **H-1:** ordinary spent-set membership does **not** authorize mint (`LC-RF-22`)  
- [ ] Stale/zero burn root rejects mint (`LC-RF-23`)  
- [ ] Happy burn-set + pool membership mint mock (`LC-RF-20`, `HINGE-02`) + claimId-once  
- [ ] Domain B asset_id mapping frozen for pilot assets  
- [ ] destCommitment mismatch rejects (`LC-DOM-06`)  

### 8.4 Cosmos / Eth IBC (class C) corridor (if used)

- [ ] Packet membership → private effect only with explicit packet schema  
- [ ] No silent promotion of ICS-20 escrow mint to “shielded bridge” without note circuit  

### 8.5 Compose

- [ ] Domain A claim uses LC or declared eligibility root  
- [ ] Domain B maps every mint to a Tacit surface + trust tier (§1–§3 of mapping SPEC)  
- [ ] Domain D accepts LC-minted notes into shared set  
- [ ] Domain E end-to-end flow names certificates from this spec  
- [ ] Oracle path tested to **reject** mint (`HINGE-03`)  
- [ ] ZEC/TZE egress **not** treated as blocker for A/B/D tracks (§3.3)  
- [ ] Msg names owned by first B+C implementation sprint; SPECs stay logical  

### 8.6 Ops honesty

- [ ] Transparent spine published: client tip heights, roots (incl. burn root on class A), lag  
- [ ] UX does not claim finality stronger than LC class  
- [ ] wasm signature-verify trust (if any) documented for Crosslink deploys  
- [ ] Tier-0 Crosslink **mint** marketing blocked until sig verify + conservation schema; mock `HINGE-01` OK if labeled  

---

## 9. Quick reference — “what must verify before private mint?”

```text
                    ┌─────────────────────────────┐
  Foreign chain     │  Finality / header update   │  ← UpdateClient / reflection attest
                    └──────────────┬──────────────┘
                                   │ consensus root / pool+spent+burn roots @ height H
                                   ▼
                    ┌─────────────────────────────┐
                    │  Membership / burn proof    │  ← bridge-burn under bitcoinBurnRoot
                    │                             │    (+ pool membership); not spent-only
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────┴──────────────┐
                    │  Domain binding + claimId   │  ← no replay, correct asset/dest
                    │  + conservation opening     │  ← Domain B A9–A11
                    └──────────────┬──────────────┘
                                   │
                                   ▼
                         AllowPrivateMint(note)
```

| If you only have… | Then… |
|-------------------|--------|
| Header update | Advance tip — **no mint** |
| Membership without domain binding | **no mint** |
| Spent-set / ordinary nullifier only | **no mint (H-1)** |
| Oracle / VE root alone | **no value mint** |
| Full row from §3 table (incl. burn root for class A burns) | **mint allowed** (subject to lag/freeze + conservation) |

---

## 10. References (absolute)

| Resource | Path |
|----------|------|
| Program plan §8.3 LC hinge | `daily-driver/plans/terp-private-shielded-dex-bridge/PLAN.md` |
| Domain B mapping (H-1, burn set, A1–A17, T1–T12) | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/SPEC-tacit-bridge-mapping.md` |
| Team clarity (burn/mint vs escrow; ZEC/TZE non-blocking) | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/CLARITY-headstash-sets-and-bridge-models.md` |
| Crosslink LC (terp-rs) | `/Users/returniflost/abstract/terp-core/crates/terp-rs/crates/crosslink/light-client/` |
| Crosslink 08-wasm | `/Users/returniflost/abstract/terp-core/crates/solidity-ibc-eureka/programs/cw-ics08-wasm-crosslink/` |
| Crosslink LC (Eureka package) | `/Users/returniflost/abstract/terp-core/crates/solidity-ibc-eureka/packages/crosslink/light-client/` |
| Eth LC + 08-wasm | `.../packages/ethereum/light-client/`, `.../programs/cw-ics08-wasm-eth/` |
| TM + SP1 ICS-07 | `.../packages/tendermint-light-client/`, `.../contracts/light-clients/sp1-ics07/` |
| Attestation LC | `.../contracts/light-clients/attestation/` |
| Bitcoin reflection amendment | `/Users/returniflost/abstract/terp-core/crates/tacit/spec/amendments/SPEC-BITCOIN-REFLECTION-AMENDMENT.md` |
| Hashmerchant | `/Users/returniflost/abstract/terp-core/x/hashmerchant/` |
| ZSA spectrum | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/zsas.md` |

---

## 11. Changelog

| Date | Change |
|------|--------|
| 2026-07-20 | Domain C initial LC hinge SPEC |
| 2026-07-20 | H-1 fix: `bitcoinBurnRoot` in class A public values / ClientState analogue; explicit spent-only mint reject; LC-RF-22/23/24/30 + LC-DOM-06; §3.3 ZEC/TZE non-blocking + Crosslink readiness; Domain B section cross-links |

---

*End of Domain C deliverable. No production LC code modified.*
