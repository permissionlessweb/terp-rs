# STATUS-GAP-META

| Field | Value |
|-------|--------|
| **Track** | META |
| **Date** | 2026-07-22 |
| **Mode** | Gap synthesis only (no product implementation) |
| **Inputs** | STATUS-GAP-{HARNESS,OBSERVE,MINT,SWAP,UI,ZAKURA,DOCS}.md (all landed) |
| **Primary deliverable** | `STATUS-GAP-SYNTHESIS.md` (matrix + DAG + certified vs next bar + non-claims) |

---

## 1. Goal for full BTC → Terp → ZEC (this track)

META owns **cross-track coherence** for the full multi-net bar:

1. One **matrix** of track × goal × reality × P0 gaps.
2. One **implementation DAG** (dependency order, not parallel marketing).
3. Clear split: **`just demo-corridor-ict` certified today** vs **next continuous multi-net bar**.
4. **Explicit non-claims** so product/docs cannot over-read green funded path as live ZEC settle.

META does **not** implement mint/swap/ZEC; it freezes the build order and residual language other tracks must use.

---

## 2. Code reality today (cite paths / commands)

| Surface | Status | Evidence |
|---------|--------|----------|
| Sibling gap reports | **Complete (7/7)** | This folder: HARNESS, OBSERVE, MINT, SWAP, UI, ZAKURA, DOCS |
| Certified funded path | **Green (shape, not continuous identity)** | `just demo-corridor-ict` → `docs/plans/spectrum/e2e/corridor-ict-funded.sh` |
| Synthesis artifact | **This epic** | `STATUS-GAP-SYNTHESIS.md` |
| Prior funded green | **Reference** | `agents/final-sprint-2026-07-22/STATUS-WASM-FUNDED-GREEN.md` |

### Certified path one-liner (META SSOT)

> BTC **regtest** fund → reporter → `deposit_observed` → ict-rs Terp + Daemon **fixture** `BridgeMintNote` (`IsBridgeMinted`; default `mock_verify=true`) → **pure** cashapp W0–W7 + automation phases. **Not** deposit-backed claim. **Not** on-chain private DEX. **Not** live ZEC egress.

---

## 3. Gaps (goal vs code) — META-level only

| ID | Gap | Severity | Notes |
|----|-----|----------|-------|
| **META-G1** | **Three uncoupled identity planes** | **P0** | Observe (txid/amount) · fixture mint (hinge ν) · pure swap film (synthetic NoteIn) — sequenced by shell/`intent_id`, not one note lifecycle |
| **META-G2** | **Full multi-net ZEC not on critical path of funded green** | **P0** | Zakura golden/offline parallel to ict; no join in `corridor-ict-funded.sh` |
| **META-G3** | **Product docs over-read continuous swap+ZEC** | **P0** (DOCS honesty) | USER-GUIDE / book sequence vs e2e honesty; engineering scripts more careful |
| **META-G4** | **UI celebrates complete with mock receipt** | **P0** (honesty) | Host pure film → automation complete → `buildMockSuccessReceipt` |
| **META-G5** | **No single STATUS “full e2e ready” until DAG stages close** | **P1** | Avoid re-labeling residual mint-only / pure film as S1 complete |

See **STATUS-GAP-SYNTHESIS.md** for full track matrix and P0 IDs.

---

## 4. Dependencies on other tracks

META depends on all tracks having filed STATUS-GAP-*.md — **done**.  
Implementation dependencies are the DAG in synthesis (OBSERVE+MINT ν/claim → HARNESS wire → SWAP note spend → ZAKURA dest join → UI honesty → DOCS residual table).

---

## 5. Recommended P0 slice for this track

1. Land **STATUS-GAP-SYNTHESIS.md** (this round).  
2. Point follow-on sprints at **DAG Phase A–B** only (continuous observe→mint identity + honest UI/docs); defer CW DEX + live ZEC egress to later phases.  
3. Do **not** open D1–D7 freezes in STATUS language.

---

## 6. Explicit non-claims

- META synthesis is **gap analysis**, not a greenlight for mainnet money.  
- All sibling tracks green on **their pure/coord surfaces** ≠ continuous multi-net.  
- D1–D7 freezes unchanged.  
- Full detail: `STATUS-GAP-SYNTHESIS.md` § Explicit non-claims.
