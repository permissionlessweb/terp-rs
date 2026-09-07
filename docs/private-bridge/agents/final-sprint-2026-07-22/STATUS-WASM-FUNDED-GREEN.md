# STATUS — wasm + `demo-corridor-ict` green (2026-07-22)

## Result

**`just demo-corridor-ict` → OK ict_local_funded**

Evidence (session log `/tmp/demo-ict9.log`):

- Observe: regtest + `corridor-btc-reporter` → `deposit_observed`
- Mint: ict-rs Terp + Daemon **BridgeMintNote** path (upload/instantiate + mint film)
- Automation: phase `complete` + mint-after-observe receipt
- Banner: `mock_verify=true` (labeled)

## Optimizer / volume build (as requested)

| Step | How |
|------|-----|
| Path libs | Host `cargo build` resolves monorepo `crates/*` path deps (cosmwasm, zcash/halo2, …) |
| Guest graph | `zk-headstash` without `host-crypto` / multicore; `cosmwasm-std` **no** `zk` feature; **`cosmwasm_2_1`** for BLS imports |
| Link | `.cargo/config.toml`: `--allow-undefined` + `-bulk-memory,-sign-ext` |
| Optimize | **Host** `wasm-opt` (binaryen ≥120) with `--llvm-memory-copy-fill-lowering` (optimizer image 116 cannot lower bulk-memory) |
| Recipe | `docs/plans/spectrum/e2e/prepare-corridor-ict-wasm.sh` |
| Just | `cd crates/headstash && just prepare-corridor-wasm-force` / `just demo-corridor-ict` |
| Image (optional bob) | `terpnetwork/workspace-optimizer-arm64:0.17.0` with monorepo → `/workspace`, `PROJECT_DIR=/workspace/crates/headstash` |

## Artifact

```
crates/headstash/contracts/cw-headstash/artifacts/cw_headstash.wasm
crates/headstash/artifacts/cw_headstash.wasm
```

Checks: BridgeMint surface, instantiate/allocate, **no** `proof_instance_verify`, **no** bulk-memory ops.

## Fail-closed journey (what we fixed)

1. Stale pre-BridgeMint wasm  
2. Multicore via zk-cosmwasm → `default-features=false` on halo2_proofs  
3. Guest graph: `host-crypto` optional, `circuit_guest` Instance  
4. `proof_instance_verify` import → drop cosmwasm-std `zk`  
5. BLS PoP instantiate → enable `cosmwasm_2_1`  
6. bulk-memory → host binaryen 126 lowering  

## META

Re-run META for GO on raised bar; prior META was NO-GO solely on unproven S1 mint.
