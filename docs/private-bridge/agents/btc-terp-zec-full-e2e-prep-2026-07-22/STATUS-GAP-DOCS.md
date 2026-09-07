# STATUS-GAP-DOCS

| Field | Value |
|-------|--------|
| **Track** | `DOCS` |
| **Date** | 2026-07-22 |
| **Mode** | Gap analysis + wording recommendations (no deep doc rewrite) |
| **Scope** | USER-GUIDE, book workflows/intro, product DEMO/OPERATOR shells, e2e honesty tables |
| **Ground truth** | `just demo-corridor-ict`: BTC regtest observe + Daemon `BridgeMintNote` + **pure** W0–W7 swap **film** — **not** live ZEC settlement |

---

## 1. Goal for full BTC → Terp → ZEC (this track)

Docs must describe **two bars** without collapsing them:

| Bar | Meaning (user + operator + book) |
|-----|----------------------------------|
| **Full multi-net e2e (north star)** | Bitcoin fund → observe → private mint on Terp (spectrum headstash) → **oracle-bound private swap settled on-chain / note pool** → **ZEC dest open / live egress** (local or mainnet rails per profile) |
| **Certified today** | Same *workflow shape* with honest residuals: regtest observe, local-chain mint (`mock_verify` default), **pure** swap film + automation phases, **dest binding / Zakura golden** — not ZEC chain send |

**DOCS success criteria for full bar:**

1. Every product surface that lists phases labels **which layer is live vs pure/film vs residual**.  
2. “Mainnet-funded-ready” / “production-shaped” never implies **mainnet money** or **live ZEC egress**.  
3. `ict_local_funded` success criteria match `corridor-ict-funded.sh` (observe + chain mint; step 5 = pure swap film).  
4. Book sequence / layer diagrams do not show `UI → ZEC dest open` as if a Zcash transfer already ran.  
5. Residual gaps called by other tracks (identity couple, on-chain swap, Zakura broadcast) appear in **user/operator residual tables**, not only in engineering STATUS files.

---

## 2. Code reality today (cite paths / commands)

| Surface | Status | Evidence |
|---------|--------|----------|
| Certified funded command | Green for observe + chain mint + pure film | `docs/plans/spectrum/e2e/corridor-ict-funded.sh` steps 1–5: “mint-after-observe **pure swap film** + automation API”; exit 0 if observe + chain mint succeed |
| Funded just target | Documented + green | `cd crates/headstash && just demo-corridor-ict` — STATUS-WASM-FUNDED-GREEN |
| Lab floor | Green pure/Mock film | `just demo-corridor-lab`; `corridor-lab-mint-after-observe.sh` (explicit “no mainnet ZEC send”) |
| USER-GUIDE | Strong profile honesty; **under-specifies residual after mint** | `docs/plans/spectrum/USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md` |
| Book workflows | Correct three-profile frame; sequence implies full ZEC open | `docs/plans/spectrum/book/src/workflows/index.md` + `diagrams/mmd/workflow-cashapp-corridor.mmd` |
| Book intro spine | Same shape; no pure-vs-chain layer | `docs/plans/spectrum/book/src/introduction.md` |
| DEMO product SPEC | Aspirational “Not a mock film” / production default | `docs/plans/spectrum/DEMO-CASHAPP-ZEC-CORRIDOR.md` §1, non-goals §2 (full ZEC egress = north star stub) |
| OPERATOR | Honest lab ≠ mainnet; **stale “when HARNESS lands”** for `demo-corridor-ict` | `websites/.../PrivateCorridor/OPERATOR.md` L15 |
| CORRIDOR-LAB-STATUS | Best residual honesty in engineering docs | `e2e/CORRIDOR-LAB-STATUS.md` “Honest non-claims” + residual list |
| e2e scripts | Honest headers | `corridor-ict-funded.sh`, `corridor-lab-mint-after-observe.sh`, `e2e/README.md` |

### What docs already get right

- Three profiles: `lab_simulated` · `ict_local_funded` · `production` (USER-GUIDE §2, book workflows).  
- Local/testnet success **≠** mainnet settlement (USER-GUIDE §2, §7.3, FAQ; book intro).  
- `mock_verify=true` default on funded deploy is labeled (USER-GUIDE §7.2 table; CORRIDOR-LAB-STATUS residual #6).  
- Oracle `bound_only` / never mints; notify is coordination only.  
- DEMO non-goal table already: “Full ZEC mainnet egress / TZE — North star; stub enough for v0.”  
- e2e layer is more honest than product narrative about pure swap film.

---

## 3. Gaps (goal vs code)

| ID | Gap | Severity | Notes |
|----|-----|----------|-------|
| **D-G1** | **USER-GUIDE §7.2 recipe lists post-mint as if live continuous path** | **P0** | Recipe: “oracle-bound swap → preauth dest binding check (Zakura local…)” without saying funded stack runs **pure** W0–W7 / mint-after-observe **film**, not on-chain private DEX settle or ZEC send. Contrasts `corridor-ict-funded.sh` L12. |
| **D-G2** | **“Mainnet-funded-ready composition” header** | **P0** | USER-GUIDE L7 status line. Sprint bar phrase is easy to misread as “ready for mainnet money / full ZEC.” Needs “workflow-shape fidelity” + explicit residual list adjacent to the status badge. |
| **D-G3** | **§3 “one continuous path” includes swap** | **P0** | “chain mint → oracle-bound swap → preauth dest is one continuous path” overclaims **coupling** (observe fields → mint claim may be sequenced, not fully identity-coupled) and **swap layer** (pure film on certified path). |
| **D-G4** | **Book sequence diagram Phase 3: `UI → Z: Open only dest_owner_binding`** | **P0** | `workflow-cashapp-corridor.mmd` L39–45. Reads as live ZEC open/egress. Missing alt: pure film / dest-binding check only / residual live egress. |
| **D-G5** | **Book intro + workflows prose omit pure-vs-chain for swap/ZEC** | **P1** | `introduction.md` spine and `workflows/index.md` profile table: mint column honest for funded (`Daemon BridgeMintNote`); **swap/ZEC columns missing**. “same workflow shape” without residual table invites overclaim. |
| **D-G6** | **DEMO §1 “Not a mock film” vs certified pure film after mint** | **P1** | Product SPEC correctly targets production surface, but book includes DEMO as SSOT without a “certified local path today” callout. Readers confuse SPEC target with `demo-corridor-ict` proof. |
| **D-G7** | **OPERATOR stale pointer + incomplete residual** | **P1** | L15: “Funded local multi-net (**when HARNESS lands**)” — HARNESS landed; should cite `just demo-corridor-ict` + pure swap residual. Still good on Zakura paste-first / no mainnet ZEC for lab. |
| **D-G8** | **Under-specified identity residual in user-facing docs** | **P1** | Full bar needs `intent_id` / deposit observation → BridgeMint claim fields. USER-GUIDE stage table binds dest/expiry well; does **not** say observation→mint claim may be host-sequenced film, not one cryptographic handoff. |
| **D-G9** | **Under-specified on-chain private swap residual** | **P1** | USER-GUIDE §5.1 “Swap under oracle bounds” cites policy primitives; does not say **no on-chain private DEX settle from minted note** on certified path (pure `private_dex` / cashapp fixtures). Engineering STATUS and CORRIDOR-LAB-STATUS residual #7 (browser→chain mint via Daemon; UI polls automation) are better. |
| **D-G10** | **Zakura = dest binding vs live receive** | **P1** | USER-GUIDE §7.3 S5 note + OPERATOR D6 are decent; book workflows never show “dest preauth only” vs “regtest ZEC receive.” Full e2e bar needs one sentence in product residual table. |
| **D-G11** | **Automation phases as success UX** | **P2** | UI film `bridging→swapping→complete` documented as production path poll; must always pair with “automation status is **not** mint/swap authority” (present in CORRIDOR-LAB-STATUS, weaker in USER-GUIDE §6). |
| **D-G12** | **Include lag / dual SSOT risk** | **P2** | Book shells include USER-GUIDE and DEMO; wording fixes must edit monorepo sources (`docs/plans/spectrum/*.md`, OPERATOR.md), then `book` sync — not `_include/` by hand. |

### Overclaim map (short)

| Claim pattern in docs | True for full bar? | True for certified today? |
|-----------------------|--------------------|---------------------------|
| Intent before fund + dest binding | Yes | Yes (pure + UI) |
| BTC regtest/signet observe → `deposit_observed` | Yes | Yes (`demo-corridor-ict`) |
| Live local Terp `BridgeMintNote` / `IsBridgeMinted` | Yes | Yes (Daemon; `mock_verify` default) |
| Oracle-bound private **swap settled** from minted note | Yes | **No** — pure W0–W7 film |
| Live ZEC open / egress to preauth dest | Yes | **No** — binding check / Zakura golden; no mainnet/regtest ZEC send required |
| Mainnet Cash App / mainnet money | Ops/external | **No** |
| “Mainnet-funded-ready” as ops go-live | No (keys/liquidity/legal) | Workflow-shape only |

---

## 4. Dependencies on other tracks

| Track | What DOCS needs from them |
|-------|---------------------------|
| **HARNESS-ICT** | Exact exit criteria of `demo-corridor-ict` (what fails hard vs soft); mint-only residual flags; whether swap film is always invoked |
| **OBSERVE** | Whether observe payload fields are wired into mint claim identity (for residual wording on “continuous path”) |
| **MINT-HEADSTASH** | `mock_verify` vs real proof policy language for funded vs production; `SeamNoteOutV0` still sole egress (D1) |
| **SWAP-DEX** | On-chain private swap settle status; phrase “pure film” vs “chain settle” |
| **UI** | Whether automation complete implies user-visible “ZEC sent”; banner strings for `ict_local_funded` |
| **ZAKURA-ZEC** | Dest binding golden vs live regtest receive/egress residual (D6) |
| **META** | Cross-track residual matrix language DOCS should mirror in USER-GUIDE residual table |

DOCS does **not** amend D1–D7 freezes; only labels current proof vs freeze north star.

---

## 5. Recommended P0 slice for this track

**Single shippable docs honesty pass** (no product redesign):

1. **USER-GUIDE header** — replace or gloss status:
   - Prefer: **“Mainnet-workflow-shape ready (local multi-net fidelity)”**  
   - Keep: local success ≠ mainnet money  
   - Add one-line: **Certified path does not yet include live ZEC settlement or on-chain private swap settle.**

2. **USER-GUIDE §3 table** — split “Ready means” row:
   - Continuous **shape**: intent → fund → observe → reverify → chain mint → **(swap policy + dest check)**.  
   - Certified layers: swap = **pure/host film**; ZEC = **binding / demo dest**, not broadcast.

3. **USER-GUIDE §7.2 recipe** — after each step, layer tag, e.g.:

   ```text
   … → live-chain BridgeMintNote (Daemon; mock_verify default)
     → oracle-bound swap **pure film** (W0–W7 / mint-after-observe; not chain DEX settle)
     → preauth dest binding check (Zakura golden / local dest — not ZEC egress)
   ```

   Align command notes with `corridor-ict-funded.sh` step 5.

4. **USER-GUIDE new residual subsection** (or §7.3 expand) — mirror CORRIDOR-LAB-STATUS honest non-claims for **product readers**:
   - No mainnet BTC/ZEC settlement  
   - No live ZEC transfer on certified funded path  
   - No on-chain private DEX settle from minted note (pure film)  
   - Default `mock_verify=true` on funded  
   - Observe→mint claim identity coupling residual (if OBSERVE confirms)  
   - Automation bus ≠ mint authority  

5. **Book `workflows/index.md`** — extend profile table:

   | Profile | Observe | Mint | Swap | ZEC |
   |---------|---------|------|------|-----|
   | `lab_simulated` | Synthetic | mock_verify / Mock | pure film | binding / film |
   | `ict_local_funded` | Regtest + reporter | Daemon BridgeMintNote | **pure film** | dest binding (live egress residual) |
   | `production` | Approved index | Deploy proof policy | Deploy policy (not certified full here) | Deploy rails |

6. **Book mermaid** — Phase 3 note:

   ```text
   Note over UI,Z: Certified today = pure swap film + dest binding check
   Note over UI,Z: Full bar residual = on-chain settle + live ZEC open/egress
   ```

   Or `alt pure film / residual live egress`.

7. **OPERATOR.md L15** — drop “when HARNESS lands”; point at `just demo-corridor-ict` + USER-GUIDE residual.

8. **DEMO** — optional one box under §1: “SPEC target vs certified local proof (`demo-corridor-ict`)” so book include of DEMO does not override e2e honesty. Do **not** weaken non-goals; full ZEC egress already north-star.

**Explicit non-work for this P0:** marketing site rewrite; diagram SVG regen can wait until mmd sources change; no freeze amendments.

---

## 6. Explicit non-claims (docs track)

- This STATUS does **not** claim full BTC→Terp→ZEC multi-net e2e is documented as complete.  
- This STATUS does **not** amend USER-GUIDE/book files in this pass (recommendations only; critical wording listed above for follow-up).  
- Engineering e2e scripts and CORRIDOR-LAB-STATUS remain more precise than product composition for swap/ZEC residual — **product docs should catch up**, not the reverse.  
- “Mainnet-funded-ready” in sprint vocabulary = **workflow fidelity on local nets**, not ops go-live or live ZEC.  
- D1–D7 freezes stand; DOCS only clarifies **what is proven vs residual under those freezes**.

---

## 7. Wording cookbook (copy-ready)

### Full bar (north star)

> **Full multi-net e2e:** Bitcoin deposit observed → private mint of `SeamNoteOutV0` on Terp → oracle-bound private swap **settled from the minted note** → **live ZEC receive/open** only to pre-authorized `dest_owner_binding`. Profiles change networks and proof policy, not the phase order.

### Certified today (`ict_local_funded`)

> **Certified local path (`just demo-corridor-ict`):** BTC **regtest** fund + `corridor-btc-reporter` → `deposit_observed` → ict-rs Terp + Daemon **`BridgeMintNote`** (`IsBridgeMinted`; default `mock_verify=true`) → **pure** cashapp W0–W7 / mint-after-observe **swap film** + automation phases. **Not** live Zcash transfer. **Not** on-chain private DEX settle. **Not** mainnet money.

### Forbidden conflations

| Do not write | Write instead |
|--------------|---------------|
| “End-to-end ZEC” for `demo-corridor-ict` alone | “Funded local mint + pure swap film; ZEC dest binding only” |
| “Mainnet-funded-ready” without residual | “Mainnet-**workflow-shape** ready; residuals: live ZEC, chain swap settle, proof policy, ops keys” |
| “One continuous path” for swap+ZEC on ict | “Sequenced host path; swap/ZEC layers pure/film until residual closes” |
| “UI open ZEC dest” as certified success | “Preauth dest binding enforced; live egress residual” |
| “Not a mock film” for funded post-mint swap | “Chain mint is live local; post-mint swap is still pure film” |

### Profile one-liners (book / USER-GUIDE)

| Profile | One-liner |
|---------|-----------|
| `lab_simulated` | Explicit lab; synthetic observe + mock mint OK; every success is lab-controlled. |
| `ict_local_funded` | Fidelity gate: real local chain mint + regtest observe; **swap pure film**; **no mainnet / no required live ZEC send**. |
| `production` | Same shape on real rails when that deploy enables them; reverify fail-closed; still not implied by local green alone. |

---

## 8. File checklist (edit targets for follow-up)

| Priority | Path | Change |
|----------|------|--------|
| P0 | `docs/plans/spectrum/USER-GUIDE-CASHAPP-PRIVATE-BRIDGE.md` | Status gloss; §3 continuous-path split; §7.2 layer tags; residual table |
| P0 | `docs/plans/spectrum/book/src/workflows/index.md` | Profile columns for Swap + ZEC |
| P0 | `docs/plans/spectrum/book/src/diagrams/mmd/workflow-cashapp-corridor.mmd` | Pure film / residual notes on Phase 3 |
| P1 | `docs/plans/spectrum/book/src/introduction.md` | Spine annotations for certified vs full |
| P1 | `websites/.../PrivateCorridor/OPERATOR.md` | Remove stale “when HARNESS lands”; residual line |
| P1 | `docs/plans/spectrum/DEMO-CASHAPP-ZEC-CORRIDOR.md` | Optional “SPEC vs certified path” callout under §1 |
| P2 | SVG regen after mmd change | `book` diagrams README / just recipe |
| — | `docs/plans/spectrum/e2e/*` | Already honest; **do not weaken** |

Book shells under `book/src/product/*.md` are thin includes — edit monorepo SSOT, then rebuild book (`scripts/sync_includes.py`).

---

## 9. Summary for META

| Question | DOCS answer |
|----------|-------------|
| Do docs overclaim full e2e? | **Yes, mildly but load-bearing:** product spine and §7.2/sequence diagrams read as live swap+ZEC; engineering e2e is more careful. |
| Do docs under-specify residual? | **Yes:** pure swap film, live ZEC egress, observe→mint identity coupling, on-chain DEX settle under-specified for end users. |
| Smallest fix? | USER-GUIDE §7.2 + status gloss + workflow mermaid Phase 3 note. |
| Blocking code? | No — pure documentation honesty pass; tracks HARNESS/SWAP/ZAKURA own residual close. |
