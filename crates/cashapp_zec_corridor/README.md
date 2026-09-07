# cashapp_zec_corridor

Pure L0 fixture: `DepositIntentV0` preauth + oracle bind + `CorridorAssetBackend` (sim / LC stub).

```bash
cd docs/plans/spectrum/fixtures/cashapp_zec_corridor && cargo test
```

**E2E import:** `cashapp_zec_corridor = { path = "docs/plans/spectrum/fixtures/cashapp_zec_corridor" }` then use `DepositIntentV0`, `intent_allows_swap`, `CorridorAssetBackend`, `sim_deposit`.
