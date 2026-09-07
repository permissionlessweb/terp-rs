# Host floor — wasmd / wasmvm alignment

Private Bridge guest contracts (`cw-headstash`) must target the **chain image’s** wasmvm capability floor — not the newest rustc or CosmWasm crate in the monorepo.

Most corridor store/instantiate failures are not broken CosmWasm application logic. They are **host-image wasmvm capability vs guest toolchain MSRV** mismatches: bulk-memory opcodes, forbidden host imports, or feature graphs that pull crypto/sys into the guest.

## Why this chapter exists

During the funded sprint (`just demo-corridor-ict`), store/instantiate failed until the guest matched the host:

| Failure | Cause | Fix |
|---------|--------|-----|
| `bulk memory support is not enabled` | Modern rustc emits bulk-memory; older wasmd rejects | Host **binaryen ≥120** `wasm-opt` with `--llvm-memory-copy-fill-lowering` |
| `unsupported import: env.proof_instance_verify` | cosmwasm-std `zk` feature | Build guest **without** `zk` unless image exports it |
| BLS default-trait panic | Missing `cosmwasm_2_1` host import wrappers | Enable `cosmwasm_2_1` for WAVS PoP |
| Multicore / secp C-sys | Guest pulled host-prove graph | `host-crypto` off; guest `Instance` surface |

**Rule:** the **running chain image** (e.g. `terp-core:local-zk`, wasmd ~0.61.x) is the source of truth. Optimizer MSRV (e.g. rustc 1.86 in workspace-optimizer) must not emit features the host cannot load.

## Monorepo has multiple wasmvm generations

Do not mix pins casually:

- Older wasmvm v1.x / v2.x in various go.mods  
- `crates/zk-wasmd` + `crates/zk-wasmvm` → **wasmvm/v3**  

A guest built for CosmWasm 3 zk-wasmd is not valid by default on an older wasmd host image. Ask: **what does the image I upload to actually export?**

## Wasmd-safe rebuild recipe

SSOT script: [`prepare-corridor-ict-wasm.sh`](wasm-prepare.md)  
(`docs/plans/spectrum/e2e/prepare-corridor-ict-wasm.sh`).

### One-liners (preferred)

From monorepo root / headstash:

```bash
# Force rebuild + install both artifact paths
cd crates/headstash && just prepare-corridor-wasm-force

# Or invoke script directly
FORCE_WASM_REBUILD=1 bash docs/plans/spectrum/e2e/prepare-corridor-ict-wasm.sh

# Funded multi-net demo (calls prepare, then corridor-ict-funded)
cd crates/headstash && just demo-corridor-ict
# equivalent from spectrum plans:
# cd docs/plans/spectrum && just demo-corridor-ict
```

Cached path (no rebuild if artifact already has BridgeMint + CosmWasm exports):

```bash
cd crates/headstash && just prepare-corridor-ict-wasm
```

### What the prepare script does

| Step | Action |
|------|--------|
| 1. Guest cargo | `cargo build -p cw-headstash --target wasm32-unknown-unknown --release` under `crates/headstash` so monorepo path deps resolve on the **host** |
| 2. Feature graph | No `host-crypto` / multicore / secp C-sys; cosmwasm-std **without** `zk`; **`cosmwasm_2_1`** for BLS host imports |
| 3. Link flags | `crates/headstash/.cargo/config.toml`: `--allow-undefined` + `target-feature=-bulk-memory,-sign-ext` |
| 4. Optimize | Prefer **host** `wasm-opt` (binaryen **≥120**) with `--llvm-memory-copy-fill-lowering` (and signext lower when available) |
| 5. Fail-closed | Reject wasm that still has bulk-memory ops (`memory.copy` / `memory.fill`) or lacks BridgeMint / `allocate` / `instantiate` surface |
| 6. Install | `crates/headstash/artifacts/cw_headstash.wasm` **and** `crates/headstash/contracts/cw-headstash/artifacts/cw_headstash.wasm` |

### binaryen ≥120 (required for bulk-memory lower)

The CosmWasm **workspace-optimizer image** often ships **wasm-opt 116**, which **cannot** lower bulk-memory for older wasmd. Install host binaryen:

```bash
# macOS
brew install binaryen
wasm-opt --version   # need ≥120 (sprint used 126)

# Typical lower flags (script tries these)
wasm-opt -Os --enable-bulk-memory --llvm-memory-copy-fill-lowering --signext-lowering \
  target/wasm32-unknown-unknown/release/cw_headstash.wasm -o artifacts/cw_headstash.wasm
```

| Tool | Role |
|------|------|
| Host `wasm-opt` ≥120 | **Preferred** — lowers bulk-memory so wasmd without bulk-memory accepts the module |
| Optimizer image only | **Fallback** — may leave bulk-memory; prepare script fails closed if ops remain |
| Optional bob path | `CORRIDOR_USE_BOB_OPTIMIZER=1` + `terpnetwork/workspace-optimizer-arm64:0.17.0` (still may need host lower) |

Env knobs (see script header):

| Env | Meaning |
|-----|---------|
| `FORCE_WASM_REBUILD=1` | Rebuild even if artifact looks OK |
| `SKIP_OPTIMIZER=1` | Copy-only path (existing valid bridge wasm) |
| `CORRIDOR_OPTIMIZER_IMAGE` | Docker image for fallback wasm-opt / bob |
| `DOCKER_PLATFORM` | e.g. `linux/arm64` / `linux/amd64` |

### Artifact checks (green bar)

After prepare:

```text
crates/headstash/contracts/cw-headstash/artifacts/cw_headstash.wasm
crates/headstash/artifacts/cw_headstash.wasm
```

Expect:

- BridgeMint surface (`BridgeMint` / `bridge_mint_note` / `SetBridgeCfg`)
- CosmWasm exports (`interface_version`, `allocate`, `instantiate`)
- **No** `proof_instance_verify` import
- **No** residual bulk-memory opcodes

## `just demo-corridor-ict`

Full **`ict_local_funded`** path (local multi-net fidelity — not mainnet money):

```bash
cd crates/headstash && just demo-corridor-ict
```

Internally:

1. `prepare-corridor-ict-wasm.sh` (guest + bulk-memory lower)  
2. `corridor-ict-funded.sh` — regtest observe + `corridor-btc-reporter` + ict-rs Daemon `BridgeMintNote` + oracle-bound swap  

Evidence from the funded sprint: [STATUS-WASM-FUNDED-GREEN](../sprint-status/sprint-wasm-green.md).

Dev residual (chain mint only — not full observe → mint → swap exit):

```bash
cd crates/headstash && just demo-corridor-ict-mint-only
```

## SSOT scripts & pins

| Path | Role |
|------|------|
| `docs/plans/spectrum/e2e/prepare-corridor-ict-wasm.sh` | Wasmd-safe build + lower ([book page](wasm-prepare.md)) |
| `docs/plans/spectrum/e2e/corridor-ict-funded.sh` | Funded harness entry |
| `crates/headstash/.cargo/config.toml` | wasm32 rustflags (host imports + no bulk-memory/sign-ext) |
| `crates/headstash/justfile` | `prepare-corridor-wasm-force`, `demo-corridor-ict` |
| [cosmwasm in-tree](cosmwasm.md) | Guest std — align features with host image |

Registry: `LIBRARIES.toml` entries `wasm-prepare`, `cosmwasm` under category `platform-wasmvm`.

## Product profiles

| Profile | Host expectation |
|---------|------------------|
| `lab_simulated` | Mock / synthetic OK; lab banner required |
| `ict_local_funded` | Real local chain + wasmd-safe `cw_headstash.wasm` |
| `production` | Same guest discipline against the **prod** wasmd image |

Local success ≠ mainnet settlement.

## Related book pages

- [Corridor e2e](../harness-ict/e2e-corridor.md) — lab + funded commands  
- [CORRIDOR-LAB-STATUS](../harness-ict/corridor-lab-status.md) — lab vs funded columns  
- [WASM funded green](../sprint-status/sprint-wasm-green.md) — sprint evidence  
- [Introduction](../introduction.md) — product spine + profiles  
