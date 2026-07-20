# IBC Data Authenticity Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make IBC script generation authenticity-provable via a shared predict→observe→diff library, golden tests, a thin CLI, and a 4-chain line Docker harness with tokenfactory multi-hop validation.

**Architecture:** Extract pure IBC derivation into `scripts::ibc` with hard invariants; backends observe fixtures/live/harness state; DiffReport fails closed. Offline golden tests and an `#[ignore]` ict-rs 4-chain line harness both use the same predict path.

**Tech Stack:** Rust package `scripts` (`tests/`), `ict-rs` (docker, terp, tokenfactory, testing), `cw-orch` / `cw-orch-interchain` for live generate, `serde_json`, `sha2`, `clap` for CLI.

**Spec:** `docs/superpowers/specs/2026-07-20-ibc-data-authenticity-design.md`

**Prompts:** `docs/superpowers/prompts/ibc-auth-shared.md`, `ibc-auth-agent-a-lib.md`, `ibc-auth-agent-b-harness.md`

---

## Parallelization map

| Track | Owner | Exclusive paths | Depends on |
|-------|--------|-----------------|------------|
| **A — Lib + golden** | Agent A | `tests/src/ibc/**`, `tests/src/lib.rs`, `tests/src/ibc_core.rs`, `tests/tests/ibc_unit.rs`, `tests/tests/ibc_golden.rs`, `tests/data/ibc/golden/**` | None |
| **B — Harness** | Agent B | `tests/tests/ibc_multihop_harness.rs`, `tests/data/ibc/harness/**`, harness sections of `tests/README.md` | Public API names from design; may stub-call `scripts::ibc` or `scripts::ibc_core` |
| **O — Orchestrator** | Human/main agent | `tests/bin/ibc_info.rs` CLI thin-out, `tests/Cargo.toml` merge, integration of A+B, docs full rewrite | A + B complete |

**Do not** let A and B both edit `tests/bin/ibc_info.rs` or `tests/Cargo.toml`. Orchestrator merges Cargo.toml `[[test]]` targets after both land.

---

## File structure (target)

```
tests/src/ibc/
  mod.rs, hash.rs, graph.rs, routes.rs, schema.rs,
  normalize.rs, predict.rs, observe.rs, diff.rs, fixtures.rs
tests/src/ibc_core.rs          # thin re-export of scripts::ibc for back-compat
tests/bin/ibc_info.rs          # thin CLI (orchestrator / later)
tests/tests/ibc_unit.rs
tests/tests/ibc_golden.rs
tests/tests/ibc_multihop_harness.rs
tests/data/ibc/golden/**
```

---

## Track A tasks (lib + golden)

### Task A1: Module skeleton + re-exports

**Files:**
- Create: `tests/src/ibc/mod.rs`, `hash.rs`, `graph.rs`, `routes.rs`
- Modify: `tests/src/lib.rs`, `tests/src/ibc_core.rs`

- [ ] **Step 1:** Create `tests/src/ibc/mod.rs` exporting public API from submodules.
- [ ] **Step 2:** Move `compute_ibc_denom_hash`, graph types, routing table from `ibc_core.rs` into `hash.rs` / `graph.rs` / `routes.rs` without behavior change.
- [ ] **Step 3:** Make `ibc_core.rs` re-export `pub use crate::ibc::*` (or explicit list) so existing `use scripts::ibc_core::...` keeps compiling.
- [ ] **Step 4:** `pub mod ibc;` in `lib.rs`.
- [ ] **Step 5:** Run `cargo test -p scripts --lib` and `cargo check -p scripts --bin ibc`.

### Task A2: normalize + schema + diff types

**Files:**
- Create: `tests/src/ibc/normalize.rs`, `schema.rs`, `diff.rs`

- [ ] **Step 1:** Port `ordering_to_str` and `finalize_channels_for_ibc_entry` from `bin/ibc_info.rs` into `normalize.rs` (public).
- [ ] **Step 2:** Port `validate_asset_entry` / `validate_ibc_data_entry` into `schema.rs`.
- [ ] **Step 3:** Define `DiffSeverity { Error, Warn, Info }`, `DiffItem`, `DiffReport { items, fn has_errors(), fn exit_code() }`.
- [ ] **Step 4:** Unit tests for ordering int→string and alpha side remap (akash < terp).

### Task A3: Hard invariants in predict/routes

**Files:**
- Create: `tests/src/ibc/predict.rs`
- Modify: `tests/src/ibc/routes.rs`, `graph.rs`

- [ ] **Step 1:** Ensure `hop_count` is always set to the number of `transfer/` segments in `trace_path` (or route hop length consistently with path).
- [ ] **Step 2:** When building preferred lookup entries, never mark a multi-hop route preferred if a direct ACTIVE preferred edge exists between origin and dest.
- [ ] **Step 3:** `PredictedWorld` struct holding ibc_data-derived graph, routes, lookup JSON values.
- [ ] **Step 4:** `fn check_invariants(world: &PredictedWorld) -> DiffReport` implementing design hard invariants.
- [ ] **Step 5:** Tests that fail if hop_count lies or preferred multi-hop beats direct.

### Task A4: observe trait + fixture backend

**Files:**
- Create: `tests/src/ibc/observe.rs`, `fixtures.rs`

- [ ] **Step 1:** `trait ObserveBackend { fn observe_denom_trace(...); fn observe_channels(...); }` minimal surface.
- [ ] **Step 2:** `FixtureBackend` loading `tests/data/ibc/golden/**`.
- [ ] **Step 3:** `compare_predict_observe(predict, observe) -> DiffReport`.

### Task A5: Golden data + tests

**Files:**
- Create: `tests/data/ibc/golden/known_hashes.json`, minimal `ibc-data/*.json`, `expected_lookup` snippet
- Create: `tests/tests/ibc_unit.rs`, `tests/tests/ibc_golden.rs`

- [ ] **Step 1:** Pin known Osmosis AKT hash and at least one synthetic multi-hop hash.
- [ ] **Step 2:** Golden test: build from golden ibc-data → predict → assert hop_count/path consistency.
- [ ] **Step 3:** Orchestrator will wire `[[test]]` in Cargo.toml if needed (package may auto-discover `tests/*.rs`).

### Task A6: Commit track A

- [ ] Commit with message like `feat(scripts): extract ibc authenticity lib and golden tests`.

---

## Track B tasks (harness)

### Task B1: Harness design notes under data/

**Files:**
- Create: `tests/data/ibc/harness/README.md`

- [ ] Document chain IDs, TF subdenom matrix, scenarios 1–6, required env (Docker, Terp image), run command.

### Task B2: Multihop harness test (#[ignore])

**Files:**
- Create: `tests/tests/ibc_multihop_harness.rs`

- [ ] **Step 1:** Scaffold `#[tokio::test] #[ignore] async fn test_line_four_chain_tokenfactory_multihop`.
- [ ] **Step 2:** Use ict-rs `Interchain` + Terp config (see `ict-rs` `TestEnv::terp_config`, `terp_tokenfactory` tests, `ibc_transfer_test`).
- [ ] **Step 3:** Spawn A–B–C–D line links; create 4 TF denoms per chain; mint.
- [ ] **Step 4:** For scenarios 1–6: compute predicted denom via `scripts::ibc`/`ibc_core` `compute_ibc_denom_hash` + recorded channels; transfer hop-by-hop; query balance/denom_trace; assert equality.
- [ ] **Step 5:** On any mismatch, panic with structured expected vs actual (path, hash, balance denom).
- [ ] **Step 6:** If full live spawn cannot be completed in-session, leave compilable scaffold with clear `todo!` only behind `cfg` **or** better: implement as far as compile-clean with `#[ignore]` and document blockers for orchestrator — prefer maximal real code over placeholders.

### Task B3: README harness section accuracy

**Files:**
- Modify: `tests/README.md` (harness + how-to-run only; do not invent tests that do not exist)

- [ ] Replace aspirational multichain paragraph with real path `tests/tests/ibc_multihop_harness.rs` and `--ignored` command once file exists.

### Task B4: Commit track B

- [ ] Commit harness + data notes + README harness fixes.

---

## Track O tasks (orchestrator after A+B)

### Task O1: Merge + Cargo.toml

- [ ] Ensure `tests/src/lib.rs` exposes `ibc`.
- [ ] Fix any import breaks in `bin/ibc_info.rs` without full CLI rewrite if timeboxed.
- [ ] Run unit/golden tests.

### Task O2: Thin CLI

- [ ] Add `clap` subcommands `generate|validate|compare` calling lib.
- [ ] Auto-validate at end of generate.

### Task O3: Wire generator bugfixes

- [ ] Prefer-direct and hop_count in live generate path.
- [ ] Replace dead_clients 0..31 with capability filters.
- [ ] `ibc_generation_meta.json`.

### Task O4: Full doc rewrite

- [ ] `tests/README.md`, `docs/tests/ibc_info.md` match reality.

### Task O5: Blind-spot review agent

- [ ] Spawn critic agent with plan + both prompts + design spec; integrate findings into backlog.

---

## Verification commands

```sh
cargo test -p scripts --lib
cargo test -p scripts --test ibc_unit --test ibc_golden
cargo check -p scripts --bin ibc
cargo test -p scripts --test ibc_multihop_harness -- --ignored --nocapture  # Docker
```

## Spec coverage checklist

| Spec item | Task |
|-----------|------|
| predict→observe→diff | A3–A4 |
| hard invariants | A3 |
| thin CLI | O2 |
| golden plane | A5 |
| 4-chain line + TF | B2 |
| hop-by-hop scenarios | B2 |
| generate fail-closed | O2–O3 |
| docs accuracy | B3, O4 |
| no new crate | — |
