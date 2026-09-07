# Branch labels (permissionlessweb forks)

## Goal

Drop opaque freeze labels (`cw3-base-local-freeze`, `cw3-base-git-freeze`, …).
Use **CosmWasm-aligned** version branches with a **zk** fork suffix and **sub-version**.

## Scheme

```text
v{MAJOR}.{MINOR}.{PATCH}-zk.{N}
```

| Piece | Meaning |
|--------|---------|
| `v{MAJOR}.{MINOR}.{PATCH}` | CosmWasm line this monorepo is built against (see `cosmwasm` workspace version / last upstream tag) |
| `-zk` | permissionlessweb ZK / monorepo fork surface (not upstream CosmWasm main) |
| `.{N}` | fork sub-version: `0` = first cut on that CosmWasm line; bump when fork contracts change without CosmWasm bump |

### Current product line (2026-08)

| | |
|--|--|
| CosmWasm workspace | `3.1.0-dev` (after `v3.0.6` + zk commits) |
| **Canonical branch** | **`v3.1.0-zk.0`** |
| Next CosmWasm-aligned bump | `v3.1.1-zk.0` or `v3.2.0-zk.0` when base line moves |
| Next fork-only bump | `v3.1.0-zk.1` (same CosmWasm base, our surface moved) |

### Allowed long-lived names

| Branch | Role |
|--------|------|
| `v3.1.0-zk.0` | **Default product tip** for monorepo consumers / matrix |
| `main` | Optional mirror of the active `v*-zk.*` tip (convenience only) |
| `feat/*` | Short-lived features (e.g. `feat/calander`) — merge into active `v*-zk.*` |

### Forbidden / retired

| Branch | Status |
|--------|--------|
| `cw3-base-local-freeze` | **Retired** — local freeze checkpoint naming |
| `cw3-base-git-freeze` | **Retired** |
| `cw3-base-local-freeze-evm` | **Retired** → use `v3.1.0-zk.0` (+ feature notes) or `feat/*` |
| `mvp` / `zk-mvp` as *defaults* | Prefer `v*-zk.*`; keep only as historical remotes until cleaned |

## Matrix

```toml
[defaults]
git_branch = "v3.1.0-zk.0"
zk_git_branch = "v3.1.0-zk.0"
```

Per-crate `[crates.*.git].branch` should match the active product line unless a
crate is intentionally pinned older.

## Ops

```bash
# Create product line from current tip (example: cosmwasm)
git checkout -B v3.1.0-zk.0
git push groot v3.1.0-zk.0

# Retire local freeze checkout
git fetch groot
git checkout -B v3.1.0-zk.0 groot/v3.1.0-zk.0   # or groot/main once mirrored
git branch -D cw3-base-local-freeze
```

**Machine basis remains groot.** Create/rename `v*-zk.*` on groot first, then
`just sync-pull` / fetch into lab laptops. Do not reintroduce freeze branches.
