---
title: Architecture
description: System map of the hashmerchant module, sidecar, and privacy layer
---

# Architecture

The hashmerchant stack bridges foreign chain state into Terp via ABCI++ vote extensions, then makes that state available to CosmWasm contracts and ZK circuits.

```
Foreign Chain            Validator Sidecar             Terp Chain              Apps
(Ethereum, etc)          (hash-market)                 (hashmerchant)          (CosmWasm)

+-----------+     +---------------------------+     +----------------+     +-------------+
| eth node  |     |  hash-market-client       |     |  VoteExtension |     | contract A  |
| state root|---->|  eth_getProof polling     |     |  Handler       |     | (receives   |
| blocks    |     |  Pallas Fp reduction      |     |                |     |  sudo calls)|
+-----------+     +------------+--------------+     |  EndBlocker    |     +-------------+
                               |                    |  - tally votes |
                               v                    |  - check 2/3   |     +-------------+
                  +---------------------------+     |    quorum      |     | contract B  |
                  |  hash-market-server       |     |  - write root  |---->| (queries    |
                  |  /extend-vote             |---->|  - sudo notify |     |  HashRoot)  |
                  |  /verify-vote-extension   |     +----------------+     +-------------+
                  |  custody: local | tkms    |
                  +---------------------------+     +----------------+     +-------------+
                                                    | HashPairTicket |     | ZK circuit  |
                  +---------------------------+     | origin + dest  |---->| Pallas-based|
                  |  headstash-server         |     | hash pairing   |     | verification|
                  |  notes, keys, PIR         |     +----------------+     +-------------+
                  +---------------------------+
```

## Components

### On-chain: `x/hashmerchant`

The Cosmos SDK module. Fully implemented in Go, wired into app.go.

| Concept | What it does |
|---|---|
| `RegisteredChain` | Governance-registered foreign chain (UID, RPCs, hash algos) |
| `RegisteredContract` | CosmWasm contract that pays escrow to receive `HashRoot` sudo callbacks |
| `VoteExtensionHashData` | ABCI++ payload validators attach to their votes |
| `HashRoot` | Confirmed foreign state root after 2/3 voting power quorum |
| `HashPairTicket` | Links an origin hash to a destination hash with ZK circuit metadata |
| `EscrowRecord` | Tracks per-contract payment; auto-prunes expired contracts |

Data flow per block:

1. CometBFT calls `ExtendVote` on each validator
2. The sidecar returns a signed `VoteExtensionHashData` (or empty)
3. CometBFT calls `VerifyVoteExtension` on peers' extensions
4. `ProcessVoteExtensions` tallies `(chain_uid, algo)` pairs by voting power
5. Roots reaching 2/3 quorum are written as `HashRoot`
6. Registered contracts with active escrow receive `sudo { hash_merchant: { ... } }`

### Off-chain: `hash-market` (sidecar)

A single Rust crate with feature flags. Two binaries:

| Binary | Features | Role |
|---|---|---|
| `hash-market-server` | `server` | Runs alongside the validator. Signs vote extensions, serves CometBFT callbacks |
| `hash-market-client` | `client` | Polls foreign chains (Ethereum), transforms proofs, feeds data to the server |

The server and client can run as separate processes or on separate machines. They communicate through the transport layer (gRPC, HTTP poll, or WebSocket).

### Off-chain: `headstash-server`

Privacy-preserving note and circuit key distribution server. Serves the MetaMask snap (`snap-n-pull-js`) with secp256k1-authenticated endpoints and XOR-based PIR for metadata privacy.

## Protobuf wire format

Both the on-chain module and the sidecar encode messages using the same protobuf field numbers. The sidecar uses `anybuf` (no codegen) while the chain uses standard proto codegen.

```
VoteExtensionHashData:
  1: runtime_id    (string)    — sidecar instance identifier
  2: chain_uid     (string)    — e.g. "ethereum-mainnet"
  3: algo          (string)    — e.g. "keccak256", "sha256", "poseidon"
  4: root          (bytes)     — 32-byte state root
  5: foreign_height (uint64)   — block height on the foreign chain
  6: foreign_block_time (int64) — unix seconds
  7: ics23_proof   (bytes)     — optional ICS-23 commitment proof
```

These field numbers are the protocol contract. Changing them breaks consensus.

## Signing

Vote extensions are domain-separated to prevent replay:

```
digest = SHA256("terp/hashmerchant/ve/v1" || chain_id || height_be8 || extension_bytes)
signature = secp256k1_sign(digest, validator_key)
```

The custody trait abstracts key management. `LocalCustody` signs in-process; `TkmsCustody` delegates to an external KMS over TCP.
