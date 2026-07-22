# Handoff — Private Bridge production readiness (for E2E specialist)

**Date:** 2026-07-22  
**To:** E2E / private-bridge specialist (HARNESS track + program)  
**From:** corridor session (D1–D7 accepted + oline play + indexer improvements)  
**Read first:** [DESIGN-DECISIONS-CORRIDOR-ACCEPTED](../DESIGN-DECISIONS-CORRIDOR-ACCEPTED-2026-07-20.md), [REVIEW-OLINE-BTC-INDEXER](../reviews/REVIEW-OLINE-BTC-INDEXER-BLINDSPOTS-2026-07-20.md)

---

## Where we are (plain English)

We can run a **lab film** that is real HTTP automation (not a PowerPoint):

1. Browser / suite seals deposit intent + opens **watch** on hash-market  
2. Lab smoke or observer posts **deposit.observed** (or reporter will when index is live)  
3. Host script runs **mint-after-observe** film: bridging → oracle-bound swap → complete  
4. UI can poll `/corridor/automation/:id` and (production path) **re-verify** observation via Esplora multi-fallback  

We are **not** yet at “Cash App mainnet → ZEC mainnet in one click.” That needs Fulcrum packaging, real funded e2e, Zakura, and non-mock proofs.

---

## What is production-shaped and green

| Layer | Status | How to run |
|-------|--------|------------|
| Design freezes D1–D7 | **ACCEPTED** (D3 swap required; D6 Zakura residual) | Acceptance doc |
| Pure corridor I1–I6 + W0–W7 sim | **Green** | `fixtures/cashapp_zec_corridor`, `cashapp_zec` harness |
| Notify plane | **Green** | `POST/GET /corridor/watches*`, SSE, observations |
| Host lab film | **Green** | `just demo-corridor-lab` + `just demo-corridor-mint-after-observe` |
| Oline play preflight/e2e | **Green (lab)** | `plays/private-bridge-corridor/preflight.sh --lab` + `e2e-test.sh` |
| Esplora multi-failover | **Implemented** | `esplora_urls` / `esplora_network` (Tacit-style) |
| Multi-bin image | **Dockerfile ships server + reporter** | rebuild image |
| UI re-verify hook | **Implemented** | `btcReverify.ts` + `depositReverify` on production sequence |
| Sovereignty default | **Documented** | `backend=electrum` + Fulcrum for oline money |

---

## What is still residual (honest)

| Item | Blocker for “production” claim? |
|------|----------------------------------|
| Fulcrum + bitcoind **running** in oline play (not just config) | **Yes** for self-hosted BTC observe |
| Image rebuild/push with multi-bin | Operators must rebuild after Dockerfile change |
| Live funded testnet watch→observe e2e | Soft-skipped; no funded deposit in CI |
| UI always passes `depositReverify` on SSE | Wire call sites must pass txid from observation |
| Real `BridgeMintNote` on daemon (not host pure film) | Mock_verify contract path still host/Mock oriented |
| Zakura local dest UX (D6) | Preauth string OK; Zakura not integrated |
| Mainnet Cash App / ZEC send | Phase 2 |

---

## Indexer defaults (after review)

| Environment | Backend | Why |
|-------------|---------|-----|
| **Lab / CI** | Esplora multi-URL **or** synthetic observer | Low friction; matches Tacit e2e |
| **Oline production** | **bitcoind → Fulcrum → electrum reporter** | Sovereignty; deposit addrs never hit public explorers |
| **Avoid** | Public mainnet Esplora for live deposit watches | Privacy + rate limits (warned in reporter) |

---

## Commands for the specialist

```bash
# Lab film (host)
cd crates/headstash && just demo-corridor-lab
just demo-corridor-mint-after-observe

# Oline play quality bar
cd crates/o-line/plays/private-bridge-corridor
bash ./preflight.sh --lab
bash ./e2e-test.sh

# Indexer unit tests
cd crates/terp-rs/tools/hash-market
cargo test -p hash-market --lib btc_index --features server
cargo test -p hash-market --lib corridor_deposits --features server

# Rebuild multi-bin image (from crates/terp-rs)
docker build -t terp/hash-market:corridor-lab -f tools/hash-market/Dockerfile \
  --build-arg FEATURES=server .
```

---

## Recommended next ownership

| Owner | Next |
|-------|------|
| **E2E specialist** | Promote oline play; real Fulcrum inventory; optional signet fund→observe; wire UI SSE → reverify with observation fields |
| **UI** | Pass `depositReverify` from SSE payload; Zakura local dest (D6) |
| **Ops** | bitcoind+Fulcrum SDL on oline; never public mainnet Esplora for prod watches |

---

## Bottom line for “can we run in production?”

| Question | Answer |
|----------|--------|
| Can we run **lab** e2e for private bridge automation? | **Yes** — green scripts, accepted design freezes |
| Can we run **production-shaped BTC observe** on oline? | **Almost** — multi-bin + Fulcrum config ready; need live Fulcrum stack + image rebuild + operator playbook |
| Can we run **mainnet Cash App → ZEC** end-to-end? | **No** — mock_verify, no Zakura mainnet, no LC proofs, no funded mainnet e2e |

**Production readiness score (private bridge corridor):**  
**Lab demo: ~85%** · **Self-hosted observe: ~60%** · **Mainnet money: ~20%**
