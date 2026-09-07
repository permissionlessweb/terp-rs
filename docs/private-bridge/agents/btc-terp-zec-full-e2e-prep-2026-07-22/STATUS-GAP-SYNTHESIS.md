# STATUS-GAP-SYNTHESIS — Full e2e BTC → Terp → ZEC

| Field | Value |
|-------|--------|
| **Date** | 2026-07-22 |
| **Epic** | `btc-terp-zec-full-e2e-prep-2026-07-22` |
| **Mode** | Cross-track gap synthesis (no product implementation) |
| **Inputs** | STATUS-GAP-HARNESS, OBSERVE, MINT, SWAP, UI, ZAKURA, DOCS, META |
| **Board** | `private-bridge-corridor` |

---

## 0. Executive answer

| Question | Answer |
|----------|--------|
| Is full BTC → Terp → ZEC multi-net e2e green? | **No.** |
| Is `just demo-corridor-ict` green for **funded workflow shape**? | **Yes** — regtest observe + local-chain mint + pure swap film. |
| What is the dominant structural gap? | **Identity uncoupled:** observe fields ↛ mint claim ↛ swap note ↛ ZEC egress. Stages are **sequenced**, not **continuous**. |
| Smallest next bar | **Deposit-backed mint identity** (ν + amount + dest from watch/obs) + harness fail-closed wire + honest UI/docs residual. Still mock_verify; still pure swap; still no live ZEC. |

```text
NORTH STAR:
  intent_I → fund → observe(I, tx, amt, dest)
           → BridgeMintNote(claim←observe) → note_N
           → private_swap(note_N, oracle bound) → zec_note_Z
           → open/egress only to dest_owner_binding

CERTIFIED TODAY (demo-corridor-ict):
  intent_A ─label─► fund ─► observe(A, real tx/amt)
  fixture_claim ──► BridgeMintNote (Daemon) ──► IsBridgeMinted
  pure W0–W7 (separate identity) + automation PUT film
  [no ZEC chain] [mock_verify default] [no deposit-backed claim]
```

---

## 1. Matrix: track × goal × reality × P0 gaps

| Track | Goal (full multi-net) | Reality today | P0 gaps (IDs) |
|-------|----------------------|---------------|---------------|
| **HARNESS-ICT** | One continuous harness: observe → deposit-backed mint → swap of **that** note → ZEC stage attach; soft-skip FAIL | `corridor-ict-funded.sh` sequences BTC observe + fixture Daemon mint + pure W0–W7 + automation; no claim env pass; no Zakura join | **H-G1** observe→mint uncoupled; **H-G2** mint→swap uncoupled; **H-G3** no ZEC stage |
| **OBSERVE** | Trusted field-complete deposit fact → continuous claim inputs (`txid`, `vout`, `amount`, addr, watch dest) | Watch/observe/SSE/reporter/S6 amount gates **green**; stages share `intent_id` string only; no claim builder / claim-inputs API | **O-G1** no continuous claim identity; **O-G2** ν rule unfrozen on chain path; **O-G3** amount not bound to mint value; **O-G4** funded path never passes obs into mint binary |
| **MINT-HEADSTASH** | Deposit-backed `BridgeMintNote` → DEX-consumable `SeamNoteOutV0` with dest/rcm continuity | Contract + ict fixture mint green (`IsBridgeMinted`, double-mint); `BridgeL1World::happy()`; mock_verify default; no deposit mapper | **M1** no claim-from-deposit builder; **M2** fixture not deposit amount/id; **M7** dest not from watch; **M11** ν identity undefined on chain path; (**M3/M4** proof trust for prod, residual under D7 lab) |
| **SWAP-DEX** | Spend **minted** SEAM → oracle-bound private swap → ZEC-side note; eventually on-chain settle | L0 pure DEX + intent gates + compose burn→swap **green film**; W5 synthetic `NoteIn`; **no** CW swap msg | **S-G1/S-G10** no on-chain private DEX; **S-G2** chain mint ↛ film swap; **S-G3** W5 does not spend SeamNoteOutV0 |
| **UI** | Honest continuous film: reverify → host/chain mint truth → swap truth → ZEC truth; no invented success | Intent + wallet + watch + D4 reverify + poll automation **implemented**; **never** executes `BridgeMintNote`; complete → **mock receipt** | **UI-G1** no mint execute (by design); **UI-G2** ignores host receipt; **UI-G5** no ZEC confirm; **UI-G6** swap phase string only; **UI-G15** no claim-field handoff |
| **ZAKURA-ZEC** | Preauth dest seal + local open/receive + (later) egress to preauth only | Golden binding / paste / offline just targets **green**; live node optional skip; **not** joined to funded ICT (`"b"*64` dest) | **Z-G1** no live egress; **Z-G2** no live receive proof; **Z-G3** funded ICT ignores golden dest; **Z-G4** W7 open = binding film only |
| **DOCS** | Two bars labeled: full multi-net vs certified local; no overclaim on swap/ZEC | Profile frame + mainnet-money non-claim strong; **product spine** still reads as live swap+ZEC continuous path; e2e scripts more honest | **D-G1–D-G4** USER-GUIDE continuous path / recipe / “mainnet-funded-ready” / book Phase 3 ZEC open |
| **META** | Matrix + DAG + residual language | All 7 track reports landed; this synthesis | **META-G1** three uncoupled identity planes; **META-G2** ZEC off critical path; **META-G3** docs over-read |

### P0 cross-cut themes (not per-track)

| Theme | Tracks | One-line |
|-------|--------|----------|
| **T1 Continuous deposit identity** | OBSERVE + MINT + HARNESS | `txid/amount/dest` → `BridgeMintClaimPublic` (ν, value, dest_commitment) |
| **T2 Note lifecycle into swap** | MINT + SWAP + HARNESS | Minted SEAM openings → `build_swap_action_from_seam_notes` (still pure OK for intermediate bar) |
| **T3 Dest seal through funded path** | ZAKURA + HARNESS + MINT | Replace placeholder dest; assert binding equality end-to-end |
| **T4 Honesty surfaces** | UI + DOCS | Host receipt truth; pure film labels; no “ZEC sent” until egress exists |
| **T5 Live ZEC / on-chain DEX** | ZAKURA + SWAP | Full multi-net residual **after** T1–T4 |

---

## 2. What `demo-corridor-ict` covers vs next bar

### 2.1 Certified today (`just demo-corridor-ict` / `corridor-ict-funded.sh`)

| Stage | Layer | Covered? | Notes |
|-------|-------|----------|-------|
| Preauth / watch open | Host HTTP | **Partial** | Watch opened with `intent_id`; crypto binds often **placeholders** (`"a"*64` / `"b"*64`) |
| Fresh BTC deposit addr | Chain regtest | **Yes** | bitcoind `getnewaddress` + fund |
| Fund | Chain regtest | **Yes** | send + generate |
| Observe + amount gate | Host + reporter | **Yes** | `deposit_observed`; S6 min_amount reject path |
| Chain private mint | Terp ict-rs Daemon | **Yes (fixture)** | `BridgeMintNote` + `IsBridgeMinted` + double-mint reject; **not** deposit-backed |
| `mock_verify` | Contract policy | **Labeled default true** | D7 residual; not Tier-0 soundness |
| Oracle-bound private swap | Pure film | **Film only** | W0–W7 + mint-after-observe automation phases |
| On-chain private DEX settle | Chain | **No** | No CW swap execute |
| ZEC dest binding | Golden / UI offline | **Parallel only** | Not required for funded exit; funded watch dest placeholder |
| Live ZEC receive/egress | Zakura / zcashd | **No** | Explicit non-goal of certified path |
| Mainnet money / Cash App API | Ops | **No** | Explicit non-goal |
| Continuous single identity | All | **No** | Shared `intent_id` label ≠ claim continuity |

**Exit criterion of funded script (honest):** observe + chain mint succeed → exit 0. Step 5 = pure swap film. Soft-skip of mint is FAIL.

### 2.2 Next bar (recommended “continuous local multi-net v1”)

Still **local nets only**. Still **`mock_verify` allowed**. Still **no mainnet**. Closes **identity**, not full ZK/ZEC product.

| Must add | Owner tracks | Closes |
|----------|--------------|--------|
| Frozen provisional ν + amount + dest from watch/obs → claim | OBSERVE + MINT | O-G1–3, M1–2, M7, M11 |
| `claim_from_deposit` (or equivalent) + fail-closed if missing | MINT | M1 |
| Funded harness: GET claim-inputs / obs → Daemon mint (no silent happy fixture) | HARNESS | H-G1, O-G4 |
| Golden (or live) `dest_owner_binding` on funded watch | ZAKURA + HARNESS | Z-G3, H-G6 partial |
| Optional: note openings → pure swap spend of **that** SEAM (not synthetic NoteIn) | SWAP + HARNESS | S-G2, S-G3 (pure) |
| Automation receipt carries real mint cm/ν/contract; UI prefers host receipt | HARNESS + UI | H-G7, UI-G2 |
| USER-GUIDE / book residual table + Phase 3 pure-film note | DOCS | D-G1–D-G4 |

**Explicitly still residual at next bar:** CW private DEX settle (S-G1), live ZEC egress (Z-G1), SP1/`mock_verify=false` (M4), Halo2 swap (S-G9).

### 2.3 Full multi-net bar (north star)

| Must add beyond next bar | Owner |
|--------------------------|-------|
| On-chain mock or real private swap settle spending minted note | SWAP + MINT + HARNESS |
| ZEC-side note artifact + preauth-only open | SWAP + ZAKURA |
| Live regtest receive and/or broadcast to preauth dest | ZAKURA + HARNESS |
| LC/reflection-fed roots + real membership proof path | MINT (+ SEAM-LC) |
| Production proof policy + fail-closed HTTP oracle mid for product path | MINT + SWAP + oracle |

---

## 3. Recommended implementation DAG

Dependencies flow **left → right**. Do not start Phase D/E work as the funded “definition of green” before Phase A/B.

```text
Phase A — Freeze identity SSOT (OBSERVE ∥ MINT)
  A1. Provisional ν rule (document + pure fn)
      nullifier = H("terp-btc-deposit-nu-v0" ‖ txid ‖ vout ‖ intent_id)
  A2. claim_from_deposit_watch(obs, watch, policy) → BridgeMintClaimPublic
      value ← amount_sats; dest ← dest_owner_binding; lab roots/flags labeled
  A3. GET /corridor/claim-inputs/:intent_id (+ optional vout on observation)
  ────────────────────────────────────────────────────────────
Phase B — Wire funded path (HARNESS + ZAKURA dest join)
  B1. corridor-ict-funded: after deposit_observed, fetch claim-inputs
  B2. corridor_ict_funded: mint from claim-inputs; FAIL if unmappable
      (happy fixture only lab / CORRIDOR_ALLOW_MINT_ONLY residual)
  B3. Assert IsBridgeMinted(ν) ∧ value consistent with obs amount
  B4. Watch dest_owner_binding = golden primary (or validateaddress when RPC up)
  B5. Emit mint evidence artifact (contract, ν, tx, cm) for automation
  ────────────────────────────────────────────────────────────
Phase C — Honesty (UI ∥ DOCS)  [can start after A wording; hard-require B5]
  C1. UI: prefer host automation receipt over buildMockSuccessReceipt
  C2. UI: show observed txid/amount/reverify; label pure vs chain
  C3. DOCS: USER-GUIDE residual table + §7.2 layer tags; book Swap/ZEC columns;
      mermaid Phase 3 pure-film note; OPERATOR drop “when HARNESS lands”
  ────────────────────────────────────────────────────────────
Phase D — Note-coupled pure swap (SWAP + HARNESS)  [after B note handoff]
  D1. W5 / product path: spend BridgeMint SEAM openings (no synthetic NoteIn)
  D2. Oracle mid required on product burn→swap path; asset_out = corridor ZEC id
  D3. Thread mint cm into automation receipt “spent” fields
  ────────────────────────────────────────────────────────────
Phase E — On-chain private swap settle (SWAP + MINT)  [after D]
  E1. CW mock_verify swap apply stub (reserves + pool ν) OR dedicated module
  E2. Funded harness asserts pool state / spend of chain-minted note
  ────────────────────────────────────────────────────────────
Phase F — Live ZEC open/egress lab (ZAKURA + HARNESS)  [after dest continuous B4;
          full product egress prefers after E for honest “settle then pay”]
  F1. Funded optional Zakura up + receive visibility (miner/lab fund)
  F2. Egress broadcast path to preauth dest (still local/regtest)
  F3. UI poll/confirm only when F proves chain ZEC
  ────────────────────────────────────────────────────────────
Phase G — Production soundness (later)
  G1. mock_verify=false + LC reflection feed + real membership
  G2. Halo2 private swap (explicit non-goal until E film/mock settle solid)
  G3. Mainnet rails / ops keys — never implied by local green
```

### DAG critical path (shortest path to “continuous local multi-net v1”)

```text
A1 → A2 → B1 → B2 → B3
A3 ↗        B4 (parallel with B1 once golden SSOT known)
B5 → C1/C2 (UI honesty)
A + certified residual wording → C3 (DOCS)  [docs can ship early]
```

### What not to parallelize as P0

| Temptation | Why wait |
|------------|----------|
| Halo2 swap circuit | DEMO non-goal until pure + policy + mock settle |
| Mainnet ZEC send | D6 Phase 2; after local open proof |
| Browser CosmWasm `BridgeMintNote` | Optional product; host mint OK for ict_local_funded |
| SP1 verify before deposit-backed claim | Wrong order: first bind deposit under mock, then harden verify |

### Per-track P0 slices (aligned to DAG)

| Track | P0 slice (from sibling STATUS) | Phase |
|-------|--------------------------------|-------|
| OBSERVE | claim-inputs API + vout + document ν inputs | A |
| MINT | `claim_from_deposit` + ν policy pure fn | A |
| HARNESS | obs→mint wire fail-closed + evidence artifact | B |
| ZAKURA | golden dest into funded watch | B |
| SWAP | SEAM-coupled pure spend + oracle on product path | D |
| UI | host receipt + observed fields; no sim celebrate | C |
| DOCS | honesty pass USER-GUIDE + workflows mermaid | C |

---

## 4. Stage graph (current vs target)

### Current (`demo-corridor-ict`)

```text
corridor-ict-funded.sh
  │
  ├─[BTC chain]  regtest fund + reporter ──► deposit_observed(intent_A, real tx/amt)
  │                                              │
  │                                              │  (no claim fields passed)
  │                                              ▼
  ├─[Terp chain] corridor_ict_funded ── BridgeMintNote(fixture_claim)
  │                    │
  │                    ├─ pure W0–W7 (simulated, zeros / scenario identity)
  │                    └─ stop chain (unless KEEP_CHAIN)
  │
  └─[pure/host]  mint-after-observe(SKIP_WAIT=1, label intent_A)
                     pure cargo tests + automation PUT
                     (no ZEC egress; UI would mock-receipt on complete)
```

### Target continuous graph

```text
intent_I (+ golden dest_owner_binding)
  → fund → observe(I, tx, vout, amt)
  → claim_from_deposit → BridgeMintNote → note_N (assert value/dest/ν)
  → private_swap(note_N, oracle bound) → zec_note_Z   [pure then chain]
  → open/egress only to dest_owner_binding            [film then live]
  → automation/receipt carry I + N + (zec_txid if live)
```

---

## 5. Explicit non-claims

### Do **not** claim from this synthesis or from `demo-corridor-ict` green

1. **Full multi-net BTC → Terp → ZEC settlement** is complete.  
2. **Deposit-backed mint** — chain mint uses hinge fixture, not regtest observation fields.  
3. **On-chain private DEX** or swap of the Daemon-minted note.  
4. **Live Zcash transfer / Zakura broadcast / regtest receive proof** as part of funded exit.  
5. **Mainnet Cash App, mainnet BTC, mainnet ZEC, or mainnet money.**  
6. **Tier-0 / production mint soundness** under default `mock_verify=true`.  
7. **UI executes `BridgeMintNote`** or proves mint on-chain by itself.  
8. **Automation `complete`** = cryptographic mint/swap/ZEC authority.  
9. **`SKIP_REGTEST` + `CORRIDOR_ALLOW_MINT_ONLY`** = funded S1 success.  
10. **`demo-zakura-local-dest`** alone = multi-net settle.  
11. **D1–D7 freezes reopened or amended** by this epic.  
12. **Oracle mints or credits balances** — pure APIs hard-reject (`bound_only` / D7).

### Do claim (accurate)

1. **ict_local_funded workflow shape** is exercised: regtest observe + local Terp `BridgeMintNote` + pure oracle-bound swap **film**.  
2. **Notify plane** (watch, observation, SSE, reporter, min_amount gates) is green as coordination.  
3. **Double-mint reject** and `IsBridgeMinted` work on fixture path.  
4. **L0 private DEX seams + cashapp intent policy + compose burn→swap sketch** are green pure surfaces.  
5. **Zakura dest domain / golden binding** offline floor is green and ready for HARNESS consume.  
6. **UI reverify call-site** fail-closed for 64-hex txids is implemented (host still mint authority).  
7. Engineering e2e scripts / CORRIDOR-LAB-STATUS residual language is **more honest** than some product-spine wording — product docs should catch up (DOCS Phase C).

---

## 6. Sibling report index

| Track | Path |
|-------|------|
| HARNESS | [`STATUS-GAP-HARNESS.md`](./STATUS-GAP-HARNESS.md) |
| OBSERVE | [`STATUS-GAP-OBSERVE.md`](./STATUS-GAP-OBSERVE.md) |
| MINT | [`STATUS-GAP-MINT.md`](./STATUS-GAP-MINT.md) |
| SWAP | [`STATUS-GAP-SWAP.md`](./STATUS-GAP-SWAP.md) |
| UI | [`STATUS-GAP-UI.md`](./STATUS-GAP-UI.md) |
| ZAKURA | [`STATUS-GAP-ZAKURA.md`](./STATUS-GAP-ZAKURA.md) |
| DOCS | [`STATUS-GAP-DOCS.md`](./STATUS-GAP-DOCS.md) |
| META | [`STATUS-GAP-META.md`](./STATUS-GAP-META.md) |

### SSOT anchors

| Piece | Path |
|-------|------|
| Orchestration | `docs/plans/spectrum/agents/btc-terp-zec-full-e2e-prep-2026-07-22/ORCHESTRATION.md` |
| Funded script | `docs/plans/spectrum/e2e/corridor-ict-funded.sh` |
| Mint-after-observe film | `docs/plans/spectrum/e2e/corridor-lab-mint-after-observe.sh` |
| Product DEMO | `docs/plans/spectrum/DEMO-CASHAPP-ZEC-CORRIDOR.md` |
| Lab honesty | `docs/plans/spectrum/e2e/CORRIDOR-LAB-STATUS.md` |
| Funded green prior | `docs/plans/spectrum/agents/final-sprint-2026-07-22/STATUS-WASM-FUNDED-GREEN.md` |

---

## 7. Bottom line for next sprint planning

| Priority | Work | Outcome |
|----------|------|---------|
| **P0** | Phase A + B (identity + funded wire + golden dest) | Continuous **observe → deposit-backed mint** under mock_verify |
| **P0** | Phase C (UI receipt + DOCS residual) | Operators/users cannot confuse pure film with ZEC settle |
| **P1** | Phase D (SEAM-coupled pure swap) | Honest “private mint → private swap” note lifecycle on host |
| **P1–P2** | Phase E–F (CW swap + live ZEC lab) | True multi-net bar approachable |
| **Later** | Phase G | Production proofs / mainnet rails |

**One sentence:** Ship continuous deposit identity on the funded path first; treat pure swap film and Zakura binding as labeled residuals until note-coupled swap and live ZEC stages land — never redefine `demo-corridor-ict` green as full BTC→ZEC settlement.
