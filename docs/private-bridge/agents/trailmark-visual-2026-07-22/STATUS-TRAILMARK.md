# STATUS — TRAILMARK-GRAPHS

**Date:** 2026-07-22  
**Agent:** TRAILMARK-GRAPHS  
**Tooling:** `trailmark` 0.2.2 + `uv run crates/tob-skills/plugins/trailmark/skills/diagramming-code/scripts/diagram.py`  
**Rule:** Call edges from Trailmark graph only (no invented edges). Labels sanitized for `mmdc`.

## Targets

| Target | Graph summary | Notes |
|--------|---------------|-------|
| `docs/plans/spectrum/fixtures/cashapp_zec_corridor` | 100 nodes / 28 fn / 139 call edges | Pure corridor spine; deps `std, sha2` |
| `crates/headstash/contracts/cw-headstash/src` | 553 nodes / 97 fn / 797 call edges | Bridge + distro + headstash |
| `crates/terp-rs/tools/hash-market/src` | 635+ nodes / 444 fn / ~3948 call edges | Full host; 4 bin entrypoints |
| `corridor_deposits.rs` (isolated `/tmp` parse of hash-market module) | 121 nodes / 14 fn / 153 call edges | Focused watches hub |

**Optional TS PrivateCorridor:** not run (no trailmark TS target pass this session).

## Deliverables

### Mermaid (`docs/plans/spectrum/book/src/diagrams/mmd/`)

**cashapp_zec_corridor**

| File | Type / focus | mmdc |
|------|----------------|------|
| `trailmark-cashapp-call-graph.mmd` | call-graph depth 3 (pruned ≤55 nodes) | OK |
| `trailmark-cashapp-call-graph-safe.mmd` | same sanitized twin | OK |
| `trailmark-cashapp-authorize-mint.mmd` | call-graph `--focus authorize_mint` | OK |
| `trailmark-cashapp-intent-allows-swap.mmd` | call-graph `--focus intent_allows_swap` | OK |
| `trailmark-cashapp-sim-deposit.mmd` | call-graph `--focus sim_deposit` | OK |
| `trailmark-cashapp-complexity.mmd` | complexity thr=3 | OK |
| `trailmark-cashapp-core.mmd` | hand product spine (kept; not Trailmark) | n/a |
| `trailmark-cashapp-modules.mmd` | module-deps → *No import edges found* | OK (placeholder) |
| `trailmark-cashapp-dataflow-authorize-mint.mmd` | data-flow → *No entrypoints found* | OK (placeholder) |

**cw-headstash (BridgeMintNote / bridge)**

| File | Type / focus | mmdc |
|------|----------------|------|
| `trailmark-bridge-mint-calls.mmd` | call-graph `--focus execute_bridge_mint_note` d=2 | OK |
| `trailmark-authorize-bridge-mint-pure.mmd` | call-graph `--focus authorize_bridge_mint_pure` d=2 | OK |
| `trailmark-cw-headstash-execute.mmd` | call-graph `--focus execute` d=1 | OK |
| `trailmark-cw-process-headstash.mmd` | call-graph `--focus process_headstash` d=2 | OK |
| `trailmark-cw-headstash-complexity.mmd` | complexity thr=8 | OK |
| `trailmark-cw-headstash-modules.mmd` | module-deps → *No import edges found* | OK (placeholder) |
| `trailmark-bridge-mint-dataflow.mmd` | data-flow → *No entrypoints found* | OK (placeholder) |

**hash-market / corridor_deposits**

| File | Type / focus | mmdc |
|------|----------------|------|
| `trailmark-corridor-open-watch.mmd` | call-graph `--focus open_watch` (isolated) | OK |
| `trailmark-corridor-report-observation.mmd` | call-graph `--focus report_observation` | OK |
| `trailmark-corridor-put-automation.mmd` | call-graph `--focus put_automation` | OK |
| `trailmark-corridor-containment.mmd` | containment (DepositNotifyHub members) | OK |
| `trailmark-corridor-complexity.mmd` | complexity thr=3 | OK |
| `trailmark-hash-market-report-observation.mmd` | focus in full hash-market tree | OK |
| `trailmark-hash-market-complexity.mmd` | complexity thr=15 (full crate) | OK |
| `trailmark-hash-market-modules.mmd` | module-deps → *No import edges found* | OK (placeholder) |

### Graph text (`docs/plans/spectrum/book/src/diagrams/graphs/`)

| File | Content |
|------|---------|
| `cashapp_zec_summary.txt` | trailmark `--summary` |
| `cashapp_zec_entrypoints.txt` | entrypoints CLI |
| `cashapp_zec_focus.txt` | callees/callers for authorize_mint, intent_allows_swap, sim_deposit, … |
| `cw_headstash_summary.txt` | summary |
| `cw_headstash_entrypoints.txt` | manual CosmWasm surface note |
| `cw_headstash_focus.txt` | BridgeMintNote / authorize_bridge_mint_pure neighborhoods |
| `hash_market_summary.txt` | summary |
| `hash_market_entrypoints.txt` | 4 bin mains |
| `hash_market_focus.txt` | open_watch / report_observation / main |
| `corridor_deposits_summary.txt` | isolated module summary |
| `corridor_deposits_focus.txt` | DepositNotifyHub methods |

### SVG (`docs/plans/spectrum/book/src/diagrams/svg/`)

Rendered via `@mermaid-js/mermaid-cli@11` for key Trailmark diagrams (call-graph-safe, authorize_mint, intent_allows_swap, complexity, bridge-mint-calls, authorize-bridge-mint-pure, execute, corridor open/report, hash-market complexity).

## Graph facts (Trailmark — do not invent)

### cashapp_zec_corridor

- **Hotspot:** `intent_allows_swap` **CC=16**
- **`authorize_mint` callees (functions):** `is_zero_hash`; proxies: `intent.validate`, `self.insert`
- **`intent_allows_swap` callees (functions):** `apply_slip_ceiling`, `apply_slip_floor`, `expected_out_from_mid` (+ `intent.validate` proxy)
- **`sim_deposit` callees (functions):** `derive_sim_burn_id` (+ validate / faucet proxies)
- **`new_preauth` →** `compute_domain_bind` (proxy), field emptiness checks, `is_zero_hash`
- **`bind_deposit_id` →** `validate`, `compute_domain_bind` (proxies), `is_zero_hash`

### cw-headstash / BridgeMintNote

- **Hotspots:** `authorize_bridge_mint_pure` **CC=23**, `BridgeMintError.as_str` CC=20, `process_headstash` CC=12, `query` CC=12, `execute` CC=11
- **`lib.execute` →** (graph) `process_headstash`, `register_eligibility_root`, `execute_mint`, `execute_burn`, `bridge::execute_update_reflection`, `execute_set_reflection_snapshot`, `execute_register_asset`, `execute_set_external_asset_registry`, `execute_set_bridge_cfg`, **`bridge::execute_bridge_mint_note`** (mostly inferred `-.->` via module path proxies)
- **`execute_bridge_mint_note` function callees:** `authorize_bridge_mint_pure`, `mock_verify_bridge_proof`, `require_hash32`, `bridge_claim_storage_key`, `terp_asset_id_from_tacit`, `asset_id_key` + storage load/save proxies
- **`authorize_bridge_mint_pure` callers:** `execute_bridge_mint_note`
- **`authorize_bridge_mint_pure` function callees:** `derive_claim_id_with_dest`, `derive_domain_binding`, `hash_source_chain_tag`, `is_zero_hash`, `require_hash32`, `terp_asset_id_from_tacit` + snapshot `tip_ok` / `lag_ok` / `conf_mature` proxies

### corridor_deposits (DepositNotifyHub)

- **Hotspots:** `report_observation` CC=10, `get_status` CC=5, `open_watch` CC=4, `publish` CC=4
- Methods: `open_watch`, `get_status`, `list_open_watches`, `already_observed`, `report_observation`, `subscribe`, `publish`, `put_automation`, `get_automation`
- **`open_watch` / `put_automation` → `validate_id`**

### hash-market (full)

- Entrypoints: `bin.client:main`, `bin.corridor_btc_reporter:main`, `bin.relay:main`, `bin.server:main` (all untrusted_external / high)
- Complexity thr=15 highlights include `bin.server:main` CC=23 and several nostr/oracle parsers (see mmd)

## Sanitize notes (mmdc)

Applied to all Trailmark flowchart `.mmd`:

1. Flatten multiline proxy labels  
2. Strip `, " ' () [] {} | ? !` from node labels (parens break mermaid even in quotes)  
3. Use `["label"]` form  
4. Prune graphs to ≤55 nodes (prefer non-proxy + edges touching functions)  
5. Replace `classDef` `rgba(...)` with hex fills (rgba parens break mmdc)

## Limitations / gaps

| Gap | Detail |
|-----|--------|
| **module-deps** | Rust targets yield `No import edges found` (Trailmark does not surface crate-local `mod`/use graph usefully here) |
| **data-flow** | `No entrypoints found` without `.trailmark/entrypoints.toml`; CosmWasm `#[entry_point]` not auto-detected |
| **method resolution** | Many real calls appear as `proxy.unresolved:self.*` / `intent.*` — diagram edges are honest to the parser, not ideal named edges |
| **depth** | Full cashapp call-graph and bridge focuses pruned for render; raw focus neighborhoods live in `graphs/*_focus.txt` |
| **TS UI** | PrivateCorridor / dao-dao-ui not diagrammed |

## Regenerate

```bash
DIAG=crates/tob-skills/plugins/trailmark/skills/diagramming-code/scripts/diagram.py
OUT=docs/plans/spectrum/book/src/diagrams

# cashapp
uv run "$DIAG" -t docs/plans/spectrum/fixtures/cashapp_zec_corridor -l rust \
  -T call-graph -f intent_allows_swap -d 2 > "$OUT/mmd/trailmark-cashapp-intent-allows-swap.mmd"

# bridge mint
uv run "$DIAG" -t crates/headstash/contracts/cw-headstash/src -l rust \
  -T call-graph -f execute_bridge_mint_note -d 2 > "$OUT/mmd/trailmark-bridge-mint-calls.mmd"

# corridor hub (isolate file for smaller graph)
mkdir -p /tmp/corridor_tm/src
cp crates/terp-rs/tools/hash-market/src/corridor_deposits.rs /tmp/corridor_tm/src/lib.rs
uv run "$DIAG" -t /tmp/corridor_tm -l rust -T call-graph -f report_observation -d 2 \
  > "$OUT/mmd/trailmark-corridor-report-observation.mmd"

# then re-run label sanitizer + mmdc (see STATUS sanitize notes)
npx --yes @mermaid-js/mermaid-cli@11 -i "$OUT/mmd/….mmd" -o "$OUT/svg/….svg" -b white
```

## Status

**DONE** — minimum targets covered with script-generated Mermaid + Trailmark summaries; mmdc-verified on primary diagrams; `STATUS-TRAILMARK.md` written.
