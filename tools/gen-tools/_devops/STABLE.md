# Dependency control plane — stable surface (v1)

Locked-in workflow for local ↔ remote crate swapping across the monorepo.

## Source of truth

| Artifact | Path | Role |
|---|---|---|
| Config | `_devops/dep-config.toml` | `monorepo_root`, policy flags |
| Matrix | `_devops/dependency-matrix.toml` | Desired state per forked crate |
| Mode state | `_devops/.dep-mode` | Last full switch mode |
| CLI | `_scripts/dep.py` | **Stable entrypoint** |
| Engine | `_scripts/dep-switch.py` + `dep_common.py` | Implementation |

**Monorepo root** defaults to `terp-core/crates` (resolved from config).  
Override with `TERP_DEP_ROOT` / `DEP_MONOREPO_ROOT` if needed.

**Rust `gen-tools deps`** is experimental (incomplete parity). Prefer `dep.py` / just recipes below.

## Daily commands

```bash
cd crates/terp-rs/tools/gen-tools

# Where am I pointed?
just deps-config          # or: python3 _scripts/dep.py config

# Matrix must be clean before switches
just deps-validate

# If validate fails (absolute paths / wrong package dirs):
just deps-heal            # rewrite locals; package-name checked
just deps-heal-dry-run

# Inspect / switch
just deps-status
just deps-local           # path deps for hot rebuilds
just deps-git             # git remotes for push verification
just deps-stable          # crates.io versions
just deps-dev             # git identity + local path patches

# Preview
just deps-switch-dry-run local

# After a switch, prove Cargo agrees (slow — cargo metadata)
just deps-verify local
just deps-switch-verify local
```

## Modes

| Mode | Manifest shape | Use |
|---|---|---|
| `local` / `zk_local` | `path = "..."` in ws-deps/members | Day-to-day editing |
| `git` / `zk_git` | `git = "...", branch = "..."` | Remote verify / push prep |
| `dev` / `zk_dev` | git in ws-deps + `[patch.'url']` → path | Hot swap with remote-shaped deps |
| `stable` | registry versions | Release / downstream fidelity |

ZK is a profile overlay on the same machinery (`zk_*` keys in the matrix).

## Integrity rules (fail-closed)

1. **No absolute paths** in matrix `local` / `zk_local`
2. Path must exist under `monorepo_root`
3. `[package].name` at that path must match matrix `package` (or key)
4. Virtual workspace manifests are not valid package locals
5. `dep.py switch` refuses to run if validate would fail (unless `--force`)

Heal algorithm: index all packages under monorepo root → rewrite broken/absolute locals → prefer non-`vancw/` forks.

## Recommended git workflow

```text
deps validate
deps switch local          # develop
# … commit per-repo as needed …

deps switch git            # or zk_git
deps ensure-branches git   # create missing remote branches
python3 _scripts/dep-graph.py --order   # push order
# push repos in topo order, then:
#   cargo update in consumers
deps verify --mode git
```

## Policy knobs (`dep-config.toml`)

```toml
[policy]
forbid_absolute_paths = true
strict_package_names = true
verify_after_switch = false   # set true to always cargo-metadata after switch
```

## Backup

Pre-heal backup of the matrix (if present):

```text
_devops/dependency-matrix.toml.bak-pre-heal
```

Heal report:

```text
_devops/matrix-heal-report.json
```

## What not to do

- Do not rely on `[patch.crates-io]` alone for cross-workspace fidelity
- Do not run experimental `cargo run -- deps baseline` against a dirty tree without validate
- Do not reintroduce absolute matrix paths
- Do not treat Rust deps CLI as the operator surface until it gains full parity + verify
