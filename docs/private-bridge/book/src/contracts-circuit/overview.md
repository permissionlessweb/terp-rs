# Contracts & circuit — overview

On-chain and ZK surfaces for Private Bridge minting live under `crates/headstash/`. This category is the **guest + circuit** layer: note formats and mint routing must match pure seams and design freezes (D1/D2), not invent a second SSOT.

| Library | Monorepo path | Role |
|---------|---------------|------|
| [headstash workspace](headstash-workspace.md) | `crates/headstash/` | Workspace root: circuit, contracts, test-press, just recipes |
| [zk-headstash (circuit)](zk-headstash-circuit.md) | `crates/headstash/circuit/` | Orchard/Headstash circuit; host-crypto vs guest `Instance` |
| [cw-headstash](cw-headstash.md) | `crates/headstash/contracts/cw-headstash/` | Mint router + BridgeMintNote; wasmd-safe guest build |

## What this category owns

- Guest contract build graph (features that may/may not land in wasm)
- Circuit / prove surfaces used by mint and claim paths
- Workspace-level demo and ICT entrypoints (`just demo-corridor-ict`, etc.)

## Non-goals

| Do not look here for… | Look instead |
|----------------------|--------------|
| DepositIntent / pure policy | [Pure seams](../pure-seams/overview.md) |
| BTC watch / SSE notify | [Observe & notify](../observe-notify/overview.md) |
| wasmd host capability floor | [Host floor](../platform-wasmvm/host-floor.md) |
| Product spine narrative | [DEMO corridor](../product/demo-corridor.md) |

## Companion commands

```bash
cd crates/headstash && just demo-e2e-l0
cd crates/headstash && just prepare-corridor-wasm-force
cd crates/headstash && just demo-corridor-ict
```

**Next:** [headstash workspace](headstash-workspace.md)
