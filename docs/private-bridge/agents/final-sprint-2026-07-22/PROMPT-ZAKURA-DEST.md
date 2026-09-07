# PROMPT — ZAKURA DEST (final sprint, raised bar)

**Track id:** `ZAKURA-DEST`  
**Board:** `private-bridge-corridor` · **Parent IMPL:** `t_563b09d5`  
**Pack:** `docs/plans/spectrum/agents/final-sprint-2026-07-22/`  
**Read first:** `ORCHESTRATION.md` + **`FEEDBACK-RAISED-BAR.md`**

**Status policy:** AWAIT GREENLIGHT → implement. Pre: plan-only `STATUS-ZAKURA-DEST.md`.

---

## Role (raised)

D6: **local Zakura support for destination UX** in the environment that will become mainnet-funded-ready.

Human decision:

- **UX primary = paste-first** (Zakura is a full node, not a full wallet product).  
- **Funded stack must still run** local Zakura regtest (or harness parity) so dest is real, not fiction.  
- One **golden** `terp-dest-binding-v0` vector shared across UI, harness, e2e scripts.

```text
zakura-local up (regtest)
  → dest address + owner_binding
  → UI paste / “use local Zakura demo dest”
  → intent.dest_owner_binding sealed
  → funded e2e asserts binding match on receipt
```

---

## Human feedback you must internalize

| Prior soft plan | Human correction |
|-----------------|------------------|
| Paste-first alone may complete D6 | Incomplete without **runnable** local node path in funded/ict compose |
| Optional Zakura not blocking demo-corridor-lab | Correct for **lab floor**; **incorrect** for funded profile exit (S5) |

---

## Success criteria (testable)

1. `just demo-zakura-local-dest` (or e2e/zakura scripts) green offline dest + live when Docker/node up.  
2. Golden vector file e.g. `docs/plans/spectrum/e2e/zakura/golden-dest-binding.json` (domain + sample dest + expected binding hex).  
3. UI `zakuraDest.ts` + Wizard dest step: paste + optional “use local Zakura” uses **same** hash as harness (`terp-dest-binding-v0`).  
4. Document how HARNESS ict funded compose attaches Zakura (env ports — align with `ZAKURA-LOCAL.md` / corridor miner fallback).  
5. Soft validation of UA/t-addr prefixes; fail closed on empty dest.  
6. `STATUS-ZAKURA-DEST.md` written.

---

## In-tree libraries

| Piece | Path |
|-------|------|
| E2E scripts | `docs/plans/spectrum/e2e/zakura/`, `ZAKURA-LOCAL.md` |
| UI | `PrivateCorridor/zakuraDest.ts` |
| Harness | `test-press/src/harness/zakura_local.rs` |
| Reviews | `reviews/REVIEW-ZAKURA-UI-AGENT-DELTA-2026-07-20.md` |
| Operator | `PrivateCorridor/OPERATOR.md` § Zakura |

**Do not** invent a browser full wallet if node has no wallet RPC — paste-first + miner transparent dest fallback on regtest is acceptable if documented.

---

## In / out

**In:** e2e/zakura, zakuraDest helpers, golden vector, binding parity tests, funded-stack docs hooks.  
**Out:** Exclusive rewrites of Wizard SSE (UI track); mainnet ZEC broadcast; amending freezes; greenfield Zcash light client.

---

## Freezes

D6 amended: local Zakura for dest UX. Dest binding opaque; display strings non-authoritative.

---

## Handoff (`STATUS-ZAKURA-DEST.md`)

```markdown
## Golden vector path
## Live node command
## UI binding parity proof
## Funded stack integration notes for HARNESS
## Residuals
```
