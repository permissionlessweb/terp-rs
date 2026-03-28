---
title: Tree Types
description: Supported hash and Merkle tree types for cross-chain state onramping
---

# Tree Types

Hashmerchant is hash-algorithm agnostic. The `algo` field in chain registration and vote extensions determines which hash function was used to produce the foreign root. This page maps out the supported and planned tree types and how they relate to ZK-friendly verification.

## The problem hashmerchant solves

Verifying a Merkle proof inside a ZK circuit requires re-implementing the hash function as an arithmetic circuit. The cost varies dramatically:

| Hash | Constraints in R1CS | Practical for ZK? |
|---|---|---|
| Keccak256 | ~150,000 per hash | Painful |
| SHA256 | ~25,000 per hash | Expensive |
| Poseidon | ~300 per hash | Native |
| Pedersen | ~1,500 per hash | Reasonable |

A 20-level Merkle proof over Keccak256 costs ~3M constraints. The same proof over Poseidon costs ~6,000. That is the gap hashmerchant bridges.

## How the bridge works

```
Foreign root (Keccak256)
    │
    ▼
Validator sidecar attests ──► on-chain HashRoot (trusted)
    │
    ▼
Pallas Fp reduction ──► HashPairTicket { origin: keccak, dest: pallas }
    │
    ▼
ZK circuit proves over Pallas field element (cheap)
    │
    ▼
Contract verifies: proof valid + HashPairTicket links to attested root
```

The trust assumption shifts from "prove the hash in-circuit" to "validators attested this root is correct." For chains already secured by Terp validator set overlap, this is equivalent security.

## Ethereum (Keccak256)

**Status:** Implemented

The Ethereum state trie uses Keccak256 for node hashing. The sidecar's `eth` module fetches proofs via `eth_getProof` JSON-RPC.

**Onramp flow:**

1. `hash-market-client` polls an Ethereum RPC
2. Fetches the latest block's `stateRoot` and an account/storage proof
3. The `pallas` module reduces `stateRoot` to a Pallas field element:

   ```
   keccak256(state_root_bytes) mod p_pallas
   ```

4. Both the raw root and the Pallas element are included in the vote extension
5. On-chain, the `HashRoot` stores the raw root; a `HashPairTicket` can link both

**What you can prove:**

- Account balance at a specific block
- Storage slot values (ERC20 balances, governance votes, NFT ownership)
- Contract code existence

**Works with:** Ethereum mainnet, Arbitrum, Optimism, Base, Polygon, any EVM chain with `eth_getProof`

## Cosmos / IBC (SHA256)

**Status:** Planned

Cosmos chains use SHA256 for their IAVL/SMT app hash. The state proof format is ICS-23.

**Onramp approach:**

```rust
// Hypothetical cosmos client module
pub struct CosmosClient { rpc_url: String }

impl CosmosClient {
    pub async fn get_app_hash(&self, height: u64) -> Result<Vec<u8>> {
        // GET /block?height={height} → block.header.app_hash
    }

    pub async fn get_store_proof(&self, store: &str, key: &[u8], height: u64)
        -> Result<Ics23Proof>
    {
        // GET /abci_query?path="/store/{store}/key"&height={height}&prove=true
    }
}
```

SHA256 is cheaper than Keccak256 in-circuit but still expensive. The Pallas reduction applies the same way.

**What you can prove:**

- Bank balances on any Cosmos chain
- Staking delegations
- Governance proposal state
- IBC channel/connection state

## ZK-native chains (Poseidon)

**Status:** Planned

Chains like Mina, Aleo, or Aztec already use ZK-friendly hashes. For these, the sidecar does not need to reduce the hash — it is already circuit-native.

**Onramp approach:**

The sidecar fetches the root directly and the `HashPairTicket` has `origin_hash == destination_hash` (no reduction needed).

```toml
# Chain registration
chain_uid = "mina-mainnet"
hash_algos = ["poseidon"]
```

**What you can prove:**

- Any state from a ZK-native chain, verified at ZK-native cost

## Bitcoin (SHA256d)

**Status:** Planned

Bitcoin uses double-SHA256 (`SHA256(SHA256(x))`) for block headers and Merkle trees over transactions.

**Onramp approach:**

Similar to Ethereum but fetches block headers via Bitcoin RPC (`getblockheader`). The Merkle tree is over transactions, not state — so the proofs verify transaction inclusion rather than account state.

## Adding a new tree type

### Step 1: Client module

Create `src/{chain}/mod.rs` with the RPC interface:

```rust
pub struct MyChainClient { rpc_url: String }

impl MyChainClient {
    pub async fn get_root(&self, height: u64) -> Result<Vec<u8>> { ... }
    pub async fn get_proof(&self, key: &[u8], height: u64) -> Result<Vec<u8>> { ... }
}
```

### Step 2: Feature flag

```toml
# Cargo.toml
my_chain = ["msg", "reqwest", "serde", "serde_json"]
```

### Step 3: Reduction (if needed)

If the chain's hash is not ZK-friendly, add a reduction in `src/pallas/mod.rs` or a new module:

```rust
pub fn my_hash_to_pallas(data: &[u8]) -> PallasLeaf {
    // Hash with the chain's algorithm, then reduce mod Pallas p
}
```

### Step 4: Chain registration

```bash
terpd tx hashmerchant register-chain \
  --chain-uid "my-chain-1" \
  --hash-algos "my_hash_algo" \
  --rpc-endpoints "https://rpc.mychain.com"
```

### Step 5: Client binary or config extension

Either extend `hash-market-client` with a config option for the chain type, or create a dedicated binary.

## Reduction math

The Pallas base field prime:

```
p = 2^254 + 45560315531506369815346746415080538113
p = 0x40000000000000000000000000000000224698fc094cf91b992d30ed00000001
```

Any 256-bit hash `h` is reduced via repeated subtraction (at most 3 iterations since `h < 2^256 < 4p`):

```
while h >= p:
    h = h - p
return h
```

The result is a valid Pallas field element that uniquely represents the original hash within the field. Collision resistance is preserved: two distinct 256-bit hashes can only collide mod `p` if they differ by a multiple of `p`, which is astronomically unlikely for cryptographic hashes.
