---
title: SEAM freeze checklist — before e2e compose
status: active
date: 2026-07-20
domain: E (compose) + domain owners
consumers: Domains A–E, Sprint integrators, pure fixture authors
related:
  - docs/plans/spectrum/FLOW-private-bridge-auth.md §5
  - docs/plans/spectrum/FLOW-private-bridge-auth-test-matrix.md
  - docs/plans/spectrum/SEAM-NOTE-OUT.md (R3-A freeze target)
  - docs/plans/spectrum/CLARITY-headstash-sets-and-bridge-models.md
  - docs/plans/spectrum/PARALLEL-CURATION-ROUND-3.md
  - docs/plans/spectrum/reviews/REVIEW-META-2026-07-20.md
---

# SEAM freeze checklist (pre–e2e)

**Purpose:** Freeze every `SEAM-*` contract (payload fields, owner domain, pure coverage) **before** investing in full C0–C9 e2e harness / MockProver loops.

**Rule:** Domain unit pure fixtures may go green first. Compose e2e stays **red-allowed** until this checklist is checked for the seams that path touches.

---

## 0. Freeze gate (program)

Do **not** start multi-domain e2e until:

| # | Gate | Status target |
|---|------|----------------|
| G1 | `SEAM-NOTE-OUT` field map A↔B↔D written (single schema bridge) | **`SEAM-NOTE-OUT.md` frozen** (`SeamNoteOutV0`) |
| G2 | Each seam below has an **owner domain** and SPEC anchor | this table |
| G3 | Pure fixture crates cover auth-critical B/D rules (no halo2 required) | R3-B / R3-D |
| G4 | Domain A single-set H1–H6 structural Part T green (or honest red log) | PART-T checkpoint |
| G5 | ZEC/TZE explicitly **not** on critical path (CLARITY) | non-blocking note §4 |

Curated SPECs ≠ progression-grade software. This checklist freezes **interfaces**, not full circuit rewrite.

---

## 1. SEAM inventory — domain ownership

| Seam ID | From → To | Owner domain | SPEC freeze anchor | Payload freeze (minimum) | Pre-e2e required? |
|---------|-----------|--------------|--------------------|--------------------------|-------------------|
| `SEAM-NOTE-OUT` | A/B → D/E | **A + B emit; D consumes; E composes** | A §5 / §6.1 H6; B §3.1; D §1.1; R3-A `SEAM-NOTE-OUT.md` | `asset_id`/`asset_tag`, value, cm/cmx, ν material, owner binding, domain bind | **Yes — first** |
| `SEAM-DISTRO-ROOT` | A/gov → A | **A** | A §1.6, §4.1 | single-set `anchor` + params; multi-root list = design-only (H7/H8) | Yes for claim path |
| `SEAM-BURN-WITNESS` | B/C → B | **B** (+ C membership) | B §4.2 H-1; C §2 membership | ν ∈ **bridge-burn set**, `v`, asset, inclusion under LC tip, re-derived claimId | Yes for bridge path |
| `SEAM-LC-STATE` | C → B/A | **C** | C §2–§3, §9 | `client_id`, tip height, roots (incl. burn root), lag, status, LC class | Yes for bridge path |
| `SEAM-DOMAIN-SEP` | S0 → all | **B** asset map + **C** mint-bind (name both) | B §5; C §3.1 | Do not collapse asset-map hash with mint-proof bind — dual-id if needed | Yes |
| `SEAM-SPEND` | D/E → notes | **D** | D §1.2 | nullifiers, anchor root, spend proof pubs, paths (private) | Yes for swap/exit |
| `SEAM-AMM-STATE` | D public | **D** | D §2 | `R_i`, fees, pool id | Yes for swap |
| `SEAM-ORACLE-BOUND` | hashmerchant → D | **D** (+ HM) | D §3 | mid, timestamp, staleness, quorum; **never mint** | Yes for bound-path swap only |
| `SEAM-CROSS-OUT` | E exit → foreign S1 | **B** | B §3.1 `bridge_burn` | dest chain, amount, asset, ν, claim fields | Yes for reverse path C8 |
| `SEAM-LC-STATE` (distro freeze optional) | C → A | **C policy / A claim** | C §6.2 | optional; claim-only demos must not require LC | Soft |

**Non-interference:** Only the owner domain freezes field encodings in its SPEC (or the shared `SEAM-NOTE-OUT.md`). Compose (E) lists seams; it does not invent a fourth note language.

---

## 2. Per-seam freeze checklist

### 2.1 `SEAM-NOTE-OUT` (blocker for C1/C7)

- [ ] A `ClaimOutputNoteV0` ↔ D note roles mapped (one page / R3-A doc)
- [ ] B bridge_mint output structurally equal to claim schema for shared set (C7.2 property named)
- [ ] Encoding: fixed byte lengths or explicit “structural only until CosmWasm msg freeze”
- [ ] No second commitment algorithm without mapping table
- [ ] Owner of conflicts: E opens issue; A/B/D fix own SPEC

### 2.2 `SEAM-BURN-WITNESS` + `SEAM-LC-STATE` (blocker for C0/C3)

- [ ] H-1: only **bridge-burn set** authorizes mint (not generic spent set) — pure B fixture
- [ ] `bitcoinBurnRoot` (or equivalent) currency against LC tip called out in C SPEC
- [ ] Confirmation depth / lag gate fields present
- [ ] claimId re-derivation inputs listed
- [ ] LC class (reflection / Crosslink / IBC) not erased on seam

### 2.3 `SEAM-DISTRO-ROOT` (blocker for C1 claim)

- [ ] Single-set root freeze for H1–H6 / C1.1
- [ ] Additive multi-root: **design only** this round (A §1.10, H7/H8 IDs) — not required for e2e v1
- [ ] Nullifier domain separation policy documented for future multi-root

### 2.4 `SEAM-DOMAIN-SEP` (blocker for C4)

- [ ] Asset map domain (B) named separately from mint bind (C) if formulas differ
- [ ] Cross-pool / cross-asset replay cases C4.x named

### 2.5 `SEAM-SPEND` / `SEAM-AMM-STATE` / `SEAM-ORACLE-BOUND` (blocker for C1/C5/C6)

- [ ] Spend nullifier + root membership
- [ ] Public reserve Δ vs proven Δ
- [ ] Oracle bounds only; explicit “oracle cannot mint” pure reject

### 2.6 `SEAM-CROSS-OUT` (blocker for C8)

- [ ] Stub fields parseable as S1 object
- [ ] Dest domain required (incomplete → reject)

---

## 3. Pure fixture crates ↔ SEAM / domain tests

Lightweight crates under `docs/plans/spectrum/fixtures/` (no halo2 / no SP1). Prefer these for agent loops and CI smoke before e2e.

| Crate path | Domain | SEAM(s) exercised | Named pure tests (illustrative) | Compose suites unblocked |
|------------|--------|-------------------|---------------------------------|--------------------------|
| `docs/plans/spectrum/fixtures/bridge_auth_seams` | **B** (+ C gate logic stub) | `SEAM-BURN-WITNESS`, `SEAM-DOMAIN-SEP`, mint side of `SEAM-NOTE-OUT` | `t1_happy_path_mint`, `t2_double_mint_reject`, `t3_ordinary_spend_not_burn`, `t4_immature_confirmation`, `t5_value_mismatch`, `t6_unmapped_asset`, `t7_domain_mismatch`, `spent_set_membership_alone_never_authorizes`, `domain_bind_is_domain_separated` | C0.*, C2.2, C3.*, C4.*, C6 mint-auth shape |
| `docs/plans/spectrum/fixtures/private_dex_seams` | **D** | `SEAM-AMM-STATE`, `SEAM-ORACLE-BOUND`, `SEAM-SPEND` (nullifier) | `swap_happy`, `min_out_fail`, `oracle_stale`, `oracle_stale_optional_allows_amm`, `oracle_inflate_reject_mint_api`, `oracle_inflate_reject_reserve_bump`, `oracle_cannot_justify_inflated_delta_out`, `double_spend_nullifier`, `wrong_asset_id`, `quote_zero_fee_matches_xyk`, `within_slippage_band` | C1.2–C1.3, C2.3, C5.*, C6.* |
| Domain A circuit Part T (not under fixtures/) | **A** | `SEAM-DISTRO-ROOT`, claim side `SEAM-NOTE-OUT` | H1–H6 (+ H12 gap) in `crates/headstash/circuit` `orchard_delta_part_t` | C1.1, C2.1, C7.1 |
| `docs/plans/spectrum/fixtures/seam_note_out` | **A/B→D seam** | `SEAM-NOTE-OUT` structural | **SEAM-N1..N6** (`cargo test` → 6 ok) | C7.x schema equality, H6 lift |
| (future) multi-root pure module | **A** | `SEAM-DISTRO-ROOT` multi | **H7** multi-root accept-any; **H8** wrong-set nullifier reject — **design IDs only this round** | later C1 multi-drop |

### 3.1 Suggested pure-only commands

```bash
# Domain B bridge auth seams
cd /Users/returniflost/abstract/terp-core/docs/plans/spectrum/fixtures/bridge_auth_seams
cargo test

# Domain D private DEX seams
cd /Users/returniflost/abstract/terp-core/docs/plans/spectrum/fixtures/private_dex_seams
# (cargo test)

cd /Users/returniflost/abstract/terp-core/docs/plans/spectrum/fixtures/seam_note_out
cargo test

# Domain A structural Part T (circuit lib; no full e2e)
cd /Users/returniflost/abstract/terp-core/crates/headstash/circuit
cargo test --lib orchard_delta_part_t --no-default-features --features "circuit,std"
```

### 3.2 What pure fixtures do **not** cover

| Gap | Owner | When |
|-----|-------|------|
| Full Halo2 claim verify (MockProver K=18) | A Part I | after H1 construction path honest |
| 08-wasm LC update against real headers | C | LC corridor sprint |
| CosmWasm cw-headstash / pool msgs | A/B | after seam encodings freeze |
| Compose C0–C9 single binary | E | after G1–G4 |

---

## 4. ZEC / TZE non-blocking

Per [`CLARITY-headstash-sets-and-bridge-models.md`](./CLARITY-headstash-sets-and-bridge-models.md) §2 and [`REVIEW-META-2026-07-20.md`](./reviews/REVIEW-META-2026-07-20.md):

| Item | On critical path for SEAM freeze / Part T? |
|------|--------------------------------------------|
| Zcash egress lock/burn → Terp private deposit | **No** |
| ZIP-222 TZE evidence channel | **No** |
| Crosslink production finality | **No** |
| IBC-class Bitcoin ↔ Zcash | **No** (north star) |
| Single-set Headstash H1–H6, Tacit B pure, D pure AMM/oracle, SEAM-NOTE-OUT freeze | **Yes** |

**Rule:** Do not block SEAM freeze or Domain A/B/D pure green on ZEC/TZE design loops. LC hinge fields for a **first corridor** (often Bitcoin reflection-class) are enough to freeze `SEAM-LC-STATE` / burn membership.

---

## 5. Next Part I circuit items (deferred past this freeze)

These remain **out of SEAM freeze scope** but are the next Domain A implementation batch after pure/structural green:

| # | Part I item | Why deferred here |
|---|-------------|-------------------|
| I1 | Suite-backed H1 with full verify (`generate_circuit_test_data` / MockProver path) | Circuit cost; not needed to freeze seam field names |
| I2 | `constrain_instance` for public `nd` / `v` / `recp` (close §1.4 gap / H12) | Circuit edit; H12 documents gap until then |
| I3 | In-circuit `nk` bind to eligibility `esk` (ownership debt) | Known review debt; not SEAM payload |
| I4 | Contract-layer H2 integration in `cw-headstash` | After instance honesty |
| I5 | Multi-root / OR-membership manifold **circuit** (H7 accept-any) | Design-only this round — see A §1.10 |
| I6 | Domain-separated nullifier across roots **in circuit** (H8) | Same; registry policy can be pure first |
| I7 | Shielded claim → pool write (vs transparent settle MVP) | Pool sprint after H6 schema agreed |
| I8 | ZSA AssetId / ZIP 227 issuance wiring | Parent ref only |
| I9 | ZEC egress + TZE | Non-blocking (§4) |
| I10 | Tachyon / Ragu aggregation | Explicit non-goal |

**Freeze exit:** G1–G5 + pure B/D green + A Part T structural → **allowed** to schedule compose e2e skeletons. Part I items I1–I4 should precede claiming progression-grade claim software.

---

## 6. Compose suite mapping (quick)

| Compose suite | Primary seams | Pure coverage available? |
|---------------|---------------|--------------------------|
| C0 bridge → note → exit | LC-STATE, BURN-WITNESS, NOTE-OUT, DOMAIN-SEP, SPEND, CROSS-OUT | Partial (B pure mint; exit stub) |
| C1 claim → swap → exit | DISTRO-ROOT, NOTE-OUT, SPEND, AMM, ORACLE | Partial (A Part T + D pure) |
| C2 double-spend | DISTRO / BURN / SPEND | B + D pure; A H2 structural |
| C3 LC lag | LC-STATE | B pure maturity; real LC later |
| C4 domain / asset | DOMAIN-SEP, AMM | B + D pure |
| C5 oracle halt | ORACLE-BOUND, AMM | D pure |
| C6 oracle cannot mint | ORACLE-BOUND | D pure |
| C7 NOTE-OUT schema | NOTE-OUT | Structural A H6 + freeze doc |
| C8 reverse path | CROSS-OUT | Spec stub until B wire freeze |
| C9 trust tiers | doc/fixture labels | Doc only |

Full case IDs: [`FLOW-private-bridge-auth-test-matrix.md`](./FLOW-private-bridge-auth-test-matrix.md).

---

## 7. Sign-off template

```text
SEAM freeze checkpoint
date:
agent / owner:
SEAM-NOTE-OUT doc:     [ ] present  path: ...
bridge_auth_seams:     cargo test → 
private_dex_seams:     cargo test → 
orchard_delta_part_t:  cargo test → 
open Part I deferrals: I1 I2 I3 I4 (list)
ZEC/TZE gated?         NO
ready for e2e skeleton: YES / NO — reason:
```

---

## 8. Changelog

| Date | Change |
|------|--------|
| 2026-07-20 | R3-E initial SEAM freeze checklist: ownership, pure fixtures B/D, ZEC/TZE non-block, Part I deferred |
