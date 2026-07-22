# SPEC: Public Distribution → Private Claim (Orchard-delta Headstash)

**Status:** curated interface + test SPEC (research complete; not a circuit rewrite)  
**Domain:** Headstash / Orchard / ZSA alignment  
**Date:** 2026-07-20  
**Owner program:** Terp private-bridge + private DEX (Tacit-threaded)  
**Crate scope:** `/Users/returniflost/abstract/terp-core/crates/headstash`  
**Related:**  
- Sprint: `daily-driver/.../sprints/2026-07-19-orchard-delta-headstash.md`  
- Graph: `daily-driver/.../HEADSTASH-CONVERSATION-GRAPH.md`  
- ZSA: [`zsas.md`](./zsas.md) (ZIP 227)  
- Parent plan: private-shielded-dex-bridge `PLAN.md`
- **Team clarity (additive privacy sets):** [`CLARITY-headstash-sets-and-bridge-models.md`](./CLARITY-headstash-sets-and-bridge-models.md) §1

**Design mandate:** each **new** Headstash is a **powered, additive** privacy/eligibility set (manifold-registered). Anonymity and eligibility **grow** with new drops; a new Headstash does **not** replace or orphan prior sets.

---

## 0. One sentence

State—and test—the **minimal Orchard-compatible delta** that turns a **public token distribution set** into a **private claim to an unassociated address**, so the claim output can later compose into Tacit-threaded private DEX / bridge notes, without inventing a permanent dead-end circuit fork.

---

## 1. Current Headstash model (fields, public set, claim path)

### 1.1 Product shape

| Aspect | Today |
|--------|--------|
| **Public side** | Genesis **distribution Merkle set**: eligibility leaves for (eligible key, denom, fixed amount, index) |
| **Private side** | Halo2 proof that claimant knows `esk` for the eligibility pubkey, leaf is under `anchor`, and derived nullifier / note commitment are consistent |
| **Settlement** | CosmWasm `cw-headstash`: verify proof, record nullifier, mint/send transparent tokens to **claim recipient** `rr` |
| **Factory** | `manifold` multi-drop registry |

### 1.2 Paths (source of truth)

| Layer | Path |
|-------|------|
| Circuit crate | `crates/headstash/circuit/` (`zk_headstash`) |
| Circuit core | `circuit/src/circuit.rs` — `Circuit`, `Instance`, `K=18` |
| Note algebra | `circuit/src/note.rs`, `note/commitment.rs`, `note/nullifier.rs` |
| Action shell | `circuit/src/action.rs` |
| Values / denom | `circuit/src/value.rs` (`NoteValue`, `NoteDenom`, `HeadstashValue`) |
| Keys | `circuit/src/keys.rs` (`EligibleSk`, Orchard-style FVK/nk, secp256k1 eligibility) |
| Recipient | `circuit/src/address.rs` (`RecpAddr` = 32-byte canonical Cosmos addr bytes → Poseidon/`recp_to_fp`) |
| Genesis leaf + Merkle | `circuit/src/circuit/headstash_merkle_tree.rs`, `circuit/src/tree.rs`, suite `HeadstashSinsemillaTree` |
| Suite / test data | `circuit/src/suite/suite.rs` (`HeadstashCircuitSuite`, bitwise + tree traits) |
| Deploy suite | `test-press/src/suites/headstash.rs` (`HeadstashSuite` + manifold) |
| Claim contract | `contracts/cw-headstash/src/headstash.rs`, `msg.rs` |
| Token strategies | `contracts/cw-headstash/src/tokenfactory.rs` |
| Genesis CSVs / scripts | `circuit/genesis/`, `scripts/` |
| Artifacts | `circuit/artifacts/headstash_vk.bin`, `data/headstash/{proving,verifying}_key.bin` |
| Docs (historical) | `docs/circuit/*`, `README.md`, Claude memory `headstash-project.md` |

### 1.3 Note fields (Headstash today)

From `circuit/src/note.rs` — a discrete claimable unit:

| Field | Type | Role |
|-------|------|------|
| `recipient` | `RecpAddr` | Claim payout address (Cosmos canonical 32 bytes, hashed to Fp in circuit) |
| `v` | `NoteValue` (`u64`) | Amount (fixed-denom pieces via `FIXED_AMOUNTS`) |
| `nd` | `NoteDenom` | Asset tag: `blake3(denom_str)` with top bits cleared for field canonicity (`NoteDenom::new_for_proof`) |
| `esk` | `EligibleSk` | secp256k1 secret for **eligibility** (not Orchard spend auth) |
| `fdi` | `u64` | Fixed-denomination index within `(addr, token)` for leaf uniqueness |
| `rho` | `Rho` | Orchard-style creation id (from spent nullifier when chaining actions) |
| `rseed` | `RandomSeed` | ZIP 212 PRFs → `psi`, `esk` (Orchard path), `rcm` |

**Note commitment message** (`note/commitment.rs` `NoteCommitment::derive`) is Sinsemilla `CommitDomain` over bits:

```text
nd || v || fdi || recp_fp || esk_pallas || rho[0..L_ORCHARD_BASE] || psi[0..L_ORCHARD_BASE]
  with trapdoor rcm
```

**Nullifier** (`note/nullifier.rs`): Orchard-style  
`nf = extract_p( K * (nk.prf_nf(rho) + psi) + cm )` with personalization `"z.cash:Orchard"`.

### 1.4 Public instance (claim public inputs)

Offsets in `circuit/src/circuit.rs`:

| Index | Constant | Field | Meaning |
|------:|----------|-------|---------|
| 0 | `ANCHOR` | `Anchor` | Genesis distribution Merkle root |
| 1 | `HS_ND` | `NoteDenom` → Fp | Token denomination (field form) |
| 2 | `HS_V` | `NoteValue` | Amount |
| 3 | `RECP` | `RecpAddr` → Fp | Recipient binding |
| 4 | `NF_OLD` | `Nullifier` | Claim nullifier (double-spend) |
| 5 | `CMX` | `ExtractedNoteCommitment` | x-coordinate of note commitment |

Wire / contract mirror: `HeadstashInstances` in `contracts/cw-headstash/src/headstash.rs`:

```text
{ anchor, nd, v, nf, recp, cmx } + proof p + raw recipient rr
```

`Instance::to_bytes()` layout (168 bytes):  
`anchor(32) || nd_fp(32) || v_le(8) || nf(32) || recp_raw(32) || cmx(32)`.

**Important current gap:** `synthesize` only `constrain_instance`s **ANCHOR, NF_OLD, CMX**.  
`HS_ND`, `HS_V`, `RECP` are present in `Instance` / `to_halo2_instance` and used by the contract as public bytes, but are **not yet forced equal to witnessed cells** in-circuit. Closing this gap is part of the delta (see §4, H5).

### 1.5 Witnesses (private)

| Witness | Role |
|---------|------|
| `path`, `pos` | Merkle authentication path for genesis leaf |
| `esk`, `epkx`, `epky` | secp256k1 eligibility keypair (pairing constraint) |
| `nk` | Nullifier deriving key (today **witnessed**; full HKDF-from-esk link is a known review item) |
| `fdi`, `v`, `nd`, `recp` | Leaf / note content |
| `rho_old`, `psi_old`, `rcm_old`, `cm_old` | Old note integrity for nullifier + cmx |

### 1.6 Public distribution set (genesis tree)

Construction (suite `HeadstashSinsemillaTree::gen_headstash_tree` / `derive_leaf`):

1. Input JSON: map of eligible address → list of `{ name, amount }`.
2. Split each amount into `FIXED_AMOUNTS` pieces (`constants/fixed_bases.rs`).
3. For each piece:  
   - `epk` natives from address-as-sk material (`derive_epk_natives`)  
   - `nd = NoteDenom::new_for_proof(token_name)`  
   - `v = fixed_amount`, `fdi = index`  
   - **Leaf** = Sinsemilla hash over `(epk_x, epk_y, nd, v, fdi)` with leaf canonicity gates (`headstash_merkle_tree.rs`).
4. Tree: Sinsemilla MerkleCRH with `MERKLE_CRH_PERSONALIZATION`, depth Orchard-family (`MERKLE_DEPTH_ORCHARD`).
5. Root stored on-chain as `InstantiateMsg.genesis_root` / config `gr`.

**This tree is not the Orchard note-commitment tree.** It is a **public eligibility set**. That is the primary semantic delta vs upstream Orchard.

### 1.7 Claim path (end-to-end)

```text
[off-chain]  genesis JSON → leaf + Merkle path + note + Instance + Halo2 proof
      │
      ▼
ExecuteMsg::ProcessHeadstash { claims: Vec<HeadstashNote> }
      │
      ├─ batch nullifier uniqueness (in-msg + storage Map "nullifiers")
      ├─ nd ∈ configured TokenStrategy proof_representation
      ├─ halo2_proof_instance_verify(cid, proof, instance_bytes)
      ├─ recp field ↔ Poseidon/hash of raw rr (CanonicalAddr)
      └─ mint (TokenStrategy::NewFungible) or bank send (ExistingFungible) to rr
```

Primary code: `process_headstash` in `contracts/cw-headstash/src/headstash.rs`.

### 1.8 Action type (today)

`Action<A>` (`action.rs`) is a **slim Orchard shell**: `nf`, `cmx`, `v`, `authorization` — with `rk`, `encrypted_note`, `cv_net` commented out.  
`Circuit::from_action_context` still requires `Rho::from_nf_old(spent_nf) == output_note.rho()`, but `from_action_context_unchecked` **ignores** `_output_note` for witness construction. Net: MVP is **one-time eligibility spend → transparent payout**, not a full Orchard spend+output into a shielded pool.

### 1.9 Dual suite naming (ops note)

| Name | Location | Role |
|------|----------|------|
| `HeadstashCircuitSuite` | `circuit/src/suite/suite.rs` | Merkle test data, leaf hash, bitwise helpers (often zero-sized) |
| `HeadstashSuite` | `test-press/src/suites/headstash.rs` | Deploy: circuit upload + manifold + cw-headstash |

Do not confuse them when wiring harnesses.

### 1.10 Additive privacy sets (mandate)

> **Source of truth:** [`CLARITY-headstash-sets-and-bridge-models.md`](./CLARITY-headstash-sets-and-bridge-models.md) §1.  
> **This section freezes design + test IDs only.** Multi-root / OR-membership **circuit work is out of this round** (after single-set H1–H6 green).

#### Mandate (product)

Each **new** Headstash is a **new public eligibility set** that is **additive** to the private anonymity surface — not a replacement that orphans prior drops.

```text
Headstash 0 → eligibility root_0 → claims join privacy pool P
Headstash 1 → eligibility root_1 → claims also join P (or proven extension P′)
Headstash 2 → eligibility root_2 → same pattern

Anonymity set size grows with cumulative successful claims across drops.
```

| Layer | Role |
|-------|------|
| **Public eligibility set \(i\)** | Merkle (or manifold-registered) distribution for drop \(i\) |
| **Claim / nullifier** | One-shot per leaf under set \(i\); **domain-separated** so set \(i\) claim cannot replay as set \(j\) |
| **Privacy set (note pool)** | Shared private surface after claim — **grows** with each Headstash; same island as private DEX / bridge mint when product allows |

#### Manifold registry

| Concern | Design |
|---------|--------|
| **Registry** | `manifold` lists active Headstashes / assets / strategies (today: multi-drop factory) |
| **Roots first-class** | Each drop has its own eligibility `root_i` (or epoch) — not one global tree forever |
| **Composition policy** | Prefer one composition policy (OR-membership / root-list commitment later) over dead-end per-drop circuit forks |
| **Powered + additive** | New drop may need new genesis tree + VK/config as needed; must **not** reset anonymity |

#### Nullifier domain separation across roots

| Rule | Meaning |
|------|---------|
| **Per-root claim uniqueness** | Nullifier of a claim under `root_i` is consumed once for that eligibility set |
| **Cross-root non-replay** | A nullifier (or claim id) valid under `root_i` **must not** authorize a second claim under `root_j` without an explicit, separate leaf under \(j\) |
| **Domain separation** | Scope nullifiers by `(distro_id \| root_i \| …)` or equivalent personalization so registries cannot be confused |
| **Shared privacy pool** | After claim, notes may share one spend nullifier domain for the **private** set — that is distinct from **eligibility** nullifier domain |

#### Design-only test IDs (not implementing multi-root circuit this round)

| ID | Scenario | Expect | Layer (when built) |
|----|----------|--------|--------------------|
| **H7** | Multi-root accept-any: claimant proves membership in **any** active manifold root \(0..N\) without revealing which; valid leaf under `root_k` accepted | **accept** | circuit + manifold registry (later) |
| **H8** | Wrong-set nullifier reject: nullifier / claim material from eligibility set \(i\) submitted against set \(j\) (or replay across roots) | **reject** | circuit domain-sep + contract registry (later) |

**This round:** document H7/H8; pure structural stubs optional. **Do not** expand Halo2 claim circuit for OR-membership or multi-anchor lists until single-set H1–H6 are progression-grade.

**Relation to H1–H6:** H1–H6 remain **single-set** (one `anchor`). H7/H8 are additive-manifold extensions, not replacements.

#### Explicit non-implement (this round)

- Multi-root claim circuit / MockProver vectors for H7–H8  
- OR-membership gadgets or root-list commitments in-circuit  
- Changing `Instance` layout for multi-anchor before single-set instance completeness (nd/v/recp constraints)

See also §6.2 (H7/H8), §6.3 (other optional single-set extensions), and Part I deferrals in [`SEAM-FREEZE-CHECKLIST.md`](./SEAM-FREEZE-CHECKLIST.md) §5.

---

## 2. Orchard parent semantics — what we keep

Reference: Zcash Orchard (ZIP 224 / protocol §4 Action, §4 note commitments & nullifiers), in-repo fork of orchard-family code under `crates/headstash/circuit`.

| Orchard concept | Keep? | Headstash use |
|-----------------|-------|---------------|
| **Pallas/Vesta + Halo2** | **Keep** | Circuit field Fp = pallas::Base; params on vesta::Affine (see memory note) |
| **Sinsemilla CommitDomain / HashDomain** | **Keep** | NoteCommit + MerkleCRH + leaf hash |
| **Note ρ, ψ, rcm (ZIP 212 rseed PRFs)** | **Keep** | Nullifier uniqueness chain + commitment trapdoor |
| **Nullifier algebra** `DeriveNullifier_nk(ρ,ψ,cm)` | **Keep** | Double-claim prevention |
| **cmx = extract_p(cm)** | **Keep** | Public commitment fingerprint |
| **Merkle depth / CRH personalizations** | **Keep** (domain-separated for genesis) | Eligibility tree reuses gadgets; personalization constants must stay explicit |
| **Action as spend+output atomic unit** | **Keep as target model** | Today partial; claim should *emit* a private output note for pool/DEX |
| **Value commitment cv_net + balance** | **Defer / optional** | Transparent payout path does not need cv; shielded pool later does |
| **rk / SpendAuth redpallas** | **Defer** | Eligibility is secp256k1 pairing, not Orchard ask/ak |
| **In-band note encryption** | **Defer** | Module present (`note_encryption.rs`) but claim path uses transparent `rr` |
| **enable_spend / enable_output flags** | **Defer** | Orchard dummy notes; claim is always spend-enabled eligibility |
| **Shared note-commitment tree as eligibility** | **Drop as design** | Eligibility is a **separate public distribution tree** |

### 2.1 Claim as an Orchard-shaped “Action”

Intended mental model (target, not full rewrite this sprint):

```text
Action_claim:
  Spend:  eligibility leaf under distribution root (public set)
  Output: private note (or cmx + encrypted payload) to unassociated address
  Reveal: nf (nullifier), anchor (distro root), asset tag, amount policy as needed
```

**Privacy goal:** public graph shows “someone claimed under root R with nf N”; does **not** link N to the eligibility address or to the payout/viewing identity beyond intentional bindings.

---

## 3. ZSA (ZIP 227) relevance

Source: `docs/plans/spectrum/zsas.md` (ZIP 227 OrchardZSA **issuance**). Transfer/burn is ZIP 226 (out of scope here).

### 3.1 What ZIP 227 gives us (read-only alignment)

| ZIP 227 idea | Meaning for Headstash / bridge |
|--------------|--------------------------------|
| **Asset Identifier** | Globally unique asset id from issuer + asset description (versioned encoding) |
| **Asset Digest / Asset Base** | Digest → protocol-specific base embedded in notes |
| **Issuance Action / Bundle** | Transparent issuance of custom assets; signed by issuer (`isk`/`ik`, BIP-340 secp) |
| **Transparent issuance first** | Supply is trackable; no private mint of asset types in v1 of ZIP |
| **Bridge-as-issuer** | Explicit use case: wrap foreign-chain assets by issuing ZSAs under a bridge issuer key |
| **finalize flag** | Cap further issuance of a given asset |

### 3.2 Mapping to Terp Headstash

| Concern | Near term (this SPEC) | Later (ZSA-aligned) |
|---------|----------------------|---------------------|
| Asset tag on claim | `NoteDenom` = field-canonical hash of denom string | Map `nd` → **AssetId / AssetBase** (or dual-encode) |
| Who “issues” airdrop supply | TokenFactory / escrow on CosmWasm | Bridge or drop **issuer** authorization analogous to IssueAuthSig |
| Public eligibility set | Genesis Merkle root | Can remain off ZIP 227; issuance still transparent |
| Private transfer after claim | Out of scope here | ZIP 226-style multi-asset notes / Tacit pool notes |
| Supply tracking | Contract balances + nullifier set | Transparent issuance counters + nullifiers |

### 3.3 Bridge-as-issuer pattern (compose later)

```text
Foreign chain finality (LC / reflection)
        │
        ▼
Issuer (bridge) produces transparent IssuanceAction(s)  [ZIP 227 spirit]
        │
        ▼
Notes with AssetBase enter private pool
        │
        ▼
DEX spend/output under same multi-asset anonymity set   [Tacit / ZIP 226 spirit]
```

Headstash airdrop is **parallel ingress**: public distribution set → claim nullifier → value into same note language. It is **not** a substitute for LC-proven bridge mint, but claim **output note schema** must not block that merge.

### 3.4 Oracle rule (program-level)

**Oracles / vote extensions (hashmerchant) must never mint balances.**  
They may bound prices for DEX (`min_out`). Issuance authority for assets is issuer keys / tokenfactory / proven bridge Δ — never VE.

---

## 4. Delta table

Legend: **K** = keep as-is · **C** = change (document + implement later) · **D** = drop / do not depend on · **L** = later (ZSA / pool)

| Orchard (parent) | Headstash today | Proposed (keep/change/drop) | ZSA later |
|------------------|-----------------|-----------------------------|-----------|
| Note `{d, pk_d, v, ρ, rseed}` | Note `{recp, v, nd, esk, fdi, ρ, rseed}` | **C**: keep ρ/rseed/v; treat `nd` as asset tag; `esk` eligibility only; `recp` → claim destination / output owner binding | Add **AssetBase** field or encode into `nd` canonically |
| NoteCommit message | `nd‖v‖fdi‖recp‖esk‖ρ‖ψ` | **K** structure; **C** canonicity of `nd` (H4); document bit lengths | AssetBase bits in commit domain (ZIP 226 note) |
| Nullifier | Orchard formula + Poseidon gadget path | **K** algebra; **C** enforce `nk` derivation from `esk` in-circuit if ownership requires it | Same nf space per pool |
| Action spend+output | Partial Action; output ignored in circuit ctor | **C**: claim **must** define output note schema (H6) even if settlement stays transparent short-term | Full multi-asset Action |
| Note commitment tree | Genesis **eligibility** tree on leaf hash | **K** separate trees; **D** conflating cm-tree with distro-tree | Pool tree separate from distro root |
| Anchor | Distro root public input | **K** | Issuance does not replace distro anchor |
| cv_net / value balance | Largely unused | **D** for transparent claim MVP; **L** for pool | Homomorphic value + asset |
| SpendAuth rk | Commented out | **D** for claim; eligibility = secp pairing | Optional issuer sig for issuance only |
| Public inputs | anchor, nd, v, recp, nf, cmx | **C**: constrain **all** instance columns; privacy of recp binding (H5) | asset id public or committed |
| Encryption | Present, unused on claim path | **L** when claim pays shielded | OrchardZSA ciphertext |
| Asset identity | string → blake3 `NoteDenom` | **C**: canonicalization rules frozen (H4) | ZIP 227 AssetId |
| Double-spend | nullifier map on contract | **K** | + issuance finalize / burn sets |
| Transparent payout | bank send / tokenfactory | **K** short-term; **C** optional “shielded claim” mode emitting only cmx | Bridge issuer mint is separate |

### 4.1 Explicit field-level claim interface (frozen for implementers)

**In (witness / public set):**

| Input | Visibility | Notes |
|-------|------------|-------|
| Distribution `anchor` | public | Matches contract `gr` |
| Merkle `path`, `pos` | private | Against genesis leaf |
| Eligibility `esk` (+ epk) | private (epk in leaf) | secp256k1 |
| `nd`, `v`, `fdi` | private in leaf; **nd,v public on instance today** | Policy choice: keep amount/denom public for transparent airdrop accounting |
| `rho`, `psi`, `rcm`, `cm` | private | Integrity |
| `nk` | private | Must bind to eligibility story |
| Claimant `recp` / `rr` | public binding | Unassociated Cosmos address |

**Out:**

| Output | Visibility | Notes |
|--------|------------|-------|
| `nf` | public | Stored once |
| `cmx` | public | Commitment to claim note / payout binding |
| Proof π | public | Halo2 |
| Optional future: encrypted note / pool insertion | — | DEX composition |
| Token transfer to `rr` | transparent chain event | Current settlement |

---

## 5. Action seams IN/OUT (private bridge + private DEX composition)

### 5.1 This domain owns

| Seam | Direction | Contract |
|------|-----------|----------|
| **Distribution publication** | IN | Merkle root + leaf schema + FIXED_AMOUNTS policy |
| **Claim proof verify** | IN | `Instance` + VK / circuit id |
| **Nullifier set** | OUT (state) | Prevent double claim |
| **Claim value release** | OUT | Transparent coins **or** private note handle |

### 5.2 Peer seams (do not implement here)

| Peer | Seam | Headstash provides | Peer consumes |
|------|------|--------------------|---------------|
| **Private DEX (Tacit)** | OUT → | Output note schema (H6): asset, value, owner binding, nf lineage | Spend note in swap circuit / confidential pool |
| **Private bridge / LC** | IN later | N/A for pure airdrop | Bridge mint notes should share asset id + nullifier domain conventions |
| **hashmerchant / VE** | IN ops only | Distro mirror, claim UX, mid prices | **Never** mint claim balances |
| **Manifold** | IN factory | Multi-root, multi-strategy deploy | Same claim interface per instance |
| **Tachyon / ragu** | L later | Aggregation / recursion | Not required for claim correctness |

### 5.3 Composition sketch (non-normative)

```text
                    ┌─────────────────────────────┐
  public distro ──► │ Headstash Claim Action      │──► nf + cmx (+ optional note)
  (Merkle root)     │  + transparent settle (MVP) │
                    └─────────────┬───────────────┘
                                  │ schema-compatible note
                                  ▼
                    ┌─────────────────────────────┐
  bridge_mint ────► │ Shared multi-asset note pool │──► private DEX spend/output
  (issuer/LC)       │  (Tacit / OrchardZSA spirit) │
                    └─────────────────────────────┘
```

**Interface rule:** Claim output fields used by DEX must be a **subset** of the agreed note schema stub (H6). Do not invent a second commitment algorithm for “DEX notes” without a mapping table.

### 5.4 Recipient privacy seam

| Goal | Mechanism |
|------|-----------|
| Eligibility address not in public inputs | Leaf hides epk inside Merkle; only anchor is public |
| Claim payout address unassociated | User chooses fresh `rr`; only `recp` hash + `rr` appear |
| No link eligibility → payout in π | Circuit must not expose epk/esk in instances (property H5) |

---

## 6. Test scenarios H1–H6 (+ extensions)

Harness language: suite builders preferred (`HeadstashCircuitSuite` / `MerkleTestDataBuilder` patterns). Tests may be red until Part I.

| ID | Scenario | Expect | Layer |
|----|----------|--------|-------|
| **H1** | Valid eligibility leaf + correct path + matching instance → proof verifies; contract accepts single claim | **accept** | circuit + contract |
| **H2** | Same nullifier claimed twice (second ProcessHeadstash or duplicate in batch) | **reject** | contract (nullifier map) |
| **H3** | Proof built against wrong distribution root / instance anchor ≠ contract `gr` | **reject** | circuit verify or contract policy |
| **H4** | Non-canonical `nd` (raw denom string not passed through `NoteDenom::new_for_proof`; top bits not cleared; alternate hash) | **reject** or **inequality** with leaf | circuit + suite |
| **H5** | Recipient binding privacy: public inputs do not contain eligibility address / epk; `recp` matches `rr` only via hash relation | **property** | circuit instances + contract check |
| **H6** | Claim output note / cmx fields map to **DEX schema stub** (§6.1) | **structural** | unit / schema test |

### 6.1 H6 — output note schema stub (DEX later)

Freeze this stub so Tacit/DEX work can proceed without waiting on full shielded claim:

```text
ClaimOutputNoteV0 {
  asset_tag:  NoteDenom | AssetIdStub,  // 32-byte field-canonical
  value:      u64,                      // NoteValue
  owner:      OwnerBinding,             // RecpAddr or future Orchard address
  cmx:        [u8; 32],                 // ExtractedNoteCommitment
  nf_claim:   [u8; 32],                 // Nullifier of claim spend
  anchor_distro: [u8; 32],              // public set root used
  // future:
  // rho, rseed, encrypted_note, pool_anchor, asset_base
}
```

**Pass criterion for H6:** every successful H1 claim can construct `ClaimOutputNoteV0` without inventing fields not already in `Instance` + note.

### 6.2 Additive multi-root (design IDs — §1.10)

| ID | Scenario | Expect | Status this round |
|----|----------|--------|-------------------|
| **H7** | Multi-root accept-any (membership in any manifold root \(0..N\), which root hidden) | **accept** | **Design only** — no multi-root circuit |
| **H8** | Wrong-set nullifier reject (claim material for root \(i\) used under root \(j\)) | **reject** | **Design only** — domain-sep policy + future tests |

See CLARITY §1 and §1.10 above. Implement **after** single-set H1–H6 green.

### 6.3 Recommended single-set extensions (optional)

| ID | Scenario | Expect |
|----|----------|--------|
| H9 | Batch claims: two distinct nullifiers, same or different recipients | accept; balances sum |
| H10 | `nd` not in TokenStrategy | reject |
| H11 | Invalid secp key pairing (epk ≠ esk·G) | reject proof |
| H12 | Instance column gap: witness nd ≠ public HS_ND (once constrained) | reject |
| H13 | Wrong Merkle path / position | reject proof |
| H14 | Mutated cmx with valid-looking other fields | reject |

> **ID note:** Prior draft used H7–H11 for single-set extensions; those IDs are **reassigned** so **H7/H8** are reserved for multi-root additive-set mandate (§1.10). Part T still documents H12 as the instance-gap case (same meaning).

### 6.4 Vector construction notes (from suite)

- Leaves: `HeadstashSinsemillaTree::derive_leaf` / `leaf_hash(epk_x, epk_y, nd, v, fdi)`.
- `nd`: always `NoteDenom::new_for_proof(raw)` / `HeadstashBitwiseInstance::derive_nd`.
- `recp`: `derive_recp` / `recp_to_fp`.
- Nullifier: `Nullifier::derive` / suite `derive_nk`.
- Prefer suite SSOT over ad-hoc test bins (Claude review 9eb8a4b7).

---

## 7. Harness commands (existing tests)

> Adjust features if the local workspace requires `circuit` / `interface` / `multicore`. Paths are absolute monorepo roots.

### 7.1 Circuit library tests

```bash
cd /Users/returniflost/abstract/terp-core/crates/headstash/circuit
cargo test --lib
# filtered examples (names evolve with Part T):
cargo test --lib note_commit
cargo test --lib -- secp256k1
cargo test --lib -- zip32
```

### 7.2 cw-headstash contract tests

```bash
cd /Users/returniflost/abstract/terp-core/crates/headstash/contracts/cw-headstash
cargo test
# optional focused:
cargo test --lib
cargo test wavs
```

### 7.3 Workspace / justfile entrypoints

```bash
cd /Users/returniflost/abstract/terp-core/crates/headstash
# root justfile (if scripts present):
just test
# circuit justfile:
cd circuit && just test   # cargo test --locked
```

### 7.4 Test-press / deploy suite

```bash
cd /Users/returniflost/abstract/terp-core/crates/headstash/test-press
cargo test
# binary helpers (when keys/data present):
# cargo run --bin gen_headstash_notes -- <genesis.json> <addr>
# cargo run --bin cc_headstash
```

### 7.5 Genesis / tree generation (data prep for H1–H3)

```bash
# scripts (historical path names may reference zk-crates; use monorepo paths)
cd /Users/returniflost/abstract/terp-core/crates/headstash
# Node distribution tooling:
# cd scripts && node main.js ...
# Suite-driven tree: via HeadstashCircuitSuite::gen_headstash_tree in a bin or test
```

### 7.6 Suggested Part T filters (once H1–H6 land)

```bash
cd /Users/returniflost/abstract/terp-core/crates/headstash/circuit
cargo test --lib h1_valid_claim
cargo test --lib h2_double_claim
cargo test --lib h3_bad_root
cargo test --lib h4_noncanonical_nd
cargo test --lib h5_recipient_privacy
cargo test --lib h6_claim_output_schema
```

Or a single module:

```bash
cargo test --lib orchard_delta::
```

### 7.7 Checkpoint template (paste into sprint card)

```text
command: cd .../crates/headstash/circuit && cargo test --lib -- <filter>
result:  <pass/fail counts>
date:
notes:
```

---

## 8. Progression-grade checklist (this domain)

- [ ] **Object boundary clear:** only Headstash claim + Orchard delta; no AMM/LC/oracle mint  
- [ ] **Delta table frozen** in this SPEC (or linked card) and reviewed against code paths in §1  
- [ ] **H1–H4** specified with pass/fail; implemented tests run (may be red until Part I)  
- [ ] **H5** property stated (public instances checklist)  
- [ ] **H6** schema stub agreed with DEX/bridge consumers  
- [ ] **H7–H8** multi-root additive-set IDs documented (§1.10); circuit deferred  
- [ ] **Additive / manifold mandate** restated (CLARITY §1); single-set first  
- [ ] **Harness commands** executed; checkpoint log filled  
- [ ] **Instance completeness:** plan to `constrain_instance` nd, v, recp (gap §1.4)  
- [ ] **nd canonicity** policy documented (H4)  
- [ ] **Action seams** listed for private bridge + DEX (§5)  
- [ ] **ZSA column** understood as later; no premature full OrchardZSA rewrite  
- [ ] **Oracle never mints** restated in any VE-adjacent ops docs  
- [ ] **No scope creep:** no AMM circuit, no LC client, no Tachyon/ragu impl, no hashmerchant rewrite  

---

## 9. Explicit non-goals

| Non-goal | Why |
|----------|-----|
| Full private AMM / confidential swap circuit | Separate DEX sprint; Tacit SSOT |
| Light client / Crosslink LC implementation | Interop fabric sprint |
| Tachyon bundle/stamp or ragu PCD | Aggregation later; not claim correctness |
| Full OrchardZSA (ZIP 226+227) implementation | Align interfaces only; transparent claim first |
| Rewriting entire Halo2 stack / keygen infra | Minimal delta |
| Oracle / VE pricing logic | Bounds only; **never mint** |
| UI ModuleId / oline deploy / dao-dao registration | Product surface after circuit freeze |
| Replacing manifold / tokenfactory economics | Settlement strategy is orthogonal |
| Formal verification of full circuit | Tracked elsewhere (TODO.md) |

---

## 10. Implementation guidance (minimal delta, for Part I)

Ordered so H1–H4 can green without a full Orchard rehost:

1. **Freeze leaf + Instance encoding** (this SPEC §1, §4.1).  
2. **Close public-input constraints** for `HS_ND`, `HS_V`, `RECP` (or explicitly document intentional free witnesses — not recommended).  
3. **Canonical `nd` only** via `NoteDenom::new_for_proof` in all builders and contract `proof_representation`.  
4. **Nullifier uniqueness** remains contract-enforced (H2); circuit derives nf.  
5. **H6 stub** as pure structural type in suite/tests — no pool write yet.  
6. **Do not** reintroduce full SpendAuth/cv_net unless shielded pool sprint pulls them.  
7. Suite remains SSOT for vectors (avoid new one-off test bins).

---

## 11. Open risks / known review debt

| Item | Source | Impact |
|------|--------|--------|
| `nk` witnessed without full in-circuit bind to `esk` | Claude deep review / README critique | Ownership story weaker than pairing alone |
| Public columns nd/v/recp not `constrain_instance`d | `circuit.rs` synthesize | Verifier may not force those instance slots |
| Dual `HeadstashSuite` names | memory note | Harness confusion |
| Transparent claim vs private note pool | product | H6 bridges the gap without overclaiming privacy of balances on-chain today |
| `nd` hash domain (blake3 + bit clear) vs ZSA AssetId | this SPEC | Need explicit mapping before multi-asset pool merge |

---

## 12. Document control

| Version | Date | Change |
|---------|------|--------|
| 0.1 | 2026-07-20 | Initial curated Orchard-delta SPEC (Domain A): inventory, ZSA alignment, H1–H6, harness, non-goals |
| 0.2 | 2026-07-20 | §1.10 Additive privacy sets mandate (CLARITY): manifold, nullifier domain-sep, design-only H7/H8; renumber optional single-set extensions |
| 0.3 | 2026-07-20 | §13 Poseidon-v1 public distro tree (ADR); note commit may remain Sinsemilla |

**Consumers:** circuit implementers (Part I), suite authors (Part T), private DEX note schema owners, bridge issuance designers (read-only ZSA column).

---

## 13. Public distribution tree = Poseidon-v1 (ADR)

**Status:** accepted product decision — integrated (pure + suite defaults + in-circuit gadgets + contract root policy)  
**ADR:** [`ADR-POSEIDON-DISTRO-TREE.md`](./ADR-POSEIDON-DISTRO-TREE.md)  
**Code SSOT (pure):** `crates/headstash/circuit/src/distro_poseidon.rs`  
**In-circuit:** `circuit/src/circuit/distro_poseidon_gadget.rs`; wired in `Circuit::synthesize`  
**Suite:** Poseidon defaults; `PartialClaimNote`; `suite_backed_claim_pair`; depth-32 path root via `to_circuit_path_and_root_poseidon_v1`  
**Contract:** anchor must match registered root; Poseidon-v1 domain policy on claims; multi-claim nullifier domain-sep  
**ZKVM keys:** `ProvingKey::build_and_write` footer `K=18`, `i=6` public inputs (168-byte instance)  
**Open:** suite-backed MockProver full-green with non-zero note values (note-commit bit packing); full Halo2 batch prove (keygen cost)

### 13.1 Split of hash domains

| Surface | Hash | Domain tag / personalization | Notes |
|---------|------|------------------------------|-------|
| **Public inclusion / distribution Merkle set** | **Poseidon-v1** | leaf `"terp-hs-distro-leaf-v1"`; CRH `"terp-hs-distro-crh-v1"` | Long-lived public integrity; PQ-friendlier posture for listed identities |
| **Private note commitment** | Orchard **Sinsemilla** (for now) | existing note-commit personalization | Unchanged this pass; separate threat model |
| **Nullifier PRF** | Poseidon (`P128Pow5T3`, arity 2) | Orchard `prf_nf(nk, ρ)` (no distro tag) | Unrelated to distro tree |

**Do not** use Sinsemilla for **new** public distro trees. Historical Sinsemilla genesis roots may still verify under `distro_hash_domain = sinsemilla-legacy` only if explicitly versioned; **new** manifold / Headstash registrations use **`poseidon-v1` only**.

### 13.2 Poseidon-v1 parameters (must match off-chain builders and circuit)

| Param | Value |
|-------|--------|
| Spec | `P128Pow5T3` (halo2_gadgets / Orchard-compatible) |
| Width `t` / rate | 3 / 2 |
| Mode | `ConstantLength<N>` |
| Leaf arity `N` | **6**: `[tag, epk_x, epk_y, nd, v, fdi]` |
| CRH arity `N` | **4**: `[tag, layer, left, right]` |
| Tag packing | UTF-8 personalization → LE bytes in `pallas::Base` (32-byte zero-pad) |
| Layer convention | `layer = 0` at leaf→parent; increments toward root (suite convention, not Orchard `l`) |
| Padding sibling | `pallas::Base::ZERO` |
| Root bytes | 32-byte LE canonical `pallas::Base::to_repr` (same as `Anchor`) |

Pure API:

- `poseidon_distro_leaf(epk_x, epk_y, nd, v, fdi) -> Fp`
- `poseidon_distro_crh(layer, left, right) -> Fp`
- `distro_hash_domain` string: `"poseidon-v1"` (`DISTRO_HASH_DOMAIN_POSEIDON_V1`)

### 13.3 Field packing (public leaf)

Unlike the Sinsemilla leaf (640-bit bitstring with y-bit and pad), Poseidon-v1 hashes **full field elements**:

- `epk_x`, `epk_y`: full eligibility pubkey coordinates as Pasta base elements (full **y**, not 1-bit).
- `nd`, `v`, `fdi`: same field forms as today’s suite leaf inputs (`v`/`fdi` as `u64` → `Fp`).

In-circuit gadgets must assign the fixed tag constants and call the same arities; see module docs on `distro_poseidon.rs`.

### 13.4 Contract / manifold migration

1. Instantiation and additive multi-root manifold carry **`distro_hash_domain`** (or assume `poseidon-v1` for all **new** Headstashes).
2. Claim path still exposes a 32-byte `anchor` / `genesis_root` / `provenance_anchor`; algorithm is **tagged**, not implied by bytes alone.
3. **Breaking:** existing Sinsemilla distro roots are not Poseidon-v1 roots. Republish trees with Poseidon helpers; do not mix domains in one tree.
4. SEAM-NOTE-OUT / H6 field roles unchanged: `provenance_anchor` remains 32 bytes; consumers that care about algorithm read the Headstash’s `distro_hash_domain`.

### 13.5 Non-goals (this section)

- Rewriting private note-commit off Sinsemilla in the same change.
- Full MockProver K=18 path for distro Poseidon (pure unit tests first).
- Post-quantum SNARK replacement.
