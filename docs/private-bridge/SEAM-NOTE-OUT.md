# SEAM-NOTE-OUT — Frozen intermediate note schema (Domain R3-A)

| Field | Value |
|-------|--------|
| **Status** | **Frozen** (structural encoding bridge; 2026-07-20) |
| **Seam ID** | `SEAM-NOTE-OUT` |
| **Direction** | Domain A (Headstash claim) / Domain B (Tacit bridge mint) **→** Domain D/E (private DEX spend, compose) |
| **Type** | Canonical intermediate: `SeamNoteOutV0` |
| **Crypto suite** | **Out of scope** for this freeze — field roles, widths, and maps only |
| **Circuit work** | **None** — pure structural; no Halo2 / MockProver |
| **Parent** | [`FLOW-private-bridge-auth.md`](./FLOW-private-bridge-auth.md) §5; reviews E-S1 / E-X1, A×D Issue 2 |

**Authority:** This document is the **single SSOT** for claim/bridge **egress** note shape consumed by DEX seams. Domains A/B **emit** into `SeamNoteOutV0`; Domain D **consumes** a documented subset as spend inputs. Implementers MUST NOT invent a fourth parallel note language.

**Related (non-authority):**

- A: [`SPEC-airdrop-orchard-delta.md`](./SPEC-airdrop-orchard-delta.md) §1.3–1.4, §6.1 `ClaimOutputNoteV0`
- B: [`SPEC-tacit-bridge-mapping.md`](./SPEC-tacit-bridge-mapping.md) §3.1, §5, §9 `BridgeMintPublic`
- D: [`SPEC-private-dex-seams.md`](./SPEC-private-dex-seams.md) §1.1–1.2, §4
- Reviews: [`REVIEW-A-D-2026-07-20.md`](./reviews/REVIEW-A-D-2026-07-20.md), [`REVIEW-E-COMPOSE-2026-07-20.md`](./reviews/REVIEW-E-COMPOSE-2026-07-20.md)
- **Off-chain persistence (not layout authority):** encrypt full 382B cleartext → envelope
  `{ciphertext, nonce, scheme: "xchacha20poly1305", cleartext_layout: "SEAM-NOTE-OUT-V0", cleartext_len: 382}` →
  hash-market `PUT /notes/{hs_id}/{addr}` (`HeadstashStore`). SSOT:
  [`tools/hash-market/docs/headstash.md`](../../../crates/terp-rs/tools/hash-market/docs/headstash.md).
  Private notes ≠ public dual-index
  [`content-distribution.md`](../../../crates/terp-rs/tools/hash-market/docs/content-distribution.md).
  **Client encrypt API:** crate `fixtures/seam_note_out` modules `envelope` / `addr` —
  `encrypt_note_out` / `decrypt_note_out` / `persist_plan_from_seam_note` /
  `note_addr_{claim,cm,pk}`; optional feature `http` → `put_note_envelope`.
  Compose thin hook: `compose_seams::persist_plan_after_bridge_mint`.

---

## 0. Purpose

Unify three **logically overlapping** note surfaces into one **typed intermediate**:

1. Headstash claim public instance / H6 stub (`ClaimOutputNoteV0`)
2. Tacit bridge mint public values + dest note leaf material
3. Private DEX multi-asset note I/O (swap spend inputs)

so compose tests (C7.x), H6, and stub DEX harnesses share one structural vocabulary **before** commitment algorithms converge.

```text
  Claim (A) ──┐
              ├─►  SeamNoteOutV0  ─►  DEX spend (D) / SEAM-SPEND
  Bridge (B) ─┘
```

**Hard rules inherited:**

- Oracle / VE **never** mint balances or invent `SeamNoteOutV0` fields.
- Bridge mint conservation: `value_out == v_burn` (equality), not “≤”.
- Claim eligibility tree ≠ privacy commitment tree `T` (CLARITY / A §1.6).

---

## 1. Canonical type — `SeamNoteOutV0`

All multi-byte integers are **little-endian** unless marked `_be`. Fixed arrays are raw bytes (no length prefix). Optional fields use presence flags; absent optional fields MUST be zero-filled when a fixed layout is required for tests.

### 1.1 Field table (normative)

| Field | Type | Bytes | Visibility | Role |
|-------|------|------:|------------|------|
| `version` | `u8` | 1 | public | Schema version; **must be `0`** for V0 |
| `domain_tag` | `[u8; 16]` | 16 | public | ASCII domain tag, NUL-padded (see §6) |
| `origin` | `u8` | 1 | public | Ingress class: `0x01` = Headstash claim; `0x02` = Tacit bridge mint; `0x00` / others = reject for product paths |
| `asset_id` | `[u8; 32]` | 32 | public on seam; private under note commitment in pool | **Terp-canonical** asset id (not raw denom string) |
| `value` | `u64` | 8 | public on claim/bridge mint statement today; private under commitment for DEX notes | Amount in **base units** after `unit_scale` |
| `owner_binding` | `[u8; 32]` | 32 | public binding / private owner material | Opaque owner handle (RecpAddr raw, diversifier digest, or destCommitment material — see maps) |
| `cm_public` | `[u8; 32]` | 32 | public (tree leaf fingerprint) | Commitment public form for membership / append narrative |
| `cm_encoding` | `u8` | 1 | public | How `cm_public` was produced (see §1.2) |
| `nullifier_lineage` | `[u8; 32]` | 32 | public | Claim nullifier **or** source burn `ν` (ingress one-shot id) |
| `nullifier_domain` | `u8` | 1 | public | Domain of `nullifier_lineage` (see §1.3) — **not** the later pool-spend `ν` |
| `provenance_anchor` | `[u8; 32]` | 32 | public | Distro root (claim) **or** source membership pin (bridge) |
| `claim_id` | `[u8; 32]` | 32 | public | Bridge re-derived `claimId`; **all-zero** for pure Headstash claim |
| `source_chain_tag_hash` | `[u8; 32]` | 32 | public | `H(source_chain_tag)` for bridge; **all-zero** for claim |
| `rcm` | `[u8; 32]` | 32 | private when present | Commitment trapdoor / blinding; **zero if `rcm_flag = 0`** |
| `rcm_flag` | `u8` | 1 | public | `0` = rcm not supplied on seam (phase-0/1); `1` = rcm present for opening tests |
| `memo` | `[u8; 64]` | 64 | optional ciphertext / tag | UX / airdrop tag; **all-zero if unused** |
| `memo_flag` | `u8` | 1 | public | `0` = no memo; `1` = memo bytes meaningful |
| `pool_domain` | `[u8; 32]` | 32 | public or fixed param | Note-set / pool instance id; zero = “demo single T default” |

**Fixed layout size (all fields present, optional zero-filled):**

```text
1 + 16 + 1 + 32 + 8 + 32 + 32 + 1 + 32 + 1 + 32 + 32 + 32 + 32 + 1 + 64 + 1 + 32
  = 382 bytes
```

**Canonical serialization order** (for structural equality / hashing tests only):

```text
version
‖ domain_tag
‖ origin
‖ asset_id
‖ value_le8
‖ owner_binding
‖ cm_public
‖ cm_encoding
‖ nullifier_lineage
‖ nullifier_domain
‖ provenance_anchor
‖ claim_id
‖ source_chain_tag_hash
‖ rcm
‖ rcm_flag
‖ memo
‖ memo_flag
‖ pool_domain
```

### 1.2 `cm_encoding` values

| Code | Name | Meaning | Source surface |
|-----:|------|---------|----------------|
| `0x00` | `UNSPECIFIED` | Invalid for accept paths | — |
| `0x01` | `ORCHARD_CMX` | `cmx = extract_p(NoteCommitment)` (32-byte x-coordinate) | Headstash A |
| `0x02` | `TACIT_KECCAK_LEAF` | Owner-free leaf `keccak(asset_id ‖ Cx ‖ Cy)` (32 bytes) | Tacit B |
| `0x03` | `ABSTRACT_LEAF_V0` | Demo/stub leaf: `H("terp-seam-leaf-v0" ‖ asset_id ‖ value_le8 ‖ owner_binding ‖ rcm)` | Harness only |
| `0x04`–`0xFF` | reserved | Reject until this SPEC amends | — |

**Shared anonymity set honesty (I5):** Product narrative “one tree `T`” requires either (a) **one** `cm_encoding` for all leaves in `T`, or (b) **dual tree + published link proof** (non-goal for v0). V0 freezes the **field** `cm_public` and tags encoding; it does **not** claim Orchard cmx == Tacit keccak leaf. See §5.

### 1.3 `nullifier_domain` values

| Code | Name | Meaning |
|-----:|------|---------|
| `0x00` | `UNSPECIFIED` | Reject on product paths |
| `0x01` | `HEADSTASH_CLAIM` | Eligibility claim nullifier (`nf` / `NF_OLD`) |
| `0x02` | `BRIDGE_BURN` | Source bridge-burn `ν` (H-1 set — not generic spent) |
| `0x03` | `POOL_SPEND` | **Not emitted by SEAM-NOTE-OUT** — pool spend `ν` is produced later by DEX/transfer (SEAM-SPEND) |

Ingress one-shot markers live in `nullifier_lineage` + `nullifier_domain` ∈ {claim, burn}. DEX spends derive a **new** pool nullifier from note opening; that is **out of this seam’s egress fields**.

### 1.4 `origin` values

| Code | Name | Emitter |
|-----:|------|---------|
| `0x01` | `ORIGIN_HEADSTASH` | Domain A claim |
| `0x02` | `ORIGIN_BRIDGE_MINT` | Domain B mint |
| other | reserved / reject | — |

### 1.5 Required vs phase-optional

| Field | Phase 0 (transparent claim MVP) | Phase 1 (append `cm` into `T`) | Phase 2 (full private note) |
|-------|----------------------------------|--------------------------------|------------------------------|
| `version`, `domain_tag`, `origin` | required | required | required |
| `asset_id`, `value` | required | required | required |
| `owner_binding` | required (`rr`/RecpAddr) | required | required (spend-auth binding) |
| `cm_public`, `cm_encoding` | required structural (`cmx` even if no pool write) | required; must be appendable encoding | required |
| `nullifier_lineage`, `nullifier_domain` | required | required | required |
| `provenance_anchor` | required (distro root) | required | required |
| `claim_id`, `source_chain_tag_hash` | zero (claim) / required (bridge) | same | same |
| `rcm` / `rcm_flag` | optional (`flag=0`) | recommended `flag=1` for openings | required for spend |
| `memo` | optional | optional | optional |
| `pool_domain` | zero or demo pin | pin shared `T` | pin shared `T` |

---

## 2. Mapping — Headstash `ClaimOutputNoteV0` / Instance → `SeamNoteOutV0`

### 2.1 Source fields (Domain A)

From A §1.4 Instance (`to_bytes` 168 bytes) and H6 stub:

| Source | Type / bytes | A name |
|--------|--------------|--------|
| `anchor` | `[u8; 32]` | `ANCHOR` / `anchor_distro` |
| `nd` | 32-byte field form (`NoteDenom` → Fp) | `HS_ND` / `asset_tag` |
| `v` | `u64` LE 8 | `HS_V` / `value` |
| `nf` | `[u8; 32]` | `NF_OLD` / `nf_claim` |
| `recp` | raw Cosmos canonical 32 bytes (hashed to Fp in circuit) | `RECP` / `owner` |
| `cmx` | `[u8; 32]` | `CMX` / `cmx` |
| (private, not on Instance) | `rcm`, `rho`, `rseed`, `esk`, `fdi` | note witnesses |

`ClaimOutputNoteV0` (A §6.1):

```text
asset_tag, value, owner, cmx, nf_claim, anchor_distro
```

### 2.2 Normative map

| `SeamNoteOutV0` field | Source | Rule |
|-----------------------|--------|------|
| `version` | const | `0` |
| `domain_tag` | const | `DOMAIN_TAG_NOTE_OUT` (§6) |
| `origin` | const | `0x01` `ORIGIN_HEADSTASH` |
| `asset_id` | `asset_tag` / `nd` | **32-byte** field-canonical `NoteDenom` bytes as used on Instance. Future: map through registry to Terp asset id (same width). Raw denom **string** is **not** placed here (H4). |
| `value` | `value` / `v` | `u64` equality; fixed-denom piece amount |
| `owner_binding` | `owner` / raw `rr` (`RecpAddr`) | 32-byte canonical Cosmos addr bytes (not Fp form). Circuit `recp` Fp is a hash of these bytes — seam stores **raw** binding. |
| `cm_public` | `cmx` | copy 32 bytes |
| `cm_encoding` | const | `0x01` `ORCHARD_CMX` |
| `nullifier_lineage` | `nf_claim` / `nf` | copy 32 bytes |
| `nullifier_domain` | const | `0x01` `HEADSTASH_CLAIM` |
| `provenance_anchor` | `anchor_distro` / `anchor` | distribution Merkle root (`gr`) |
| `claim_id` | — | **32 zero bytes** |
| `source_chain_tag_hash` | — | **32 zero bytes** |
| `rcm` | note `rcm` if available | copy or zero; set `rcm_flag` accordingly |
| `rcm_flag` | — | `0` if transparent-only H6 stub; `1` if opening material attached for phase-1 tests |
| `memo` / `memo_flag` | optional encrypted note / tag | zero / `0` unless product attaches tag |
| `pool_domain` | demo config | pin shared privacy set id, or zero for “default single T” |

### 2.3 Claim-only fields **not** carried on `SeamNoteOutV0`

| Field | Why stripped |
|-------|----------------|
| `esk` / epk | Eligibility only; must not appear on pool spend surface |
| `fdi` | Distro leaf uniqueness; not a pool note field |
| `rho`, `psi`, `rseed` | Orchard integrity; required for full Orchard-style respend later — **phase-2 private opening**, not V0 public seam |
| `nk` | Private; not public egress |
| Transparent bank/tokenfactory payout event | Settlement side-effect of MVP; **not** a note field |

### 2.4 Instance completeness note (A debt)

Until H12 / `constrain_instance` for `HS_ND`, `HS_V`, `RECP`, contract-side equality does **not** prove circuit binding. Mapping above still holds for **structural** construction from Instance bytes + note witnesses; SEAM-N* tests do not require Halo2.

---

## 3. Mapping — Tacit bridge mint public values → `SeamNoteOutV0`

### 3.1 Source: `BridgeMintPublic` (B §9) + note effect (B §3.1)

```text
BridgeMintPublic {
  source_chain_tag,       // string / tag bytes
  tacit_asset_id,         // 32 bytes foreign id
  value_u64,              // opened; == v_burn
  nullifier,              // ν of burned note
  dest_commitment_or_leaf,// destCommitment / new leaf material
  claim_id,               // re-derived
  source_pool_root,
  source_burn_root,       // bridge-burn IMT root (H-1)
  source_height,
  proof_or_lc_height,
}
```

Note effect: dest appends note with `v = v_burn`, mapped Terp `asset_id`; Tacit leaf spirit `keccak(asset_id ‖ Cx ‖ Cy)` for owner-free cross-chain form.

### 3.2 Asset id (B §5) — map **before** seam field

```text
terp_asset_id = H(
  "terp-tacit-asset-v1"
  ‖ source_chain_tag
  ‖ tacit_asset_id_bytes32
  ‖ unit_scale_be
  [ ‖ pool_domain ]
)
```

`H` for this freeze: **SHA-256** → 32 bytes (fixture-stable). Registry may precompute the same value. Unknown registry entry → **reject mint** (B A15); do not emit `SeamNoteOutV0`.

### 3.3 Normative map

| `SeamNoteOutV0` field | Source | Rule |
|-----------------------|--------|------|
| `version` | const | `0` |
| `domain_tag` | const | `DOMAIN_TAG_NOTE_OUT` |
| `origin` | const | `0x02` `ORIGIN_BRIDGE_MINT` |
| `asset_id` | registry map of `(source_chain_tag, tacit_asset_id, unit_scale)` | **Terp** `asset_id` 32 bytes (§3.2); **not** raw `tacit_asset_id` alone |
| `value` | `value_u64` | **must equal** `v_burn` (B A9) |
| `owner_binding` | `dest_commitment_or_leaf` **or** opening owner digest | Prefer 32-byte `destCommitment` pin when that is the burn-bound dest; if mint creates a new local owner note, use that owner’s 32-byte binding and still enforce A6 dest match at mint gate |
| `cm_public` | dest leaf / commitment public bytes | 32-byte leaf fingerprint actually appended (or stubbed) |
| `cm_encoding` | const for Tacit-native | `0x02` `TACIT_KECCAK_LEAF` when leaf is Tacit keccak form; harness may use `0x03` only for non-crypto stubs |
| `nullifier_lineage` | `nullifier` (`ν`) | copy 32 bytes — **bridge-burn** `ν` |
| `nullifier_domain` | const | `0x02` `BRIDGE_BURN` |
| `provenance_anchor` | `source_burn_root` (preferred) or `source_pool_root` | Prefer **burn root** for H-1 currency (B A5/A17); document which is stored if only one 32-byte slot is filled |
| `claim_id` | `claim_id` | 32 bytes; must equal re-derive `keccak(destChain ‖ destCommitment ‖ ν ‖ assetId)` (B A14) at mint gate |
| `source_chain_tag_hash` | `SHA256(source_chain_tag_utf8)` | 32 bytes |
| `rcm` / `rcm_flag` | Pedersen blinding `r` if exposed to dest mint | `flag=1` when opening available to owner; else `0` |
| `memo` / `memo_flag` | optional | zero / `0` typical |
| `pool_domain` | Terp dest pool / note-set id | **must** bind mint domain (B A16); non-zero recommended |

### 3.4 Mint-auth fields **not** on `SeamNoteOutV0` (stay on other seams)

These authenticate the mint but are **not** note body fields. They remain on `SEAM-BURN-WITNESS` / `SEAM-LC-STATE`:

| Field | Seam home |
|-------|-----------|
| `source_height`, confirmation depth K | LC state / burn witness |
| Membership proof under burn/pool root | burn witness |
| `proof_or_lc_height` | LC state |
| Reflection roots tuple full set | LC state |
| `bridgeMinted` / consumed bitmap update | host state, not note |

### 3.5 Claim vs bridge structural equality (C7.2)

For compose property C7.2, **field presence and widths** of `SeamNoteOutV0` MUST match across `origin ∈ {0x01, 0x02}`. Allowed differences:

- `origin`, `nullifier_domain`, `cm_encoding`
- `claim_id` / `source_chain_tag_hash` zero vs non-zero
- `provenance_anchor` semantics (distro root vs burn root)

**Not allowed:** missing `asset_id`, `value`, `owner_binding`, `cm_public`, or `nullifier_lineage` on either path.

---

## 4. Mapping — `SeamNoteOutV0` → DEX swap spend inputs

### 4.1 Domain D note / swap spend (D §1.1–1.2)

DEX private inputs for a spent note:

| D field | Role |
|---------|------|
| `notes_in[]` | notes of `asset_in` with openings |
| `paths[]` | Merkle auth paths under allowed `root` |
| `nullifiers[]` | **new** pool-spend `ν_i` derived on spend |

Per-note minimum fields (D §1.1): `asset_id`, `value`/`v`, `cm`, nullifier material, owner, `rcm`, optional memo/domain.

### 4.2 Normative map (egress → spendable note view)

| DEX spend input (logical) | From `SeamNoteOutV0` | Rule |
|---------------------------|----------------------|------|
| `asset_id` | `asset_id` | byte-equal; must equal pool leg `asset_in` (else `ErrWrongAsset`) |
| `v` / amount | `value` | same base units; conservation over commitments by asset |
| `cm` / leaf for membership | `cm_public` | tree must use compatible `cm_encoding`; mismatch → `E_SCHEMA` / `ErrBadRoot` class |
| Merkle `path` | **not on seam** | supplied at spend time against current `T` root |
| Pool-spend `ν` | **derived**, not `nullifier_lineage` | `nullifier_lineage` is **ingress marker only**; do not reuse claim/burn id as pool nullifier |
| Owner / spend auth | `owner_binding` | V0: binding equality / preimage; full Orchard diversifier/spend-auth = phase-2 |
| `rcm` | `rcm` if `rcm_flag=1` | if `rcm_flag=0`, note is **not DEX-spendable** under private opening rules (phase-0 transparent / structural-only) |
| `memo` | `memo` if `memo_flag=1` | optional |
| `domain` / pool pin | `pool_domain` | must match venue |
| Public swap statement fields (`pool_id`, `Δ_reserves`, `min_out`, …) | **not from note** | SEAM-SPEND / SEAM-AMM-STATE |

### 4.3 Spendability gates (structural)

A `SeamNoteOutV0` is **DEX-consumable** for SEAM-SPEND only if all hold:

1. `version == 0` and `domain_tag == DOMAIN_TAG_NOTE_OUT`
2. `origin ∈ {0x01, 0x02}`
3. `asset_id ≠ 0³²` (or registered non-zero policy id)
4. `cm_encoding ∈ {0x01, 0x02, 0x03}` and `cm_public` well-formed length 32
5. `nullifier_domain ∈ {0x01, 0x02}` (ingress recorded)
6. `rcm_flag == 1` **or** demo policy explicitly allows abstract stub spend with `cm_encoding == 0x03`
7. `pool_domain` matches the DEX tree domain (or both zero under single-demo policy)

Fail → `E_SCHEMA` / C7.1 reject; **no** reserve mutation.

### 4.4 What DEX must never do with this seam

- Treat `nullifier_lineage` as the only double-spend tag for pool notes (would collide claim marker domain with spend domain).
- Mint balances from oracle mid using any field of `SeamNoteOutV0`.
- Accept claim/bridge notes from a second “island” tree without membership under the DEX root (C7.3).

---

## 5. Explicit non-equivalences (not 1:1 yet)

These are **frozen honesty labels**. Do not paper over in Part T.

| # | Left | Right | Status |
|---|------|-------|--------|
| NE-1 | Headstash **Orchard-family** note commit (Sinsemilla / Pallas, `cmx`) | Tacit **secp Pedersen** `C = v·H + r·G` + keccak leaf | **Not crypto-equivalent.** Unified only at `SeamNoteOutV0` field roles + `cm_encoding` tag. |
| NE-2 | `cmx = extract_p(cm)` | Full note commitment point / Tacit `(Cx, Cy)` | Public seam carries **32-byte fingerprint only** in V0. |
| NE-3 | Claim Instance `v` / `nd` **public** (airdrop accounting) | DEX `v` **private** under commitment | Posture flip is product-phase; seam still holds cleartext `value` for mint/claim statements. |
| NE-4 | `nf_claim` / burn `ν` (`nullifier_lineage`) | Pool-spend `ν` on swap | **Different domains** (`nullifier_domain`); no 1:1 identity. |
| NE-5 | `RecpAddr` / Cosmos 32-byte `owner_binding` | Orchard `pk_d` / spend-auth keys | Binding only; not full spend authority object. |
| NE-6 | Headstash `NoteDenom` (blake3 + bit clear → Fp) | B `terp_asset_id` domain hash over tacit id | Same **width** (32); **derivation differs**. Registry / dual-encode required before multi-asset merge. |
| NE-7 | Claim MVP **transparent** bank/tokenfactory payout | D requirement: claim leaf in shared tree `T` | Phase gap (A×D Issue 1). H6/`SeamNoteOutV0` is structural; append-to-`T` is phase-1. |
| NE-8 | `rho` / `rseed` / `psi` (Orchard) | Tacit note opening (`v`, `r`) | Not unified on V0 public layout; private phase-2 opening bags remain origin-specific. |
| NE-9 | Eligibility Merkle leaf `(epk, nd, v, fdi)` | Privacy tree leaf `cm_public` | **Never** the same object; distro root ≠ pool root. |
| NE-10 | B `claimId` binding preimage | C LC `domain_binding` preimage | **Different objects** (asset/claim bind vs mint-proof bind). Only `claim_id` sits on this note seam; LC bind is other seams. |
| NE-11 | `ABSTRACT_LEAF_V0` (`cm_encoding=0x03`) | Production Orchard or Tacit leaf | Harness-only; not Tier-0 privacy. |

---

## 6. Versioning / domain tags

### 6.1 Domain tags (ASCII, 16 bytes NUL-padded)

| Constant | Bytes (ASCII) | Use |
|----------|---------------|-----|
| `DOMAIN_TAG_NOTE_OUT` | `terp-note-out-v0` + NUL pad to 16 | `SeamNoteOutV0.domain_tag` |
| Asset map (B, not this type’s tag) | preimage label `"terp-tacit-asset-v1"` | Registry / `asset_id` derivation only |
| Abstract leaf | preimage label `"terp-seam-leaf-v0"` | `cm_encoding = 0x03` only |
| Source chain tag hash | raw UTF-8 tag → SHA-256 | `source_chain_tag_hash` |

`domain_tag` on the struct is the **schema domain**, not the B asset-map domain and not the C mint `domain_binding`.

### 6.2 Version policy

| `version` | Meaning |
|----------:|---------|
| `0` | This freeze |
| `≥ 1` | Requires SPEC amendment; V0 consumers **reject** unknown versions |

Bump `version` and `DOMAIN_TAG_NOTE_OUT` together if layout width or field semantics change.

### 6.3 Origin × nullifier_domain consistency

| `origin` | Allowed `nullifier_domain` |
|----------|----------------------------|
| `0x01` Headstash | `0x01` only |
| `0x02` Bridge mint | `0x02` only |

Any other pair → structural reject (SEAM-N2).

### 6.4 Replay / pool domain

Bind at least `pool_domain` (and for bridge, `source_chain_tag_hash` + `claim_id`) so the same foreign burn or claim material cannot be reinterpreted across note-set instances without failing mint gates (B A16). Zero `pool_domain` is allowed **only** under explicit single-demo policy.

---

## 7. Acceptance tests — pure structural (SEAM-N1 … SEAM-N6)

**No Halo2, no MockProver, no production circuit edits.**  
Implement as unit/fixture checks over bytes / structs.

### SEAM-N1 — Layout widths and version

**Setup:** Construct a minimal valid `SeamNoteOutV0` for claim and for bridge (fixture constants).

**Checks:**

1. Fixed serialization length = **382** bytes when all fields included.
2. `version == 0`.
3. `domain_tag` equals `terp-note-out-v0` NUL-padded to 16.
4. Every multi-byte field has the width in §1.1 (`asset_id` 32, `value` 8, etc.).

**Expect:** accept.

---

### SEAM-N2 — Origin / nullifier_domain pairing

**Setup:** Four candidates:

| origin | nullifier_domain | Expect |
|--------|------------------|--------|
| `0x01` | `0x01` | accept |
| `0x02` | `0x02` | accept |
| `0x01` | `0x02` | **reject** |
| `0x02` | `0x01` | **reject** |

**Expect:** only consistent pairs pass a pure validator `validate_seam_note_out_v0`.

---

### SEAM-N3 — Headstash Instance / H6 → seam map

**Setup:** Synthetic Instance-shaped bytes:

```text
anchor(32) ‖ nd(32) ‖ v_le(8) ‖ nf(32) ‖ recp_raw(32) ‖ cmx(32)
```

Map per §2.2 (no proof verify).

**Checks:**

1. `origin == 0x01`, `cm_encoding == 0x01`, `nullifier_domain == 0x01`.
2. `asset_id == nd`, `value` decodes from `v_le`, `owner_binding == recp_raw`, `cm_public == cmx`, `nullifier_lineage == nf`, `provenance_anchor == anchor`.
3. `claim_id` and `source_chain_tag_hash` are 32 zero bytes.
4. No field invented beyond Instance + optional `rcm` / `pool_domain`.

**Expect:** accept; this is the H6 structural criterion lifted to the shared seam.

---

### SEAM-N4 — Bridge mint public packet → seam map

**Setup:** Synthetic `BridgeMintPublic` with:

- non-zero `tacit_asset_id`, `value_u64`, `nullifier`, `dest_commitment`, `claim_id`, roots, `source_chain_tag`
- `terp_asset_id` precomputed via §3.2 with fixed `unit_scale`

Map per §3.3.

**Checks:**

1. `origin == 0x02`, `nullifier_domain == 0x02`.
2. `asset_id == terp_asset_id` (not raw tacit id unless registry defines identity map).
3. `value == value_u64`.
4. `nullifier_lineage == nullifier`, `claim_id` preserved, `source_chain_tag_hash == SHA256(tag)`.
5. `cm_encoding ∈ {0x02, 0x03}` for fixtures.
6. **Negative:** map attempt with `value_u64' ≠ value_u64` labeled as conservation break — validator on seam alone may only check field presence; conservation equality is asserted in the **mint gate** test companion (still pure arithmetic, no circuit).

**Expect:** happy map accept; document companion assert `value_out == v_burn`.

---

### SEAM-N5 — Seam → DEX spend consumability

**Setup:** Two seams:

- **S-good:** `rcm_flag=1`, valid tags, `asset_id` = pool `asset_in`, `cm_encoding` allowed.
- **S-bad-schema:** clear `asset_id` to zeros **or** set `rcm_flag=0` with `cm_encoding=0x01` under strict policy **or** drop conceptually by zeroing `cm_public` and encoding `0x00`.

**Checks:**

1. `is_dex_consumable(S-good) == true` per §4.3.
2. `is_dex_consumable(S-bad-schema) == false` → would map to C7.1 / `E_SCHEMA`.
3. Mapping table §4.2 produces defined `asset_id`, `v`, `cm_public` for S-good without reading oracle fields.
4. `nullifier_lineage` is **not** equal-required to a synthetic pool-spend `ν` (assert inequality in fixture: pool `ν' = H("pool-nf-v0" ‖ …)` ≠ lineage).

**Expect:** good consumable; bad rejected; lineage ≠ pool nf property holds.

---

### SEAM-N6 — Claim vs bridge structural equality (C7.2)

**Setup:** One claim-mapped seam and one bridge-mapped seam; ignore payload bytes that §3.5 allows to differ.

**Checks (property):**

1. Both serialize to 382-byte layout.
2. Same set of field offsets/widths (single struct type).
3. Both pass `validate_seam_note_out_v0`.
4. Both pass `is_dex_consumable` when `rcm_flag=1` and demo `pool_domain` match.
5. Differing `origin` / `nullifier_domain` / zero vs non-zero `claim_id` **does not** fail equality-of-schema (type equality), only equality-of-bytes.

**Expect:** schema property pass (structural equality), not byte-identical payloads.

---

## 8. Phase table (claim → shared set)

Joint honesty for A×D Issue 1 / E-I2:

| Phase | Settlement | `SeamNoteOutV0` | In tree `T`? | DEX spend? |
|------:|------------|-----------------|--------------|------------|
| **0** | Transparent claim to `rr` | Structural H6 map (`rcm_flag` may be 0) | no | no (unless abstract stub policy) |
| **1** | Claim/bridge emits leaf | Full required public fields; `cm_public` appended | **yes** | yes if opening available |
| **2** | Full private note + encryption | + private opening bag (rho/rseed or Tacit `r`) | yes | yes |

Compose C1.1 “private claim note in shared set” targets **phase ≥ 1**. SEAM-N* green does **not** require phase 1 host write.

---

## 9. Non-goals

This freeze does **not**:

1. Choose production proof system or merge Orchard + Tacit curves.
2. Define CosmWasm message names byte-for-byte.
3. Expand `SEAM-BURN-WITNESS` / LC certificate (sibling freezes).
4. Implement dual-tree link proofs.
5. Resolve manifold multi-root claim nullifier personalization (additive Headstash — A follow-on).
6. Make oracles mint or supply `value`.
7. Require ZEC egress / TZE.

---

## 10. Checklist (R3-A exit)

- [x] `SeamNoteOutV0` fully typed with byte lengths (§1)
- [x] Headstash / ClaimOutputNoteV0 map (§2)
- [x] Tacit bridge mint map (§3)
- [x] DEX spend input map (§4)
- [x] Non-equivalences NE-1…NE-11 (§5)
- [x] Versioning / domain tags (§6)
- [x] SEAM-N1…N6 pure structural tests (§7)
- [x] No circuit MockProver; no production circuit edits

**Consumers:** Domain E compose (C7), Part T structural harnesses, A H6, B mint packet adapters, D note stubs.

---

## 11. Document control

| Version | Date | Change |
|---------|------|--------|
| 1.0 | 2026-07-20 | Initial freeze: `SeamNoteOutV0` + A/B/D maps + SEAM-N1…N6 (Domain R3-A) |
