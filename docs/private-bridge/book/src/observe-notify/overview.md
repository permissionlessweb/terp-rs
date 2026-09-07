# Observe & notify — overview

After a sealed DepositIntent, the corridor must **see** the BTC deposit and **notify** downstream stages without redefining mint policy. This category is the observe plane: watches, observations, SSE, and topology plays — not note formats or guest wasm.

| Library | Monorepo path | Role |
|---------|---------------|------|
| [hash-market](hash-market.md) | `crates/terp-rs/tools/hash-market/` | Watches, SSE, observations, oracle bounds |
| [Corridor deposit notify](corridor-deposit-notify.md) | `…/hash-market/docs/corridor-deposit-notify.md` | Watch/observe/SSE notify plane for deposits |
| [corridor-btc-reporter](corridor-btc-reporter.md) | `…/hash-market/docs/corridor-btc-reporter.md` | Production BTC deposit monitor → observations |
| [oline private-bridge play](oline-play.md) | `crates/o-line/plays/private-bridge-corridor/` | D4 topology preflight / e2e |

## What this category owns

- Observation records and notify paths used by the corridor film
- Hash-market tooling docs specific to Private Bridge deposits
- oline play topology checks (D4) before funded multi-net runs

## Non-goals

| Do not look here for… | Look instead |
|----------------------|--------------|
| Bridge mint authorize / SeamNoteOut | [Pure seams](../pure-seams/overview.md), [SEAM-NOTE-OUT](../design/seam-note-out.md) |
| cw-headstash store/mint | [Contracts & circuit](../contracts-circuit/overview.md) |
| ZEC dest preauth | [ZAKURA-LOCAL](../zakura/zakura-local.md) |
| Funded multi-net harness | [Harness & ict-rs](../harness-ict/e2e-corridor.md) |

## Place in the spine

```text
… sealed DepositIntent
  → observe (hash-market + reporter) → BridgeMintNote (cw-headstash)
  → …
```

**Next:** [hash-market](hash-market.md)
