# terp-recovery-poseidon-demo

Toy circuit + host-verify scaffold that pairs **`terp-recovery` Poseidon-Pallas break-glass digests** with the same **hash-alg + circuit** pattern used by `terp-zkjwt`.

## Pattern (JWT ↔ recovery)

| Layer | zk-jwt | recovery Poseidon demo |
|-------|--------|-------------------------|
| On-contract / policy | issuer config + claim layout | `RecoveryHashAlg::PoseidonPallas` + domain preimage |
| Digest | claim commitment / msg bind | `rehash(PoseidonPallas, challenge_preimage)` → 32 LE bytes |
| Circuit public instance | nullifier \|\| claim \|\| msg_bind | challenge digest (32) \|\| rest |
| Host verify | `proof_instance_verify(zkid, proof, instances)` | same API (`zk-host` feature) |
| CI / Mock | structural envelope | structural envelope + optional witness rehash |

## Fixed Poseidon instance

Must match `terp_recovery::hash` module docs:

- **Field:** `pasta_curves::pallas::Base`
- **Spec:** `halo2_poseidon::P128Pow5T3` (T=3, rate=2, R_F=8, R_P=56)
- **Bytes → field:** 31-byte little-endian chunks, last chunk zero-padded; empty → one zero word
- **Variable length:** length-tagged MD fold of `ConstantLength<2>` hashes (`F(len_bytes)` then words)
- **Output:** 32-byte little-endian field encoding

```rust
use terp_recovery_poseidon_demo::{recovery_poseidon_digest, verify_toy, PoseidonToyPayload};
use terp_recovery_poseidon_demo::{build_public_instances, CIRCUIT_ID_RECOVERY_PREIMAGE};

let digest = recovery_poseidon_digest(preimage);
let instances = build_public_instances(&digest, &[])?;
// fill PoseidonToyPayload { circuit_id, proof, public_inputs, witness_preimage, zkid }
// verify_toy(deps, &payload)?;
```

## CosmWasm ZK APIs

1. Register a circuit / VK under a `zkid` on the chain (see CosmWasm ZK proof verification architecture).
2. Circuit id string used in payloads: `recovery.poseidon_pallas.preimage.v1`.
3. With feature **`zk-host`**, `verify_toy` calls `deps.api.proof_instance_verify`.
4. Without it (default), only structural checks run — suitable for Mock suite CI.

## Replacing the toy with a real Halo2 Poseidon chip

1. Keep the **same public instance**: 32-byte challenge digest (and optional rest).
2. Witness: preimage bytes (or field words after the documented 31-LE map).
3. Constraints: Poseidon sponge / fold matching `poseidon_pallas_hash_once`.
4. Export proof bytes + instances; host path already matches zk-jwt.

This crate does **not** ship proving keys or a full authenticator contract — it documents and tests the pairing surface so recovery and ZK teams share one digest definition.

## Tests

```bash
cargo test -p terp-recovery-poseidon-demo
cargo test -p terp-recovery
```
