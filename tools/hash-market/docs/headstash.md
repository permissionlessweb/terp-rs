---
title: Headstash Server
description: Privacy note and circuit key server for the MetaMask snap integration
---

# Headstash Server

The headstash-server distributes encrypted notes and ZK circuit keys to the MetaMask snap (`snap-n-pull-js`) with secp256k1 authentication and optional PIR for metadata privacy.

## Quick start

```bash
cd tools/headstash-server

cargo build --release
cp config.example.toml config.toml
mkdir -p data/{headstash,notes,keys}

./target/release/headstash-server -c config.toml
```

## Configuration

```toml
# config.toml
bind = "0.0.0.0:8080"
data_dir = "./data"
```

For JWT auth, set the `JWT_SECRET` environment variable.

## API reference

### Public endpoints

| Endpoint | Method | Description |
|---|---|---|
| `/health` | GET | Liveness check |
| `/headstash/{id}` | GET | Fetch a headstash registration record |
| `/keys` | GET | List available circuit key IDs |

### Authenticated endpoints

All require `X-Auth-Type: secp256k1` headers (or `Authorization: Bearer <jwt>`).

| Endpoint | Method | Description |
|---|---|---|
| `/headstash/{id}` | POST | Register a new headstash |
| `/headstash/{id}/root` | GET | Get the Merkle root for a headstash |
| `/headstash/{id}/sync` | POST | Sync/merge state into a headstash record |
| `/notes/{hs_id}/{addr}` | GET | Fetch an encrypted note for an address |
| `/notes/{hs_id}` | GET | List note keys for a headstash |
| `/notes/{hs_id}/pir` | POST | PIR-fetch a note (hides which address) |
| `/keys/{key_id}` | GET | Download a circuit key |
| `/keys/{key_id}/pir` | POST | PIR-fetch a key (hides which key) |

## Authentication

### secp256k1 (snap client)

The snap derives a secp256k1 key via `snap_getBip32Entropy` (path `m/44'/133'/0'/0'/0'`) and signs requests:

```
msg       = "{unix_timestamp}\n{compressed_pubkey_hex}"
msg_hash  = SHA256(msg)
signature = secp256k1.sign(msg_hash, secret_key)
```

Headers sent:

```
X-Auth-Type: secp256k1
X-Pubkey:    <66-char compressed hex>
X-Timestamp: <unix seconds>
X-Signature: <128-char compact r||s hex>
```

**The server uses `PrehashVerifier::verify_prehash()`**, not `Verifier::verify()`. The client already hashed the message before signing. Using `verify()` would double-hash and reject every request.

### JWT (service-to-service)

```
Authorization: Bearer <HS256 JWT>
```

Secret is read from `JWT_SECRET` env var.

## PIR (Private Information Retrieval)

PIR lets the snap fetch a note or key without the server learning which item was requested.

**Protocol:**

1. Client calls `GET /notes/{hs_id}` to get the list of keys
2. Client builds a selector: `[0, 0, 1, 0, ...]` (1 at the target index)
3. Client POSTs `{ "selector": [0, 0, 1, 0, ...] }` to `/notes/{hs_id}/pir`
4. Server XORs all blobs where `selector[i] == 1` and returns the result
5. Since only one bit is set, the result is the target blob

The server processes every blob regardless of the selector, so it cannot distinguish which item was requested from timing or access patterns.

```
POST /notes/my-headstash/pir
Content-Type: application/json
X-Auth-Type: secp256k1
X-Pubkey: 02abc...
X-Timestamp: 1700000000
X-Signature: deadbeef...

{ "selector": [0, 0, 1, 0] }

→ { "result": "aabbccdd..." }   // hex-encoded XOR result
```

## File storage layout

```
data/
  headstash/
    {id}.json               # Registration records
  notes/
    {hs_id}/
      {addr}.json           # Encrypted notes (ciphertext + nonce + scheme)
  keys/
    {key_id}.bin            # Circuit proving/verifying keys
    {key_id}.blake3         # Integrity hash
```

All IDs are validated against `[a-zA-Z0-9_\-\.]{1,200}` to prevent path traversal. Writes use atomic temp-file-then-rename.

## Populating data

### Headstash records

```bash
# Create a headstash entry
curl -X POST http://localhost:8080/headstash/season-1 \
  -H "Content-Type: application/json" \
  -H "X-Auth-Type: secp256k1" \
  -H "X-Pubkey: 02abc..." \
  -H "X-Timestamp: $(date +%s)" \
  -H "X-Signature: ..." \
  -d '{"root": "aabb...", "total_accounts": 1000}'
```

### Notes

Place encrypted note JSON files directly in the data directory:

```bash
echo '{"ciphertext":"...","nonce":"...","scheme":"xchacha20poly1305"}' \
  > data/notes/season-1/0xabc123.json
```

### Circuit keys

Copy proving/verifying key binaries:

```bash
cp circuit.pk data/keys/poseidon-2.bin
blake3 data/keys/poseidon-2.bin > data/keys/poseidon-2.blake3
```
