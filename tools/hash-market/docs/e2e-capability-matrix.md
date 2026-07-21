# Hashmerchant e2e capability matrix

First-principles modular harness replacing a single “headstash e2e blob.”

## Layers

| Layer | Location | Needs Docker? | What it proves |
|-------|----------|---------------|----------------|
| **L0 in-process** | `hash-market` `tests::suite` | No | trees, oracle_bounds, kind separation, VE wire |
| **L1 config contract** | `ict-rs` `hashmerchant_modular_matrix` | No | TOML generators for default vs elevated oracle |
| **L2 sidecar + chain** | `ict-rs` example `hashmerchant` | Yes | Anvil → VE → HashRoot on-chain (Model A) |
| **L3 bootstrap install** | `ict-rs` example `hashmerchant_bootstrap_ubuntu` | Yes | Fresh Ubuntu → hm-support → runtime health/VE schema |
| **L3+ client ingress** | `ict-rs` example `hashmerchant_client_ingress_e2e` (milestone) | Yes | Client → server island + DB Merkle VE + dual-path claims + optional Mode C price math |
| **L3 oline shape** | `o-line` parallel-stack e2e | Yes | deploy/upload/health shape (not full VE) |

## Run

```bash
# L0 — always run in CI
cd crates/terp-rs/tools/hash-market
cargo test -p hash-market --lib tests::suite

# L1 — ICT_MOCK safe
cd crates/ict-rs
just test-file hashmerchant_modular_matrix

# L2 — full pipeline (operator machine)
cargo run -p ict-rs --example hashmerchant --features full

# L3 — install packaging (B1–B5 + SANITY/EXT/B7/B8)
cargo run --example hashmerchant_bootstrap_ubuntu --features docker
# ICT_L3_REPORT=./hashmerchant_l3_report.json  # optional JSON report
# ICT_HM_RUNTIME=auto   # prefer local hash-market:local|mint when present
# ICT_HM_RUNTIME=full   # force real images

# Client ingress milestone (S1–S4 default; +S5 Mode C math)
cargo run --example hashmerchant_client_ingress_e2e --features docker
# ICT_INGRESS_REPORT=./hashmerchant_ingress_report.json
# ICT_INGRESS_PHASES=S1,S2,S3,S4,S5
# just client-ingress-milestone
```

### Session freeze verify

Five-command matrix (L0 → L3 mock → ingress S1–S5 → loyalty Docker optional → zk-jwt offline):  
see [`HANDOFFS.md` § Verify matrix (session freeze)](./HANDOFFS.md#verify-matrix-session-freeze).

| Layer | Soft / hard | Report `schema_version` |
|-------|-------------|-------------------------|
| L0 `tests::suite` | Hard | n/a (cargo) |
| L3 bootstrap mock | Hard | `1` in `hashmerchant_l3_report.json` |
| L3+ ingress milestone S1–S4(+S5) | Hard product story | `1` in `hashmerchant_ingress_report.json` |
| Loyalty dual-path on-chain | Soft / nightly gold | n/a |
| zk-jwt offline | Hard offline | n/a |

**just:** from `crates/ict-rs` — `just l3-bootstrap-mock` · `just client-ingress-pr1` · `just client-ingress-milestone` · `just verify-hm-freeze` (`heavy=1` includes loyalty Docker).

Handoff: `docs/bootstrap/hashmerchant-ict-bootstrap-e2e-handoff.md`  
**B4/B5 full hash-market + product e2e ladder:** `docs/bootstrap/hashmerchant-b4-b5-full-runtime-and-product-e2e.md`  
**Client→isolated server ingress (conductor over L3 + loyalty + zk-jwt + NFT):** `docs/bootstrap/hashmerchant-client-ingress-sequence.md`  
Multi-source design: `x/hashmerchant/spec/10_multi_source_oracle.md` (Model B after module P0).

### L3 acceptance IDs

| ID | What |
|----|------|
| B1–B3 | `hm-support` prepare / install / whitelist self-test on Ubuntu |
| B4 | P-mint runtime health + data dir persist across restart |
| SANITY | Dense checks: config path, data dir, pid, logs, restart counters |
| SANITY-HC / VE / RST / LOG / CONC | health↔files; VE curl inside Ubuntu; pidfile restart; log growth; concurrent curls |
| B5 | P-sidecar `/vote-extension` schema |
| EXT-WL / EXT-WL-BAD / EXT-WL-PROOF | whitelist build; bad members fail-closed; proof for fixture addr |
| EXT-USB / EXT-USB-STRICT | package-usb kit; scripts + SHASUMS layout |
| EXT-SMOKE / EXT-SMOKE-BAD | smoke URL OK; wrong URL fail-closed |
| EXT-SWITCH / EXT-AUTH / EXT-CLIENT | profile switch w/o wipe; authenticate checklist; 2nd Ubuntu client attach |
| EXT-DEMO / EXT-FEE-DRY | soft: demo docs; feegrant dry-run (no chain) |
| B7 | Idempotent re-bootstrap (no wipe without flag) |
| B8 | Bad config fail-closed (`profiles/P-bad.toml`) |
| B6 | Optional Terp wire (L2 `hashmerchant` example) |

### L3 runtime modes

| Mode | Env | B4/B5 backend | Cadence |
|------|-----|---------------|---------|
| mock (default) | unset / `ICT_HM_RUNTIME=mock` | `mock_runtime.py` | every PR |
| auto | `ICT_HM_RUNTIME=auto` | prefer `hash-market:local` or `:mint` if present, else mock | operator |
| full (target) | `ICT_HM_RUNTIME=full` | pinned `hash-market:mint` / `:sidecar` images | nightly |

See full-runtime doc for `/ve` path, feeder, and dual-image build.

## Capability flags

```text
trees                 — merkle whitelist (headstash claim surface)
oracle_bounds         — multi-source mid, role=bound_only (Tacit cUSD-like)
vote_extension        — ABCI++ root path (interop fabric)
content_distribution  — BUD+IPFS dual-index (opt-in; not default CI)
calendar_nostr        — Phase B: off-chain cid bind + NIP-52 egress (opt-in)
nostr                 — opt-in domain bus (not default matrix)
marketplace_inventory — Postgres inventory Merkle → Purchase buffer → Settle (loyalty parallel)
loyalty_rewards       — Postgres rewards Merkle → ClaimRewards dual-path (gold standard)
```

### marketplace_inventory (loyalty parallel)

ICT harness for **marketplace-mirror** local-SKU purchase buffer + optional inventory Merkle attest.
Delta design: `crates/ict-rs/docs/MARKETPLACE_VS_LOYALTY_DELTA.md`  
Harness notes: `crates/ict-rs/docs/marketplace_harness.md`

**L0 commands:** leaf codec · buffer math (default 7) · oversell · UpdateStock · **cancel auth (buyer|admin OK, seller REJECT)**.  
**Notes:** Purchase uses **local stock** (hashmerchant root decorative until proof-gated restock); Nostr **ingress stub** / egress sketch only; Docker seller-cancel race offline-only (single key).

```bash
# L0 — offline leaf codec + buffer + oversell + cancel policy (no Docker)
cd crates/ict-rs
cargo run --example marketplace_inventory_offline

# L2 — Docker Postgres Merkle + P2–7 Agent A msgs (needs wasm + terp image)
MARKETPLACE_GAS_REPORT=./marketplace_gas_report.md \
  cargo run --example marketplace_inventory --features "docker hashmerchant"

# Compare with loyalty gold standard
cargo run --example loyalty_rewards --features "docker hashmerchant"
cargo run --example loyalty_rewards_zkjwt
```

### content_distribution (L0)

```bash
cd crates/terp-rs
cargo test -p hash-market --lib content:: --features server
# cid parse/encode, dual_index_upload_meta_get_smoke, calendar bind, auth jwt
```

### calendar_nostr (Phase B)

```bash
# L0 — bind + egress shape (no Docker)
cargo test -p hash-market --lib content::calendar --features server
cargo test -p hash-market --lib client::nostr::calendar_egress --features nostr
cargo test -p terp-scripts --test nostr_orch_suite

# L2 — Docker relay (+ optional local chain)
cargo test -p terp-scripts --test nostr_orch_suite -- --ignored --nocapture
```

Docs: `docs/content-distribution.md`, `docs/PREP-BLOSSOM-S3-CALENDAR-NOSTR.md`.

## Design rules

1. **Modular** — run `oracle_demo()` without trees; run `trees_only()` without prices.
2. **Default safe** — elevated oracle TOML is opt-in (`hash_market_config_toml_with_oracle_bounds`).
3. **No mint** — oracle scenarios assert `role == "bound_only"`.
4. **Same language** — ICT-RS `HashMarketE2eCapability` mirrors hash-market suite names.

## Evolution from headstash e2e

| Old | New |
|-----|-----|
| One Docker example focused on CW + ZK keys | Capability matrix + optional Docker VE |
| Headstash trees implicit | Explicit `trees` scenario |
| No pricing | `oracle_bounds` + Connect-shaped multi-source |
| Monolithic pass/fail | Per-capability report (`MatrixReport`) |
