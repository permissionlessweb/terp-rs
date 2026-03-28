---
title: Blob DA Design (Future)
description: EIP-4844-inspired temporary blob storage for sub-store mirroring and fee manifold
---

# Blob DA Design

**Status:** Design reference. Not implemented.

This document captures the design direction for extending hashmerchant with EIP-4844-inspired temporary blob storage, sub-store mirroring, and a fee manifold for reward distribution.

## Motivation

Four gaps identified in the current implementation:

1. **Multi-client ingestion** — server only accepts one transport/chain at a time
2. **Sub-store mirroring** — only top-level roots are stored; contracts cannot verify sub-tree paths without full Merkle proof replay in CosmWasm
3. **Fee manifold** — escrow is binary (on/off); no reward routing to validators or relayers based on participation
4. **Blob lifecycle** — sub-store snapshots are large and temporary, but the current design has no concept of prunable off-chain data tied to on-chain commitments

EIP-4844 (proto-danksharding) solves an analogous problem on Ethereum: large temporary data structures committed to on-chain but stored off-chain with a bounded availability window.

## EIP-4844 reference

Spec: <https://www.eip4844.com/>

Key properties:

- Blob transactions carry up to 128KB blobs (max 6 per block)
- On-chain: only the KZG commitment is stored (~48 bytes)
- Off-chain: blob data is available for ~18 days (4096 epochs), then pruned by consensus clients
- Separate fee market (blob gas) from execution gas
- Verification: point evaluation precompile checks blob contents against commitment

## Mapping to hashmerchant

| EIP-4844 | Hashmerchant equivalent |
|---|---|
| Blob | Sub-store snapshot: intermediate Merkle nodes at a specific path and height |
| KZG commitment (48 bytes, permanent) | `HashRoot` or new `HashSubRoot` (32-byte hash, permanent) |
| ~18 day availability window | Configurable `ttl_blocks` per blob, tied to escrow funding |
| Blob gas market | Fee manifold: prices blob availability by size * duration |
| Consensus client stores blobs | Validator sidecar stores blobs |
| Point evaluation precompile | Contract hashes supplied calldata against stored commitment |

## Proposed data flow

```
Foreign Chain              Sidecar                    On-chain                  Contract
                     ┌──────────────────┐      ┌──────────────────┐
state root ────────► │ fetch full proof  │      │ HashRoot         │
sub-store path ────► │ extract sub-tree  │      │   commitment     │
                     │ store as blob     │      │   blob_id        │
                     │ attest via VE     │─────►│   ttl            │
                     │                   │      │   size_class     │
                     │ blob store (TTL)  │      └────────┬─────────┘
                     │   serve on GET    │               │ sudo callback
                     └────────┬─────────┘               ▼
                              │                  ┌──────────────────┐
                              │    calldata      │ contract         │
                              └─────────────────►│   hash(calldata) │
                                (relayer submits │   == commitment? │
                                 blob at exectime)│   verify claim  │
                                                 └──────────────────┘
```

## Proposed on-chain types

```protobuf
// Extends existing types.proto

message HashSubRoot {
  string chain_uid = 1;
  string algo = 2;
  uint64 foreign_height = 3;
  bytes commitment = 4;           // hash of the blob contents
  string store_path = 5;          // e.g. "accounts/0xabc.../storage/nfts"
  uint64 ttl_expiry_height = 6;   // prune blob availability after this height
  uint32 size_bytes = 7;          // blob size for fee calculation
  uint32 attestation_count = 8;
  bytes parent_root = 9;          // links to the top-level HashRoot
}

message BlobFeeSchedule {
  cosmos.base.v1beta1.Coin base_rate_per_kb_per_block = 1;
  repeated SizeClass size_classes = 2;
  uint64 default_availability_window = 3;
}

message SizeClass {
  string label = 1;              // "light", "medium", "heavy"
  uint32 max_kb = 2;
  string multiplier = 3;         // decimal string, e.g. "1.5"
}
```

## Fee manifold

Current escrow is binary: funded → callbacks enabled, expired → disabled. The manifold replaces this with a distribution engine.

```
EscrowDeposit (contract pays for blob availability)
    │
    ▼
FeeManifold
    ├── 30% → RelayerRewardPool
    │         Per-tx reimbursement for submitting blob calldata
    │         Registered tokens: [uterp, uatom, ...]
    │
    ├── 60% → ValidatorRewardPool
    │         Pro-rata by participation score:
    │           score = inclusion_rate * sqrt(voting_power) * uptime_factor
    │         Distributed every distribution_epoch blocks
    │
    └── 10% → Treasury
              Protocol fee
```

Participation tracking requires new keeper state:

```protobuf
message ValidatorParticipation {
  string validator_addr = 1;
  uint64 extensions_submitted = 2;   // total VEs included in blocks
  uint64 extensions_expected = 3;    // blocks where validator was in active set
  uint64 last_update_height = 4;
}
```

The `ProcessVoteExtensions` EndBlocker already iterates all votes — adding a participation counter is straightforward.

## DA layer options

### Option A: Sidecar as DA (self-contained)

Validators already attest they had the data via vote extensions. Extend the sidecar with:

- Blob store with TTL-based pruning
- HTTP endpoint: `GET /blob/{blob_id}` for relayers to fetch
- Attestation: VE includes blob commitment + size

Trust assumption: same as consensus (2/3 validators stored the blob).

Pros: no external dependencies, simpler ops.
Cons: validator disk usage scales with blob demand.

### Option B: External DA (Celestia / Avail)

Sidecar posts blobs to an external DA layer. On-chain stores the DA commitment (namespace + height + commitment).

Pros: decouples storage from validators, proven DA guarantees.
Cons: additional infrastructure, latency, dependency on external chain.

### Recommendation

Start with Option A. The sidecar already has the proof data in memory during the vote extension cycle. Adding a TTL cache with disk persistence is minimal work. Option B can be added later as an alternative backend behind a trait interface.

## Multi-client ingestion (prerequisite)

The current server accepts one transport. For blob DA, the server must ingest from multiple chains simultaneously.

Changes needed in hash-market sidecar:

- `AppState.latest_data` → `HashMap<(chain_uid, algo), VoteExtensionHashData>`
- Config: `[[transport]]` array instead of `[transport]`
- `/extend-vote` accepts `chain_uid` param or returns all pending extensions
- Blob store keyed by `(chain_uid, height, path)`

## Divergence from EIP-4844

- No KZG ceremony — use simple hash commitments (SHA256 or BLAKE3 of blob contents). KZG could be added later for point evaluation (proving a specific element within a blob without revealing the full blob).
- No consensus-layer blob gossip — blobs are stored per-validator in the sidecar and served on request. Availability is attested, not gossiped.
- TTL is configurable per-contract via escrow funding, not a fixed protocol constant.
- Blob fee market is governed by the fee manifold parameters (governable), not by an EIP-1559-style base fee mechanism. Could adopt EIP-1559 dynamics later if blob demand is volatile.

## Implementation order (when revisited)

1. Multi-client ingestion (sidecar Rust refactor)
2. Blob store + TTL in sidecar
3. `HashSubRoot` proto + keeper + EndBlocker changes (Go)
4. Fee manifold proto + distribution logic (Go)
5. Validator participation tracking (Go)
6. Contract SDK helpers for blob verification (CosmWasm)
7. Custody documentation (docs only, can happen anytime)
