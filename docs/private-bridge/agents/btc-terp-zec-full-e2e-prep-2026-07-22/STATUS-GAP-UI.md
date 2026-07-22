# STATUS-GAP-UI

| Field | Value |
|-------|--------|
| **Track** | UI (PrivateCorridor / dao-dao-ui) |
| **Date** | 2026-07-22 |
| **Mode** | Gap analysis only (no implementation this pass) |
| **Surface** | `websites/dao-dao-ui/packages/stateful/modules/modules/PrivateCorridor/**` |

## Direct answer — does UI execute `BridgeMintNote`?

**No. The browser UI never submits CosmWasm `ExecuteMsg::BridgeMintNote` (or any headstash execute).**

Production-shaped post-deposit path is:

1. SSE/poll **observation** (`depositNotify.ts`)  
2. Optional **Esplora re-verify** (`btcReverify.ts` via Wizard call-site) — fail-closed  
3. Oracle mid **probe** only (`GET /oracle/bounds`) — never mints  
4. **Poll** host automation `GET /corridor/automation/:intent_id` until `complete` / `failed` / timeout  

Mint authority stays on **host / HARNESS** (`corridor-lab-mint-after-observe.sh`, `just demo-corridor-ict`, cw-orch Daemon, etc.). UI only consumes coordination status.

Evidence:

- `automationClient.ts` header + `runProductionPostDepositSequence`: polls; comments explicitly state browser cannot run Halo2 / cw-orch and host runs mint-after-observe.  
- Grep of `PrivateCorridor/*`: **no** signing client, `MsgExecuteContract`, or `BridgeMintNote` payload construction — only copy/strings and module field `headstashContract` (display + automation label).  
- On poll `complete`, Wizard still calls `buildMockSuccessReceipt` — does **not** render host `st.receipt` or chain mint refs.

---

## 1. Goal for full BTC → Terp → ZEC (this track)

For a continuous **operator/user film** of true multi-net:

| Stage | UI responsibility |
|-------|-------------------|
| Preauth | Capture real ZEC dest; bind `owner_binding` consistent with mint claim |
| Rate | Seal `min_out` / slip / market; show oracle as bound-only |
| Deposit | Fresh browser P2WPKH on correct BTC network (incl. regtest for funded lab) |
| Observe | Open watch; receive `deposit_observed`; **re-verify** chain before advancing |
| Mint | **Honest UX**: either trigger/await real host mint **or** wallet-sign BridgeMintNote; never invent mint success |
| Swap | Show oracle-bound private swap progress only when host/chain reports it |
| Egress / receipt | Reflect real mint note + swap outcome + dest binding; no sim receipt after funded automation |

Full path requires UI to **wire** observe → reverify → mint identity → swap film → ZEC dest honesty end-to-end — without overclaiming chain work the browser does not perform.

---

## 2. Code reality today (cite paths / commands)

| Surface | Status | Evidence |
|---------|--------|----------|
| Module shell (editor/renderer) | **Implemented** | `PrivateCorridor/{index,PrivateCorridorEditor,PrivateCorridorRenderer}.tsx`; modes `production` \| `lab_simulated` \| `lc_live` \| `ict_local_funded` |
| Wizard steps W0–W7 film | **Implemented (UI film)** | `PrivateCorridorWizard.tsx` steps preauth → rate → wallet → QR → status → receipt |
| Deposit intent seal | **Implemented (client JSON)** | `mock.ts` `buildDepositIntent` / `computeDomainBind`; `types.ts` `DepositIntentV0` |
| Browser deposit wallet + P2WPKH QR | **Implemented** | `browserWallet.ts`, `btcAddress.ts` (`mainnet` \| `testnet` only) |
| Client proof + auth link | **Implemented** | `browserWallet.ts` `buildClientWalletProof` / `buildCorridorAuthLink` |
| Watch open + SSE/poll | **Implemented** | `depositNotify.ts` `openCorridorWatch`, `subscribeCorridorDeposits`, `pollCorridorWatch` |
| Lab observation publish | **Implemented** | `reportLabObservation` + Wizard lab button (`lab-…` synthetic txids) |
| D4 Esplora re-verify call-site | **Implemented (fail-closed)** | Wizard builds `depositReverify` from SSE obs (`PrivateCorridorWizard.tsx` ~179–202, ~534–537); `automationClient.ts` ~235–254 returns `'failed'` on `!rv.ok` |
| Re-verify implementation | **Implemented (Esplora UTXO)** | `btcReverify.ts` `reverifyDepositObservation` |
| Post-deposit automation | **Poll only** | `pollAutomationStatus` → `GET …/corridor/automation/:id`; loop 40×3s |
| Host mint / swap | **Out of UI** | Operator: `INTENT_ID=… bash docs/plans/spectrum/e2e/corridor-lab-mint-after-observe.sh`; ict: `just demo-corridor-ict` (HARNESS) |
| Oracle | **Display probe** | `fetchOracleMid` — `bound_only` copy; no mint |
| Lab STATUS film | **Local fake phases** | `STATUS_FILM_PHASES` + `onDepositFunded` lab branch — no host, no reverify |
| Zakura / ZEC dest | **Paste + optional RPC demo** | `zakuraDest.ts` soft validate, golden miner dest, RPC probe; **no ZEC send / balance confirm** |
| Receipt | **Always mock builder** | `buildMockSuccessReceipt` even after production `complete` (`Wizard` ~238–241) |
| Module default chain gate | **Terp mainnet only** | `index.ts` `isChainSupported: chainId === ChainId.TerpMainnet` |
| Operator docs | **Honest on host mint** | `PrivateCorridor/OPERATOR.md` §4–5 |
| Prior sprint STATUS | **UI reverify bar closed for call-site** | `docs/plans/spectrum/agents/final-sprint-2026-07-22/STATUS-UI-MINT-SWAP.md` |

### Production post-deposit sequence (code shape)

```
SSE deposit_observed
  → depositReverify (if 64-hex txid, not lab-*)
  → fail-closed on Esplora failure
  → GET /oracle/bounds (display)
  → poll GET /corridor/automation/:intent_id
       phases: deposit_observed | bridging | swapping | complete | failed
  → on complete: buildMockSuccessReceipt (UI-local sim refs)
```

UI does **not**: build Halo2 proofs, call cw-headstash, put notes, execute private DEX, broadcast ZEC.

### Host script reality (UI dependency)

`docs/plans/spectrum/e2e/corridor-lab-mint-after-observe.sh` PUTs automation phases and runs **pure** W0–W7 / mock_verify-oriented film. Funded **Daemon BridgeMintNote** is HARNESS (`demo-corridor-ict`), not the browser. UI will show `complete` if host PUTs complete — even when host was pure film only.

---

## 3. Gaps (goal vs code)

| ID | Gap | Severity | Notes |
|----|-----|----------|-------|
| **UI-G1** | **UI never executes / signs `BridgeMintNote`** | **P0** (architecture) | By design today; full continuous e2e needs either (a) documented operator host always-on, (b) one-click host trigger API, or (c) wallet CosmWasm execute. Without (a–c), production path ends at `awaiting_host`. |
| **UI-G2** | **Receipt ignores host automation `receipt`** | **P0** | `pollAutomationStatus` can return `receipt`; Wizard discards it and always `buildMockSuccessReceipt` with `sim-mint-*` / `sim-swap-*` / random `btc_txid_or_intent_id`. Funded film looks like sim on step 6. |
| **UI-G3** | **`headstashContract` is config/label only** | **P1** | Never used for `IsBridgeMinted` query, note cm display, or execute. Operator may set mint-router but UI cannot prove mint on-chain. |
| **UI-G4** | **Intent field not rebound to chain txid / mint claim id** | **P1** | Sealed `btc_txid_or_intent_id` stays intent id; observation txid not written into intent packet; receipt randomizes a fake funded id. Identity coupling deposit→claim is **display-uncoupled**. |
| **UI-G5** | **No ZEC egress / receive confirmation in UI** | **P0** (full multi-net) | Dest binding only; no balance/RPC confirmation that ZEC arrived. Zakura helpers stop at preauth dest load. |
| **UI-G6** | **No on-chain private swap surface** | **P0** (full multi-net) | Status phase `swapping` is host poll string only; pure film ≠ chain settle. |
| **UI-G7** | **BTC regtest address / network incomplete** | **P1** | `BtcNetwork = 'mainnet' \| 'testnet'`; non-testnet → reverify network `mainnet`. `btcReverify` supports `signet` but Wizard never selects it. Regtest funded stack relies on custom `esploraBase` while QR may still be `bc1`/`tb1` not `bcrt1`. |
| **UI-G8** | **SIM asset registry ids in all backends** | **P1** | `SIM_ASSET_IN_ID` / `SIM_ASSET_OUT_ID` always sealed into intent — even `production` / `ict_local_funded`. Real registry ids from mint track not UI-wired. |
| **UI-G9** | **Module only enabled on `ChainId.TerpMainnet`** | **P1** | Local ict Terp DAOs may not surface module without chain-id allowlist change. |
| **UI-G10** | **Automation poll timeout → soft hang UX** | **P2** | 40 × 3s then `awaiting_host`; operator must re-run or wait without long-lived poll restart unless retry buttons after fail. |
| **UI-G11** | **Re-verify skipped without 64-hex txid** | **P1** | Missing / non-hex txid → no fail-closed reverify; still enters bridging poll. Open observation bus + no chain check if reporter omits real txid shape. |
| **UI-G12** | **Lab path can complete without any host or chain** | **P2** (honest if labeled) | `lab_simulated` STATUS_FILM is correct for CI; risk is operator confusion if mode wrong. Banners exist (D5). |
| **UI-G13** | **Dangerous skip-reverify remains** | **P2** | Intentional operator override; residual trust hole if misused. |
| **UI-G14** | **CORS / Zakura RPC browser limits** | **P2** | Documented; paste-first fallback OK for dest preauth, not for egress proof. |
| **UI-G15** | **No UI-driven continuous identity into mint claim fields** | **P0** (cross-track) | UI opens watch with `domain_bind`, `dest_owner_binding`, proof digest — but does not pass observation amount/txid into a mint request body (no mint request at all). OBSERVE/MINT own claim field binding; UI cannot close that loop alone. |

### Severity legend (this track)

- **P0** — blocks honest full BTC→Terp→ZEC film or invents success  
- **P1** — funded-local fidelity / identity / chain proof gaps  
- **P2** — ergonomics, residual trust knobs, env friction  

---

## 4. Dependencies on other tracks

| Track | Why UI depends |
|-------|----------------|
| **HARNESS-ICT** | Real Daemon `BridgeMintNote`, automation PUT with truthful phases/receipt, hash-market URL, local Esplora for reverify |
| **OBSERVE** | Reporter posts real 64-hex `txid` + amount + confs bound to `intent_id`; open watch schema stability |
| **MINT-HEADSTASH** | Claim field schema (ν / amount / owner_binding); queryable `IsBridgeMinted`; mock_verify vs proof policy labels |
| **SWAP-DEX** | Whether automation `swapping`/`complete` means pure film or chain settle — UI copy must match |
| **ZAKURA-ZEC** | Dest binding SSOT (already shared golden); any egress proof UI could poll |
| **DOCS** | USER-GUIDE / book must say UI polls host, does not mint |

**Does not depend on UI for mint correctness:** chain rejects bad proofs regardless of film. UI honesty depends on not celebrating `complete` with sim receipt when host was pure-only.

---

## 5. Recommended P0 slice for this track

**Smallest shippable UI slice (no CosmWasm-in-browser required):**

1. **Honest complete path**  
   - When automation returns `phase=complete`, prefer host `receipt` (if present) over `buildMockSuccessReceipt`.  
   - Label receipt: `mock_verify` / pure vs chain (from automation flags).  
   - Keep `buildMockSuccessReceipt` for `lab_simulated` only.

2. **Optional chain mint attestation (read-only)**  
   - If `headstashContract` + LCD/RPC configured: query `IsBridgeMinted` (or display tx hash from host receipt). Fail-closed or warn if host complete but chain not minted.

3. **Bind observation into status/receipt display**  
   - Show observed `txid`, amount, confs + reverify result on receipt.  
   - Do not invent mint/swap refs.

4. **Operator continuity**  
   - Persist `awaiting_host` with one-click copy of `hostAutomationHint` (already partial) + re-poll without full wizard reset.

**Explicit non-goal for this P0 slice:** browser Halo2 / wallet `BridgeMintNote` (defer unless product requires user-signed mint). Host remains mint executor for ict_local_funded.

---

## 6. Explicit non-claims

- UI does **not** claim mainnet Cash App private API or mainnet money movement.  
- UI does **not** execute `BridgeMintNote`, private DEX settle, or ZEC broadcast.  
- Poll `complete` ≠ chain mint or ZEC settlement — especially when host is `corridor-lab-mint-after-observe.sh` pure film.  
- `ict_local_funded` workflow **shape** in UI is production-like (reverify + poll); **money proof** remains HARNESS.  
- Fail-closed Esplora reverify is implemented at Wizard call-site (prior I-P0-2 residual from REVIEW-OLINE is **closed in code** for 64-hex txids); remaining trust gaps are host authority + open observation bus + skip override.  
- D1–D7 freezes not amended by this report.

---

## File map (UI track)

| Path | Role |
|------|------|
| `…/PrivateCorridor/PrivateCorridorWizard.tsx` | Film + SSE → reverify → poll automation |
| `…/PrivateCorridor/automationClient.ts` | Fail-closed reverify + automation poll (no mint execute) |
| `…/PrivateCorridor/depositNotify.ts` | Watches / SSE / lab report |
| `…/PrivateCorridor/btcReverify.ts` | Esplora re-verify |
| `…/PrivateCorridor/mock.ts` | Intent seal, STATUS_FILM, **mock receipt** |
| `…/PrivateCorridor/zakuraDest.ts` | Dest soft-validate + binding domain |
| `…/PrivateCorridor/browserWallet.ts` | HD deposit wallet + client proof |
| `…/PrivateCorridor/types.ts` | Intent / module / phases |
| `…/PrivateCorridor/OPERATOR.md` | Operator host mint instructions |
| `docs/plans/spectrum/e2e/corridor-lab-mint-after-observe.sh` | Host automation UI polls |

---

## Bottom line

| Question | Answer |
|----------|--------|
| Does UI call `BridgeMintNote`? | **No** |
| What does UI do after observe? | **Re-verify (optional/fail-closed) + poll host automation** |
| Can UI alone complete true BTC→Terp→ZEC? | **No** — needs host/HARNESS mint+swap and ZAKURA egress; today even “complete” receipt is still sim-shaped |
| Strongest UI green today | Intent + wallet + notify + D4 reverify call-site + honest lab/ict banners + host poll loop |
| Strongest UI gap for full e2e | No mint execute; mock receipt after host complete; no ZEC/swap chain proof in UI |
