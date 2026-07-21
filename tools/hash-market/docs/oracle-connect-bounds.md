# Hashmerchant as oracle — multi-source bounds (Tacit-aligned)

**Status:** design + lib types (`oracle` module). Default VE path unchanged.  
**Program spine:** `plans/terp-private-shielded-dex-bridge/PLAN.md` · Tacit SSOT `crates/tacit`  
**Reference (do not reinvent):** [skip-mev/connect](https://github.com/skip-mev/connect) — providers, market map, aggregation sidecar, VE broadcast.

---

## 1. Why this exists

Tacit locks:

| Asset class | Oracle role |
|-------------|-------------|
| **cBTC-like conservation** | **No** oracle mint. Peg/backing from proven locks + kernels. |
| **cUSD-like bounds** | Oracles **price bounds only** — e.g. \(\textit{min_out} \ge g(\textit{mid}, \textit{slippage})\) |

hashmerchant / VEs **never invent balances**. They may supply multi-source mid / TWAP **bounds** that private DEX settle uses as quote guards, while pool math / LCs own solvency.

Trust is **feature-tiered** (Tacit Tier 0/1/2) — multi-source aggregation is Tier-declared honesty, not “all trustless.”

---

## 2. Two ingress kinds (separation of concerns)

| Kind | What it attests | Default today | Elevated option |
|------|-----------------|---------------|-----------------|
| **`state_root`** | Foreign chain root / height (Ethereum, etc.) | ✅ Current providers + `VoteExtensionHashData` | unchanged |
| **`price_bound`** | Multi-source mid for a market id (cUSD-like) | ❌ not aggregated | Connect-style feeders + aggregate |

Do **not** collapse these into one blob. Roots are conservation/interop fabric (with LCs). Bounds are pricing honesty for CDP/quote guards only.

```text
  ┌──────────────── state_root path (default) ────────────────┐
  │ eth_getProof / LC headers → provider feeder → VE root     │
  │ → HashRoot quorum → contracts / ZK (interop fabric)       │
  └───────────────────────────────────────────────────────────┘

  ┌──────────────── price_bound path (optional elevated) ─────┐
  │ Connect-style providers (API/WS) per market_id            │
  │   → attribute (source, ts, raw)                           │
  │   → aggregate (median / weighted / trimmed)               │
  │   → OracleBoundObservation                                 │
  │   → VE or query API as bounds only (never mint)           │
  └───────────────────────────────────────────────────────────┘
```

---

## 3. Skip Connect mapping (synthesis, not fork)

| Connect concept | hash-market placement |
|-----------------|------------------------|
| Provider plugins (API + WS) | Extend `[[providers]]` + `transport`; optional Connect-compatible market map config |
| Market / currency pair map | `market_id` (e.g. `USDT/USD`, `ETH/USD`) on price providers |
| Oracle sidecar aggregation | `hash_market::oracle::aggregate_*` then store under `ProviderKey` or dedicated bounds map |
| Vote extensions | Existing ABCI++ path; bounds payload is **additional** kind or tagged algo (`price_median_v1`) |
| On-chain oracle module | Terp may later mirror Connect’s module; v1 can keep bounds in hashmerchant query + VE only |

**Rule:** Prefer **vendoring patterns and config shapes** from Connect over rewriting a price network. If/when we path-dep Connect crates or gRPC APIs, they plug in as a `SourceBackend::ConnectSidecar { url }`.

---

## 4. Granularity: source → attribute → aggregate

### 4.1 Source

A **source** is one named feeder endpoint:

```toml
[[providers]]
name = "binance_eth_usd"
kind = "price_bound"          # default if omitted: state_root (legacy)
market_id = "ETH/USD"
mode = "http_poll"            # or websocket; Connect backends later
address = "https://…"
interval_secs = 5
weight = 1.0                  # optional, aggregation
```

Legacy entries without `kind` remain **`state_root`**.

### 4.2 Attribute

Each observation carries:

| Field | Purpose |
|-------|---------|
| `source` | Provider name |
| `market_id` | What is priced |
| `price` | Decimal or fixed-point integer + decimals |
| `observed_at` | Unix secs |
| `foreign_height` | Optional if source is on-chain feed |
| `raw_ref` | Optional exchange trade id / ticker hash |

Stored in-process; exposed on `GET /providers` / future `GET /oracle/bounds?market_id=`.

### 4.3 Aggregate (sum / equal / mid)

| Method | Use |
|--------|-----|
| `median` | Default multi-source mid (Connect-like resilience) |
| `mean` | Equal weight average |
| `weighted_mean` | By `weight` |
| `trimmed_mean` | Drop high/low then mean |
| `min` / `max` | Bound envelope (strict min_out / max_in) |

**“Sum”** is not used for mid pricing; it is for **portfolio / basket** markets if we define composite `market_id`s later. Equal weight = `mean`. Quorum: require `min_sources` fresh within `max_age_secs`.

Hard invariant in code comments and API: **aggregated price is a bound input, not a mint instruction.**

---

## 5. Tacit trust mapping

| Bound path | Tier posture | Notes |
|------------|--------------|-------|
| Single operator feeder | Tier 1-ish (pilot) | Declare; not Tier 0 |
| Multi-source median + ⅔ VE | Stronger attestation of *published mid* | Still not conservation proof |
| LC-proven reserves + AMM curve | Tier 0 settle path | Mid only clamps slippage |

cBTC-like assets **must not** consume price_bound as mint authority.

---

## 6. oline / elevated packaging

| Mode | Deploy | Config |
|------|--------|--------|
| **Default** | Current hashmerchant elevated (trees + optional state_root VE) | `ve_enabled` + root providers only |
| **Elevated sophistication** | Same lease; config enables `price_bound` providers + aggregation | `oracle_bounds = true` in config.toml via SFTP |

No new football phase. Optional peer env later: `OLINE_HASHMERCHANT_ORACLE_BOUNDS=1` to seed example multi-source TOML.

---

## 7. Iterative delivery order

| Slice | Deliverable | Status | Breaks default? |
|-------|-------------|--------|-----------------|
| **O0** | This doc + `oracle` types + pure aggregate + tests | **Done** | No |
| **O1** | Config: optional `kind` / `market_id` / `[oracle]` section | **Done** | No (defaults preserve state_root) |
| **O2** | Feeder path for price HTTP tickers → attribute store | **Done** (`oracle::feeder`, `OracleAttributeStore`) | No if unused |
| **O3** | `GET /oracle/bounds` + optional VE `price_median_v1` | **Done** | Opt-in |
| **O4** | Connect sidecar adapter (gRPC/HTTP to existing Connect process) | Planned | Opt-in |
| **O5** | Wire private DEX `min_out` path to bounds query | Planned | App-level |

### O2/O3 runtime behaviour (shipped)

1. Config: `[oracle] bounds_enabled = true` + `[[providers]]` with `kind = "price_bound"` and `market_id`.
2. Each price feeder polls `address` for ticker JSON (`{"price":"…"}` / nested `data`).
3. Ticks land in `OracleAttributeStore`; market re-aggregates under policy (median default).
4. **API:** `GET /oracle/bounds?market_id=ETH/USD` → one mid, `role=bound_only`. Debug: `GET /oracle/ticks`.
5. **VE (optional):** when `ve_enabled`, publishes `VoteExtensionHashData` with `algo=price_median_v1`, `root = decimals_be‖mantissa_be` (20 bytes), empty attestations. Quorum is classic power vote on that mid — same security model as state roots, not multi-source bag consensus.

**Hard rule:** multi-source is **off-chain ingress**. Each provider (sidecar) submits **one** aggregated price per pair.

---

## 8. Non-goals (v1)

- Replacing Tacit ConfidentialPool / reflection with oracle mint  
- Full Connect chain module port as day-1  
- Claiming multi-source mid is Tier 0 solvency  
- Folding Skip Go *widget* (`crates/skip-go` frontend) into the sidecar — different product  

---

## 9. Public API (stable library)

**Module:** `hash_market::oracle` (always on; no feature flag for pure aggregate).  
**Session handoff:** `reviews/HASHMERCHANT-ORACLE-CONNECT-2026-07-20.md`.  
**Example:** `examples/price_oracle.rs`.

### 9.1 Types & functions (library contract)

| API | Notes |
|-----|--------|
| `ProviderKind::{StateRoot, PriceBound}` | Default `StateRoot` for legacy |
| `AttributedPrice` | `source`, `market_id`, fixed-point `mantissa`/`decimals`, `observed_at`, `weight` |
| `AggregationMethod` | `median` (default), `mean`, `weighted_mean`, `trimmed_mean`, `min`, `max` |
| `AggregationPolicy` | method + `min_sources` + `max_age_secs` |
| `aggregate_bounds(market, &[AttributedPrice], &policy, now)` | Pure; drops stale; enforces min_sources |
| `OracleBound` | Result mid; always document `role = "bound_only"` |
| `OracleAttributeStore` | Upsert tick + re-aggregate per market |
| `parse_ticker_json` / `parse_decimal_str` | Ingress helpers for HTTP feeders |
| `ALGO_PRICE_MEDIAN_V1` | `"price_median_v1"` — never a keccak state root |
| `encode_bound_root` / `decode_bound_root` | 20 bytes: `u32_be(decimals) ‖ i128_be(mantissa)` |
| `bound_to_vote_extension` | Mid → VE payload; **empty** ICS-23 / bag attestations |
| `price_chain_uid(market)` | Default synthetic uid `price:{market_id}` |

**Invariants (semver-stable intent):**

1. Aggregation is deterministic given the same fresh multiset of ticks + policy + `now`.  
2. Output is never a mint instruction (`role=bound_only`).  
3. VE root encoding for mids is only valid under `price_median_v1` (or future `price_*` family).  
4. Callers must not collapse `state_root` and `price_bound` into one semantic blob.

### 9.2 HTTP (runtime, O3)

```http
GET /oracle/bounds?market_id=ETH/USD
→ {
  "market_id": "ETH/USD",
  "method": "median",
  "price": "3450.12",
  "decimals": 8,
  "mantissa": 345012000000,
  "n_sources": 3,
  "sources": ["binance", "coinbase", "okx"],
  "as_of": 1710000000,
  "role": "bound_only"
}
```

Also: `GET /oracle/ticks?market_id=…` (raw attributed sources).  
`GET /health` includes `"oracle": { "bounds_enabled": true, ... }`.

### 9.3 VE packing (optional)

When `ve_enabled` and bounds feeders run: publish `VoteExtensionHashData` with:

- `algo = price_median_v1`
- `root = encode_bound_root(decimals, mantissa)`
- `chain_uid =` provider `chain_uid` or `price:{market_id}`
- empty attestations (mid is the commitment)

Quorum remains classic voting-power agreement on that root — same class of security as foreign state-root oracles, **not** multi-source bag consensus.
