# STATUS — DOCS-USER-GUIDE

| Field | Value |
|-------|--------|
| **Track** | `DOCS-USER-GUIDE` |
| **Date** | 2026-07-22 |
| **Greenlight** | Human GO — first pass shipped |
| **Canonical guide** | [`docs/plans/spectrum/USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md`](../../USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md) |
| **Bar** | [`ORCHESTRATION.md`](./ORCHESTRATION.md) · [`FEEDBACK-RAISED-BAR.md`](./FEEDBACK-RAISED-BAR.md) |

---

## Sections changed

| Section | Change |
|---------|--------|
| Header / status | Composition framed as **mainnet-funded-ready** with explicit local ≠ mainnet money |
| §1 Product spine | Same workflow shape for mainnet; profiles change nets/policy, not a demo-only fork |
| §2 **Three profiles** | `lab_simulated` · `ict_local_funded`/testnet · `production` — banner language table |
| §3 Mainnet-funded-ready | Ready vs ops/external residual (keys, liquidity, legal, Cash App) |
| §4 Intent-before-fund | Fields + I1–I6 refuse matrix tied to primitives; reverify fail-closed |
| §5 Stage ownership | Code-backed table: deposit HD, `dest_owner_binding`, `expiry`, `domain_bind`, `IsBridgeMinted`, oracle bound_only; optional `recovery_owner_binding` residual |
| §6 UI map | Production-shaped reverify; publish surface note for in-module link |
| §7 **ict-rs proof path** | Lab floor commands live; funded profile recipe + **placeholder** `just demo-corridor-ict` until HARNESS STATUS |
| §8 FAQ | Local success ≠ mainnet; banner guidance |
| §9–10 Related + maintainer checklist | Sprint pack links; second-pass rule |

---

## Profile language

| Profile | Guide treatment |
|---------|-----------------|
| `lab_simulated` | CI/training film; D5 banner; synthetic observe / mock_verify OK |
| `ict_local_funded` / testnet | Fidelity gate; real local-chain mint + regtest/signet observe; **not** mainnet settlement |
| `production` / mainnet | Ops after keys/liquidity; reverify fail-closed; explicit non-claims until that deploy enables mainnet rails |

Lab is **not** the product story; product story is mainnet-funded-ready path with honest profile labels.

---

## Recovery language vs code

| Primitive | Documented in USER-GUIDE | Code / SSOT anchor |
|-----------|-------------------------|--------------------|
| `dest_owner_binding` | Only preauth dest may open/receive | DEMO `DepositIntentV0`; domain `terp-dest-binding-v0` |
| `expiry` | Late mint reject (I3) | DEMO + pure I3 |
| `domain_bind` | Intent integrity `terp-cashapp-intent-v0` | D2; DEMO §3 |
| Deposit HD mnemonic | Pre-observe UTXO recovery = user-held | UI `browserWallet` |
| Double-mint reject | Same ν / intent | `IsBridgeMinted` / cw-headstash |
| Oracle bounds | Never mints | D7 bound_only |
| `recovery_owner_binding` / timeout vault | **Optional / residual** if not live on named deploy | Prefer implement nearly free (HARNESS); else ops = intent id + deposit HD; **no admin seize** |
| Open admin seize / Cash App clawback | Explicit **non-claims** | FEEDBACK-RAISED-BAR DΔ-4 |

Stage table matches FEEDBACK-RAISED-BAR recovery primitives; residual called residual.

---

## Funded profile command (docs side)

| Item | Status |
|------|--------|
| Lab floor | Documented: `just demo-corridor-lab` (+ smoke, mint-after-observe, zakura dest, e2e L0/L1) |
| Funded multi-net | **Placeholder:** `just demo-corridor-ict` (or test-press Daemon/ict-rs binary per HARNESS) |
| `mock_verify` on funded deploy | **TBD** — second pass after `STATUS-HARNESS-OBSERVE.md` |
| What remains mainnet-only | Keys, liquidity, legal, Cash App freezes, mainnet ZEC egress policy |

**No STATUS-HARNESS at first-pass ship time** — placeholders per prompt; second pass when HARNESS lands real command names.

---

## Links updated

| File | Update |
|------|--------|
| `docs/plans/spectrum/USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md` | Full revision |
| `docs/plans/spectrum/README.md` | USER-GUIDE blurb + ORCHESTRATION pack link |
| `PrivateCorridor/OPERATOR.md` | Honest three-profile scope; sprint bar; USER-GUIDE §7 / HARNESS pointer; configure table expanded |

**In-module “How it works” UI link:** preferred publish surface (FEEDBACK); not implemented in this docs-only track — file for UI-MINT-SWAP if missing.

---

## Publish surface

| Surface | Status |
|---------|--------|
| Spectrum path | **Shipped** — `docs/plans/spectrum/USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md` |
| OPERATOR → USER-GUIDE | **Linked** (strengthened) |
| In-module UI “How it works” | **Preferred; residual** for UI track |
| Public marketing site | Later OK |

---

## Open questions remaining

1. Exact funded just/binary name + env once HARNESS STATUS exists.  
2. Whether funded ict deploy runs `mock_verify=true` or false (must be documented for honesty).  
3. Whether UI exposes a distinct `ict_local_funded` mode string vs reuse of production-shaped + local endpoints.  
4. `recovery_owner_binding` land date — keep residual until code is live on a named deployment.  
5. Hermes checklist comment on `t_4a2cd032` — operator should mark progress in board tooling (docs agent does not drive Hermes here per greenlight instruction).

---

## Handoff (PROMPT format)

### Sections changed
See table above (§1–10); primary rewrite of profiles, readiness, stage ownership, ict proof path.

### Profile language
Three-way; lab = training/CI; funded local = fidelity gate; production = ops. Local success ≠ mainnet money (repeated).

### Recovery language vs code
Code-backed gates primary; optional recovery binding residual; no open admin seize / Cash App clawback claims.

### Publish surface (in-module link?)
Spectrum + OPERATOR linked; in-module UI link preferred residual for UI track.

### Open questions remaining
HARNESS command names, mock_verify on funded deploy, UI mode string, recovery_owner_binding land.

---

## Second-pass checklist (when STATUS-HARNESS appears)

- [ ] Replace §7 placeholder table with real command(s) from HARNESS  
- [ ] Document mock_verify true/false on funded path  
- [ ] Align OPERATOR “funded local” row with actual env vars from UI/HARNESS  
- [ ] Note any golden dest-binding path HARNESS/ZAKURA publish  
- [ ] Bump STATUS date + mark second pass done  
