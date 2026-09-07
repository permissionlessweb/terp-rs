Rebuild

From crates/ with Terp optimizer:

cd /Users/returniflost/abstract/terp-core/crates
docker run --rm \
  -v "$(pwd):/workspace" \
  -e PROJECT_DIR=/workspace/terp-rs \
  --mount type=volume,source=terp-rs_cache,target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  terpnetwork/optimizer-arm64:0.17.0

New checksum

┌─────┬──────────────────────────────────────────────────────────────────┐
│     │                                                                  │
├─────┼──────────────────────────────────────────────────────────────────┤
│ Old │ da7a1843e511c9d16b25d26b373c259d063780da6cd16f66dd0f22bf08dceceb │
├─────┼──────────────────────────────────────────────────────────────────┤
│ New │ 33d506ee481a06a14668bc5f79cc0a1363cf1f3bcc214e82deaf2a69b11f7d33 │
└─────┴──────────────────────────────────────────────────────────────────┘

Updated:
• terp-rs/artifacts/cw_ics08_wasm_crosslink.wasm
• terp-rs/artifacts/checksums.txt
• WASM_CHECKSUM_HEX in tests/tests/crosslink_light_client.rs

Re-run:

cd /Users/returniflost/abstract/terp-core/crates/terp-rs/tests
cargo test -p terp-scripts --test crosslink_light_client -- --ignored --nocapture