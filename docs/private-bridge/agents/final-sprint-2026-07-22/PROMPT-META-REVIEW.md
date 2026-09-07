# PROMPT — META REVIEW (final sprint, raised bar)

**Track id:** `META-REVIEW`  
**Board:** `private-bridge-corridor` · **Parent IMPL:** `t_563b09d5`  
**Pack:** `docs/plans/spectrum/agents/final-sprint-2026-07-22/`  
**Read first:** `ORCHESTRATION.md` + **`FEEDBACK-RAISED-BAR.md`** + all `STATUS-*.md`

**Status policy:** Run **after** other tracks produce STATUS (or mid-sprint on human request). No deep feature implementation.

---

## Role

Independent check that the sprint did **not** re-lower the bar and did **not** overclaim mainnet.

---

## Checklist (fail any critical)

| # | Check | Critical? |
|---|-------|-----------|
| M1 | D1–D7 freezes files **unchanged** in meaning | Yes |
| M2 | Funded profile exists and is **not** only synthetic observe + Mock mint | Yes |
| M3 | S1 command documented and claimed green with evidence | Yes |
| M4 | UI reverify fail-closed for non-lab modes | Yes |
| M5 | Lab banner only for `lab_simulated` | Yes |
| M6 | Oracle never described as minter | Yes |
| M7 | Asset map id ≠ intent domain_bind in code/docs touched | Yes |
| M8 | Zakura golden binding parity UI↔harness | Medium |
| M9 | USER-GUIDE does not claim mainnet settlement from local nets | Yes |
| M10 | Residuals listed are truly mainnet-ops (keys, liquidity), not abandoned P0 | Medium |
| M11 | File fences roughly respected (no thrash wars) | Low |
| M12 | Libraries used match ORCHESTRATION §2 (no unexplained greenfield frameworks) | Medium |

---

## Human bar reminder

Sprint end = **ready for mainnet-funded workflow**, proven by **ict-rs fresh multi-net local simulation**.  
Passing lab `demo-corridor-lab` alone is **not** enough.

---

## Deliverable

`STATUS-META-REVIEW.md`:

```markdown
## GO / NO-GO for mainnet-funded-ready (local proof)
## Critical fails
## Medium nits
## Overclaim risks
## Recommended next ops for true mainnet money
```

**Out:** Implementing fixes yourself unless trivial doc typos; amending freezes.
