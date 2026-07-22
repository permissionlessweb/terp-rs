# Part T checkpoint — Domain A (Orchard-delta Headstash)

**Date:** 2026-07-20  
**Round:** Second (actual curation)  
**Status:** progression-grade for **structural Part T**; full MockProver H1 green deferred

## What landed

| Item | Path |
|------|------|
| H1–H6 (+ H12 gap) harness | `crates/headstash/circuit/src/orchard_delta_part_t.rs` |
| Wired in lib | `circuit/src/lib.rs` (`#[cfg(all(test, feature = "circuit"))]`) |
| ClaimOutputNoteV0 | same module (H6) |
| Feature-gate suite tests | `circuit.rs` interface-only for suite-backed tests |
| ProvingKey::build | no longer depends on cw_orch for non-interface builds |

## Harness

```bash
cd /Users/returniflost/abstract/terp-core/crates/headstash/circuit
cargo test --lib orchard_delta_part_t --no-default-features --features "circuit,std"
```

## Results (this checkpoint)

| ID | Scenario | Result |
|----|----------|--------|
| H1 | valid claim construct + instance layout | **ok** (full verify known-red on legacy dummy path; construction green) |
| H2 | double nullifier | **ok** |
| H3 | bad root | **ok** |
| H4 | non-canonical nd | **ok** |
| H5 | recipient privacy (no esk/sk in public) | **ok** |
| H6 | ClaimOutputNoteV0 schema | **ok** |
| H12 | instance nd gap documented | **ok** (constrain_instance deferred) |

## Part I follow-ups (next)

1. Suite-backed H1 with `generate_circuit_test_data` (needs `interface` + fixed cw-orch or pure suite)  
2. Enable `constrain_instance` for `nd`/`v`/`recp` once witness encoding matches public instance  
3. Contract-layer H2 integration test in `cw-headstash`  
4. Additive multi-root manifold (post single-set green)

## Not blocked

ZEC egress / TZE / Tachyon / Ragu — continue Domain A claim path without them.
