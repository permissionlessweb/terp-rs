# Terp-RS: Standard Development Kit for Terp Network

Rust client library for Terp Network. Includes IBC asset derivation, CosmWasm contract deployment, test orchestration, schema-validated output generation, and language-agnostic protobuf definitions.

---

## Workspace structure

```
crates/terp-rs/
├── Cargo.toml                  # Workspace root
├── src/                        # SDK — Cosmos SDK types, IBC proto (terp-rs crate)
├── tests/                      # Test & orchestration suite (scripts crate)
│   ├── src/
│   │   ├── ibc_core.rs         # IBC derivation: denom hashes, channel graph, routing table
│   │   ├── suite/              # SidecarFleet + TerpSidecar + contract deployment
│   │   └── environments/       # Test env primitives (nostr, quickspawn)
│   ├── bin/
│   │   ├── ibc_info.rs         # IBC info pipeline — generates schema-compliant JSON output
│   │   ├── e2e.rs              # Full integration test (Docker sidecars, multi-chain)
│   │   └── delegations.rs      # Delegation-specific test binary
│   ├── tests/
│   │   ├── ibc_info.rs         # 11 schema validation + derivation unit tests
│   │   └── nostr_orch_suite.rs # Nostr relay integration tests
│   ├── public/                 # Generated output + schemas
│   │   ├── asset_list.schema.json
│   │   ├── ibc_data.schema.json
│   │   ├── assetlist.json      # Generated asset list
│   │   ├── ibc_routing_table.json
│   │   └── ibc_lookup_table.json
│   └── README.md               # Full test suite documentation
├── ict-rs/                     # IBC container toolkit — spawn real chain containers
├── tools/                      # Hash-market server/client, merkle-server, etc.
└── cw-orchestrator/            # Fork of cw-orch with Terp-specific patches
```

---

## Quick Start

### IBC info derivation (live chain data → schema-compliant JSON)

```sh
cd crates/terp-rs/tests
cargo run --bin ibc
```

Produces `public/assetlist.json`, `public/{chain-pair}.json` (ibc_data), routing table, and lookup table — all validated against schemas.

### Schema validation tests

```sh
cd crates/terp-rs/tests
cargo test -p scripts --test ibc_info -- --skip test_multichain_ibc_info_routing --nocapture
```

11 tests covering schema compliance, IBC denom hash computation, channel graph routing, and asset validation.

### Multi-chain IBC test (Docker)

```sh
cd crates/terp-rs/tests
cargo test --test ibc_info -- --nocapture --ignored
```

---

## IBC Asset Derivation Pipeline

The `ibc_info` binary connects to live Terp mainnet (and connected chains), queries IBC clients/connections/channels, and generates chain-registry-compliant output:

1. **Query chains** — IBC client states, connections, channels via gRPC
2. **Build ibc_data** — Schema-compliant entries per chain pair
3. **Write JSON files** — `public/{chain_1}-{chain_2}.json` with `$schema` pointer
4. **Build channel graph** — BFS traversal of IBC topology
5. **Premine routing table** — All possible multi-hop routes for all assets
6. **Derive IBC assets** — Denom hashes from trace paths, dedup against native assets
7. **Write assetlist** — Schema-compliant `public/assetlist.json`

Key design: the `ibc_core` module contains all derivation logic as pure functions (SHA256 denom hashing, BFS graph traversal, route computation), making them testable without chain access.

---

## Core SDK types

The `terp-rs` crate at `src/` provides Cosmos SDK protobuf types, IBC light client state decoding, and CosmWasm contract interfaces. Used by the test suite for on-chain data processing.

---

## TODO

- Unify and implement shims for replacing cosmrs/cosmos-sdk-proto libraries
- Wire blossom client traits for hashmerchant types (replaces manual URL format fetch keys)
- Spawn anvil + wire full hashmerchant production workflow
- Ensure relayer functionality: chain → argus → hash-merchant → nostr relay for dao-calendar events