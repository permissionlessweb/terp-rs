# gen-tools

**Unified Rust runtime for CosmWasm contract type generation.**  
Part of the [Terp Network](https://terp.network) ecosystem.

gen-tools is a single binary + library that drives a configurable pipeline of
code generators against one or more CosmWasm workspace roots. Each step reads
contract JSON schemas (produced by `cargo schema`) and emits typed output in a
target language or format.

---

## Installation

```bash
# From source (within the terp-rs monorepo)
cd crates/terp-rs/tools/gen-tools
cargo install --path .

# Or run directly
cargo run -- --help
```

Requires Rust 1.75+ and a CosmWasm workspace with `cosmwasm-std` dependencies.

---

## Quick Start

### Direct mode — single workspace

```bash
# Current directory — runs full default pipeline
gen-tools

# Specific workspace with step filter
gen-tools ./contracts/my-contract --steps openapi,readme --skip-schema
```

### Config mode — multi-project monorepo

Place a `gen-tools.yaml` at your monorepo root:

```yaml
projects:
  - name: cw-infuser
    path: crates/cw-infuser

  - name: dao-contracts
    path: crates/dao-contracts
    proto_modules: terp,cosmos,ibc
```

Then run:

```bash
# All projects
gen-tools --config gen-tools.yaml --all

# Single project
gen-tools --config gen-tools.yaml --project cw-infuser

# With step filter
gen-tools --config gen-tools.yaml --all --steps schema,ts-codegen,ts-bundles
```

Each project `path` is resolved relative to the config file's directory.

---

## Generated Outputs

All generated types are placed under `terp-api/<lang>/` within each workspace
root.  For a project at `crates/cw-infuser`, the output layout looks like this:

```
crates/cw-infuser/
  terp-api/
    ts/              ← TypeScript types, clients, bundles
      {Contract}.types.ts
      {Contract}.client.ts           # transact+query client class
      {Contract}.message-composer.ts
      {Contract}.zod.ts              # Zod validation schemas
      {Contract}.bundle.mjs          # lightweight ESM JS bundle
      {Contract}.filters.ts          # websocket event filter table
      {Contract}.argus.ts            # Argus indexer formulas
    go/              ← Go struct types
    zod/             ← Standalone Zod schemas
    proto/           ← Protobuf definitions
    python/          ← Python dataclasses
    openapi/         ← OpenAPI 3.0 specs (JSON + YAML)
    tz/              ← TensorZero function specs
    trailmark/       ← Semantic code graphs
      structural/    # Trailmark AST hierarchy
      githem/        # Githem semantic dependency graph
      qmd-probe.json # QMD knowledge base probe result
```

### Using the generated files

**TypeScript types + Zod validation (recommended workflow):**

```typescript
// Types and client — strong typing at compile time
import { CwInfuser, CwInfuserClient } from './terp-api/ts';

// Zod schemas — runtime validation
import { CwInfuserSchemas } from './terp-api/ts';
const parsed = CwInfuserSchemas.ExecuteMsg.Create.parse(data);
```

**Lightweight ESM bundle (no bundler needed):**

```javascript
// Direct import in browser or Node ESM — no TypeScript, no bundler
import { createApi } from './terp-api/ts/CwInfuser.bundle.mjs';

const api = createApi('cosmwasm14...');
const result = await api.query.config({});
const tx = await api.execute.transfer({ recipient: '...', amount: '1000' });
```

**WebSocket event filters:**

```typescript
import { FilterTable, contractFilter } from './terp-api/ts/CwInfuser.filters';

// Subscribe to all events for a contract
const ws = new WebSocket('wss://rpc.terp.network/websocket');
ws.send(JSON.stringify({
  jsonrpc: '2.0',
  method: 'subscribe',
  params: { query: FilterTable.contract(contractAddress) }
}));

// Filter for specific execute actions
ws.send(JSON.stringify({
  jsonrpc: '2.0',
  method: 'subscribe',
  params: { query: FilterTable.execute.Create + " AND " + contractFilter(addr) }
}));
```

**Go types (Cosmos SDK compatible):**

```go
import "github.com/terpnetwork/terp-core/crates/cw-infuser/terp-api/go"

func handle(msg cw_infuser.ExecuteMsg) {
    switch v := msg.(type) {
    case cw_infuser.ExecuteMsg_Create:
        // v.Create is a *ExecuteMsgCreate
    }
}
```

### Output directory overrides

Each step's output directory can be overridden per-project in `gen-tools.yaml`
or via CLI flags.  Without overrides, the defaults resolve to
`{workspace_root}/terp-api/<lang>`.

---

## Pipeline Steps

Each step is a self-contained generator implementing the `Generator` trait.
Steps run in the order listed below.  Use `--steps <name1,name2,...>` to select
specific steps, or `--steps default` for the full default set.

| Step | Default | Description | Output dir |
|------|---------|-------------|------------|
| `schema` | default | Runs `cargo schema` on each contract to produce JSON schema files. | `{contract}/schema/*.json` |
| `ts-codegen` | default | Generates TypeScript types (`*.types.ts`), query/execute clients, and message composers following the **cw-infuser** design pattern. | `terp-api/ts/` |
| `ts-bundles` | default | Generates co-located Zod schemas (`*.zod.ts`), lightweight ESM JS bundles (`*.bundle.mjs`), combined transact+query client (`*.client.ts`), websocket filter tables (`*.filters.ts`), and Argus indexer formulas (`*.argus.ts`). | `terp-api/ts/` |
| `proto` | default | Converts schema types into `.proto` definitions compatible with Cosmos SDK protobuf conventions. | `terp-api/proto/` |
| `python` | default | Generates Python dataclasses with JSON encode/decode methods. | `terp-api/python/` |
| `zod` | default | Generates Zod validation schemas (standalone — also available co-located via `ts-bundles`). | `terp-api/zod/` |
| `go` | default | Generates Go struct definitions with JSON tags, using cosmwasm-go-gen type mappings (Addr→string, Uint128→string, etc.). Handles allOf/anyOf unwrapping. | `terp-api/go/` |
| `readme` | default | Generates API documentation in Markdown format — tables of instantiate/execute/query messages with type annotations. | `docs/api/` |
| `openapi` | default | Generates OpenAPI 3.0 specs (JSON + YAML) with contract endpoints. | `terp-api/openapi/` |
| `trailmark` | default | Invokes external tools: Trailmark (AST hierarchical graph), githem (semantic dependency graph), QMD probe (knowledge base detection). Scans both workspace sources AND generated terp-api types. Writes agent recipe JSON. | `terp-api/trailmark/` |
| `tensorzero` | opt-in | Generates TensorZero function specs, model configs, and recipe/episode definitions. | `terp-api/tz/` |

### Adding a new step

1. Create `src/<name>.rs` implementing the `Generator` trait:

```rust
use crate::{GenerationResult, Generator, config::GenerationContext};

pub struct MyGenerator;

impl Generator for MyGenerator {
    fn name(&self) -> &'static str { "my-step" }
    fn enabled_by_default(&self) -> bool { false }  // opt-in
    fn generate(&self, ctx: &GenerationContext) -> anyhow::Result<GenerationResult> {
        // ... generate files ...
        Ok(GenerationResult {
            name: "my-step",
            success: true,
            files_generated: n,
            output_dir: Some(path),
            message: None,
        })
    }
}
```

2. Register it in `pipeline.rs` by adding to the `generators` vec in `Pipeline::new()`.
3. Add to the default step list in `config.rs` `parse_steps()` if it should run by default.
4. Add a row to the table above.

---

## Config File Reference

Full `gen-tools.yaml` schema:

```yaml
# Optional base path for all project paths. Defaults to the config file's directory.
base_path: ../../..

projects:
  # Required: unique project name (used with --project <name>).
  - name: cw-infuser

    # Required: path relative to base_path (or config file dir if base_path unset).
    path: crates/cw-infuser

    # Optional: override the CLI --steps for this project.
    steps: schema,ts-codegen,ts-bundles,proto,python,zod,go,readme,openapi,trailmark

    # Optional: skip schema generation for this project.
    skip_schema: false

    # Optional: comma-separated proto modules.
    proto_modules: terp,cosmos,ibc

    # Per-step output directory overrides (relative to project root).
    # Defaults: terp-api/{ts,go,zod,proto,python,openapi,tz}
    ts_out: custom/ts/src
    go_out: custom/go-types
    zod_out: custom/zod
    proto_out: custom/proto
    py_out: custom/python
    openapi_out: custom/docs/openapi

    # TensorZero configuration (all optional, relative to project root).
    # Paths resolve to gen-tools own _devops/ subdirectory if unset.
    tz_out: terp-api/tz
    tz_heuristics: ../terp-rs/tools/gen-tools/_devops/tz-heuristics.toml
    tz_recipes_dir: ../terp-rs/tools/gen-tools/_devops/tz-recipes
    tz_episodes_dir: ../terp-rs/tools/gen-tools/_devops/tz-episodes
    tz_profiles_dir: ../terp-rs/tools/gen-tools/_devops/tz-profiles
```

When both CLI flags and per-project fields are set, the project-level value
wins.  Output dirs in the config are relative to the project root; CLI output
dirs are relative to the workspace root positional arg.

---

## CLI Options

```
Usage: gen-tools [OPTIONS] [WORKSPACE_ROOT]

Arguments:
  [WORKSPACE_ROOT]          Workspace root (default: .)

Options:
  --config <PATH>           Path to gen-tools.yaml project config file
  --project <NAME>          Run only this project from the config
  --all                     Run all projects from the config
  -s, --steps <STEPS>       Comma-separated steps
  -o, --output-dir <DIR>    Base output directory root
  --ts-out <DIR>            TypeScript output directory
  --py-out <DIR>            Python output directory
  --zod-out <DIR>           Zod output directory
  --proto-out <DIR>         Proto output directory
  --go-out <DIR>            Go type output directory
  --openapi-out <DIR>       OpenAPI output directory
  --proto-modules <MODS>    Proto modules to generate (default: terp,osmosis,ibc,cosmos)
  --skip-schema             Skip cargo schema invocation
  -v, --verbose             Verbose logging
```

---

## Dependency Management

gen-tools includes a `deps` subcommand for managing crate dependency versions
across the workspace using a dependency matrix:

```bash
# List available dependency profiles
gen-tools deps status

# Switch a crate between profiles (stable, git, local, zk)
gen-tools deps switch cw-infuser --profile git

# Scrape a workspace for dependency info
gen-tools deps scrape

# Show dependency graph for a crate
gen-tools deps graph cw-infuser

# Overview of all crates and their profiles
gen-tools deps overview

# Pull latest dependency versions
gen-tools deps update

# Push dependency snapshot to S3
gen-tools deps push
```

The dependency matrix lives at `_devops/dependency-matrix.toml` in the gen-tools
directory.  Each entry defines source options (stable, git, local, zk) for a
crate, and each workspace consumer selects which profile to use.

---

## Library Usage

gen-tools is also a Rust library.  Use the `Generator` trait and `build_context()`
to embed generation into other tools:

```rust
use gen_tools::config::{build_context, ProjectOverrides};
use gen_tools::pipeline::Pipeline;

let overrides = ProjectOverrides {
    steps: "schema,ts-codegen,openapi".into(),
    skip_schema: true,
    ..Default::default()
};
let ctx = build_context(&workspace_root, &overrides, None)?;
let pipeline = Pipeline::new("default");
let results = pipeline.run(&ctx);
```

---

## Architecture

```
                     ┌─────────────────────┐
                     │     gen-tools        │
                     │   (binary / lib)     │
                     └──────────┬───────────┘
                                │
              ┌─────────────────┴──────────────────┐
              │             config                  │
              │  CLI args / gen-tools.yaml          │
              │  _devops/dependency-matrix.toml     │
              └─────────────────┬───────────────────┘
                                │
              ┌─────────────────┴───────────────────┐
              │           pipeline                  │
              │     step filter + execution order   │
              └──┬──────────┬──────────┬──────────┬─┘
                 │          │          │          │
           ┌─────┴──┐  ┌───┴────┐  ┌──┴────┐  ┌──┴───────┐
           │schema  │  │ts-code │  │ts-bun │  │trailmark │
           │        │  │-gen    │  │-dles  │  │+ githem  │
           └───┬────┘  └───┬────┘  └──┬────┘  └──┬───────┘
               │           │          │          │
         ┌─────┴┐   ┌──────┴──────┐   │    ┌─────┴──────┐
         │cargo │   │ *.types.ts  │   │    │trailmark   │
         │schema│   │ *.client.ts │   │    │structural  │
         │*.json│   │ *.message-  │   │    │githem graph│
         └──────┘   │  composer   │   │    │QMD probe   │
                    │ *.zod.ts    │   │    │agent recipe│
                    │ *.bundle.mjs│   │    └────────────┘
                    │ *.filters.ts│   │
                    │ *.argus.ts  │   │
                    └─────────────┘   │
                               ┌──────┴────────┐
                               │ ts-codegen ->  │
                               │ ts-bundles     │
                               │ (overwrites    │
                               │  .client.ts)   │
                               └───────────────┘
```

`ts-codegen` and `ts-bundles` both produce `.client.ts` — ts-bundles runs after
ts-codegen and overwrites with the enhanced transact+query client class.  All
other files are additive and don't conflict.

---

## Second Wave: LLM Runtime

The `src/llm.rs` module defines the interface for LLM-powered generation targets
(mermaid, indexer, tz-episodes).  Currently a stub that returns placeholders.

- Providers: TensorZero gateway (local port 3000), OpenAI, Anthropic
- Each prompt is constructed from contract schema types
- Output is validated against expected formats

Enable LLM mode by setting `LLM_API_KEY` or configuring a provider.

---

## Related

- [cw-infuser](https://github.com/terpnetwork/terp-core/tree/main/crates/cw-infuser)
  — CosmWasm contract deployment framework whose TS codegen pattern inspired
  the ts-codegen and ts-bundles steps.
- [dao-contracts](https://github.com/terpnetwork/terp-core/tree/main/crates/dao-contracts)
  — DAO framework contracts; `generate_api_markdown()` informed the readme step.
- [O-Line](https://github.com/terpnetwork/o-line)
  — Terp Network deployment orchestrator (Akash phases A-E).
- [Argus indexer](https://github.com/terpnetwork/argus)
  — Event indexing framework consuming the `.argus.ts` formulas.
- [Trailmark](https://github.com/terpnetwork/trailmark)
  — Semantic code graph analysis used by the trailmark step.

---

## License

Apache 2.0 — see [LICENSE](../../LICENSE) in the terp-core repository root.