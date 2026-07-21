# Terp Scripts Agentic Automation Layer

Date: 2026-07-20  
Status: draft (ready for PR plan execution)  
Scope: package `terp-scripts` (`tests/` path), binary surface, just recipes, agent catalog  
Related: [IBC Data Authenticity design](./2026-07-20-ibc-data-authenticity-design.md)

## Problem

`terp-rs` is three different things sharing one mental model and burning tokens every agent session:

| Layer | Package / path | Agents think it is | Actually |
|-------|----------------|--------------------|----------|
| SDK | `terp-rs` crate (`crates/sdk`) | “the product” | protos + clients |
| Scripts | package `terp-scripts`, path `tests/` | “tests” | orchestration + bins + lib |
| Harness | `ict-rs` | part of scripts | separate framework |

Observed failure modes (this branch / authenticity work):

1. **Docs lie** — root `README.md` still describes `tests/tests/ibc_info.rs`, old multichain paths, and `public/` under `tests/`.
2. **Bins are monoliths** — `tests/bin/ibc_info.rs` is still ~1.3k lines: orchestrate + query + write; lib is only partially the source of truth.
3. **Env contracts are invisible** — `MAIN_MNEMONIC`, gRPC, Docker image tags fail late (panic inside cw-orch) instead of structured preflight.
4. **Outputs are path-relative and half-published** — default `../public`, UI-adjacent paths, write-before-validate risk of mixed trees.
5. **No machine contract for agents** — exit codes + `DiffReport` exist partly; no catalog of offline vs live verbs; no uniform JSON `RunReport`.
6. **Parallel agents need informal fences** — ownership of `bin/` vs `src/` vs `Cargo.toml` is prompt-only, not catalog-enforced.

Agents work well when every tool answers four questions without reading 3k lines of Rust:

1. **What does this do?** (one verb, stable name)
2. **What does it need?** (env / features / Docker, declared up front)
3. **What does it emit?** (paths + schema + JSON status)
4. **Did it succeed?** (exit code + structured report, not “look at the log”)

Human DX can stay nice; agents need a **contract layer**.

## Goals

1. Stable CLI grammar: `terp-<domain> <verb> [flags]` with global `--format text|json`, `--out`, `--dry-run`, `--require-env-file`.
2. Structured **`RunReport`** for every automation bin (extends existing `DiffReport` findings).
3. Explicit **capability modes**: offline / live-query / live-tx / docker-harness with preflight.
4. Atomic publish: stage → invariants → rename into `--out`.
5. First-class **agent catalog** (`agent/COMMANDS.md` + optional JSON) that orchestrators load first.
6. **Just recipes** as the only human+agent entry for common offline/live/harness flows.
7. Feature flags that make offline lib CI cheap (`--no-default-features` or empty default).
8. Kill README / path drift that causes cargo archaeology.

## Non-goals

- Full workspace monorepo rename of path `tests/` → `terp-scripts/` in P0 (package rename may come in P1; path move is optional if cost is high).
- Replacing ict-rs or folding it into scripts.
- Automatic chain-registry PRs / publishing.
- Another 3k-line binary with more `println!`.
- Silent golden overwrite without audited vs synthetic provenance (keep authenticity design rules).
- Treating “unit tests green” as “mainnet authentic” without a named live mode.

## Current ground truth (2026-07-20)

| Fact | Location |
|------|----------|
| Package name | `scripts` in `tests/Cargo.toml` |
| IBC bin | `cargo run -p terp-scripts --bin terp-ibc` → `tests/bin/ibc_info.rs` (~1307 lines) |
| CLI today | `generate` (default) \| `validate --public-dir` \| `compare --public-dir --snapshot --strict` |
| Pure lib | `tests/src/ibc/*` (`DiffReport`, predict/observe/diff, goldens) |
| Offline tests | `cargo test -p terp-scripts --test ibc_unit --test ibc_golden` |
| Harness | `cargo test -p terp-scripts --test ibc_multihop_harness -- --ignored` |
| Default features | `docker`, `nostr`, `hash-market` (heavy) |
| Public artifacts | repo-root `public/` (not under `tests/public/`) |
| justfile | proto/gen only — no scripts recipes |
| Agent catalog | missing (`agent/` does not exist) |

Authenticity design already landed lib modules + CLI verbs; this design is the **agent/ops contract** on top of that work.

---

## Target shape

### 1. Rename and split the surface (syntax)

Preferred long-term layout (P1+; path move optional):

```
crates/terp-rs/
  crates/sdk/                 # terp-rs (unchanged)
  tests/                      # keep path if migration cost high
    Cargo.toml                # package name: terp-scripts (was scripts)
    src/
      lib.rs
      ibc/                    # pure (exists)
      chain/                  # live gRPC adapters behind trait (P2)
      report/                 # RunReport + serialization
    bin/
      terp_ibc.rs             # bin name: terp-ibc
      # later: terp-e2e, terp-testnet thin wrappers
    agent/
      COMMANDS.md             # single catalog agents always load first
      schemas/
        run_report.schema.json
        artifact_manifest.schema.json
      catalog.json            # optional, generated (P3)
```

**Package rename** (`scripts` → `terp-scripts`) is the high-value rename. Path `tests/` may stay to avoid thrash; docs must say “package `terp-scripts`, path `tests/`”.

Compatibility: keep `[[bin]] name = "ibc"` as alias to `terp-ibc` for one transition window, or document deprecation in COMMANDS.md only (prefer single bin name once just recipes exist).

### 2. One CLI grammar

```text
terp-<domain> <verb> [flags]

# IBC (primary)
terp-ibc generate --mode live-query|offline|from-public
terp-ibc validate --out … --format json
terp-ibc compare --strict --snapshot …
terp-ibc rebuild-from-public   # offline rebuild of derived views from public/ibc-data
terp-ibc preflight --mode live-query

# global flags every automation bin supports
--format text|json          # default text for humans; agents use json
--dry-run
--out <dir>                 # absolute preferred; resolve relative to CWD, never magic ../public
--require-env-file          # fail if required env missing *before* connect
```

Default offline-safe verbs must not touch mainnet. Live verbs preflight:

```json
{
  "ok": false,
  "error": "missing_env",
  "vars": ["MAIN_MNEMONIC"],
  "hint": "export MAIN_MNEMONIC or pass --require-env-file with .env"
}
```

instead of panicking inside cw-orch.

### 3. Structured RunReport

Every bin ends with human text **or** JSON:

```json
{
  "tool": "terp-ibc",
  "verb": "validate",
  "ok": true,
  "exit_code": 0,
  "artifacts": [
    {
      "path": "public/ibc-data/akash-terp.json",
      "sha256": "…",
      "role": "ibc_data"
    }
  ],
  "findings": [
    {
      "severity": "error",
      "code": "prefer_direct",
      "message": "…",
      "path": "lookup/akash/uterp"
    }
  ],
  "env": {
    "mode": "offline",
    "docker": false
  },
  "duration_ms": 120
}
```

Implementation notes:

- Reuse `DiffReport` / `DiffItem` for `findings` (add `Serialize` if missing).
- New type `RunReport` in `src/report/` (or `src/ibc/report.rs` initially) wraps tool/verb/ok/artifacts/env/duration + findings.
- Exit codes:
  - `0` — ok, no error findings
  - `1` — domain failure (invariants, compare mismatch)
  - `2` — preflight / usage / missing env / bad args
  - `3` — internal/unexpected (optional; may map to 1 initially)

Agents parse JSON on stdout (or `--report-file`); humans use `--format text`.

### 4. Capability modes

| Mode | Network | Docker | Mnemonic | Use |
|------|---------|--------|----------|-----|
| `offline` | no | no | no | pure lib, goldens, rebuild-from-public, validate, compare fixtures |
| `live-query` | yes | no | optional / read-only | generate channels from gRPC |
| `live-tx` | yes | no | required | deploys, TF mint, harness funding |
| `docker-harness` | yes | yes | test keys | multihop ignore test |

Gate:

```sh
cargo run -p terp-scripts --bin terp-ibc -- preflight --mode live-query --format json
# exit 2 if cannot run
```

### 5. Atomic publish

```
out/_staging/<run-id>/…
  → check_invariants / schema
  → on success: rename/move into out/ (ibc-data, tables, meta)
  → on failure: leave staging, exit non-zero, no partial out/ tree
```

No more “ibc-data written, routing failed, mixed tree.”

Default `--out` for generate: resolve to repo-root `public/` explicitly (document absolute path in COMMANDS.md), not hard-coded `../public` relative to bin CWD surprises.

### 6. Agent catalog

`tests/agent/COMMANDS.md` (and optional `catalog.json`):

```yaml
- id: ibc.validate
  cmd: cargo run -p terp-scripts --bin terp-ibc -- validate --format json
  mode: offline
  inputs: [public/ibc-data]
  outputs: [RunReport]
  success: exit 0 and findings.errors empty
```

Orchestrator prompts always say: **Read `tests/agent/COMMANDS.md` first.**

### 7. Just recipes as entrypoints

Extend root `justfile` (or `scripts/just/scripts.just` imported):

```make
scripts-ibc-offline:
  cargo test -p terp-scripts --lib --test ibc_unit --test ibc_golden --no-default-features
  cargo run -p terp-scripts --bin terp-ibc -- rebuild-from-public --format json

scripts-ibc-validate:
  cargo run -p terp-scripts --bin terp-ibc -- validate --format json --out public

scripts-ibc-harness:
  cargo test -p terp-scripts --test ibc_multihop_harness -- --ignored --nocapture
```

Agents run **just targets**, not invent cargo flags.

Until package rename: use `-p terp-scripts` in recipes and COMMANDS.md; update both in the same PR as rename.

### 8. Parallel agent packing

Keep what worked (A/B ownership fences + C critic), standardize under `docs/superpowers/`:

```
docs/superpowers/
  specs/
  plans/
  prompts/
    _template-agent.md
    _template-critic.md
  catalogs/                 # optional mirror of agent catalog for planners
```

Rules:

- Exclusive file paths per agent
- Shared prompt with hard “must not edit”
- Critic after merge, not before
- Orchestrator owns `Cargo.toml` + thin bins
- Every agent returns a RunReport JSON path (or test summary path)
- Critic checks catalog commands still match reality (doc drift detector)

### 9. Feature flags for agent builds

```toml
# target
default = []   # or minimal offline
live = ["cw-orch-daemon"]   # when daemon is optionalized
docker = ["ict-rs/docker"]
nostr = []
hash-market = ["dep:hash-market"]
```

Agent offline CI:

```sh
cargo test -p terp-scripts --lib --no-default-features
# or after empty default: cargo test -p terp-scripts --lib --test ibc_unit --test ibc_golden
```

P0 may only document `--no-default-features` where it already works; full default flip is P1 (watch e2e/nostr bins).

### 10. Deprecate ambiguous paths in docs

One pass to kill:

- `cargo run --bin terp-ibc` without `-p`
- references to non-existent test files (`tests/tests/ibc_info.rs`)
- dual `ibc_core` vs `ibc` without “re-export only / legacy” note
- `public/` under `tests/` when artifacts live at repo root

Docs agents load: **COMMANDS.md + package README only**; long architecture stays in `docs/`.

---

## Phased roadmap

| Phase | Outcome | Agent payoff |
|-------|---------|--------------|
| **P0** | `rebuild-from-public`, preflight env, `--format json`, atomic `--out`, fix README drift, `agent/COMMANDS.md`, just `scripts-ibc-*` | Agents refresh correct IBC views without mainnet; contracts runnable |
| **P1** | Rename package → `terp-scripts`, unify clap (`terp-ibc`), `RunReport` type in lib, empty/minimal default features | Stable verbs; less cargo archaeology |
| **P2** | Split live query adapter from pure lib (`LiveQueryBackend` / `src/chain/`) | Offline vs live is explicit in types |
| **P3** | Generate `agent/catalog.json` from clap; CI “docs match catalog” | Stops README lies |
| **P4** | Skill `terp-scripts` for Grok/Claude (“when user says IBC data…”) | One-shot agent onboarding |

### P0 detail (highest leverage slice)

Implement without package rename first (minimize thrash):

1. **`rebuild-from-public`** — pure offline: load `public/ibc-data/*.json` (+ assetlist if present), rebuild routing/lookup (and any derived views currently ad-hoc in Python/scripts), write via staging, validate invariants. Codify what was done manually after authenticity fixes.
2. **Global `--format json` + exit codes** on `validate` / `compare` / `rebuild-from-public` / `preflight` (generate can follow same shape even if still live-only).
3. **`preflight --mode …`** for generate path — check env/files before Daemon connect; emit missing_env JSON, exit 2.
4. **Atomic out dir** for generate + rebuild — `_staging` then promote.
5. **`just scripts-ibc-offline` / `scripts-ibc-validate` / `scripts-ibc-harness`**.
6. **`tests/agent/COMMANDS.md`** + root/package README truth pass.

P0 still uses package name `terp-scripts` and bin `ibc` unless rename is free; COMMANDS.md documents the rename as next step.

### P1 detail

- Package rename `scripts` → `terp-scripts` in `tests/Cargo.toml` + all docs/just/CI references.
- Bin rename `ibc` → `terp-ibc` (path `bin/terp_ibc.rs` or keep `ibc_info.rs` with new `[[bin]] name`).
- `RunReport` + `serde` on findings; shared clap globals via small `cli` module.
- Feature flag default minimization with CI matrix note.

### P2–P4

As table; authenticity design’s `ObserveBackend` already anticipates Live vs Fixture — P2 completes the split so generate body leaves the bin.

---

## What not to do

- Another 3k-line binary with more println
- More parallel agents editing `bin/ibc_info.rs` without path fences
- Treating “tests green” as “mainnet authentic” without a named live mode
- Silent golden overwrite without audited vs synthetic labels
- Inventing cargo flags in agent prompts when a just target exists

---

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Spec before bulk rename | P0 implements contracts on current package name | Agents get value immediately; rename is mechanical P1 |
| Package rename over path move | `terp-scripts` name; keep path `tests/` unless cheap | Path moves break local habits and CI more than Cargo package names |
| RunReport wraps DiffReport | findings = DiffItem[]; add tool/verb/artifacts/env | Reuse authenticity types; avoid dual report systems |
| Exit code 2 for preflight | Distinct from domain fail (1) | Agents can branch “can’t run” vs “ran and failed” |
| Atomic publish | staging → invariants → promote | Prevents mixed public/ trees that poison goldens |
| Just as public API | just recipes call cargo | Stops flag drift across sessions |
| Catalog is source of agent truth | COMMANDS.md first load | Replaces README archaeology |
| Offline default for validate/rebuild | No network | Safe for CI and untrusted agent sandboxes |
| Heavy features off default (P1) | empty/minimal default | Fast lib CI; live/docker opt-in |
| Single domain bin first | `terp-ibc` only in P0/P1 | e2e/testnet stay as-is until grammar proven |

## Alternatives considered

| Alternative | Why not (now) |
|-------------|----------------|
| Implement P0 without a design doc | Already burned sessions on rediscovery; catalog needs a locked shape |
| Full crate path move in P0 | High thrash; package rename delivers most of the naming fix |
| JSON-only CLI | Humans still run bins; dual format is cheap with clap |
| Keep panics for missing env | Agents waste time on stack traces; preflight is explicit |
| Generate catalog from clap in P0 | Needs clap structure + CI hook; do after grammar stabilizes (P3) |
| Fold ict-rs into scripts | Separate lifecycle; harness mode only depends on it |

## Open Questions

1. **Default package name in P0 docs** — Document as `scripts` with “rename to `terp-scripts` in P1”, or rename in the same PR as just recipes?  
   **Recommendation:** document both (`scripts` current / `terp-scripts` target); rename in P1 PR alone for review clarity.

2. **`--out` default** — Always require `--out` for generate, or default to repo-root `public/` via `CARGO_MANIFEST_DIR` join?  
   **Recommendation:** default to absolute path derived from manifest → `../../public` (repo root), print resolved path in RunReport; agents may override.

3. **JSON on stdout vs `--report-file`** —  
   **Recommendation:** JSON on stdout when `--format json`; optional `--report-file` for large artifacts later. Text mode keeps findings on stdout as today.

4. **Bin alias `ibc` vs hard cut** —  
   **Recommendation:** P1 renames to `terp-ibc`; keep `ibc` as second `[[bin]]` pointing at same path for one release, listed as deprecated in COMMANDS.md.

## Success criteria

- Offline agent session can: read COMMANDS.md → `just scripts-ibc-offline` → `just scripts-ibc-validate` → parse RunReport JSON with `ok: true` without mainnet or Docker.
- Live generate either preflights with exit 2 + missing env list, or completes with atomic out + meta.
- Root README and `tests/README.md` do not reference non-existent tests or wrong public paths.
- No new logic in a 3k-line bin without corresponding lib API.

## Dependencies

- Existing: `terp_scripts::ibc` (DiffReport, PredictedWorld, fixtures), clap, serde_json, just.
- Authenticity design modules already present under `tests/src/ibc/`.
- ict-rs only for docker-harness mode / e2e (not for offline catalog verbs).

---

## PR Plan

### PR1 — P0 contracts on `ibc` bin (no package rename)

**Title:** `feat(scripts): agentic IBC CLI contracts (json, preflight, rebuild-from-public)`

**Files / components:**

- `tests/bin/ibc_info.rs` — global flags; verbs `preflight`, `rebuild-from-public`; JSON RunReport emission; exit codes; atomic out
- `tests/src/ibc/` or `tests/src/report.rs` — `RunReport` (+ Serialize on Diff types as needed)
- `tests/src/ibc/*` — pure rebuild helpers if extracted from bin
- `tests/agent/COMMANDS.md` — new
- `tests/agent/schemas/run_report.schema.json` — new (optional but preferred)
- `justfile` or `scripts/just/scripts.just` — `scripts-ibc-offline`, `scripts-ibc-validate`, `scripts-ibc-harness`
- `README.md`, `tests/README.md`, `docs/tests/ibc_info.md` — truth pass

**Dependencies:** none (builds on landed authenticity lib)

**Description:** Highest leverage agent slice. Keep package `terp-scripts` and bin `ibc`. Codify offline rebuild, preflight, JSON reports, atomic publish, just entrypoints, agent catalog, README drift fix.

### PR2 — P1 rename + unified grammar

**Title:** `refactor(scripts): rename package to terp-scripts, bin terp-ibc`

**Files:**

- `tests/Cargo.toml`, workspace members if needed
- bin path/name, all docs/just/CI/COMMANDS references
- optional second bin alias `ibc`
- feature default minimization + CI note

**Dependencies:** PR1

**Description:** Naming fix so agents stop typing `-p terp-scripts` vs “tests folder”. Stabilize `terp-ibc <verb>` grammar and RunReport in lib.

### PR3 — P2 live adapter split

**Title:** `refactor(terp-scripts): LiveQueryBackend out of terp-ibc bin`

**Files:**

- `tests/src/chain/` or `tests/src/ibc/observe.rs` live adapter
- thin `bin/terp_ibc.rs` orchestration only
- unit tests with mock backend

**Dependencies:** PR2

**Description:** Offline builds never pull daemon path for pure verbs; live generate is trait-backed.

### PR4 — P3 catalog generation + drift CI

**Title:** `ci(terp-scripts): generate agent/catalog.json and doc-drift check`

**Files:**

- clap command enumeration or hand-maintained generator
- `tests/agent/catalog.json`
- CI job: catalog commands parse; README does not mention deleted tests

**Dependencies:** PR2 (stable clap)

### PR5 — P4 skill packing

**Title:** `docs(skills): terp-scripts agent skill`

**Files:**

- `.grok/skills/terp-scripts/SKILL.md` (or repo docs skill)
- templates under `docs/superpowers/prompts/`

**Dependencies:** PR1 at minimum; better after PR2

---

## Implementation notes for PR1 (engineers)

Suggested verb behavior:

| Verb | Mode | Network | Writes | Success |
|------|------|---------|--------|---------|
| `preflight` | any | no (checks only) | no | exit 0 if mode satisfiable |
| `validate` | offline | no | no | exit 0 if no error findings |
| `compare` | offline | no* | no | exit 0 if no error findings (*snapshot local) |
| `rebuild-from-public` | offline | no | yes (atomic) | exit 0 + artifacts |
| `generate` | live-query | yes | yes (atomic) | preflight then existing pipeline |

Thin extraction: prefer implementing rebuild/validate report serialization in lib; bin stays match/dispatch.

Parallel agent fences if splitting PR1:

| Agent | Owns | Must not edit |
|-------|------|---------------|
| A | `src/report.rs`, Diff Serialize, rebuild pure helpers | `bin/ibc_info.rs`, Cargo.toml |
| B | `bin/ibc_info.rs` clap + preflight + JSON print | `src/ibc/*` except re-exports |
| C critic | read-only after merge | all write |
| Orchestrator | Cargo.toml, justfile, COMMANDS.md, README | — |

---

## References

- Session review (user): three-layer confusion, principles, P0–P4 roadmap
- [2026-07-20-ibc-data-authenticity-design.md](./2026-07-20-ibc-data-authenticity-design.md)
- `tests/src/ibc/diff.rs` — DiffReport / exit_code
- `tests/bin/ibc_info.rs` — current generate/validate/compare
- `docs/superpowers/prompts/ibc-auth-*.md` — parallel agent fence precedent
