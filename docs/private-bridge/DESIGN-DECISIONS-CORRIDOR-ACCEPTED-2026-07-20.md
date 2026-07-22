# ACCEPTANCE — Private Bridge corridor design freezes D1–D7

**Status:** **ACCEPTED**  
**Date:** 2026-07-20 (amended same day for D3 / D6)  
**Authority:** product / program (session decision)  
**Supersedes:** open asterisk on `DESIGN-DECISIONS-CORRIDOR-2026-07-20.md`  

HARNESS and UI tracks may treat the following as **locked** for lab container demo and for production wiring sessions.

---

## D1 — One note format — **ACCEPTED**

- **SeamNoteOutV0 (382B)** is the only egress format for claim, bridge_mint, and DEX-consumable notes.
- Allowed differences: `origin`, nullifier domain, provenance fields only; fixed widths.
- **Do not invent a second note struct** for the demo or production path.

---

## D2 — Two different hashes — **ACCEPTED**

| Name | Role | Formula family |
|------|------|----------------|
| **Asset map id** | Registry: Tacit/source asset → Terp asset | Domain B `terp-tacit-asset-v1` |
| **Mint / intent bind** | DepositIntent proves BTC deposit → ZEC dest | `terp-cashapp-intent-v0` / C-style mint binds |

- **Never merge** into one function or one seam name.
- Implementers must keep two digests in code and docs.

---

## D3 — Mint paths + private DEX for demo — **ACCEPTED (amended)**

**Amendment vs draft:** Do not treat “bridge-only, ignore pool forever” as the whole product story.

| Path | Required |
|------|----------|
| **Public mint** | Transparent / bank-style (legacy claim) — allowed as separate surface |
| **Private mint** | Bridge mint produces **SeamNoteOutV0** compatible with **private DEX spend** |
| **Demo corridor** | BTC deposit → private bridge mint **+ oracle-powered private swap** (swap is **required** to demo oracle-bound private DEX) |
| **Airdrop claim into same pool** | Phase 2 — not required for first container film |

- Shared anonymity pool for claim + bridge + DEX remains the **target**; demo must show **private mint → private swap** under oracle bounds, not claim.

---

## D4 — BTC observation via open reporter — **ACCEPTED**

- UI opens **watch** (`POST /corridor/watches`) after client proof.
- **corridor-btc-reporter** (Fulcrum/Esplora) posts `deposit.observed`.
- UI **SSE/poll** drives automation.
- Trust: coordination only; UI may re-verify.
- Lab: synthetic observation OK; production: bitcoind → Fulcrum → reporter.
- **Wire into oline service e2e.**

---

## D5 — Label lab honestly — **ACCEPTED**

- Container/host e2e default: **`lab_simulated`**.
- UI must show **“Lab mode — not production”** when not production/lc_live.
- Real HTTP (hash-market, notify) + mocked BTC faucet / burn / proof for lab.

---

## D6 — ZEC destination preauth + Zakura — **ACCEPTED (amended)**

- Demo binds **preauth ZEC dest** → `owner_binding` (no mainnet ZEC send required for lab film).
- **Implement local Zakura support** for destination / wallet UX in the environment (local Zakura, not necessarily mainnet broadcast).
- Phase 2: full ZEC LC egress / mainnet send.

---

## D7 — Mint = cw-headstash + mock_verify; oracle bound_only — **ACCEPTED**

- Demo mint: **`cw-headstash`** only, **`mock_verify=true`**.
- Oracle (hash-market Connect): **bounds only**, never mints.
- Private swap demo uses oracle mid against depositor policy.

---

## Immediate build order (post-acceptance)

1. **Oline play:** Fulcrum (or lab stub) + `corridor-btc-reporter` + `hash-market-server`.  
2. **UI production path:** on `deposit_observed` → `BridgeMintNote` + `put_note_after_mint` (still mock_verify).  
3. **Demo film:** private mint → **oracle-bound private swap** (D3).  
4. **Zakura local** for ZEC dest UX (D6).  

## Explicit non-claims (until later)

- Mainnet Cash App / mainnet ZEC settlement  
- Full Halo2 / LC proof mint without mock_verify  
- Full ICS-02 dregg↔dregg  

---

## Signature

```
ACCEPTED: D1, D2, D3(amended), D4, D5, D6(amended), D7
Next owners: HARNESS (oline + e2e), UI (deposit_observed → mint + persist + swap film), Zakura local (D6)
```
