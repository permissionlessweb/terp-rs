# Plan: Terp Scripts Agentic P0 (PR1)

Date: 2026-07-20  
Spec: [../specs/2026-07-20-terp-scripts-agentic-automation-design.md](../specs/2026-07-20-terp-scripts-agentic-automation-design.md)  
Package (current): `scripts` (`tests/`)  
Binary (current): `ibc` (`tests/bin/ibc_info.rs`)

## Goal

Make the next agent session about **running contracts**, not rediscovering the repo:

1. Offline rebuild of derived IBC views from `public/ibc-data`
2. `--format json` + stable exit codes on validate/compare/preflight/rebuild
3. Preflight for live generate (no cw-orch panic on missing env)
4. Atomic `--out` publish
5. `just scripts-ibc-*` + `tests/agent/COMMANDS.md` + README truth pass

**Out of scope for this plan:** package rename, bin rename to `terp-ibc`, default-feature flip, clap catalog generator, skill packaging (P1–P4).

## Success criteria (verify before claiming done)

```sh
# From crates/terp-rs
just scripts-ibc-offline          # or equivalent cargo if just not wired yet
just scripts-ibc-validate

# Direct checks
cargo test -p terp-scripts --lib --test ibc_unit --test ibc_golden --no-default-features
cargo run -p terp-scripts --bin terp-ibc -- preflight --mode offline --format json
# expect: exit 0, {"ok":true,...}

cargo run -p terp-scripts --bin terp-ibc -- validate --format json --out "$(pwd)/public"
# expect: exit 0 or 1 with parseable RunReport; no panic

cargo run -p terp-scripts --bin terp-ibc -- rebuild-from-public --format json --out "$(pwd)/public"
# expect: atomic write; RunReport.artifacts non-empty when ok

# Preflight fail path (unset mnemonic, live-tx or generate preflight)
env -u MAIN_MNEMONIC cargo run -p terp-scripts --bin terp-ibc -- preflight --mode live-tx --format json
# expect: exit 2, ok:false, error missing_env
```

Docs:

- `tests/agent/COMMANDS.md` lists every verb with mode/inputs/outputs
- Root README and `tests/README.md` do not claim `tests/tests/ibc_info.rs` or `tests/public/` as live layout

## Task breakdown

### T1 — RunReport + serde on findings (lib)

**Owns:** `tests/src/report.rs` (new) or `tests/src/ibc/run_report.rs`; wire in `lib.rs` / `ibc/mod.rs`  
**Must not:** rewrite `bin/ibc_info.rs` beyond re-exports

- Add `Serialize`/`Deserialize` to `DiffSeverity`, `DiffItem`, `DiffReport` (or map into serializable DTOs if enum style needs `#[serde(rename_all = "snake_case")]`).
- Define:

```rust
pub struct RunReport {
    pub tool: String,       // "ibc" for P0
    pub verb: String,
    pub ok: bool,
    pub exit_code: i32,
    pub artifacts: Vec<ArtifactRef>,
    pub findings: Vec<DiffItemDto>, // or DiffItem if Serialize
    pub env: RunEnv,
    pub duration_ms: u64,
    pub error: Option<String>,      // machine code e.g. missing_env
    pub vars: Option<Vec<String>>,  // for preflight
    pub hint: Option<String>,
}
```

- Helpers: `RunReport::from_diff(tool, verb, DiffReport, duration)`, `print_text` / `to_json_value`.
- Unit test: round-trip JSON shape matches design example keys.

### T2 — Atomic out + path resolution (lib)

**Owns:** small helper module e.g. `tests/src/ibc/publish.rs` or `tests/src/publish.rs`

- `resolve_out_dir(cli_out: Option<PathBuf>) -> PathBuf`  
  - default: `CARGO_MANIFEST_DIR/../../public` (repo-root `public/`) when called from scripts package  
  - or accept explicit default from bin via `env!("CARGO_MANIFEST_DIR")`
- `AtomicPublisher { out, staging }`  
  - write all files under `out/_staging/<uuid>/`  
  - on success: replace targets under `out/` (ibc-data files, routing, lookup, meta)  
  - on failure: leave staging, do not touch final files  
- Document: never write final artifacts before invariants pass for multi-file publish.

### T3 — CLI globals + preflight + rebuild + JSON (bin)

**Owns:** `tests/bin/ibc_info.rs`  
**Must not:** reimplement hash/route math; call lib

Clap shape (illustrative):

```rust
struct Cli {
    #[arg(long, default_value = "text")]
    format: OutputFormat, // text | json
    #[arg(long)]
    out: Option<PathBuf>,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    require_env_file: bool,
    #[command(subcommand)]
    cmd: Option<Commands>,
}

enum Commands {
    Generate { #[arg(long, default_value = "live-query")] mode: Mode },
    Validate { /* public_dir deprecated alias of --out */ },
    Compare { snapshot, strict },
    Preflight { mode: Mode },
    RebuildFromPublic,
}
```

- Map legacy `--public-dir` to `--out` for one PR if needed.
- `preflight`: check mode requirements (see table in spec); no network.
- `rebuild-from-public`: load ibc-data from `--out`, rebuild routing/lookup via existing pure APIs (`PredictedWorld`, route table builders already used in generate), run `check_invariants`, atomic write if not dry-run.
- All verbs: emit RunReport; `std::process::exit(code)` or return error mapping to exit codes (prefer single place that prints JSON then exits).
- `generate`: call preflight first; on fail exit 2 with JSON; on success keep existing live path but write via AtomicPublisher when feasible (minimal: stage routing+ibc-data together after invariants — if full atomicity is large, at least fail closed before promoting routing after ibc-data).

### T4 — Just recipes

**Owns:** root `justfile` and/or `scripts/just/scripts.just` + import

```make
scripts-ibc-offline:
    cargo test -p terp-scripts --lib --test ibc_unit --test ibc_golden --no-default-features
    cargo run -p terp-scripts --bin terp-ibc -- rebuild-from-public --format json

scripts-ibc-validate:
    cargo run -p terp-scripts --bin terp-ibc -- validate --format json

scripts-ibc-harness:
    cargo test -p terp-scripts --test ibc_multihop_harness -- --ignored --nocapture

scripts-ibc-preflight mode="offline":
    cargo run -p terp-scripts --bin terp-ibc -- preflight --mode {{mode}} --format json
```

If `--no-default-features` breaks lib tests due to cfg, drop that flag in P0 and note in COMMANDS.md; do not block on feature flip.

### T5 — Agent catalog + README truth

**Owns:** `tests/agent/COMMANDS.md`, README files, light touch `docs/tests/ibc_info.md`

COMMANDS.md entries (minimum):

| id | cmd | mode |
|----|-----|------|
| ibc.preflight.offline | `… preflight --mode offline --format json` | offline |
| ibc.validate | `… validate --format json` | offline |
| ibc.compare | `… compare --format json` | offline |
| ibc.rebuild-from-public | `… rebuild-from-public --format json` | offline |
| ibc.generate | `… generate` | live-query |
| ibc.unit | `cargo test -p terp-scripts --test ibc_unit --test ibc_golden` | offline |
| ibc.harness | `cargo test -p terp-scripts --test ibc_multihop_harness -- --ignored` | docker-harness |

Each: inputs, outputs, success rule (exit 0 + no error findings).

README fixes:

- Package `scripts`, path `tests/`
- Real test files: `ibc_unit.rs`, `ibc_golden.rs`, `ibc_multihop_harness.rs`
- Artifacts at repo-root `public/`
- Point agents to `tests/agent/COMMANDS.md`
- Remove dead references to `tests/tests/ibc_info.rs` multichain test as if present

### T6 — Critic / verification (orchestrator)

- Run success criteria commands; paste exit codes
- Confirm JSON parses (`jq .ok`)
- Confirm no partial write: kill mid-rebuild simulation optional; at least unit test AtomicPublisher rollback
- Critic checklist: catalog commands match clap subcommands; README paths exist

## Parallel agent fences (optional)

| Agent | Paths | Forbidden |
|-------|-------|-----------|
| A (lib) | `tests/src/report.rs`, `tests/src/ibc/diff.rs` (serde), `tests/src/ibc/publish.rs`, unit tests | `bin/`, justfile, README |
| B (bin+docs) | `tests/bin/ibc_info.rs`, `tests/agent/**`, just recipes, READMEs | pure hash/route algorithms in `src/ibc/{hash,routes,predict}.rs` |
| C critic | read-only | all writes |

Orchestrator merges, owns `Cargo.toml` if new deps (should be none beyond serde already present).

## Risks

| Risk | Mitigation |
|------|------------|
| rebuild duplicates generate logic bugs | Only use lib `PredictedWorld` / existing builders; validate before promote |
| `--no-default-features` fails | Drop for P0 just target; track in P1 |
| Atomic replace on Windows/NFS | Use rename of files; document POSIX assumption |
| generate atomicity too large for one PR | Preflight + JSON + offline verbs complete; generate atomic as stretch with fail-closed validate at end (already partially true) |

## Order

1. T1 RunReport (unblocks JSON)
2. T2 Atomic helpers
3. T3 CLI verbs
4. T4 just
5. T5 docs/catalog
6. T6 verify

## After merge

Next session: execute **P1** (rename `terp-scripts` / `terp-ibc`) per design PR2, or open Graphite stack PR1→PR2.
