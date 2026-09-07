---
title: FLOW — Private bridge auth compose test matrix
status: draft
domain: E
created: 2026-07-20
parent_flow: docs/plans/spectrum/FLOW-private-bridge-auth.md
type: test-matrix
---

# Compose test matrix — private bridge authentication

> **Scope:** Seam and end-to-end **authentication** tests only.  
> Domain unit tests (Headstash H1–H6, LC header vectors, AMM math, Tacit op maps) stay in Domains A–D.  
> This matrix **composes** them. Cases may be scaffolded red until objects land.

Parent: [`FLOW-private-bridge-auth.md`](./FLOW-private-bridge-auth.md)

---

## 1. Conventions

| Field | Meaning |
|-------|---------|
| **ID** | Stable case id (`C#` suite / `C#.#` case) |
| **Seams** | `SEAM-*` ids from parent FLOW §5 |
| **Steps** | S0–S6 exercised |
| **Expect** | `accept` / `reject` / `property` |
| **Auth focus** | Which checklist items must fire |
| **Owner** | Who implements fixture; E owns suite wiring |

**Fixture style (preferred):** JSON vectors or multi-test scenarios with explicit public inputs, expected reject codes, and no production exploit payloads.

**Reject codes (placeholder — freeze with contracts):**

| Code | Meaning |
|------|---------|
| `E_LC_LAG` | Finality / confirmations insufficient |
| `E_LC_FROZEN` | Client misbehaviour or expired |
| `E_NULLIFIER` | Duplicate ν / burn id |
| `E_DOMAIN` | Domain bind / asset_id mismatch |
| `E_PROOF` | Circuit verify fail |
| `E_CONSERVATION` | Value creation / mismatch |
| `E_MEMBERSHIP` | Bad Merkle / inclusion |
| `E_ORACLE_STALE` | Mid missing or stale for bound path |
| `E_SLIPPAGE` | min_out not met |
| `E_SCHEMA` | Note not consumable by next seam |
| `E_ORACLE_MINT` | Attempted balance credit from oracle alone |

---

## 2. Suite C0 — Happy path bridge → note → exit

| ID | Scenario | Seams | Steps | Expect | Auth focus | Owner |
|----|----------|-------|-------|--------|------------|-------|
| C0.1 | Valid reflection-class burn under LC tip → bridge_mint note | `SEAM-LC-STATE`, `SEAM-BURN-WITNESS`, `SEAM-NOTE-OUT`, `SEAM-DOMAIN-SEP` | S0–S3-B | accept | S2.*, S3.1–S3.5, S3.7 | B+C+E |
| C0.2 | Same note unshield to transparent payout | `SEAM-SPEND` | S5-U | accept | S5.1–S5.4 | B+E |
| C0.3 | Same note bridge_burn emits `SEAM-CROSS-OUT` | `SEAM-SPEND`, `SEAM-CROSS-OUT` | S5-B | accept | S5.1–S5.5 | B+E |
| C0.4 | Conservation audit: burn in = unshield out (single hop) | — | S6 | property | I1, S6.1 | E |

**Pass:** C0.1+C0.2 or C0.1+C0.3 green; C0.4 holds on transcript.

---

## 3. Suite C1 — Happy path claim → swap → exit

| ID | Scenario | Seams | Steps | Expect | Auth focus | Owner |
|----|----------|-------|-------|--------|------------|-------|
| C1.1 | Valid eligibility → private claim note in shared set | `SEAM-DISTRO-ROOT`, `SEAM-NOTE-OUT` | S3-A | accept | S3.1–S3.4, S3.6–S3.9 | A+E |
| C1.2 | Claim note spends into swap; public reserves Δ applied | `SEAM-SPEND`, `SEAM-AMM-STATE` | S4 | accept | S4.1–S4.4, S4.7 | D+E |
| C1.3 | Optional oracle-bound swap (fresh mid) | `SEAM-ORACLE-BOUND` | S4 | accept | S4.5–S4.6 | D+E |
| C1.4 | Post-swap note exits (unshield or burn) | `SEAM-SPEND` | S5 | accept | S5.* | B+D+E |
| C1.5 | Public transcript does not hard-link claim leaf to swap ν (anonymity set ≥ 2 fixture) | — | S3–S4 | property | S3.8, privacy table | A+D+E |

**Pass:** C1.1–C1.2–C1.4 green; C1.3 if oracle path in scope; C1.5 property documented.

---

## 4. Suite C2 — Double-spend / double-claim / double-burn

| ID | Scenario | Seams | Steps | Expect | Code | Owner |
|----|----------|-------|-------|--------|------|-------|
| C2.1 | Replay same claim nullifier | `SEAM-DISTRO-ROOT` | S3-A | reject | `E_NULLIFIER` | A+E |
| C2.2 | Replay same burn id for second bridge_mint | `SEAM-BURN-WITNESS` | S3-B | reject | `E_NULLIFIER` | B+E |
| C2.3 | Swap with already-spent note ν | `SEAM-SPEND` | S4 | reject | `E_NULLIFIER` | D+E |
| C2.4 | Unshield after swap spent same ν | `SEAM-SPEND` | S5 | reject | `E_NULLIFIER` | E |
| C2.5 | bridge_burn replay for remote mint twice | `SEAM-CROSS-OUT` | S5 / reverse S3 | reject | `E_NULLIFIER` | B+E |

---

## 5. Suite C3 — LC lag / finality / client health

| ID | Scenario | Seams | Steps | Expect | Code | Owner |
|----|----------|-------|-------|--------|------|-------|
| C3.1 | Burn included but confirmations &lt; gate | `SEAM-LC-STATE` | S2–S3-B | reject | `E_LC_LAG` | C+E |
| C3.2 | Client frozen (misbehaviour) | `SEAM-LC-STATE` | S2 | reject | `E_LC_FROZEN` | C+E |
| C3.3 | Inclusion proof against **wrong** root | `SEAM-BURN-WITNESS` | S2–S3 | reject | `E_MEMBERSHIP` | C+E |
| C3.4 | LC tip advances; previously lagging burn now mints | `SEAM-LC-STATE` | S2–S3-B | accept | — | C+B+E |
| C3.5 | Claim-only path does **not** require LC (if policy) | — | S3-A | accept | — | A+E |

---

## 6. Suite C4 — Domain binding / asset_id

| ID | Scenario | Seams | Steps | Expect | Code | Owner |
|----|----------|-------|-------|--------|------|-------|
| C4.1 | Valid proof with wrong `asset_id` in public inputs | `SEAM-DOMAIN-SEP` | S3 | reject | `E_DOMAIN` / `E_PROOF` | B+E |
| C4.2 | Cross-pool replay (pool instance bind) | `SEAM-DOMAIN-SEP` | S3–S4 | reject | `E_DOMAIN` | E |
| C4.3 | Foreign denom not in S0 map | S0 map | S3-B | reject | `E_DOMAIN` | B+E |
| C4.4 | Swap output asset not in pair config | `SEAM-AMM-STATE` | S4 | reject | `E_DOMAIN` | D+E |

---

## 7. Suite C5 — Oracle halt / stale (bounds path)

| ID | Scenario | Seams | Steps | Expect | Code | Owner |
|----|----------|-------|-------|--------|------|-------|
| C5.1 | Bound-required swap; mid missing | `SEAM-ORACLE-BOUND` | S4 | reject | `E_ORACLE_STALE` | D+E |
| C5.2 | Bound-required swap; mid older than staleness | `SEAM-ORACLE-BOUND` | S4 | reject | `E_ORACLE_STALE` | D+E |
| C5.3 | Pure AMM path allowed when policy disables oracle | `SEAM-AMM-STATE` | S4 | accept | — | D+E |
| C5.4 | Fresh mid but Δ_out fails slippage | `SEAM-ORACLE-BOUND` | S4 | reject | `E_SLIPPAGE` | D+E |
| C5.5 | After oracle halt, unspent notes still exit | `SEAM-SPEND` | S5 | accept | — | E |

---

## 8. Suite C6 — Oracle cannot mint (adversarial)

| ID | Scenario | Seams | Steps | Expect | Code | Owner |
|----|----------|-------|-------|--------|------|-------|
| C6.1 | Submit “mint note” authorized only by VE/oracle sig | `SEAM-ORACLE-BOUND` | S3 | reject | `E_ORACLE_MINT` | E |
| C6.2 | Oracle message attempts reserve inflation without swap proof | `SEAM-AMM-STATE` | S4 | reject | `E_ORACLE_MINT` / `E_CONSERVATION` | D+E |
| C6.3 | Conservation holds with malicious mid extreme values | `SEAM-ORACLE-BOUND` | S4 | reject or bound-clamp | no balance invent | D+E |

**Pass:** No case creates a note or increases user balance without S3 burn/claim or S4 conservation settle.

---

## 9. Suite C7 — `SEAM-NOTE-OUT` schema / shared set

| ID | Scenario | Seams | Steps | Expect | Code | Owner |
|----|----------|-------|-------|--------|------|-------|
| C7.1 | Claim output missing fields required by DEX spend | `SEAM-NOTE-OUT` → `SEAM-SPEND` | S3-A→S4 | reject | `E_SCHEMA` | A+D+E |
| C7.2 | Bridge_mint output same schema as claim (structural equality) | `SEAM-NOTE-OUT` | S3-A vs S3-B | property | H6 / map | A+B+E |
| C7.3 | Note from alternate “island” tree rejected by DEX root | `SEAM-SPEND` | S4 | reject | `E_MEMBERSHIP` | E |
| C7.4 | Domain D documents consumable schema; A/B emit it | docs | — | accept | checklist | A+B+D |

---

## 10. Suite C8 — Reverse path / cross_out

| ID | Scenario | Seams | Steps | Expect | Owner |
|----|----------|-------|-------|--------|-------|
| C8.1 | bridge_burn output parses as foreign S1 object stub | `SEAM-CROSS-OUT` | S5-B | accept | B+E |
| C8.2 | Round-trip vector: mint on Terp → burn → (stub) remote mint inputs complete | `SEAM-CROSS-OUT` | S3–S5 | property | B+C+E |
| C8.3 | Incomplete cross_out (missing dest domain) rejected | `SEAM-CROSS-OUT` | S5-B | reject `E_DOMAIN` | B+E |

---

## 11. Suite C9 — Trust tier honesty (doc/fixture)

| ID | Scenario | Expect | Owner |
|----|----------|--------|-------|
| C9.1 | Each step in FLOW lists trust tier consistent with Tacit analog | property / review | E |
| C9.2 | No fixture labeled Tier 0 while depending on worker for soundness | reject labeling | E |
| C9.3 | Oracle path labeled non-mint / bounds-only in fixture metadata | accept | D+E |

---

## 12. Mapping to Domain unit tests

| Compose | Domain seeds (landed SPEC sections) |
|---------|-------------------------------------|
| C1.1, C2.1, C7.x | Domain A §6 H1–H6 |
| C0.x, C2.2, C2.5, C4.x, C8.x | Domain B §6 bridge ingress + §9 mint packet |
| C3.x | Domain C §5.1–§5.7 (create/update/membership/lag/domain/misbehaviour/hinge mock) |
| C1.2–C1.3, C5.x, C6.x | Domain D §6 `swap_happy`, `min_out_fail`, `oracle_stale`, `oracle_cannot_inflate_balance`, `double_spend_nullifier`, `wrong_asset_id` |
| All | Domain E wiring only |

---

## 13. Harness checkpoint template

```text
date:
command:
packages:
result: PASS | FAIL | RED-EXPECTED
notes:
cases_green:
cases_red:
blockers (domain):
```

### Progression-grade for compose suite

1. C0.1 + (C0.2 or C0.3) green **or** C1.1 + C1.2 + C1.4 green (at least one full ingress path)  
2. C2.1–C2.3 green (nullifier discipline)  
3. C6.1 green (oracle non-authority)  
4. C7.2 property written (shared schema)  
5. Checkpoint pasted in sprint card  

Full C0–C9 green = software progression for the **compose** object; curated SPECs may precede that.

---

## 14. Changelog

| Date | Change |
|------|--------|
| 2026-07-20 | Initial compose matrix C0–C9 aligned to FLOW seams |
