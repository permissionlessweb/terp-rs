# Revenue contracts (terp-rs)

Product-facing CosmWasm under the **revenue** family.

| Package | Role |
|---------|------|
| `auction/` | NFT auction + royalties |
| `residual-registry/` | Residual / royalty registry |
| **`private-dex/`** *(target home)* | Private swap **settle** — currently developed as `crates/headstash/contracts/cw-private-dex` |

## Private DEX migration (planned)

See `docs/plans/spectrum/UNIFY-TERP-RS-ECOSYSTEM-2026-07-22.md` (terp-core):

1. Path-dep or copy `cw-private-dex` → `contracts/revenue/private-dex`
2. Pure seams → `terp-rs/crates/private-bridge-seams` (or equivalent)
3. Bridge mint/egress stays with headstash / LC neighbors until split

**Do not** treat grant-folder spectrum as long-term package root.
