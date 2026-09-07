# STATUS-GAP-SWAP

| Field | Value |
|-------|--------|
| **Track** | SWAP-DEX |
| **Date** | 2026-07-22 |
| **Mode** | Gap analysis only (no implementation) |
| **Agent** | Grok |
| **North star (this track)** | BridgeMint **SeamNoteOutV0** (BTC-side note on Terp) → **oracle-bound private swap** → **ZEC-side note** spendable for egress; oracle **bound_only** (D7) |

---

## 1. Goal for full BTC → Terp → ZEC (this track)

Full multi-net workflow requires SWAP-DEX to provide:

1. **Spend of the actual BridgeMint note** (not a synthetic `NoteIn` with a host u64 nullifier).
2. **Oracle-bound private swap** (constant-product / `SwapActionV0`) that:
   - enforces depositor policy via `DepositIntentV0` + `intent_allows_swap` (dest, min_out, slip, expiry, mid freshness);
   - uses oracle mid as **acceptance bounds only** — never credits balances / never mints.
3. **ZEC-side output note** (`SeamNoteOutV0` or equivalent openings) whose `owner_binding` matches preauth dest and whose `asset_id` is the registered ZEC / sim-ZEC Terp id.
4. **Eventually on-chain settle** on Terp (pool reserves + pool-spend nullifiers in shared anonymity set), not only pure host film — so the chain mint from ict/Daemon is not stranded while a parallel pure suite pretends to swap.

**D freezes consumed (do not reopen):** D1 SeamNoteOutV0 only; D3 private mint **+** oracle-powered private swap required for demo; D7 oracle `bound_only`, mint = `cw-headstash` only.

**Honest split (today’s certified path vs goal):**

| Path | What it is |
|------|------------|
| **Pure swap film** (green) | `private_dex_seams` + `cashapp_zec_corridor` W0–W7 + `compose_seams` product path + host automation phases `bridging → swapping → complete` |
| **On-chain private DEX** (not green) | CosmWasm / Daemon apply of swap that spends the **minted** BridgeMint note into a ZEC-side note |

---

## 2. Code reality today (cite paths / commands)

| Surface | Status | Evidence |
|---------|--------|----------|
| **`private_dex_seams` L0** | **Green — 23 tests** | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/fixtures/private_dex_seams/` — `quote_exact_in`, `apply_swap`, `check_oracle_bound`, hard-reject `oracle_mint_note` / `oracle_update_reserves` → `ErrOracleDisabledMint`, `SwapActionV0` + `build_swap_action_from_seam_notes` / `apply_swap_action`, SEAM→swap + nullifier / wrong asset / min_out / stale / slippage. Run: `cd docs/plans/spectrum/fixtures/private_dex_seams && cargo test` |
| **`cashapp_zec_corridor` intent gate** | **Green — 11 tests** | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/fixtures/cashapp_zec_corridor/src/lib.rs` — `DepositIntentV0`, **`intent_allows_swap`**, `oracle_mint_forbidden`, I1–I6 reject matrix, `sim_deposit`. Run: `cd docs/plans/spectrum/fixtures/cashapp_zec_corridor && cargo test` |
| **`compose_seams` product pure e2e** | **Green — 9 tests** | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/fixtures/compose_seams/src/lib.rs` — `run_product_path_burn_to_swap_sketch`: register → bridge mint SEAM(+rcm) → `SwapActionV0` → `apply_swap_action` → reserves + pool ν; pool ν ≠ bridge ingress ν. C1–C4. Run: `cd docs/plans/spectrum/fixtures/compose_seams && cargo test product_path_burn_to_swap_sketch` |
| **Harness W0–W7 Simulated** | **Green (pure film)** | `/Users/returniflost/abstract/terp-core/crates/headstash/test-press/src/harness/cashapp_zec_corridor.rs` — `run_cashapp_zec_corridor_w0_w7` uses L0 `authorize_bridge_mint` + `private_dex_seams::apply_swap`; comment: *“Oracle is bound_only — never mints.”* Test: `cashapp_zec_w0_w7_happy_simulated`. Run: `cd crates/headstash && cargo test -p zk-test-press --lib cashapp_zec_w0_w7_happy_simulated --features 'interface,l0-seams'` |
| **compose L0 re-export** | **Green** | `/Users/returniflost/abstract/terp-core/crates/headstash/test-press/src/harness/compose_l0.rs` → `assert_product_path_burn_to_swap_sketch` |
| **mint-after-observe host film** | **Green film (not chain swap)** | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/e2e/corridor-lab-mint-after-observe.sh` — runs pure `cashapp_zec_corridor` + harness W0–W7; PUTs automation phases with `oracle_bound_only: true`; optional `GET /oracle/bounds?market_id=BTC-ZEC` (may 503). **Does not** execute CW swap or spend a Daemon-minted note. Recipe: `just demo-corridor-mint-after-observe` (spectrum / headstash justfile) |
| **`demo-corridor-ict` swap leg** | **Pure film after chain mint** | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/e2e/corridor-ict-funded.sh` — chain `BridgeMintNote` via `corridor_ict_funded`, then calls mint-after-observe with `SKIP_WAIT=1`. Chain mint and pure swap suite are **sequenced**, not **identity-coupled**. |
| **hash-market automation API** | **Coord-only film status** | `/Users/returniflost/abstract/terp-core/crates/terp-rs/tools/hash-market/src/corridor_deposits.rs` — phases `deposit_observed \| bridging \| swapping \| complete \| failed`; `oracle_bound_only` flag; **not** mint or swap authority |
| **hash-market oracle bounds** | **Optional mid source** | `GET /oracle/bounds`; suite role `bound_only` (`tools/hash-market` oracle matrix). Used as probe in film; pure tests supply mids in-process |
| **cw-headstash swap execute** | **Absent** | `/Users/returniflost/abstract/terp-core/crates/headstash/contracts/cw-headstash/src/msg.rs` — `BridgeMintNote` / `IsBridgeMinted`; **no** `Swap` / `ApplySwap` / private DEX msg. Contracts tree: no DEX settle path (grep clean for swap) |
| **Halo2 private swap circuit** | **Absent (by design freeze)** | ROUND2-SWAP: no `circuit/src/swap`; E2E-HARNESS-PLAN: *Private swap on Terp = pure seams only* |
| **Just spine** | **L0 green** | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/justfile` — `demo-e2e-l0` / spine includes `private_dex_seams`, `compose_seams` product path, `cashapp_zec_corridor` |

### What pure film actually does on the swap leg

| Layer | Input “BTC note” | Output “ZEC” | Oracle |
|-------|------------------|--------------|--------|
| **W5 harness** | Synthetic `NoteIn { AssetB, value, nullifier: u64 }` from mint **value only** — not SEAM openings / not chain cm | `AssetId::Hub` as stand-in “sim-ZEC”; **no** `SeamNoteOutV0` out note | In-process mid; curve-implied mid patched so DEX band passes; product mid for intent floor |
| **compose product path** | Real pure SEAM from bridge mint (+rcm), DEX-consumable | `asset_out = asset_id_hub()` (hub), not labeled ZEC id; out openings are stubs | **`oracle_mid: None`, `oracle_params: None`** |
| **mint-after-observe** | N/A — re-runs pure suite; automation phase string only | Receipt JSON claims “oracle-bound private swap” | Flag `oracle_bound_only: true`; optional HTTP probe |

### Bound_only hard rules (implemented, pure)

```text
private_dex_seams::oracle_mint_note        → always ErrOracleDisabledMint
private_dex_seams::oracle_update_reserves  → always ErrOracleDisabledMint
cashapp_zec_corridor::oracle_mint_forbidden → always Err OracleDisabledMint
AssetRegistryView (compose)                → resolve only; never mints balances
```

---

## 3. Gaps (goal vs code)

| ID | Gap | Severity | Notes |
|----|-----|----------|-------|
| **S-G1** | **No on-chain private DEX settle** that spends a BridgeMint note and updates pool state | **P0** | Full BTC→Terp→ZEC needs Terp-side settle (CW or module). Today: pure host only. `E2E-HARNESS-PLAN`: *Private swap on Terp = pure seams only*; CW DEX listed as stub / future. |
| **S-G2** | **Chain mint ↔ swap film identity uncoupled** | **P0** | `demo-corridor-ict` mints on Daemon, then mint-after-observe runs **independent** pure W0–W7. No handoff of chain `cm_public` / pool leaf / openings into swap. Film can “complete” even if mint identity ≠ swap inputs. |
| **S-G3** | **W5 does not spend SeamNoteOutV0 / BridgeMint openings** | **P0** | `run_oracle_bound_swap` builds synthetic `NoteIn` + u64 nullifier; does not call `build_swap_action_from_seam_notes` / `to_dex_spend_inputs` on the mint note. Violates D3 spirit of *private mint → private swap* as one note lifecycle. |
| **S-G4** | **No durable ZEC-side note artifact** from swap | **P1** | Goal: ZEC-side `SeamNoteOutV0` for Zakura/egress. Product path emits hub out stub; W5 returns scalar `amount_out` + reserve pair only. Receipt has amounts, not a ZEC note blob. |
| **S-G5** | **Product path lacks oracle bounds** | **P1** | `run_product_path_burn_to_swap_sketch` sets `oracle_mid/params: None` — proves burn→swap structure + ν separation, **not** D3 oracle-bound demo property. Oracle path covered separately in `private_dex_seams` + cashapp I2/I5/I6. |
| **S-G6** | **Asset_out is hub / enum Hub, not corridor ZEC registry id** | **P1** | Corridor intent has `asset_out_id` (sim-ZEC); DEX pure pool uses `AssetId::Hub` or `asset_id_hub()`. Registry-resolved 32-byte ZEC id not end-to-end in product path. |
| **S-G7** | **Dual `intent_allows_swap` / intent types** | **P2** | Fixture `cashapp_zec_corridor` vs harness-local reimplementation in `test-press/.../cashapp_zec_corridor.rs` (comment: *TODO: swap to cashapp_zec_corridor import*). Drift risk for I1–I6. |
| **S-G8** | **HTTP oracle mid not required for film green** | **P2** | mint-after-observe: bounds probe optional; pure suite injects mid. Production path should fail closed when Connect bounds missing/stale (I6) **and** wire that mid into swap — still pure or chain. |
| **S-G9** | **No Halo2 private swap prove** | **P2** (later) | Explicit non-goal until pure + policy + mock settle land (DEMO-CASHAPP §2; ROUND2-SWAP). Not blocking honest **lab** film; blocks production ZK settle. |
| **S-G10** | **CW msg surface has no swap entrypoint** | **P0** (same epic as S-G1) | `ExecuteMsg` mint-only for bridge notes; cannot submit SwapAction on-chain today. |

### Severity map (one glance)

```text
P0  S-G1/S-G10  on-chain settle missing
P0  S-G2        ict mint note not input to swap film
P0  S-G3        W5 synthetic note, not SEAM spend
P1  S-G4–S-G6   ZEC note + oracle on product path + asset_id
P2  S-G7–S-G9   dual impl, HTTP mid optional, Halo2 later
```

---

## 4. Dependencies on other tracks

| Track | Dependency |
|-------|------------|
| **MINT-HEADSTASH** | Provides BridgeMint `SeamNoteOutV0` with **rcm / DEX-consumable** openings; mock_verify ok for lab (D7). Swap cannot spend what mint does not emit with correct widths / rcm_flag. |
| **HARNESS-ICT** | Must pass mint result (cm, value, asset_id, owner_binding, openings or note bytes) into swap film / future CW apply — replace “mint then pure suite” sequencing. |
| **OBSERVE** | `intent_id` / burn identity must match once-per-burn mint **and** swap policy context; deposit fields → claim coupling is peer gap. |
| **UI** | Polls automation `swapping` / receipt; must label pure film vs chain settle; should not present host phase as ZEC settlement. |
| **ZAKURA-ZEC** | Consumes **ZEC-side note / dest binding** after swap; today preauth dest is checked on intent, but no ZEC note from swap for local open/egress. |
| **DOCS** | Keep USER-GUIDE / book: pure film ≠ on-chain private DEX ≠ mainnet ZEC (align with this gap file). |
| **Oracle / hash-market** | Mid source for `bound_only`; never mint authority (already coded hard-reject). |

---

## 5. Recommended P0 slice for this track

**Smallest shippable slice toward full path (still honest labels):**

> **Identity-coupled pure spend:** take the **same** BridgeMint `SeamNoteOutV0` (or openings) produced by L0 authorize / (later) Daemon mint → `build_swap_action_from_seam_notes` / `apply_swap_action` with **BTC→ZEC asset ids** + **oracle_mid from intent market** + **`intent_allows_swap` gate** → emit **ZEC-side note stub** (SeamNoteOutV0-shaped) + receipt fields (cm_out, pool ν, Δ).

Concrete acceptance for this P0 slice:

1. One harness function (prefer extending `run_cashapp_zec_corridor_w0_w7` W5, not a third film) that **does not** invent a u64 `NoteIn` when SEAM openings exist.
2. `product_path_burn_to_swap_sketch` (or sibling test) with **oracle params required** and ZEC `asset_out` registry id (not only hub).
3. mint-after-observe / ict film **threads** mint note identity into that path (even if still pure host apply) so automation receipt cites the mint cm that was “spent.”
4. Explicit non-claim retained: **not** CW pool state; **not** live ZEC egress.

**Next after P0 (P1 epic, other track collab):** CosmWasm **mock_verify** swap apply stub (reserves + nullifier set) consuming the same `SwapActionV0` public packet — still no Halo2; closes S-G1 partially.

**Do not** start Halo2 swap circuit as this track’s P0.

---

## 6. Explicit non-claims

- **Do not claim** on-chain private DEX or Daemon pool settle exists.
- **Do not claim** that `demo-corridor-ict` swaps the **minted** note — it mints on-chain, then runs a **separate pure** W0–W7 film.
- **Do not claim** ZEC mainnet / Zakura broadcast from swap output.
- **Do not claim** oracle mints or credits balances — pure APIs **hard-reject** (`bound_only` / D7).
- **Do not claim** Halo2 private swap prove.
- **Do claim (accurate):** L0 private DEX seams + cashapp intent policy + compose burn→swap sketch are **green pure film**; host automation phases advertise oracle-bound swap for UI poll only.

---

## Appendix — commands (re-verify green pure surfaces)

```bash
cd /Users/returniflost/abstract/terp-core/docs/plans/spectrum/fixtures/private_dex_seams && cargo test
cd /Users/returniflost/abstract/terp-core/docs/plans/spectrum/fixtures/cashapp_zec_corridor && cargo test
cd /Users/returniflost/abstract/terp-core/docs/plans/spectrum/fixtures/compose_seams && cargo test
cd /Users/returniflost/abstract/terp-core/crates/headstash && \
  cargo test -p zk-test-press --lib cashapp_zec_w0_w7_happy_simulated --features 'interface,l0-seams'
# Host film (pure + automation PUT): just demo-corridor-mint-after-observe
# ICT: chain mint + pure film — not chain swap: just demo-corridor-ict
```

## Appendix — pure film vs goal (diagram)

```text
GOAL:
  BTC fund → observe → BridgeMintNote (chain) → spend mint SEAM → oracle-bound swap → ZEC SEAM → egress

TODAY (certified ict):
  BTC regtest → observe → BridgeMintNote (Daemon) ──┐
                                                     ├→ (no shared note identity)
  pure W0–W7 apply_swap(synthetic NoteIn) ──────────┘
  + PUT automation phase "swapping" (coord only)

TODAY (compose L0):
  pure authorize_bridge_mint → SEAM(+rcm) → SwapAction → apply_swap_action (hub out; no oracle)
```
