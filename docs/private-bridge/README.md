# Private Bridge (product SSOT in terp-rs)

This tree is the **product home** for Cash App → Terp private mint → private DEX → Option D ZEC egress.

Formerly under `docs/plans/spectrum/` (grant layout). **Do not put SSOT back in spectrum.**

## Layout

| Path | Contents |
|------|----------|
| `*.md` (root) | Freezes, SPECs, USER-GUIDE, DEMO |
| `e2e/` | Local multi-net scripts, Zakura, preflight |
| `agents/` | Design/impl STATUS packs |
| `book/` | mdBook |
| `fixtures/README.md` | Pointers to pure crates under `../../crates/` |

## Pure libraries (workspace members)

```text
crates/private_dex_seams
crates/compose_seams
crates/bridge_auth_seams
crates/seam_note_out
crates/cashapp_zec_corridor
```

## Contracts

| Contract | Path |
|----------|------|
| Private DEX settle | `../../contracts/revenue/private-dex` |
| Bridge mint / egress | `crates/headstash` (cw-headstash) — LC-adjacent |

## Commands

```bash
cd crates/terp-rs
cargo test -p private_dex_seams
cargo test -p compose_seams

cd ../headstash
just preflight-corridor-local
just demo-corridor-ict-egress-d
```
