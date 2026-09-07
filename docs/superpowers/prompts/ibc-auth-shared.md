# Shared context — IBC authenticity parallel agents

## Repo root

Work inside the terp-rs workspace (this worktree if isolated). Package: `scripts` at `tests/`.

## Spec (source of truth)

`docs/superpowers/specs/2026-07-20-ibc-data-authenticity-design.md`

## Plan

`docs/superpowers/plans/2026-07-20-ibc-data-authenticity.md`

## Why this exists

Generated IBC JSON is schema-shaped but not authenticity-proven. Known issues:

- Lookup paths sometimes multi-hop via Osmosis even when direct channels exist.
- `hop_count` can disagree with `transfer/` segment count.
- README claims a multichain Docker test that was never implemented.
- Logic lives in a 3.4k-line `tests/bin/ibc_info.rs` plus `tests/src/ibc_core.rs`.

## Authenticity claim

`predict(origin_denom, route) → (trace_path, ibc_denom)` must equal observed DenomTrace / bank denom after real transfer (or fixture truth offline).

## Hard invariants (errors)

1. Channel sides correct under alpha `chain_1`/`chain_2`.
2. `hop_count` == number of `transfer/` segments (consistent with path).
3. Prefer direct ACTIVE preferred channel over longer preferred routes.
4. `ibc/` + uppercase SHA256(path) binding.
5. chain-registry schema compliance.

## Parallel ownership (strict)

| Agent | May edit | Must NOT edit |
|-------|----------|---------------|
| A | `tests/src/ibc/**`, `tests/src/lib.rs`, `tests/src/ibc_core.rs`, `tests/tests/ibc_unit.rs`, `tests/tests/ibc_golden.rs`, `tests/data/ibc/golden/**` | `tests/bin/**`, `tests/tests/ibc_multihop_harness.rs`, `tests/Cargo.toml` (unless absolutely required for compile — prefer document needed lines in your report) |
| B | `tests/tests/ibc_multihop_harness.rs`, `tests/data/ibc/harness/**`, harness-only edits to `tests/README.md` | `tests/src/ibc/**`, `tests/src/ibc_core.rs`, `tests/bin/**`, `tests/Cargo.toml` |

Orchestrator owns CLI thin-out, Cargo.toml merge, generator bugfixes in the binary, and integration.

## Key code references

- Pure hash/graph today: `tests/src/ibc_core.rs`
- Generator + embedded tests: `tests/bin/ibc_info.rs`
- ict-rs TF: `crates/ict-rs/ict-rs/tests/terp_tokenfactory.rs`
- ict-rs IBC transfer ignore-test: `crates/ict-rs/ict-rs/tests/ibc_transfer_test.rs`
- Package deps already include `ict-rs` with docker/terp/testing features

## Engineering rules

- Prefer small focused modules; no drive-by refactors outside your paths.
- TDD where practical; tests must compile.
- Frequent commits on your track only.
- Do not force-push; do not rewrite unrelated history.
- If blocked, leave a clear BLOCKER section in your final report rather than inventing APIs that contradict the design.
- Karpathy-style: surgical, no over-engineering, verifiable success.

## Final report format (required)

1. **Summary** (what shipped)
2. **Files created/modified**
3. **Commands run + results**
4. **Invariants covered / not covered**
5. **Blockers / handoff notes for orchestrator**
6. **Commit SHAs** (if any)
