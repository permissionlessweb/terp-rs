# `CosmWasm` ICS-08 Crosslink Light Client

This repository contains a `CosmWasm` implementation of a **Zcash Crosslink** light client, designed for integration with [`ibc-go`](https://github.com/cosmos/ibc-go)'s [`08-wasm`](https://github.com/cosmos/ibc-go/tree/main/modules/light-clients/08-wasm) client wrapper.

It manages IBC client and consensus states, delegating all light client-specific verification logic to the core module in [`packages/crosslink/light-client`](../../packages/crosslink/light-client).

## Overview

**Zcash Crosslink** is a Trailing Finality Layer (TFL) that provides economic finality to Zcash PoW blocks through a BFT consensus protocol (Malachite + Tenderlink). This light client allows Cosmos SDK chains to verify Crosslink-finalized PoW anchors and route IBC packets based on Zcash's hybrid finality.

The architecture consists of:
- `crosslink-light-client` — reusable, IBC-agnostic verification primitives
- `cw-ics08-wasm-crosslink` — the CosmWasm 08-wasm contract
- `crosslink-relayer` — standalone Rust binary that polls `zebrad` and submits updates

## Light Client Functions

### Update Client Logic

Client updates are driven by **CrosslinkHeaders**, which bundle:
- A BFT block containing the PoW anchor header
- A `FatPointerToBftBlock2` with aggregated ed25519 signatures from the current finalizer roster

Verification steps (implemented in `crosslink-light-client`):
1. Client is not frozen
2. Trusted height matches latest stored BFT height
3. BFT block has the expected confirmation depth (`sigma`)
4. Fat pointer correctly points to the BFT block hash
5. Batch ed25519 signature verification succeeds
6. New PoW anchor height is strictly greater than the previous one
7. Header chain continuity (via `zebra-crosslink` types)

After successful verification, the contract updates the `ClientState` (latest heights, roster, etc.) and stores a new `ConsensusState` for the finalized PoW anchor.

### Membership Proofs

Membership and non-membership proofs verify values (or their absence) under the **state commitment root** stored in the `ConsensusState`.

- For the initial version, these are **stubs** that return an error (`not yet implemented`).
- Full implementation will integrate with **ZIP 222** (the finalized state commitment format on Zcash) once the proof format and Merkle structure are finalized.
- Future work may include VM precompiles for BLAKE3, Poseidon, and other Zcash-specific primitives.

See `packages/crosslink/light-client/src/membership.rs`.

### Misbehavior Handling

Misbehavior is currently stubbed and always returns `false` (no misbehaviour detected).

Future detection will cover **equivocation**: two valid but conflicting fat pointers (different finalization candidates) at the same BFT height signed by a sufficient quorum of the finalizer set. Upon detection, the client is frozen. Recovery requires a governance proposal.

## Deployment

### Build Requirements

- Rust (2021 edition or later)
- `cargo` + `wasm32-unknown-unknown` target
- Docker (for optimized builds via `cosmwasm/optimizer`)

```bash
# Build the contract
cargo build -p cw-ics08-wasm-crosslink --lib --target wasm32-unknown-unknown --release

# Or use the optimizer for smaller size
just build-cw-ics08-wasm-crosslink   # if a Justfile is present



### Storing the Wasm Binary

Store the optimized `.wasm` binary on the target Cosmos chain following the [IBC-Go 08-wasm documentation](https://ibc.cosmos.network/main/ibc/light-clients/wasm/overview.html).

### Creating the Light Client

Use a `MsgCreateClient` transaction with:
- The stored bytecode checksum
- Serialized initial `ClientState` (containing parameters, finalizer roster, genesis heights, etc.)
- Serialized initial `ConsensusState`

The relayer (`crosslink-relayer`) can then begin submitting updates.

## Relayer

A minimal Rust relayer is provided in `tools/crosslink-relayer/`. It:
- Polls a `zebrad` (crosslink-enabled) JSON-RPC endpoint
- Fetches finalized BFT blocks + fat pointers
- Constructs and submits `CrosslinkHeader` updates via the Cosmos chain's gRPC/CLI

## Development & Testing

```bash
# Check and test the light client primitives
cargo check -p crosslink-light-client
cargo test -p crosslink-light-client

# Test the full CosmWasm contract
cargo test -p cw-ics08-wasm-crosslink

# Build Wasm
cargo build -p cw-ics08-wasm-crosslink --lib --target wasm32-unknown-unknown --release
```

End-to-end tests (using `cw-orchestrator` + `ict-rs` for Docker-based zebrad + Terp chains) are planned in `tests/e2e/`.

## Acknowledgements

This implementation builds on:
- Zcash's [`zebra-chain`](https://github.com/ZcashFoundation/zebra) and the new `zebra-crosslink` workspace for consensus types and serialization.
- The broader IBC 08-wasm ecosystem and previous Ethereum light client work by [Union Labs](https://github.com/unionlabs/union).

---

**Next Steps / Future Work**
- Full ZIP 222 membership proof verification
- Robust misbehaviour detection (equivocation)
- VM precompiles for Zcash cryptography (ed25519 already embedded, others coming)
- Production-grade relayer with proper signing and retry logic
- Comprehensive e2e test suite

For questions or contributions, see the implementation plan in the repository root.
```

 