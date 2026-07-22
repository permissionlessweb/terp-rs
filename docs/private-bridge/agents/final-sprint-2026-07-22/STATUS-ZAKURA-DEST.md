# STATUS — ZAKURA-DEST (final sprint, raised bar)

| Field | Value |
|-------|--------|
| **Track** | `ZAKURA-DEST` |
| **Date** | 2026-07-22 |
| **Bar** | FEEDBACK-RAISED-BAR DΔ-3 + ORCHESTRATION S5 |
| **Result** | **PASS** (offline golden + dest film green; live node path documented & scripted) |

---

## Golden vector path

**`docs/plans/spectrum/e2e/zakura/golden-dest-binding.json`**

| Field | Value |
|-------|--------|
| Domain | `terp-dest-binding-v0` |
| Preimage | `terp-dest-binding-v0\|` ‖ `utf8_trim(dest_display)` → SHA-256 |
| Primary dest | `tmJymvcUCn1ctbghvTJpXBwHiMEB8P6wxNV` (corridor `miner_address`) |
| Primary binding | `8b5cac11e39905d56126a0c538b84ff8daa379d8009d4e8b121112479607f09b` |
| Lab sample | `u1sim_diversified_zec_lab` → `386008ec5f5c2f9eeb23e0d72949b3631eb45b754fbf67bdcceb7ceb33463efe` |

Verify:

```bash
bash docs/plans/spectrum/e2e/zakura/zakura-local.sh golden
```

---

## Live node command

```bash
# Offline floor (always; no Docker):
cd crates/headstash && just demo-zakura-local-dest

# Live regtest when Linux zakurad + Docker available:
cd crates/zakura && cargo build -p zakura --bin zakurad
export ZAKURAD_BIN="$(pwd)/target/debug/zakurad"
bash docs/plans/spectrum/e2e/zakura/zakura-local.sh up
bash docs/plans/spectrum/e2e/zakura/zakura-local.sh rpc-smoke
cd crates/headstash && just demo-zakura-local
```

| Env | Default |
|-----|---------|
| `ZAKURA_RPC` | `http://127.0.0.1:18232` |
| `ZAKURA_DEST_ADDR` | miner dest (override paste) |
| `ZAKURAD_BIN` | host-built Linux `zakurad` for compose |
| `NEXT_PUBLIC_ZAKURA_RPC` | browser optional health |
| `NEXT_PUBLIC_ZAKURA_DEMO_DEST` | pin paste when CORS/no wallet |

**This run:** `just demo-zakura-local-dest` → **exit 0**  
- golden parity OK (both samples)  
- offline dest prints primary binding matching golden  
- harness: **6/6** `zakura_local` tests OK (live RPC skipped cleanly — no node up on this host)

---

## UI binding parity proof

| Surface | Symbol | Same digest? |
|---------|--------|--------------|
| Golden JSON | `owner_binding_hex` primary | SSOT file |
| Shell | `zakura-local.sh` `owner_binding_hex` | ✓ verified by `golden` cmd |
| Harness | `owner_binding_from_dest_display` / `REGTEST_MINER_OWNER_BINDING_HEX` | ✓ `golden_vector_matches_owner_binding` |
| UI | `zakuraDest.ts` `DEST_BINDING_DOMAIN` + `GOLDEN_OWNER_BINDING_HEX` + `destOwnerBindingHex` | ✓ same preimage string |
| UI mock | `bindingFromDestDisplay` → uses `destOwnerBindingHex` / `DEST_BINDING_DOMAIN` | ✓ |
| Wizard dest step | paste + “Use local Zakura demo dest” → `bindingFromDestDisplay` | no exclusive Wizard rewrite (helpers only) |

Soft validation: fail-closed empty dest; soft UA/`tm`/`t1` prefixes (`softValidateZecDest` / harness `soft_validate_dest_prefix`).  
Display strings non-authoritative; seal is `owner_binding`.

**W0–W7 offline:** `cashapp_w0_w7_with_golden_primary_dest` seals receipt with golden binding.

---

## Funded stack integration notes for HARNESS

Zakura is a **sidecar** (not ict-rs `ChainSpec`). Host networking.

| Port | Role |
|------|------|
| **18232** | JSON-RPC (`ZAKURA_RPC`) |
| **18233** | P2P |
| **19901** | metrics |

Compose: `docs/plans/spectrum/e2e/zakura/docker-compose.corridor-zakura.yml`  
Config: `node-corridor.toml` (`miner_address` = golden primary dest)

**Attach order for `ict_local_funded`:**

1. HARNESS spins Terp + hash-market (ict-rs / play).  
2. `zakura-local.sh up` (or document SKIP + use golden offline dest for CI without Docker).  
3. `zakura-local.sh golden && dest` → seal `dest_owner_binding`.  
4. Or call harness `primary_golden_dest()` / `owner_binding_from_dest_display` without RPC.  
5. Assert receipt open/payout binding == sealed binding.  
6. Live optional: `validateaddress` when RPC up.

Libs used (no greenfield): existing e2e/zakura, `zakura_local.rs`, `zakuraDest.ts`, corridor miner fallback.

---

## Residuals

| Item | Severity | Notes |
|------|----------|--------|
| Live Docker Zakura not exercised on this macOS agent run | Low for offline S5 floor | Script path ready; needs Linux `ZAKURAD_BIN` + Docker for full smoke |
| Zakura core ≠ wallet | Documented | paste-first primary; miner dest fallback on regtest |
| lightwalletd / zcashd-compat UA mint | Residual | not required for preauth digest |
| Browser CORS to RPC | Residual | optional; paste works offline |
| Mainnet ZEC broadcast | Out of scope | honest non-claim |
| ict-rs ChainSpec for Zakura | Will not add | sidecar + env only |

### What is mainnet-only vs code-complete

| Code-complete | Mainnet-only |
|---------------|--------------|
| Binding domain + golden + UI/harness parity | Real user ZEC keys / UA from production wallet |
| Offline dest film + W0–W7 with golden binding | Mainnet Zakura/zcashd sync & send |
| Live regtest compose + RPC health/validate scripts | Production liquidity / legal rails |
| Funded-stack port/env docs for HARNESS | Operator secrets, CORS policy on public RPC |

### Funded profile command (for STATUS checklist)

```bash
cd crates/headstash && just demo-zakura-local-dest   # floor — always
# + when bin/docker: just demo-zakura-local
```

In-tree libraries that closed D6 residual: `e2e/zakura/*`, `test-press` `zakura_local`, UI `zakuraDest.ts` + `mock.ts` domain import.

---

## Deliverable map

| Path | Change |
|------|--------|
| `docs/plans/spectrum/e2e/zakura/golden-dest-binding.json` | **new** golden |
| `docs/plans/spectrum/e2e/zakura/zakura-local.sh` | `golden` command |
| `docs/plans/spectrum/e2e/zakura/README.md` | golden + HARNESS attach |
| `docs/plans/spectrum/e2e/ZAKURA-LOCAL.md` | raised-bar status |
| `crates/headstash/test-press/src/harness/zakura_local.rs` | golden load/assert, soft validate, ports, W0–W7 golden test |
| `crates/headstash/test-press/src/harness/mod.rs` | re-exports |
| `crates/headstash/justfile` | `demo-zakura-local-dest` runs golden + harness golden tests |
| `PrivateCorridor/zakuraDest.ts` | domain, golden hex, `destOwnerBindingHex`, offline demo helper |
| `PrivateCorridor/mock.ts` | binding via shared domain/helper |
| `PrivateCorridor/OPERATOR.md` | paste-first + golden + ports |

**Freezes:** D1–D7 untouched. No mainnet ZEC. No exclusive Wizard SSE rewrite.
