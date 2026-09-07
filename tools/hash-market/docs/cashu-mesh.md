# Cashu off-chain mesh (hash-market)

Fast **canonical** mint discovery cache + encrypted wallet backup stash for Cashu
e-cash on Terp. This is **not** an “official” mint list and **not** endorsement.

| Plane | Path | Public dual-index? |
|-------|------|--------------------|
| Mint discovery cache | `data/cashu/mints/{mint_id}.json` | Optional descriptor only (not proofs) |
| Encrypted wallet stash | `data/cashu/wallets/{wallet_id}/{item_id}.json` | **Never** public `/content` SSOT |
| Headstash notes | `data/notes/…` | Never (separate product) |

## Schema SSOT

**`MintDescriptor` + `mint_id` rules:**  
[`docs/plans/cashu/CANONICAL-MINT-REGISTRY.md`](../../../../../docs/plans/cashu/CANONICAL-MINT-REGISTRY.md)

Sprint design: [`docs/plans/cashu/SPRINT-CASHU-MESH-2026-07-20.md`](../../../../../docs/plans/cashu/SPRINT-CASHU-MESH-2026-07-20.md)

## HTTP routes

| Method | Path | Auth |
|--------|------|------|
| `PUT`/`POST` | `/cashu/mints/{mint_id}` | notes_auth |
| `GET` | `/cashu/mints/{mint_id}` | public when `public_mint_list` (default) |
| `GET` | `/cashu/mints` | same; `?status=active\|paused\|revoked\|all` |
| `DELETE` | `/cashu/mints/{mint_id}` | notes_auth |
| `PUT`/`POST` | `/cashu/wallets/{wallet_id}/{item_id}` | notes_auth |
| `GET` | `/cashu/wallets/{wallet_id}/{item_id}` | notes_auth |
| `GET` | `/cashu/wallets/{wallet_id}` | notes_auth |
| `DELETE` | `/cashu/wallets/{wallet_id}/{item_id}` | notes_auth |

Auth reuses blossom/`[notes_auth]` (`state.blossom.auth.verify`), same plane as `/notes/*`.

### Config

```toml
[cashu_mesh]
enabled = true
public_mint_list = true   # GET /cashu/mints* without auth
# registry_contract = "terp1…"   # chain SSOT later
# chain_rpc = "http://…"
```

When the section is omitted, mesh routes stay **enabled** with public mint list
(lab-friendly). Set `enabled = false` to hide `/cashu/*`.

## Wallet envelope (opaque)

```json
{
  "ciphertext": "<hex>",
  "nonce": "<hex>",
  "scheme": "xchacha20poly1305",
  "cleartext_layout": "CASHU-TOKEN-BACKUP-V0",
  "cleartext_len": 0
}
```

Server never decrypts. Cleartext is Cashu token/proofs JSON after client decrypt
(layout frozen for mesh; CDK serialization later).

## Naming

| Say | Do not say |
|-----|------------|
| Canonical mint registry / discovery | “Official Cashu mint list” |
| Cashu mint | “the mint” alone (collides with Headstash) |
| Headstash note mint | Cashu issuer |

## Tests

```bash
cargo test -p hash-market --lib cashu --features server
```
