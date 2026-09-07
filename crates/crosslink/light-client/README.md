# crosslink-light-client

Self-contained verification primitives for the Zcash Crosslink Trailing Finality Layer (TFL) IBC light client.

## Location

```
crates/terp-rs/
  crates/crosslink/light-client/   ← this crate (library, no IBC deps)
    src/types.rs                   ← all crosslink types + ZcashSerialize/ZcashDeserialize
    src/verify.rs                  ← verify_header() — structure, continuity, signatures
    src/update.rs                  ← update_consensus_state()
    src/header.rs                  ← CrosslinkHeader
    src/client_state.rs            ← ClientState, FinalizerEntry
    src/consensus_state.rs         ← ConsensusState
  contracts/light-clients/
    cw-ics08-wasm-crosslink/       ← CosmWasm 08-wasm contract wrapping this crate
```

## Architecture

### Self-contained types (types.rs)

Defines the core Crosslink types natively with **zero dependency on `zebra-crosslink`** or `zebra-chain` — those crates are heavy, pull in tokio/async infrastructure, and don't compile to `wasm32-unknown-unknown`.

| Light-client type | Zebra-crosslink equivalent | Notes |
|---|---|---|
| `BftBlock` | `zebra_crosslink::chain::BftBlock` | serde-derivable, no zcash_serialize dependency |
| `FatPointerToBftBlock2` | `zebra_crosslink::FatPointerToBftBlock2` | Vote bundle with aggregated signatures |
| `FatPointerSignature2` | `zebra_crosslink::FatPointerSignature2` | Ed25519 pk + signature per finalizer |
| `Blake3Hash` | `zebra_crosslink::chain::Blake3Hash` | 32-byte BLAKE3 hash |
| `ZcashCrosslinkParameters` | `zebra_crosslink::chain::ZcashCrosslinkParameters` | σ (confirmation depth) and L (gap bound) |
| `PowHeader` | `zebra_chain::block::Header` (slimmed) | Hash + timestamp + height only |

### Zcash binary serialization (`ZcashSerialize` / `ZcashDeserialize`)

Implements the **identical binary format** as `zebra-crosslink` — same field order, endianness, and byte layout. This ensures:

- **Interoperability**: bytes produced by this crate can be consumed by zebra-crosslink and vice versa
- **Deterministic hashing**: `BftBlock::blake3_hash()` uses the same serialization order
- **No typecasting across workspace boundaries**: the contract and relayer share the same encoded format

The serialization traits are defined in `types.rs` alongside the types:

```rust
pub trait ZcashSerialize { fn zcash_serialize<W: Write>(&self, writer: W) -> Result<(), io::Error>; }
pub trait ZcashDeserialize { fn zcash_deserialize<R: Read>(reader: R) -> Result<Self, SerializationError>; }
```

They use `byteorder::LittleEndian` for multi-byte fields and `Read`/`Write` traits from `std::io` — consistent with the upstream `zebra-chain` approach.

### Type invariance tests

**Dev-dependency on `zebra-crosslink` and `zebra-chain`** enables tests that validate our internal types against the real zebra-crosslink types at the byte level:

```
#[cfg(feature = "zebra-crosslink-compare")]
#[test]
fn test_bft_block_hash_matches_zebra() {
    // Construct equivalent BftBlock instances with our types and zebra types
    // Assert blake3_hash() produces identical bytes
    // Assert zcash_serialize() produces identical bytes
}
```

This means any refactor or field reorder that would break compatibility is caught at test time. The dev-dep crates are **never pulled into the contract's dependency tree**.

## Signature verification

### `validate_signatures()` (types.rs)

Uses `ed25519-zebra` batch verification on the fat pointer signatures. The signed message is the 44-byte `vote_for_block_without_finalizer_public_key` — **without** the validator address (the address is in the certificate envelope, not the signed payload). This matches `zebra-crosslink::FatPointerToBftBlock2::validate_signatures()` in `malctx.rs`.

1. Client is not frozen
2. Trusted height matches latest state
3. Confirmation depth (σ) is correct
4. Fat pointer block hash matches BFT block hash
5. **Signature verification (native only)** — relayer-verified on wasm
6. PoW anchor height is strictly ahead

## Feature flags

| Feature | Enables | Used by |
|---|---|---|
| `sigverify` (default) | `ed25519-zebra` + `rand` + `rand08` batch verification | Native builds, tests |
| — (default-features = false) | No signature deps | CosmWasm contract (wasm target) |
| `zebra-crosslink-compare` (dev) | Type invariance tests against zebra-crosslink | CI, test suite |

## Verification flow

```text
zebrad (crosslink node) → RPC → relayer
  ↓ constructs CrosslinkHeader (BftBlock + FatPointer signatures)
  ↓ verifies ed25519 batch (native binary)
  ↓ submits MsgUpdateClient to Cosmos chain

cw-ics08-wasm-crosslink contract
  ↓ verify_header():
    1. is_frozen?         → reject if frozen
    2. trusted height?     → reject if wrong anchor
    3. confirmation depth? → reject if wrong σ
    4. block commitment?   → reject if hash mismatch
    5. signatures?         → batch ed25519 (host precompile on wasm)
    6. height progression? → reject if not strictly ahead
  ↓ update_consensus_state() → store new ConsensusState:
      (bft_height, pow_height, pow_hash, timestamp,
       shielded_commitment ← PowHeader.commitment_bytes,
       app_state_commitment ← 0  // ZIP-222 / IBC-v2)
```

## v1 shielded anchors (not ZIP-222)

`PowHeader.commitment_bytes` carries the Zcash header field that, depending on
era, is FinalSaplingRoot, ChainHistoryRoot, or NU5+
`ChainHistoryBlockTxAuthCommitment` (binds FlyClient history + `hashAuthDataRoot`).

On update, that value is stored as:

| Field | Role |
|---|---|
| `ConsensusState.shielded_commitment` | Anchor at `(bft_height, pow_anchor_height)` |
| `ClientState.latest_shielded_commitment` | Tip copy of the same root |
| `ConsensusState.app_state_commitment` | **Reserved zero** — ZIP-222 app state for IBC-v2 |

Membership proofs in v1 target `shielded_commitment` via a compact BLAKE3
pool-root existence format whose parameters are documented as
`pool_root_proof_specs()` (ICS-23-shaped: leaf prefix `0x00`, 32-byte
children, BLAKE3). ZIP-222 app-state proofs wait for IBC-v2
(`app_state_commitment`). See `src/membership.rs`.