# Design decisions — Private Bridge corridor (resolve open choices)

**Date:** 2026-07-20  
**Audience:** HARNESS / BRIDGE / UI tracks  
**Status:** **ACCEPTED** (see [DESIGN-DECISIONS-CORRIDOR-ACCEPTED-2026-07-20.md](./DESIGN-DECISIONS-CORRIDOR-ACCEPTED-2026-07-20.md))  
**Amendments:** D3 — private mint + **oracle-powered private swap required** for demo; D6 — preauth ZEC + **local Zakura**  

This freezes the choices needed to run a **honest local container demo** of the CashApp→ZEC corridor without pretending mainnet BTC/ZEC are live.

---

## D1 — SEAM-NOTE-OUT single schema (E-S1)

| Choice | **Freeze: SeamNoteOutV0 382B is the only egress** for claim *and* bridge_mint *and* DEX-consumable note |
|--------|----------------------------------------------------------------------------------------------------------|
| Claim/bridge differences | Only `origin` + nullifier domain + provenance fields may differ; widths fixed |
| Implementation | Already in `seam_note_out` + SEAM-NOTE-OUT.md; domain A/B maps must **not** invent parallel structs |
| Demo impact | Persist always encrypts SeamNoteOutV0 |

---

## D2 — Domain binding (E-S3)

| Object | Formula / name |
|--------|----------------|
| **Asset map id** | Domain B: `terp_asset_id = H("terp-tacit-asset-v1" ‖ …)` — registry key only |
| **Mint proof bind** | Domain C / intent: `domain_bind` on DepositIntentV0 = `H("terp-cashapp-intent-v0" ‖ canonical)` |
| Rule | **Two different digests** — never collapse into one seam name |

---

## D3 — Mint paths + private DEX (E-I2) — **ACCEPTED amended**

| Path | Behavior |
|------|----------|
| **Public mint** | Transparent / bank (legacy claim) — separate surface |
| **Private mint** | Bridge → **SeamNoteOutV0** DEX-compatible |
| **Demo required** | BTC deposit → private bridge mint **+ oracle-bound private swap** (swap demos private DEX + oracle) |
| **Claim into shared pool** | Phase 2 |

---

## D4 — BTC deposit observation

| Choice | **Open reporter via Fulcrum/Esplora → hash-market `/corridor/observations`** |
|--------|-----------------------------------------------------------------------------|
| UI | Opens watch only (`POST /corridor/watches`); SSE for automation |
| Trust | Observation = coordination; UI may re-verify |
| Local demo | Esplora **testnet** API or lab reporter posting observations without full bitcoind |

---

## D5 — Asset backends for container demo

| Mode | Use |
|------|-----|
| `lab_simulated` | Default **container e2e** — faucet + mock burn id + mock_verify mint |
| `production` | Staging with real heads (mint-router, notes, oracle URLs) |
| `lc_live` | When LC containers attached (future) |

**Honest film:** container e2e = **lab_simulated** end-to-end with real HTTP services (hash-market, reporter lab mode). Label as lab, not mainnet Cash App.

---

## D6 — ZEC destination + Zakura — **ACCEPTED amended**

| Demo | Preauth dest → `owner_binding`; prove note bound to dest |
| Local | **Implement Zakura support** for destination/wallet UX |
| Target | Mainnet ZEC send / LC egress later |

---

## D7 — Mint & oracle

| Mint | `cw-headstash` only, `mock_verify=true` for container |
| Oracle | hash-market Connect bounds; feed lab mid or static feeder |
| Never | Oracle mint balances |

---

## Demo success criteria (container)

1. `just demo-e2e-l0` + `cashapp_zec` harness green (host).  
2. Docker: hash-market-server up; open watch; lab observation; SSE delivers `deposit_observed`.  
3. Optional: Mock chain bridge mint + note PUT.  
4. UI module can point at container `notesBase` and show notify status.  
5. **No claim** of mainnet BTC/ZEC transfer.

---

## Explicitly deferred (do not block container demo)

- Full ICS-02 dregg↔dregg  
- Halo2 private swap prove  
- H1 composite MockProver green  
- Mainnet Cash App / BIP84 live funding  
- Resolve all Domain A transparent-vs-shielded pool forever  

---

## Cold-start for HARNESS agent

```
You are HARNESS E2E (ROUND3 + DEMO-CORRIDOR-E2E).
Curate local container env for Private Bridge corridor lab film.
Read:
  docs/plans/spectrum/DESIGN-DECISIONS-CORRIDOR-2026-07-20.md
  docs/plans/spectrum/DEMO-CASHAPP-ZEC-CORRIDOR.md
  docs/plans/spectrum/E2E-HARNESS-PLAN.md
  crates/terp-rs/tools/hash-market/docs/corridor-deposit-notify.md
  crates/terp-rs/tools/hash-market/docs/corridor-btc-reporter.md

Deliver:
  docker-compose (or extend oline parallel-stack) with hash-market-server
  + corridor-btc-reporter in lab mode (can post observations without bitcoind)
  + smoke script: open watch → observe → SSE/poll status
  Document which design decisions D1–D7 are assumed frozen for the film.
Do not claim mainnet BTC/ZEC.
```
