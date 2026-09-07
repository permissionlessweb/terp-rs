# STATUS-GAP-ZAKURA

| Field | Value |
|-------|--------|
| **Track** | ZAKURA-ZEC |
| **Date** | 2026-07-22 |
| **Mode** | Gap analysis only (read-first; no implementation) |
| **Surfaces** | `zakuraDest.ts`, `e2e/zakura/**`, golden binding, `zakura_local.rs` — paste/binding vs live ZEC receive/egress |
| **North star** | Full multi-net: BTC fund → Terp mint/swap → **preauth ZEC dest sealed and exercised as local/regtest open/receive (and later egress)** |

---

## 1. Goal for full BTC → Terp → ZEC (this track)

For a true multi-net corridor, ZAKURA-ZEC must supply more than a UI string and a hash film:

1. **Preauth dest seal** — operator/user provides a real (or lab-regtest) ZEC receive address; system seals  
   `owner_binding = SHA-256("terp-dest-binding-v0|" ‖ utf8_trim(dest_display))` and carries that binding through intent → mint → swap → receipt (D6).
2. **Local node fidelity** — optional but required for funded-profile honesty: regtest Zakura (or zcashd-compat) is up; dest can be `validateaddress`-checked on the same network the corridor claims.
3. **Receive / open** — after Terp-side settle (or interim stand-in), prove the sealed dest is the **only** allowed open target: binding re-check + (regtest) ability to **receive** at that address on a live ZEC stack.
4. **Egress (Phase 2 / full multi-net)** — a product or lab path that **broadcasts / settles ZEC** to the preauth dest (or LC egress), not pure W7 JSON claiming `status: complete`.
5. **Honest labels** — paste-first offline golden ≠ live ZEC transfer; mainnet money out of scope.

**What this track owns:** dest domain/golden parity, local node/RPC helpers, UI dest helpers, sidecar compose/runbooks, binding attach docs for HARNESS.  
**What this track does not own alone:** observe→mint identity (OBSERVE/MINT/HARNESS), on-chain private swap (SWAP), Wizard SSE/mint sequence (UI), book marketing copy (DOCS).

---

## 2. Code reality today (cite paths / commands)

### Binding / paste film (green offline)

| Surface | Status | Evidence |
|---------|--------|----------|
| Golden SSOT | **Green** | `/Users/returniflost/abstract/terp-core/docs/plans/spectrum/e2e/zakura/golden-dest-binding.json` — domain `terp-dest-binding-v0`; primary dest `tmJymvcUCn1ctbghvTJpXBwHiMEB8P6wxNV` → binding `8b5cac11e39905d56126a0c538b84ff8daa379d8009d4e8b121112479607f09b` |
| Shell parity | **Green** | `docs/plans/spectrum/e2e/zakura/zakura-local.sh` `golden` / `dest` — verified this run: both samples OK; offline dest prints primary binding |
| Harness digest | **Green** | `crates/headstash/test-press/src/harness/zakura_local.rs` — `owner_binding_from_dest_display`, `primary_golden_dest`, golden load/assert tests |
| UI helpers | **Green (lab)** | `websites/dao-dao-ui/packages/stateful/modules/modules/PrivateCorridor/zakuraDest.ts` — `DEST_BINDING_DOMAIN`, `GOLDEN_OWNER_BINDING_HEX`, `destOwnerBindingHex`, `softValidateZecDest`, `localZakuraDemoDestOffline`, `fetchZakuraDemoDest` |
| UI mock seal | **Green (lab)** | `PrivateCorridor/mock.ts` — `bindingFromDestDisplay` → `destOwnerBindingHex` / same domain |
| Wizard dest step | **Wired to helpers** | `PrivateCorridorWizard.tsx` imports paste / “local Zakura demo dest” → `bindingFromDestDisplay` |
| Just floor | **Green offline** | `crates/headstash/justfile` → `demo-zakura-local-dest` (golden + dest + harness offline/golden tests) |
| Just live attempt | **Scripted** | `demo-zakura-local` → up/rpc-smoke when bin+Docker available; live tests **skip cleanly** if RPC down |

**This run (evidence):**  
`zakura-local.sh golden` → OK both samples; `dest` → offline primary binding match; `status` → RPC **not** ready at `http://127.0.0.1:18232`.

### Local node / RPC (implemented, not required for lab green)

| Surface | Status | Evidence |
|---------|--------|----------|
| Corridor regtest compose | **Present** | `docs/plans/spectrum/e2e/zakura/docker-compose.corridor-zakura.yml` — host network, bind-mount `ZAKURAD_BIN`, config `node-corridor.toml` |
| Node config | **Present** | `node-corridor.toml` — RPC `:18232`, P2P `:18233`, metrics `:19901`, `miner_address` = golden primary dest |
| RPC smoke | **Scripted** | `zakura-local.sh` `rpc-smoke` → `getblockchaininfo` + `validateaddress` |
| Harness live tests | **Optional skip** | `zakura_rpc_health_and_validate_when_available`, `cashapp_w0_w7_with_zakura_dest_when_available` |
| Wallet address mint | **Not on bare Zakura** | Core is consensus node; `fetchZakuraDemoDest` tries `z_getnewaddress` / `getnewaddress` then falls back to miner/demo dest (documented residual) |

### What pure / funded paths do with dest today

| Path | Uses real golden / Zakura? | Live ZEC receive/egress? |
|------|----------------------------|---------------------------|
| `zakura_local` + pure W0–W7 (`cashapp_w0_w7_with_golden_primary_dest`) | **Yes** — seals scenario `dest_owner_binding` from golden | **No** — pure film + receipt JSON |
| `cashapp_w0_w7_with_zakura_dest_when_available` | Live dest string → same domain digest when RPC up | **No** — still pure `run_cashapp_zec_corridor_w0_w7` |
| `just demo-corridor-ict` / `corridor-ict-funded.sh` | **No** — watch payload `dest_owner_binding: "b" * 64` placeholder | **No** — no Zakura attach; ends pure film + automation |
| Pure W7 “open” | Binding equality / min_out in pure code (`intent_allows_swap`, mint owner check) | Receipt `status: "complete"` is **not** a ZEC chain proof |
| Mainnet ZEC send | Explicit non-claim (D6 Phase 2 / honesty tags) | **Absent** |

### W0–W7 dest semantics (honest)

`crates/headstash/test-press/src/harness/cashapp_zec_corridor.rs`:

- W0 seals `dest_owner_binding` from scenario (can be golden).
- W3 mint `owner_binding` defaults to intent dest.
- W6 re-checks swap owner vs intent; W7 writes `dest_owner_binding_hex` on receipt.
- `dest_display_hint` in default happy path is synthetic (`"u1zec…preauth"`), **not** the corridor miner UA/t-addr.
- No `sendtoaddress` / `z_sendmany` / submit-tx / balance-open against Zakura.

### D6 freeze vs full multi-net

`DESIGN-DECISIONS-CORRIDOR-ACCEPTED-2026-07-20.md` **D6 ACCEPTED (amended):**

- Demo binds preauth ZEC dest → `owner_binding` (**no mainnet ZEC send required for lab film**).
- Local Zakura for destination / wallet UX in environment.
- **Phase 2:** full ZEC LC egress / mainnet send.

**Interpretation for this epic:** D6 lab floor is largely **code-complete**. Full BTC→Terp→**ZEC chain** still needs **open/receive + egress** beyond binding.

### Layout (SSOT map)

| Path | Role |
|------|------|
| `docs/plans/spectrum/e2e/zakura/README.md` | runbook + HARNESS attach checklist |
| `docs/plans/spectrum/e2e/ZAKURA-LOCAL.md` | operator status (offline vs live) |
| `docs/plans/spectrum/e2e/zakura/golden-dest-binding.json` | shared golden |
| `docs/plans/spectrum/e2e/zakura/zakura-local.sh` | status/up/down/dest/rpc-smoke/golden |
| `docs/plans/spectrum/e2e/zakura/docker-compose.corridor-zakura.yml` | single regtest node |
| `docs/plans/spectrum/e2e/zakura/node-corridor.toml` | miner dest + RPC ports |
| `crates/headstash/test-press/src/harness/zakura_local.rs` | digest + golden + live helpers |
| `PrivateCorridor/zakuraDest.ts` | UI dest + soft validate + optional RPC |
| Prior sprint closeout | `agents/final-sprint-2026-07-22/STATUS-ZAKURA-DEST.md` (PASS offline floor) |

```text
IMPLEMENTED TODAY (ZAKURA track):
  paste dest_display  ──► owner_binding (domain v0)     ✓ UI + shell + Rust
  golden vector parity                                  ✓ offline CI floor
  regtest node up + validateaddress                     ✓ when bin/Docker
  pure W0–W7 with golden dest_owner_binding             ✓ film only
  sidecar ports / attach docs for HARNESS               ✓ docs only

NOT IMPLEMENTED (full multi-net ZEC):
  funded watch / ICT stages use golden or live dest     ✗ placeholder "b"*64
  continuous identity dest through mint+swap+receipt    ✗ uncoupled harness
  live ZEC receive confirmation (balance/txid open)     ✗ missing
  live ZEC egress / broadcast from corridor settle      ✗ missing
  bare Zakura wallet UA generation                      ✗ residual (zcashd-compat)
  mainnet ZEC                                           ✗ non-claim
```

---

## 3. Gaps (goal vs code)

| ID | Gap | Severity | Notes |
|----|-----|----------|-------|
| **Z-G1** | **No live ZEC egress / broadcast path** | **P0** | Full multi-net requires settle → ZEC transfer (or LC egress) to preauth dest. No in-tree `z_send*`, submit-tx, or corridor egress stage. Pure W7 receipt ≠ chain ZEC. Aligns with HARNESS **H-G3** and PROMPT-COMMON “Live Zcash transfer / Zakura broadcast” not green. |
| **Z-G2** | **No live receive/open proof on ZEC stack** | **P0** | “Open only to preauth dest” is enforced as **hash equality** in pure film / intent checks, not as “funds visible at dest on regtest.” Missing: fund/mine/receive assertion against `dest_display` after (or simulating) egress. |
| **Z-G3** | **Funded ICT path ignores golden / Zakura dest** | **P0** | `corridor-ict-funded.sh` sets `dest_owner_binding` to `"b" * 64`. Does not call `zakura-local.sh dest`, `primary_golden_dest()`, or assert receipt binding. `just demo-zakura-local*` is **parallel** to `demo-corridor-ict`, not joined. |
| **Z-G4** | **W0–W7 “open” is binding film, not node open** | **P0** (product honesty) | Even when golden binding is used, workflow ends at `CorridorReceiptV0` JSON. No RPC to Zakura after swap. `dest_display_hint` default is synthetic, not miner dest. |
| **Z-G5** | **Zakura core ≠ wallet — generate UA path residual** | **P1** | Product goal “generate diversified dest” needs zcashd-compat / external wallet / lightwalletd client. Bare `zakurad` only supports health + `validateaddress` + config `miner_address` fallback. UI already paste-first (correct); residual if operators expect in-page generate from full node alone. |
| **Z-G6** | **Live regtest node tax (macOS / CI)** | **P1** | Compose requires Linux-compatible `ZAKURAD_BIN` + Docker host networking. Offline golden green without node; funded fidelity for Z-G1/Z-G2 needs live stack or explicit SKIP label. |
| **Z-G7** | **lightwalletd / zcashd-compat not wired to corridor** | **P1** | `crates/zakura/docker/docker-compose.lwd.yml` exists but is not joined to `e2e/zakura` corridor compose. Blocks easier UA tooling without inventing wallet RPC on bare node. |
| **Z-G8** | **Browser CORS / optional RPC** | **P2** | `NEXT_PUBLIC_ZAKURA_RPC` optional; paste works offline. Live health-from-browser needs CORS or proxy — residual, not blocking preauth seal. |
| **Z-G9** | **Non-SubtleCrypto digest fallback in `destOwnerBindingHex`** | **P2** | Non-browser fallback is non-cryptographic (commented). Production seals must use browser SubtleCrypto or harness SHA-256. Risk only if someone seals outside crypto env. |
| **Z-G10** | **Display non-authoritative — no hard chain-format check** | **P2** | Soft prefix validate only (not full bech32m). Lab demo handles (`u1sim_…`) bind as digests. Fine for D6 film; insufficient for “must be spendable regtest UA/t-addr” without `validateaddress` gate in funded path. |

---

## 4. Dependencies on other tracks

| Track | Dependency |
|-------|------------|
| **HARNESS-ICT** | Must attach Zakura sidecar (or golden offline) into funded script: seal real `dest_owner_binding` on watch; optional RPC health wait; assert receipt/automation binding; own stage order for future egress step. Today **H-G3 / H-G6** block continuous dest identity. |
| **SWAP-DEX** | On-chain / product swap must consume note whose `owner_binding` is the sealed dest; egress trigger (if any) after swap out-amount — pure film does not hand off to ZEC. |
| **MINT-HEADSTASH** | BridgeMintNote `owner_binding` must equal intent dest (already intended in pure authorize); chain mint fixture must not invent a different owner. |
| **OBSERVE** | Watch fields should carry real `dest_owner_binding` (not placeholders) for reverify / automation fidelity. |
| **UI** | Dest step already uses `zakuraDest` helpers; full path needs post-swap UI honesty (no “ZEC sent” until egress exists). Coord only for copy/labels. |
| **DOCS** | Must not equate `demo-zakura-local-dest` or W7 complete with live ZEC settlement. |
| **META** | Build order: seal golden into funded path **before** investing in full LC egress. |

**Outbound from ZAKURA:** provides golden constants, `owner_binding_from_dest_display` / `primary_golden_dest()`, shell dest printer, ports `18232/18233/19901`, compose + runbooks — ready for HARNESS consume without reinventing domain string.

---

## 5. Recommended P0 slice for this track

**Smallest shippable slice toward multi-net ZEC (no mainnet, no deep LC):**

### P0-A — Join dest seal into funded ICT (coord HARNESS; ZAKURA owns golden truth)

1. In funded watch open, set `dest_owner_binding` from **primary golden**  
   (`8b5cac11…` / `primary_golden_dest()` / `zakura-local.sh dest`) instead of `"b" * 64`.
2. Export `ZAKURA_DEST_DISPLAY` + binding into environment for later stages / automation labels.
3. Assert pure film (or automation receipt) `dest_owner_binding_hex` **equals** golden primary when suite uses corridor dest.
4. Optional soft: if `ZAKURA_RPC` up, `validateaddress` on primary dest; if down, label evidence `rpc_ready=false` (offline seal still valid for preauth).

**Exit criteria:** `demo-corridor-ict` evidence log shows real `terp-dest-binding-v0` primary binding, not placeholder hex; golden remains single SSOT.

### P0-B — “Open” on regtest without full wallet product (follow-on, still ZAKURA-owned)

1. When live node up: keep miner dest as receive target; document that mining reward / lab fund-to-miner stands in for **receive visibility** only.
2. Add harness helper: after (lab) fund step, RPC-visible balance or `validateaddress` + optional gettxout / mining hook — **not** claiming product egress.
3. Explicit SKIP when no Linux bin/Docker (CI floor stays golden).

### Explicitly defer (not this P0)

- Mainnet send, Cash App ZEC, full zcashd-compat wallet productization.
- Wiring lightwalletd as address factory (optional P1).
- Real private-swap → automatic ZEC broadcast (needs SWAP + product operator keys).

---

## 6. Explicit non-claims

- **`just demo-corridor-ict` does not** start Zakura, seal golden dest, or broadcast ZEC.
- **`just demo-zakura-local-dest` does not** prove multi-net settle; it proves **domain/golden/offline dest film**.
- **Pure W0–W7 with golden `dest_owner_binding`** proves binding continuity in simulated backend only — **not** live ZEC receive or egress.
- **Zakura RPC health / `validateaddress`** prove node + address validity on regtest — **not** payment settlement.
- **Paste / demo handle / miner fallback** are lab-valid preauth inputs; they are **not** mainnet diversified UA generation.
- **D6 ACCEPTED** closes local dest UX + preauth binding for lab film; it does **not** close Phase 2 full ZEC LC egress / mainnet send.
- **No mainnet money**, no Cash App mainnet ZEC payout claimed by this track.

---

## Appendix — Commands (operator)

```bash
# Offline floor (always)
cd crates/headstash && just demo-zakura-local-dest

# Golden + dest only
bash docs/plans/spectrum/e2e/zakura/zakura-local.sh golden
bash docs/plans/spectrum/e2e/zakura/zakura-local.sh dest

# Live regtest (Linux zakurad + Docker)
cd crates/zakura && cargo build -p zakura --bin zakurad
export ZAKURAD_BIN="$(pwd)/target/debug/zakurad"
bash docs/plans/spectrum/e2e/zakura/zakura-local.sh up
bash docs/plans/spectrum/e2e/zakura/zakura-local.sh rpc-smoke
cd crates/headstash && just demo-zakura-local

# Harness
cargo test -p zk-test-press --lib zakura_local --features 'interface,l0-seams' -- --nocapture
```

| Env | Default / role |
|-----|----------------|
| `ZAKURA_RPC` | `http://127.0.0.1:18232` |
| `ZAKURA_DEST_ADDR` | override dest (else miner) |
| `ZAKURAD_BIN` | host-built Linux `zakurad` for compose |
| `NEXT_PUBLIC_ZAKURA_RPC` | browser optional health |
| `NEXT_PUBLIC_ZAKURA_DEMO_DEST` | pin paste when no wallet RPC |

---

## Appendix — Cross-track matrix row (for META)

| Full multi-net stage | ZAKURA status |
|----------------------|---------------|
| Preauth dest + domain binding | **Green** (offline golden + UI + harness) |
| Local node health / validate | **Green when bin/Docker**; optional skip |
| Funded ICT uses sealed dest | **Gap Z-G3** |
| Continuous binding mint→swap→receipt (chain) | **Depends HARNESS/MINT/SWAP**; pure-only today |
| Live ZEC receive/open | **Gap Z-G2** |
| Live ZEC egress/broadcast | **Gap Z-G1** |
| Mainnet ZEC | **Non-claim** |
