# Epic — Prepare full e2e: BTC → Terp → ZEC (gap analysis)

| Field | Value |
|-------|--------|
| **Date** | 2026-07-22 |
| **Board** | `private-bridge-corridor` |
| **Mode** | **Read-first / gap report** — no deep implementation unless a one-line fix unblocks the report |
| **North star** | True multi-net workflow: **Bitcoin fund → observe → private mint on Terp (spectrum headstash) → oracle-bound private swap → ZEC dest open/egress** |
| **Today’s certified path** | `just demo-corridor-ict`: regtest observe + Daemon BridgeMintNote + **pure** W0–W7 swap film — **not** live ZEC settlement |

## Goal of this round

Each specialist answers, for **their surface only**:

1. **Goal** — what full BTC→Terp→ZEC requires of this track  
2. **Code reality** — what is implemented / green today (cite paths)  
3. **Gaps** — concrete deltas (missing wire, mock_only, pure-vs-chain, identity uncoupled, etc.)  
4. **Dependencies** on other tracks  
5. **Proposed next P0** (smallest shippable slice)  

**Deliverable per track:** `STATUS-GAP-<TRACK>.md` in this folder.  
**META** synthesizes `STATUS-GAP-SYNTHESIS.md` after siblings land (or after wait).

## Team (same roles as funded sprint)

| Track | Focus surface |
|-------|----------------|
| **HARNESS-ICT** | ict-rs, corridor-ict-funded, corridor_ict_funded, pure vs chain mint/swap coupling |
| **OBSERVE** | hash-market watches/observations, reporter, amount gates, **intent_id → mint claim identity** |
| **MINT-HEADSTASH** | cw-headstash BridgeMintNote, SeamNoteOutV0, mock_verify vs proof, claim fields from deposit |
| **SWAP-DEX** | pure private_dex / cashapp swap vs on-chain private swap settle |
| **UI** | PrivateCorridor end-to-end film vs production path after observe |
| **ZAKURA-ZEC** | dest binding vs live ZEC receive/egress on local/regtest |
| **DOCS** | How USER-GUIDE / book should describe full vs current e2e without overclaim |
| **META** | Cross-track gap matrix + recommended build order |

## Constraints

- Do **not** amend D1–D7 freezes  
- Do **not** claim mainnet money  
- Prefer evidence: file paths, just targets, what `demo-corridor-ict` does **not** do  
- Honest labels: pure film ≠ chain ZEC  

## SSOT reads (all tracks)

- `docs/plans/spectrum/DEMO-CASHAPP-ZEC-CORRIDOR.md`  
- `docs/plans/spectrum/DESIGN-DECISIONS-CORRIDOR-ACCEPTED-2026-07-20.md`  
- `docs/plans/spectrum/e2e/corridor-ict-funded.sh`  
- `docs/plans/spectrum/e2e/corridor-lab-mint-after-observe.sh`  
- `docs/plans/spectrum/agents/final-sprint-2026-07-22/STATUS-WASM-FUNDED-GREEN.md`  
- `docs/plans/spectrum/book/src/workflows/index.md`  
