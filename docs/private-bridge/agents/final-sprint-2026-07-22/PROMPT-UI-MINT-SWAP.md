# PROMPT — UI MINT + SWAP (final sprint, raised bar)

**Track id:** `UI-MINT-SWAP`  
**Board:** `private-bridge-corridor` · **Parent IMPL:** `t_563b09d5`  
**Pack:** `docs/plans/spectrum/agents/final-sprint-2026-07-22/`  
**Read first:** `ORCHESTRATION.md` + **`FEEDBACK-RAISED-BAR.md`**

**Status policy:** AWAIT GREENLIGHT → then implement. Pre-greenlight: plan-only `STATUS-UI-MINT-SWAP.md`.

---

## Role (raised)

You own **PrivateCorridor** UI so it is safe for a **mainnet-funded workflow shape**, while proving against **local funded endpoints** from HARNESS (`ict_local_funded` / hash-market + reverify).

```text
deposit_observed (SSE)
  → depositReverify (production / ict_local_funded / lc_live)  [FAIL-CLOSED]
  → poll automation → bridging → swapping → complete
  → optional direct mint/persist client hooks when router env set
  → lab_simulated keeps banner; never claims mainnet settlement
```

HARNESS owns chain `BridgeMintNote` on ict-rs. You own **UI trust + film** and must not leave reverify as a dead hook.

---

## Human feedback you must internalize

| Prior soft plan | Human correction |
|-----------------|------------------|
| Reverify visible / prefer fail-closed | **Fail-closed** required for production-shaped modes (`production`, `lc_live`, and any `ict_local_funded` UI mode). Lab may continue with banner. |
| Host film phases enough | UI must work against **funded profile URLs** HARNESS documents (notes base, automation, reverify network = testnet/regtest as configured). |
| “Mainnet out of scope” misread as “don’t harden UI” | Harden UI **as if mainnet** (fail-closed, honest labels); money still only on test nets in this sprint. |

---

## Success criteria (testable)

1. **P0:** SSE/`deposit_observed` path passes  
   `depositReverify: { address, txid, minAmountSats?, network }` into production post-deposit sequence  
   (`btcReverify.ts` + `automationClient.ts` call-site in `PrivateCorridorWizard.tsx` — not docs-only).

2. **Fail-closed:** If reverify fails in non-lab modes → do **not** advance automation to success; show error; optional explicit operator override only if labeled dangerous.

3. **D5:** `lab_simulated` → “Lab mode — not production”. Funded local mode must **not** use that banner; use truthful “local test network / not mainnet” if you add copy.

4. Automation poll phases: `deposit_observed` → `bridging` → `swapping` → `complete` against live hash-market from HARNESS.

5. Oracle bounds UI: never “oracle mints”; show mid/floor vs intent policy when configured.

6. Intent preauth steps unchanged in spirit: dest + min_out/slip **before** fund; reject matrix I1–I6 reflected in UX messaging where applicable.

7. Module typecheck/build for touched surfaces; surgical `OPERATOR.md` update for fail-closed + funded env vars.

8. `STATUS-UI-MINT-SWAP.md` written.

---

## In-tree libraries / files

| Piece | Path |
|-------|------|
| Wizard | `PrivateCorridorWizard.tsx` |
| Automation | `automationClient.ts` |
| Reverify | `btcReverify.ts` |
| Notify/SSE | `depositNotify.ts` |
| Wallet/QR | `browserWallet.ts`, `btcAddress.ts` |
| Types | `types.ts` |
| Operator | `OPERATOR.md` |
| Notify contract | `hash-market/docs/corridor-deposit-notify.md` |
| Blindspot | `reviews/REVIEW-OLINE-BTC-INDEXER-BLINDSPOTS-2026-07-20.md` (I-P0-2) |
| User language | `USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md` |

Coordinate dest step with ZAKURA (`zakuraDest.ts`); **you exclusive-own** Wizard SSE + post-deposit sequence.

---

## In scope / out of scope

**In:** fail-closed reverify wiring; production sequence; automation film; honest mode labels; env for notes/oracle/mint router; optional client calls toward mint/persist if already stubbed.

**Out:** Owning ict-rs spawn; Fulcrum packaging; Zakura docker monorepo; Cash App API; full Halo2 in browser; amending freezes; inventing second note format.

---

## Freezes

D1 note format · D2 two hashes · D3 swap required in film · D4 reverify trust · D5 labels · D6 dest preauth (coord) · D7 oracle bound_only, mint = headstash.

---

## Handoff (`STATUS-UI-MINT-SWAP.md`)

```markdown
## Reverify call-site (file:line)
## Fail-closed behavior (modes)
## Env vars for ict_local_funded
## Lab vs funded banner copy
## Manual test steps against HARNESS endpoints
## Residuals
```
