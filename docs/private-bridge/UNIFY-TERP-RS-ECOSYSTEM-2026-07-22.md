# Unification complete — product SSOT in terp-rs

**Date:** 2026-07-22

Grant-era `docs/plans/spectrum/` is **retired** (pointer README only).

| Surface | Home |
|---------|------|
| Pure seams | `crates/{private_dex_seams,compose_seams,bridge_auth_seams,seam_note_out,cashapp_zec_corridor}` |
| Private DEX settle | `contracts/revenue/private-dex` (crate `private-dex`) |
| Product docs / e2e / agents / book | `docs/private-bridge/` |
| Bridge mint / egress CW | still `crates/headstash` (path-deps pure crates + private-dex) |
| LC | `crates/crosslink/light-client` + `contracts/light-clients/` |
| Observe | `tools/hash-market` |
