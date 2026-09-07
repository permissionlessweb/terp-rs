# cw-cashu-registry

**Canonical** on-chain discovery registry for Cashu mint URLs and metadata.

This contract is the chain-side source of truth for a **unified mint descriptor schema** on Terp. It is **not** an “official” mint list, **not** an endorsement or reputation system, and **does not** mint Cashu ecash or Headstash notes.

Interface freeze: [`docs/plans/cashu/CANONICAL-MINT-REGISTRY.md`](../../../../../docs/plans/cashu/CANONICAL-MINT-REGISTRY.md)

## What it does

| Capability | Detail |
|------------|--------|
| Register | Admin (or anyone if `allow_public_register`) inserts a `MintDescriptor` |
| Update | Admin or original registrar patches fields |
| Status | Admin sets `active` / `paused` / `revoked` |
| Remove | Soft revoke (tombstone kept for audit) |
| Query | Config, Mint, MintByUrl, ListMints (default: active only), IsRegistered |

### `mint_id`

- Charset: `[A-Za-z0-9._-]{1,200}`, no `..`
- Default: `lowercase_hex(sha256(normalize_url(url)))`
- Explicit override allowed on register (must pass charset)

### Status

| Status | Meaning |
|--------|---------|
| `active` | Discoverable (default list) |
| `paused` | Hidden from default list |
| `revoked` | Tombstone; audit retained |

## Non-goals

- No Cashu blind signatures, proofs, or Lightning
- No Headstash note minting
- No endorsement / reputation scores (v0)
- Wallets must apply their own mint trust (allowlists, WoT, user confirm)

## Build & test

Standalone crate (own workspace root, same pattern as `marketplace-mirror`):

```bash
cd crates/terp-rs/contracts/cashu/cw-cashu-registry
cargo test
```

Schema export (optional):

```bash
cargo run --example schema
```

## Trust note

**Canonical ≠ trusted.** Registration only provides unified discovery under a common on-chain schema. Operators and wallets remain responsible for mint trust.
