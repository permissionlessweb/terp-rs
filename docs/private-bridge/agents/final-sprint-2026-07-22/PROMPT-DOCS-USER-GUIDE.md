# PROMPT — DOCS USER GUIDE (final sprint, raised bar)

**Track id:** `DOCS-USER-GUIDE`  
**Board:** `private-bridge-corridor` · **Parent DOCS:** `t_4a2cd032`  
**Pack:** `docs/plans/spectrum/agents/final-sprint-2026-07-22/`  
**Canonical file:** `docs/plans/spectrum/USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md`  
**Read first:** `ORCHESTRATION.md` + **`FEEDBACK-RAISED-BAR.md`**

**Status policy:** AWAIT GREENLIGHT. Early polish allowed after greenlight; second pass after HARNESS/UI STATUS.

---

## Role (raised)

End-user (and operator-adjacent) documentation for a product that is **heading to mainnet-funded workflow**, with honesty about:

1. Intent seals dest + swap bounds **before** fund.  
2. **Three profiles:** lab_simulated · ict_local_funded / testnet · production mainnet.  
3. How local **ict-rs multi-net simulation** proves the workflow without claiming mainnet settlement.  
4. Stage ownership / recovery using **real primitives** (expiry, dest binding, deposit HD keys) — not open admin seize.  
5. What “ready for mainnet-funded” means vs what still requires ops keys/liquidity.

---

## Human feedback you must internalize

| Prior soft plan | Human correction |
|-----------------|------------------|
| Recovery “docs non-claims only” default | Document **code-backed** gates (expiry, dest binding, double-mint) + user-held deposit keys; only residual what code truly lacks |
| Lab film as product story | Product story is **mainnet-funded-ready path**; lab is labeled training/CI |

---

## Success criteria

1. USER-GUIDE revised sections: profiles table; mainnet-funded readiness; local ict proof path (pointer to e2e/just commands when HARNESS lands them).  
2. Stage ownership table matches FEEDBACK-RAISED-BAR recovery primitives.  
3. Links: spectrum README, OPERATOR.md, DEMO SPEC, this pack.  
4. Checklist progress on Hermes `t_4a2cd032` (comment when done).  
5. No overclaim: local success ≠ mainnet money.  
6. `STATUS-DOCS-USER-GUIDE.md`.

---

## In-tree sources

| Doc | Use |
|-----|-----|
| `USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md` | Edit |
| `DEMO-CASHAPP-ZEC-CORRIDOR.md` | Intent fields |
| `DESIGN-DECISIONS-CORRIDOR-ACCEPTED-2026-07-20.md` | Freezes |
| `FEEDBACK-RAISED-BAR.md` | Recovery + bar |
| `PrivateCorridor/OPERATOR.md` | Operator steps |
| `e2e/README.md`, `CORRIDOR-LAB-STATUS.md` | Commands |

**Out:** Rewriting freezes; inventing marketing that erases lab/mainnet distinction; implementing code (file bugs/links for HARNESS/UI instead).

---

## Handoff

```markdown
## Sections changed
## Profile language
## Recovery language vs code
## Publish surface (in-module link?)
## Open questions remaining
```
