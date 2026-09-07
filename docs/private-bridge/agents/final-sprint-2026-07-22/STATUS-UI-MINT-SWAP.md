# STATUS — UI-MINT-SWAP (final sprint 2026-07-22)

| Field | Value |
|-------|--------|
| **Track** | UI-MINT-SWAP |
| **Date** | 2026-07-22 |
| **Bar** | Mainnet-funded **workflow shape** (fail-closed reverify + honest profiles); money proof remains HARNESS ict path |
| **Greenlight** | Implemented after human GO |

---

## Reverify call-site (file:line)

**Primary call-site (not docs-only):**

| Location | Role |
|----------|------|
| `websites/dao-dao-ui/packages/stateful/modules/modules/PrivateCorridor/PrivateCorridorWizard.tsx:179-190` | Builds `depositReverify: { address, txid, minAmountSats?, minConfirmations?, network, esploraUrls? }` from SSE observation |
| `PrivateCorridorWizard.tsx:192-202` | Passes `depositReverify` into `runProductionPostDepositSequence` |
| `PrivateCorridorWizard.tsx:534-537` | SSE `deposit_observed` handler → `startProductionPostDeposit(sealed, obs)` |
| `automationClient.ts:235-254` | Executes Esplora reverify; **fail-closed** `return 'failed'` on `!rv.ok` |

Supporting:

| File | Lines / notes |
|------|----------------|
| `btcReverify.ts` | `reverifyDepositObservation` multi-Esplora UTXO check |
| `depositNotify.ts` | SSE / poll → `deposit_observed` + `DepositObservation` |
| `mock.ts:30-51` | `isProductionBackend` includes `ict_local_funded`; `isFailClosedReverifyBackend` |

---

## Fail-closed behavior (modes)

| Mode | Behavior |
|------|----------|
| `production` | Reverify when 64-hex chain txid present; **fail-closed** on failure — no automation success advance; status=`failed` |
| `lc_live` | Same as production |
| `ict_local_funded` | Same fail-closed path; local Esplora via `esploraBase` / `NEXT_PUBLIC_BTC_ESPLORA` |
| `lab_simulated` | Local `STATUS_FILM_PHASES` only; lab banner; synthetic `lab-…` txids skip Esplora |

On fail-closed:

- Toast + status detail: reverify/automation failed  
- **Retry re-verify + automation** (safe)  
- **DANGEROUS: continue without re-verify** only when reverify was the blocker (`skipReverify: true`, labeled)

---

## Env vars for `ict_local_funded`

| Var / field | Purpose |
|-------------|---------|
| Module `assetBackend=ict_local_funded` | Production-shaped UI path + local-net banner |
| Module `notesBase` / `oracleBase` | HARNESS hash-market (notify, automation poll, oracle bounds) |
| Module `headstashContract` | Local ict-deployed cw-headstash mint-router |
| Module `btcNetwork` | Prefer `testnet` for non-mainnet rails |
| Module `esploraBase` | Local Esplora for D4 reverify (regtest) |
| `NEXT_PUBLIC_BTC_ESPLORA` | Env override for Esplora base |
| `NEXT_PUBLIC_ZAKURA_RPC` / `NEXT_PUBLIC_ZAKURA_DEMO_DEST` | D6 dest (ZAKURA track) |
| Host `HASH_MARKET_URL` | Operator script for mint-after-observe |

Documented surgically in `PrivateCorridor/OPERATOR.md` §3.

---

## Lab vs funded banner copy

| Mode | Banner |
|------|--------|
| `lab_simulated` | “Lab mode (lab_simulated) — not production. Synthetic film only; not mainnet settlement.” |
| `ict_local_funded` | “Local test network (ict_local_funded) — same workflow shape as production … Not mainnet settlement.” |
| `production` / `lc_live` | Production Private Bridge banner (fail-closed reverify mentioned); **not** lab banner |

Receipt labels mirror lab vs ict_local_funded honesty.

---

## Automation + oracle

- Poll phases: `deposit_observed` → `bridging` → `swapping` → `complete` via `GET /corridor/automation/:intent_id`  
- Oracle: bounds probe only; UI copy **“oracle never mints”** / `bound_only`; shows intent `min_out` + `slip_bps` vs mid when available  
- Mint authority remains host / cw-headstash (`BridgeMintNote`), not oracle  

---

## Manual test steps against HARNESS endpoints

1. Configure module: `assetBackend=ict_local_funded`, `notesBase`/`oracleBase`=`$HASH_MARKET_URL`, mint-router, optional `esploraBase`.  
2. Preauth dest → rate (min_out/slip) → generate deposit wallet → proof + QR.  
3. Fund deposit (or HARNESS reporter observation with real 64-hex txid).  
4. Expect SSE `deposit_observed` → status re-verify line → fail-closed if Esplora cannot confirm.  
5. With reverify OK, run host mint-after-observe (or HARNESS automation) → poll `bridging` → `swapping` → `complete`.  
6. Confirm oracle mid hint does **not** claim mint; receipt shows local-net label for ict mode.  
7. Negative: break Esplora / wrong txid → status failed; success path blocked; dangerous override only if intentionally used.  

Lab floor: `lab_simulated` + lab publish still runs STATUS_FILM without production reverify.

---

## Funded profile command (UI consumption)

UI does not spawn ict-rs. Consume HARNESS-documented stack, e.g.:

```bash
# After HARNESS STATUS publishes the authoritative command:
export HASH_MARKET_URL=http://127.0.0.1:19090
# Module: notesBase=oracleBase=$HASH_MARKET_URL, assetBackend=ict_local_funded
# Optional: NEXT_PUBLIC_BTC_ESPLORA=<local esplora>
export INTENT_ID=<from wizard intent>
bash docs/plans/spectrum/e2e/corridor-lab-mint-after-observe.sh
```

**In-tree libs used:** `btcReverify.ts` (Esplora), `depositNotify.ts` (hash-market watches/SSE), `automationClient.ts` (automation poll), existing wizard wallet/intent builders.

**Mainnet-only residuals (not this track):** mainnet keys/liquidity, Cash App private API, Fulcrum packaging (oline), live mainnet ZEC egress.

---

## Residuals

- Browser cannot execute Halo2 / cw-orch Daemon mint — host/HARNESS owns chain `BridgeMintNote`.  
- Regtest reverify requires reachable local Esplora; public multi-fallback will fail-closed correctly if misconfigured.  
- Dangerous skip-reverify remains available for operator emergencies only.  
- Zakura dest helpers owned by ZAKURA-DEST (`zakuraDest.ts`).  
- Typecheck of full dao-dao-ui monorepo not run in this subagent pass (touched surfaces only; follow-up CI).  

---

## Files touched

- `PrivateCorridorWizard.tsx` — reverify call-site, banners, fail UI  
- `automationClient.ts` — fail-closed + oracle intent policy display  
- `types.ts` — `ict_local_funded`, `esploraBase`  
- `mock.ts` — production-shaped + fail-closed helpers  
- `PrivateCorridorEditor.tsx` — mode + esplora field  
- `index.ts` — default `esploraBase`  
- `OPERATOR.md` — fail-closed + funded env  
- this STATUS  
