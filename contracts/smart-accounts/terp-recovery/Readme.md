# terp-recovery

Break-glass / social recovery authenticator with **configurable rehash algorithms**.

## Challenge digest

```text
preimage = domain || 0x00 || chain_id || 0x00 || account || 0x00
           || authenticator_id || 0x00 || sign_mode_direct
digest   = rehash_N(hash_alg, preimage)
```

Default domain: `terp-recovery/break-glass/v1`

## Hash algorithms (`RecoveryHashAlg`)

| Variant | Notes |
|---------|--------|
| `Sha256` | default |
| `Sha512` | |
| `Keccak256` | Ethereum-family |
| `Sha3_256` | FIPS SHA3 |
| `Blake2s256` | |
| `Blake2b256` | BLAKE2b-512 truncated to 32 |
| `Sha256d` | double-SHA256 |
| `PoseidonPallas` | Poseidon over Pallas (`P128Pow5T3`); pure-Rust until host Poseidon exists |

`rehash_rounds` (1..=64) applies the algorithm sequentially.

### Poseidon-Pallas fixed instance

CosmWasm has no native Poseidon import yet. Recovery therefore uses monorepo
`halo2_poseidon` + `pasta_curves` so the contract can rehash challenges that a
ZK circuit can prove. **Must match** `terp-recovery-poseidon-demo` and any
production Halo2 chip:

| Parameter | Value |
|-----------|--------|
| Field | `pallas::Base` |
| Spec | `P128Pow5T3` (width 3, rate 2, R_F=8, R_P=56, sbox x⁵) |
| Byte → field | 31-byte little-endian chunks (pad last); empty → one zero word |
| Variable length | length-tagged MD fold: `H₂(F(len), w₀)` then `H₂(state, wᵢ)` |
| Output | 32-byte little-endian field encoding (`to_repr`) |

See `src/hash.rs` module docs and `reviews/POSEIDON-RECOVERY-DEMO.md`.

Pairing with a circuit (JWT-style hash-alg + host verify) is demonstrated in
[`terp-recovery-poseidon-demo`](../terp-recovery-poseidon-demo/).

## Approvals

1. **Ed25519** (preferred): guardian has `pubkey`; `signature` is 64-byte host `ed25519_verify` over `digest`.
2. **Address-only attestation** (MVP): `signature == rehash(hash_alg, guardian || 0x00 || digest)` when `allow_address_only_approvals`.

Auth payload must **echo** `hash_alg` and `rehash_rounds` matching on-chain config (alg confusion protection).

## Query

`Challenge { chain_id, account, authenticator_id, sign_mode_direct }` returns the digest clients must sign/attest.
