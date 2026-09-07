# terp-rs justfile
# Single devops entrypoint for proto management and Rust code generation.
#
# Usage:
#   just setup                        # fetch protos from GitHub and generate
#   just setup /path/to/terp-core     # fetch protos from local clone and generate
#   just sync-protos                  # re-fetch all protos (keeps them fresh)
#   just gen                          # regenerate Rust types from existing protos
#   just fetch-protos [source]        # fetch only terp-core protos
#   just fetch-deps                   # fetch all vendored proto deps (cosmos-sdk, ibc, cosmwasm, gogoproto, cosmos-proto, googleapis)

# ---------------------------------------------------------------------------
# Configuration — override these when running, e.g.:
#   just terp_core_branch=v4.2.0 fetch-protos
# ---------------------------------------------------------------------------

terp_core_url    := "https://github.com/terpnetwork/terp-core"
cosmos_sdk_url   := "https://github.com/cosmos/cosmos-sdk"
cosmwasm_url     := "https://github.com/CosmWasm/wasmd"
ibc_url          := "https://github.com/cosmos/ibc-go"
gogoproto_url    := "https://github.com/cosmos/gogoproto"
cosmos_proto_url := "https://github.com/cosmos/cosmos-proto"
ics23_url        := "https://github.com/cosmos/ics23"
googleapis_url   := "https://github.com/googleapis/googleapis"

terp_core_branch  := "main"
cosmos_sdk_branch := "main"
cosmwasm_branch   := "main"
ibc_branch        := "main"
gogoproto_branch  := "main"
cosmos_proto_branch := "main"
ics23_branch       := "master"
googleapis_branch := "master"

proto_dir  := "proto"
vendor_dir := "proto/vendor"

# ---------------------------------------------------------------------------
# Default: list all recipes
# ---------------------------------------------------------------------------

default:
    @just --list

# ---------------------------------------------------------------------------
# Tool checks
# ---------------------------------------------------------------------------

# Verify required tools are installed
check-tools:
    @command -v protoc  >/dev/null 2>&1 || { echo "ERROR: protoc not found.  Install: brew install protobuf  or  apt install protobuf-compiler"; exit 1; }
    @command -v cargo   >/dev/null 2>&1 || { echo "ERROR: cargo not found.  Install Rust: https://rustup.rs"; exit 1; }
    @command -v git     >/dev/null 2>&1 || { echo "ERROR: git not found."; exit 1; }
    @echo "All required tools found."

# ---------------------------------------------------------------------------
# Proto fetching
# ---------------------------------------------------------------------------

# Fetch terp-core proto files.
# Pass a local terp-core clone path to avoid a network fetch:
#   just fetch-protos /path/to/terp-core
fetch-protos source="":
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{proto_dir}}"

    if [[ -n "{{source}}" ]]; then
        echo ">>> Copying protos from local path: {{source}}"
        if [[ ! -d "{{source}}/proto" ]]; then
            echo "ERROR: {{source}}/proto does not exist."
            exit 1
        fi
        cp -r "{{source}}/proto/." "{{justfile_directory()}}/{{proto_dir}}/"
        echo "Done — copied from {{source}}/proto"
    else
        echo ">>> Fetching protos from {{terp_core_url}} (branch: {{terp_core_branch}})"
        tmpdir=$(mktemp -d)
        trap "rm -rf \$tmpdir" EXIT

        git clone --depth 1 --filter=blob:none --sparse \
            --branch "{{terp_core_branch}}" \
            "{{terp_core_url}}" "$tmpdir/terp-core"

        cd "$tmpdir/terp-core"
        git sparse-checkout set proto
        cp -r proto/. "{{justfile_directory()}}/{{proto_dir}}/"
        echo "Done — fetched terp-core protos"
    fi

# Fetch all vendor proto dependencies needed by terp-core protos.
# Placed in proto/vendor/ and used only as import include paths.
#
# Fetched repos and what they provide:
#   cosmos-sdk    → cosmos/, amino/
#   ibc-go        → ibc/, core/
#   cosmwasm/wasmd → cosmwasm/
#   gogoproto     → gogoproto/gogo.proto
#   cosmos-proto  → cosmos_proto/cosmos.proto
#   ics23         → cosmos/ics23/v1/proofs.proto (IBC commitment dependency)
#   googleapis    → google/api/annotations.proto, google/api/http.proto
fetch-deps:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{vendor_dir}}"
    tmpdir=$(mktemp -d)
    trap "rm -rf \$tmpdir" EXIT
    ROOT="{{justfile_directory()}}"
    VENDOR="$ROOT/{{vendor_dir}}"

    # Clone one sparse subdirectory from a repo.
    # $1=label  $2=url  $3=branch  $4=sparse-path  $5=src-subdir  $6=dest-subdir
    _fetch() {
        local label="$1" url="$2" branch="$3" sparse="$4" src="$5" dest="$6"
        echo ">>> Fetching $label..."
        git clone --depth 1 --filter=blob:none --sparse \
            --branch "$branch" "$url" "$tmpdir/$label" -q
        cd "$tmpdir/$label"
        git sparse-checkout set "$sparse" -q
        mkdir -p "$VENDOR/$dest"
        cp -r "$src/." "$VENDOR/$dest/"
        cd "$ROOT"
    }

    # cosmos-sdk: proto/ → vendor/  (provides cosmos/, amino/, etc.)
    _fetch "cosmos-sdk"   "{{cosmos_sdk_url}}"   "{{cosmos_sdk_branch}}"   "proto"               "proto"               "."

    # ibc-go: proto/ → vendor/  (provides ibc/)
    _fetch "ibc-go"       "{{ibc_url}}"           "{{ibc_branch}}"          "proto"               "proto"               "."

    # cosmwasm/wasmd: proto/ → vendor/  (provides cosmwasm/)
    _fetch "wasmd"        "{{cosmwasm_url}}"       "{{cosmwasm_branch}}"     "proto"               "proto"               "."

    # gogoproto: gogoproto/ → vendor/gogoproto/  (provides gogoproto/gogo.proto)
    _fetch "gogoproto"    "{{gogoproto_url}}"      "{{gogoproto_branch}}"    "gogoproto"           "gogoproto"           "gogoproto"

    # cosmos-proto: proto/cosmos_proto/ → vendor/cosmos_proto/  (provides cosmos_proto/cosmos.proto)
    _fetch "cosmos-proto" "{{cosmos_proto_url}}"   "{{cosmos_proto_branch}}" "proto/cosmos_proto"  "proto/cosmos_proto"  "cosmos_proto"

    # ics23: proto/cosmos/ics23/ → vendor/cosmos/ics23/  (provides cosmos/ics23/v1/proofs.proto — IBC commitment dep)
    _fetch "ics23"        "{{ics23_url}}"           "{{ics23_branch}}"        "proto/cosmos/ics23"  "proto/cosmos/ics23"  "cosmos/ics23"

    # googleapis: google/api/ → vendor/google/api/  (provides annotations.proto, http.proto)
    _fetch "googleapis"   "{{googleapis_url}}"     "{{googleapis_branch}}"   "google/api"          "google/api"          "google/api"

    echo "Done — vendor proto dependencies written to {{vendor_dir}}/"

# Sync (refresh) terp-core protos and all vendor dependencies.
# Pass a local path to avoid cloning terp-core from GitHub:
#   just sync-protos /path/to/terp-core
sync-protos source="":
    just fetch-protos "{{source}}"
    just fetch-deps
    @echo "Proto sync complete."

# ---------------------------------------------------------------------------
# Code generation
# ---------------------------------------------------------------------------

# Generate Rust types from the proto files in proto/.
gen:
    @echo ">>> Generating Rust types from proto files..."
    cargo run --bin proto-gen
    @echo "Generation complete — files written to proto/src/gen/"

# ---------------------------------------------------------------------------
# Clean helpers
# ---------------------------------------------------------------------------

# Remove generated Rust source files.
clean-gen:
    rm -rf proto/src/gen/ proto/src/py_gen.rs
    @echo "Cleaned proto/src/gen/ and py_gen.rs"

# Remove fetched proto files (both terp and vendor).
clean-protos:
    rm -rf "{{proto_dir}}/"
    @echo "Cleaned {{proto_dir}}/"

# Remove everything generated (protos + Rust output).
clean-all: clean-gen clean-protos
    @echo "All generated artifacts removed."

# ---------------------------------------------------------------------------
# Composite workflows
# ---------------------------------------------------------------------------

# Full first-time setup: check tools, fetch protos + deps, generate Rust types.
# Pass a local terp-core path to skip the GitHub clone:
#   just setup /path/to/terp-core
setup source="":
    just check-tools
    just sync-protos "{{source}}"
    just gen

# Wipe generated output and redo from scratch.
regen source="":
    just clean-gen
    just setup "{{source}}"

# ---------------------------------------------------------------------------
# Zod schema generation (TS/JS from prost types)
# ---------------------------------------------------------------------------

# Generate Zod schemas from prost-generated .rs files → ts/zod/
zod modules="terp,osmosis,ibc":
    MODULES={{modules}} ./scripts/gen/prost-to-zod.sh

# Generate Python dataclasses + PyO3 registry → proto/python/ and proto/src/py_gen.rs
py-gen modules="terp,osmosis,ibc,cosmos":
    MODULES={{modules}} ./scripts/gen/prost-to-pyo3.sh

# Build the Python wheel (requires maturin: pip install maturin)
py-build:
    cd proto && maturin build --features python --release

# Install the Python package in editable/dev mode (fast iteration)
py-dev:
    cd proto && maturin develop --features python

# ---------------------------------------------------------------------------
# API client library generation (gen-tools)
# ---------------------------------------------------------------------------

# Generate all API client libraries for all CosmWasm workspaces.
# Runs gen-tools pipeline on every project in gen-tools.yaml.
gen-api:
    cargo run --package gen-api

# Generate API client libraries for a single project.
gen-api-project name:
    cargo run --package gen-api -- --project {{name}}

# Generate only specific pipeline steps (comma-separated).
gen-api-filter filter:
    cargo run --package gen-api -- --filter {{filter}}

# Regenerate schemas + generate all API client libraries.
gen-api-full:
    cargo run --package gen-api

# Full pipeline: Rust types → Zod → Python
gen-all source="": (gen) (zod) (py-gen)

# ---------------------------------------------------------------------------
# scripts package (tests/) — IBC agent entrypoints
# See tests/agent/COMMANDS.md for the full catalog.
# ---------------------------------------------------------------------------

# Offline: pure lib + unit/golden tests, then rebuild derived tables from public/ibc-data
scripts-ibc-offline:
    cargo test -p terp-scripts --lib --test ibc_unit --test ibc_golden
    cargo run -p terp-scripts --bin terp-ibc -- rebuild-from-public --format json

# Offline validate public/ against hard invariants (JSON RunReport)
scripts-ibc-validate:
    cargo run -p terp-scripts --bin terp-ibc -- validate --format json

# Capability preflight.
# Usage: just scripts-ibc-preflight
#        just scripts-ibc-preflight live-query
#        just scripts-ibc-preflight mode=live-tx
scripts-ibc-preflight mode="offline":
    cargo run -p terp-scripts --bin terp-ibc -- preflight --mode {{mode}} --format json
# Docker multihop authenticity harness (ignored; needs Docker + images)
scripts-ibc-harness:
    cargo test -p terp-scripts --test ibc_multihop_harness -- --ignored --nocapture

# ---------------------------------------------------------------------------
# CI parity (see docs/ci.md and .github/workflows/ci-*.yml)
# ---------------------------------------------------------------------------

# Tier 0 — always-on offline gate
# Offline IBC stays serial; internal libs via dense multi-package cargo (see scripts/ci/dense-packs.sh).
ci-core: scripts-ibc-preflight scripts-ibc-offline scripts-ibc-validate
    ./scripts/ci/dense-packs.sh core-libs

# Tier 1 — contracts (dense multi-package) + full non-ignored terp-scripts (no Docker)
# Host default CI_DENSE_MODE=all (one multi-p invocation). GHA uses matrix packs.
# Optional: CI_DENSE_MODE=matrix CI_DENSE_PARALLEL=1 for isolated parallel packs on host.
ci-extended:
    #!/usr/bin/env bash
    set -euo pipefail
    ./scripts/ci/dense-packs.sh contracts-core
    ./scripts/ci/dense-packs.sh contracts-wasm
    cargo test -p terp-scripts --lib --tests

# List dense pack membership (debug / docs)
ci-dense-list:
    ./scripts/ci/dense-packs.sh list

# Host simulation of GHA dense matrix (serial packs, shared target/)
ci-core-matrix:
    #!/usr/bin/env bash
    set -euo pipefail
    just scripts-ibc-preflight offline
    just scripts-ibc-offline
    just scripts-ibc-validate
    CI_DENSE_MODE=matrix ./scripts/ci/dense-packs.sh core-libs

ci-extended-matrix:
    #!/usr/bin/env bash
    set -euo pipefail
    CI_DENSE_MODE=matrix ./scripts/ci/dense-packs.sh contracts-core
    ./scripts/ci/dense-packs.sh contracts-wasm
    cargo test -p terp-scripts --lib --tests

# Tier 2 — expensive (Docker + workspace). Maintainer / nightly only.
ci-heavy:
    cargo check --workspace --tests
    just scripts-ibc-harness

# ---------------------------------------------------------------------------
# Local CI exercise — host first, act optional, always teardown
# See docs/ci.md and scripts/act/README.md
# ---------------------------------------------------------------------------

# Kill act containers/volumes (run when Desktop lags)
act-teardown:
    ./scripts/act/teardown.sh

# Authoritative Tier 0 (no Docker) — timed, writes scripts/act/runs/latest-host-core.md
act-host-core:
    ./scripts/act/run-host-core.sh

# Fast YAML wiring: act -l on ci-*.yml + teardown (no cargo under act)
act-wire:
    ./scripts/act/run-wire.sh

# Full act Core jobs (expensive; prefer act-host-core + act-wire)
act-core:
    ./scripts/act/run-core.sh

# Full act Extended jobs (prefer just ci-extended on host)
act-extended:
    ./scripts/act/run-extended.sh

# Optimized daily loop: teardown → host core → wire → teardown
act-exercise:
    #!/usr/bin/env bash
    set -euo pipefail
    ./scripts/act/teardown.sh
    ./scripts/act/run-host-core.sh
    ./scripts/act/run-wire.sh
    ./scripts/act/teardown.sh
    echo "act-exercise complete — see scripts/act/runs/latest-status.md"

# ---------------------------------------------------------------------------
# Standard cargo helpers (for convenience)
# ---------------------------------------------------------------------------

build:
    cargo build

test:
    cargo test

fmt:
    cargo fmt

clippy:
    cargo clippy --all-targets
