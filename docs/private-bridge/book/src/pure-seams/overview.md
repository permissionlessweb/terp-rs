# Pure seams — overview

These are **pure Rust crates** under `docs/plans/spectrum/fixtures/`. They encode policy without Docker or wasmd — the “circuit / manifold” layer for tests and SSOT checks.

| Crate | Shell | Role | How to test |
|-------|-------|------|-------------|
| `cashapp_zec_corridor` | [fixture](fixture-cashapp.md) | DepositIntent I1–I6 + corridor sim | `cd fixtures/cashapp_zec_corridor && cargo test` |
| `bridge_auth_seams` | [fixture](fixture-bridge-auth.md) | Bridge mint authorize policy | `cd fixtures/bridge_auth_seams && cargo test` |
| `private_dex_seams` | [fixture](fixture-private-dex.md) | Oracle-bound swap policy | `cd fixtures/private_dex_seams && cargo test` |
| `compose_seams` | [fixture](fixture-compose.md) | Mint → SEAM → swap pure | `cd fixtures/compose_seams && cargo test` |
| `seam_note_out` | [fixture](fixture-seam-note.md) | SeamNoteOutV0 + notes client helpers | `cd fixtures/seam_note_out && cargo test` |

Paths above are relative to `docs/plans/spectrum/`.

## Non-goals

Higher layers ([cw-headstash](../contracts-circuit/cw-headstash.md), [ict-rs](../harness-ict/ict-rs.md), UI) **must not invent** a second note format or merge asset-map vs intent binds (D1/D2). Pure crates do not own funded nets, wasmd hosts, or operator film paths.

Companion commands:

```bash
cd crates/headstash && just demo-e2e-l0
cd crates/headstash && just demo-corridor-lab   # pure + harness + notify
```

**Next:** [cashapp_zec_corridor](fixture-cashapp.md)
