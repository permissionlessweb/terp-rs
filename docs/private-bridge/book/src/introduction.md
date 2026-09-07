# Private Bridge — Spectrum Library Book

This book navigates the **Cash App → Private Bridge → ZEC** corridor: product intent, pure seams, contracts, harness, observe plane, UI, and host-floor discipline.

Start with **[Workflows and semantic graphs](workflows/index.md)** for sequence diagrams and Trailmark-derived structure. Library chapters are thin shells that include live monorepo READMEs — not a second copy of the same prose.

## How documentation stays honest

| Mechanism | Purpose |
|-----------|---------|
| **Include mirror** (`scripts/sync_includes.py`) | Build pulls current READMEs/SPECs into `src/_include/` |
| **`LIBRARIES.toml`** | Category registry (product, pure-seams, contracts, harness, …) |
| **Shell pages only** | Role, path, non-goals — no duplicated README bodies |
| **Trailmark + Mermaid/SVG** | Code graphs and workflow sequences under `src/diagrams/` |

```bash
cd docs/plans/spectrum/book
just build     # sync → gen shells → mdbook
just serve     # local review
```

## Product spine

One workflow shape. Three named profiles. Terms used throughout this book:

| Term | Meaning |
|------|---------|
| `domain_bind` | Intent integrity hash under `terp-cashapp-intent-v0` (not Domain B asset-map ids) |
| `bound_only` | Oracle supplies mid/bounds for swap policy; **never** mints |
| `fail-closed` | On re-verify or policy fail, stop as failure — do not continue as success |

```text
wallet / Cash App
  → seal DepositIntentV0 (dest_owner_binding + min_out/slip + domain_bind)
  → fund fresh P2WPKH
  → observe (watches + reporter) [± fail-closed reverify]
  → BridgeMintNote → SeamNoteOutV0
  → oracle-bound private swap (bound_only)
  → open only preauth ZEC dest
```

| Profile | Meaning |
|---------|---------|
| `lab_simulated` | Explicit lab mode; CI/training; banner required; synthetic observe OK |
| `ict_local_funded` | Local multi-net fidelity (`just demo-corridor-ict`) |
| `production` | Same shape against real rails when that deploy is enabled |

Local success is **not** mainnet settlement.

## Category overview

1. **Product** — user + operator narrative  
2. **Design** — freezes and domain SPECs  
3. **Pure seams** — pure Rust fixtures (policy without Docker)  
4. **Contracts & circuit** — headstash / cw-headstash  
5. **Harness & ict-rs** — funded multi-net proof  
6. **Observe & notify** — hash-market, oline play  
7. **UI** — PrivateCorridor  
8. **Zakura** — dest preauth  
9. **Platform / wasmvm** — wasmd-safe guest builds (host image floor)  
10. **Sprint status** — latest epic snapshot  

## Related monorepo index

Spectrum plans index (not included in full): `docs/plans/spectrum/README.md`.

## After the funded sprint

Local proof path (`ict_local_funded` — not mainnet settlement):

```bash
cd crates/headstash && just demo-corridor-ict
```

Wasmd-safe guest rebuild (host floor / binaryen ≥120) when store fails with bulk-memory or unsupported imports:

```bash
cd crates/headstash && just prepare-corridor-wasm-force
```

See:

- [Host floor](platform-wasmvm/host-floor.md) — wasmd/wasmvm MSRV alignment  
- [prepare-corridor-ict-wasm](platform-wasmvm/wasm-prepare.md) — script SSOT  
- [Corridor e2e](harness-ict/e2e-corridor.md) — lab + funded harness  
- [WASM funded green](sprint-status/sprint-wasm-green.md) — sprint evidence  

Category orientation (thin shells, no README paste): [Pure seams](pure-seams/overview.md) · [Contracts & circuit](contracts-circuit/overview.md) · [Observe & notify](observe-notify/overview.md).
