# Terp Network Test Suite

Date: 2026-05-30

---
 TODO:
- use our blossom clent traits for types fetching from hashmerchant server (replaces manual format url fetch keys)
- spawn anvil and wire in actual ingest and production workflow using hashmerchant
# nostr client tests:
- 
- ensure relayer functionality of chain -> argus -> hash-merchant -> nostr relatyer of dao-calendar event. we do this by using a dao to use a dao-lcalendar module using nip-52 events and confirm dao-calendar events emitted are:
    - picked up by argus formula (indexer queries and caches smart contract data from contract)
    - argus exporter webhook event sent out to hashmerchant relayer (indexer configured via webhook || websocket to do things when events happen )
    - argus can retrieve information from hashmerchant relayers via cid (testing sanity of retrieveing offchain data via cid)
    - hashmerchant relayer relays event to nostr relayer container (hashmerchant relayer is sound and compatible to broader nostr network)


## What this workspace is

`tests/` is the Terp Network integration and end-to-end test workspace. It is a Rust library that provides:

1. **Contract deployment** — a unified `TerpNetworkSuite` that composes cw-orch sub-suites (DAO, SVG, headstash, billboards, shitstraps) and deploys them on any Terp chain
2. **Sidecar lifecycle management** — a fleet of off-chain services (hashmarket, merkle-server, indexer, MinIO, Nostr relay) that run alongside the chain during tests
3. **Two binaries** — `e2e` (full integration) and `delegations` (focused test)

The workspace is a member of the `crates/terp-rs` workspace and is named `scripts` in `Cargo.toml`.

---

## Architecture

```
tests/
├── src/
│   ├── lib.rs                  # Public re-exports
│   ├── environments/           # Test environment primitives
│   │   ├── mod.rs
│   │   ├── nostr.rs            # Nostr relay environment
│   │   └── quickspawn.rs       # Quick chain spawn helpers
│   └── suite/
│       ├── mod.rs              # SidecarFleet — builder for all sidecars
│       ├── sidecar.rs          # TerpSidecar trait + SubprocessSidecar, DockerSidecar, InProcessSidecar
│       ├── contracts.rs        # TerpNetworkSuite — contract deployment
│       ├── deploy_data.rs      # TerpNetworkDeployData — per-chain deployment config
│       ├── hashmerchant.rs     # HashMerchantSuite + MerkleServerSuite
│       ├── relayer.rs          # RelayerSuite (Hermes/IBC)
│       ├── indexer.rs          # IndexerSuite
│       ├── minio_ipfs.rs       # MinioIpfsSuite
│       └── nostr.rs            # NostrRelaySuite
├── bin/
│   ├── e2e.rs                  # Full integration test binary
│   └── delegations.rs          # Delegation-specific test binary
└── Cargo.toml
```

### The core traits

**`TerpSidecar`** — the fundamental abstraction. Every off-chain service implements this:

```rust
#[async_trait]
pub trait TerpSidecar: Send + Sync {
    fn id(&self) -> &SidecarId;
    async fn start(&mut self, deps: &SidecarRegistry) -> Result<EndpointMap>;
    async fn stop(&mut self) -> Result<()>;
    async fn health(&self) -> Result<HealthStatus>;
    fn endpoint(&self, name: &str) -> Option<String>;
}
```

Three concrete implementations live in `sidecar.rs`:

- **`SubprocessSidecar<C>`** — spawns a compiled binary from the local filesystem. Config is provided via `C: SubprocessConfig` which handles binary resolution, argument construction, config file serialization, health-check polling, and port extraction. Used for hashmarket-server, hashmarket-client, merkle-server.
- **`DockerSidecar`** — wraps an `ict_rs::sidecar::SidecarProcess` (Docker container lifecycle via ict-rs). Used for MinIO, indexer, relayer.
- **`InProcessSidecar`** — runs a `tokio::task` in-process. Not currently used but available for lightweight services.

**`SubprocessConfig`** — the configuration trait for subprocess sidecars:

```rust
pub trait SubprocessConfig {
    fn binary_path(&self) -> Result<PathBuf>;
    fn args(&self, config_path: &Path) -> Vec<String>;
    fn env(&self) -> Vec<(String, String)>;
    fn write_config(&self, config_dir: &Path) -> Result<PathBuf>;
    fn health_check(&self) -> Option<(String, String)>;
    fn bind_port(&self) -> u16;
    fn startup_timeout_secs(&self) -> u64;
}
```

Each suite type (`HashMerchantSuite`, `MerkleServerSuite`, etc.) implements `SubprocessConfig` for its config wrapper. The wrappers in `hashmerchant.rs` delegate serialization to the canonical `hash_market::config` types — `Config` for the server, `ClientConfig` for the client. This means the TOML shape defined in `tools/hash-market/src/config.rs` is the single source of truth shared between the binary and the test suite. When the binary's config shape changes, the test suite picks up the change at compile time. No drift.

### The fleet builder

`SidecarFleet` (in `suite/mod.rs`) is the top-level builder. It holds typed `Option` fields for each sidecar and provides builder methods:

```rust
let fleet = SidecarFleet::new("my-test")
    .with_hashmerchant_defaults("terp-test-1")
    .with_merkle_server_defaults()
    .with_minio_ipfs_defaults()
    .with_nostr_relay("my-relay")
    .with_minimal_indexer("argus");

fleet.start_all().await?;
// use fleet.hashmerchant, fleet.merkle_server, etc.
fleet.stop_all().await?;
```

Sidecars start in dependency order via `SidecarRegistry` — already-running sidecars share their endpoints (e.g., the indexer needs the MinIO URL, the relayer needs chain gRPC addresses).

### Contract deployment

`TerpNetworkSuite` (in `suite/contracts.rs`) wraps `cw-orch` sub-suites. It takes a `TerpNetworkDeployData` struct that specifies which suites to deploy:

```rust
pub struct TerpNetworkDeployData {
    pub cw_infuser: Option<InfuserDeployData>,
    pub shitstraps: Option<ShitstrapDeployData>,
    pub dao: Option<DaoDeployData>,
    pub terp_billboards: Option<AdminConfig>,
    pub zk: Option<ZkDeployData>,
}
```

Each field is `Option` — the suite only deploys what is `Some`. Tests that don't need SVG can pass `None` for `cw_infuser`. Tests that need only the DAO can deploy just that.

---

## Config type architecture

The canonical config types live in `tools/hash-market/src/config.rs`:

```rust
pub struct Config {
    pub bind: String,
    pub chain_id: String,
    pub signing_key: String,
    pub data_dir: Option<String>,
    pub providers: Vec<ProviderConfig>,
}

pub struct ProviderConfig {
    pub name: String,
    pub chain_uid: String,
    pub algo: String,
    pub mode: String,
    pub address: String,
    pub interval_secs: u64,
}

pub struct ClientConfig {
    pub eth_rpc: String,
    pub sidecar_url: String,
    pub runtime_id: String,
    pub chain_uid: String,
    pub interval_secs: u64,
    pub account_address: String,
    pub storage_keys: Vec<String>,
}
```

The test suite's `MerchantSuiteConfig` and `MerchantClientSuiteConfig` are thin wrappers around these. They add `binary_path: Option<PathBuf>` for subprocess resolution but delegate TOML serialization to `toml::to_string(&self.inner)` — the canonical types are serialized directly.

This means: if you change `ProviderConfig.interval_secs` in the library, the test suite automatically uses the new field. No duplicate definitions, no silent drift.

`MerkleServerConfig` is not yet unified — it lives in `tests/src/suite/hashmerchant.rs` because it corresponds to a different binary (`tools/merkle-server`) not in the hash-market crate. This is intentional — the rule is: canonical types belong in the library crate that owns the binary.

---

## How to run tests

### Prerequisites

```sh
# 1. Build the sidecar binaries
cargo build --bin hash-market-server -p hash-market --features "server,ve,blossom"
cargo build --bin hash-market-client -p hash-market --features "client"
cargo build --bin merkle-server -p merkle-server  # if merkle-server is in workspace

# 2. Docker (for docker feature — MinIO, indexer, relayer)
docker images | grep terpnetwork/terp-core  # must have a local image

# 3. mc (MinIO client) for website uploads in e2e
which mc || brew install minio/stable/mc
```

### Running the e2e binary

```sh
# Full integration — two chains, relayer, all sidecars, contract deployment
cargo run --bin e2e --features docker,nostr

# Without Docker features (subprocess sidecars only, no containers)
cargo run --bin e2e --features nostr --no-default-features
```

The `e2e` binary:
1. Cleans up any stale Docker containers from previous runs
2. Spawns two Terp chains (`terp-test-1`, `terp-test-2`) and a Hermes relayer via ict-rs
3. Deploys the contract stack (DAO, SVG, headstash, billboards, shitstraps) via cw-orch
4. Patches `websites/terp.network/public/config.json` with deployed contract addresses from `~/.cw-orchestrator/state.json`
5. Starts the sidecar fleet (MinIO, merkle-server, hashmarket, indexer, nostr-relay)
6. Uploads `websites/terp.network/dist/` to MinIO via `mc cp`
7. Prints all deployed contract addresses
8. Waits for Ctrl+C — **no teardown until signal**

The binary uses a dedicated single-thread `tokio::Runtime` (not `#[tokio::main]`) to avoid conflicts with cw-orch's own lazy static runtime.

### Running unit/integration tests

```sh
# All tests
cargo test -p scripts

# Without Docker (subprocess only)
cargo test -p scripts --no-default-features --features nostr

# Specific test
cargo test -p scripts hashmerchant -- --nocapture
```

### Running the delegations binary

```sh
cargo run --bin delegations --features docker,nostr
```

---

## Feature flags

| Flag | Default | Description |
|------|---------|-------------|
| `docker` | yes | Enable Docker sidecars (MinIO, indexer, relayer). Requires Docker daemon and `terpnetwork/terp-core` image |
| `nostr` | yes | Enable Nostr relay suite |

Default features: `docker, nostr`

Disabling `docker` means sidecars that normally run as containers (`minio_ipfs`, `indexer`, `relayer`) are unavailable — `SidecarFleet` builder methods for them will panic or silently skip. The subprocess sidecars (`hashmerchant`, `merkle-server`) work without Docker.

---

## Current state and known gaps

### Config type unification (hash-market)

**Done.** `Config`, `ProviderConfig`, `ClientConfig` are canonical and shared. The five duplicate structs from the previous version have been removed.

**Not done:**
- `MerkleServerConfig` is still defined in the test suite, not unified with the binary's config. The comment explains why (separate crate), but it's the same drift risk that was fixed for hash-market.
- `with_defaults()` uses an empty `signing_key: Default::default()`. The binary parses this as a hex string — an empty string decodes to an empty byte vector and fails at `SigningKey::from_bytes(...)` with a runtime error. The test will fail at server startup. This is not a compile-time safety net.
- `MerkleServerConfig::with_new_keypair()` is commented out — it returns default/empty values. Any test that calls `MerkleServerSuite::with_defaults()` (which calls `with_new_keypair()`) gets an empty public key and the server will reject signature verification.

### ProviderStatus integration gap

`HashMerchantSuite` can query vote extensions via `vote_extension(chain_uid)` → `GET /ve/vote-extension?chain_uid=...`. It cannot query provider runtime state (is provider X running? what was the last foreign_height?).

`ProviderStatus` (runtime state) and `ProviderConfig` (TOML shape) are different types with no shared structure. The test suite only holds `ProviderConfig` (from `Config`). If a test needs to assert on `ProviderStatus` fields, it has no path — `feed_provider()` is a no-op stub and there is no `GET /ve/providers` call wired into the suite.

### Binary resolution

`SubprocessConfig::binary_path()` searches well-known relative paths:

- `hash-market-server`: `../target/debug/hash-market-server`, `../target/release/hash-market-server`, `../../tools/hash-market/target/debug/hash-market-server`
- `hash-market-client`: `../../target/debug/hash-market-client`, `../../target/release/hash-market-client`
- `merkle-server`: `../../target/debug/merkle-server`, etc.

If binaries are built with a different path scheme, resolution fails and the test errors at `binary_path()`.

---

## Adding a new sidecar

1. Define a config struct that implements `SubprocessConfig` (or `DockerSidecar` / `InProcessSidecar`)
2. Define a suite struct that holds the sidecar and implements `TerpSidecar`
3. Add a `with_*_defaults` builder method to `SidecarFleet`
4. Add the field to `SidecarFleet` and wire `start_all` / `stop_all`

The pattern for a subprocess sidecar:

```rust
pub struct MySuiteConfig {
    pub inner: hash_market::config::Config,  // or your canonical type
    pub binary_path: Option<PathBuf>,
}

impl SubprocessConfig for MySuiteConfig {
    fn binary_path(&self) -> Result<PathBuf> { /* resolve */ }
    fn args(&self, config_path: &Path) -> Vec<String> { vec!["-c".into(), config_path.to_string_lossy().into()] }
    fn write_config(&self, config_dir: &Path) -> Result<PathBuf> {
        let path = config_dir.join("config.toml");
        std::fs::write(&path, toml::to_string(&self.inner)?)?;
        Ok(path)
    }
    fn health_check(&self) -> Option<(String, String)> { Some(("GET".into(), "/health".into())) }
    fn bind_port(&self) -> u16 { /* parse from config */ }
    fn startup_timeout_secs(&self) -> u64 { 30 }
}
```