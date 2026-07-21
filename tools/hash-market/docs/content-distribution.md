# Content distribution (BUD + IPFS dual-index)

**Status:** Phase A content plane (opt-in). Default off — mint / VE support unchanged.

## Invariants (frozen)

1. **Primary key:** BUD **sha256** of raw bytes — lowercase 64-char hex everywhere public.
2. **Secondary:** IPFS CID (optional pin via Kubo).
3. **Announce:** `content.available` (sha256, ipfs_cid, urls.bud / urls.ipfs / urls.s3).
4. **Default off:** `[distribution] enabled = false` keeps mint/VE hosts simple.
5. **Single store:** all blob bytes through `TreeStore` (`/f/{sha256}`) — sole `BlobStore`.
6. **Private notes ≠ public content plane:** encrypted claim/bridge note envelopes live only
   under `HeadstashStore` (`notes/{hs_id}/{addr}.json`, auth + PIR). Do **not** put private
   note cleartext (or owner-addressable private bodies) on `GET /content/*`. Calendar
   off-chain `cid` → `/content/*` is the template for **public** off-chain data only.
   See `docs/headstash.md`.

## `cid` encoding (dao-calendar / Nostr off-chain)

| Prefer | Form | Resolve |
|--------|------|---------|
| **Yes (default)** | bare sha256 hex | `GET /content/{sha256}` |
| Optional | `bud:{sha256}` / `sha256:{sha256}` | strip → same |
| Secondary | `ipfs:{cid}` or `ipfs://{cid}` | dual-index or gateway |

**Do not** put bare IPFS CIDs in new `MetadataExt.cid` without `ipfs:` — bare 64-hex is reserved for BUD.

Helpers: `hash_market::content::parse_cid` / `encode_for_chain` / `content_path` /
`bind_offchain_event` / `resolve_local` / `resolve_http` (calendar Phase B).

```http
# Store off-chain body (authenticated BUD)
POST /blobs  → { "sha256": "<64 hex>", "url": "/blobs/..." }

# Chain holds MetadataExt.cid = "<64 hex>"

# Resolve later
GET /content/{sha256}       → bytes (local) or IPFS
GET /content/{sha256}/meta  → { sha256, ipfs_cid, urls }
GET /content/by-ipfs/{cid}  → via dual-index
```

### Calendar off-chain bind (Rust)

```rust
use hash_market::{bind_offchain_event, resolve_local, KIND_TIME_BASED};

let (bind, meta) = bind_offchain_event(&store, body, KIND_TIME_BASED)?;
// meta.cid → MetadataExt.cid on create_event
let body = resolve_local(&store, &meta.cid)?;
// egress: hash_market::metadata_to_nip52_event(...) → relay publish
```

## Config

```toml
[distribution]
enabled = true
ipfs_api = "http://127.0.0.1:5001"
ipfs_gateway_public = "https://ipfs.example/ipfs/"
pin_on_upload = true
pin_required = false
webhooks = ["http://collector:8080/hooks/content"]
webhook_bearer = "optional-shared-secret"

auth_mode = "both"                           # bearer | jwt-plane | both
ingest_bearer = "optional-shared-secret"
# jwt_secret via OLINE_SERVICE_JWT_SECRET preferred
jwt_audience = "hash-market-distribution"
jwt_require_role = "content.ingest"

default_labels = ["public"]
public_get = true
redirect_ipfs_on_miss = false
```

### Plane JWT (oline)

```bash
export OLINE_SERVICE_JWT_SECRET='…'
oline auth jwt service-issue \
  --sub webhook-collector \
  --aud hash-market-distribution \
  --content-ingest
```

## HTTP API

| Method | Path | Auth | Purpose |
|--------|------|------|---------|
| GET | `/content/{sha256}` | public if `public_get` | Resolve: local BUD → IPFS |
| GET | `/content/by-ipfs/{cid}` | public if `public_get` | cid → sha256 → resolve |
| GET | `/content/{sha256}/meta` | public | Dual-index + urls |
| GET | `/content` | public | List registry |
| POST | `/content/register` | ingest auth | Manual dual-index |
| POST | `/webhooks/s3` | ingest auth | MinIO / content.available |
| POST | `/blobs` | blossom auth | BUD upload; pin+announce if distribution on |

## Event shape (`content.available`)

```json
{
  "type": "content.available",
  "sha256": "hex",
  "ipfs_cid": "bafy…",
  "size": 1234,
  "content_type": "application/octet-stream",
  "origin": "bud",
  "bucket": null,
  "key": null,
  "labels": ["public"],
  "urls": {
    "bud": "http://127.0.0.1:9090/blobs/{sha256}",
    "ipfs": "https://ipfs.example/ipfs/bafy…",
    "s3": null
  }
}
```

## Oline / MinIO

1. webhook-collector pins S3 → IPFS and POSTs `content.available` when `HM_DISTRIBUTION_URL` set.
2. MinIO → collector may use static notify token (or Headscale-only network).
3. Collector → hash-market: prefer `HM_AUTH_MODE=jwt-plane`.

## MinioIpfsClient durable root

```rust
MinioIpfsClient::new().with_durable(MinioIpfsDurableConfig {
    local_dir: Some("/data".into()),
    distribution_url: Some("http://127.0.0.1:9090".into()),
    distribution_auth: Some("Bearer …".into()),
});
```

On root confirm: pin optional CID, write JSON under `local_dir/hashmerchant/roots/…`, register sha256 on content plane.

## Smoke (L0)

```bash
cd crates/terp-rs
cargo test -p hash-market --lib content:: --features server
# dual_index_upload_meta_get_smoke + cid + auth
```

## Non-disruption

| Surface | Default |
|---------|---------|
| VE / mint trees | Unchanged when distribution off |
| BUD `/blobs/*` auth | Unchanged |
| Public GET | Only `/content/*` when enabled |

## Layout

```text
{data_dir}/content-registry/by-sha256/{sha256}.json
{data_dir}/trees/f/{sha256}
```

## Phase B gate

Calendar Nostr e2e only after: dual-index smoke green, cid convention documented (this file), no second blob store.
