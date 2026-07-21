# host ci-core — 20260721T001505Z

| Field | Value |
|-------|--------|
| result | **PASS** |
| exit | 0 |
| duration_s | 56 |
| commit | `8f52a7f` (`8f52a7f172d77d3be90687e7aa2a2898cda8cd44`) |
| cwd | `/Users/returniflost/abstract/terp-core/crates/terp-rs` |
| command | `just ci-core` |

## Recipe coverage

- `scripts-ibc-preflight offline`
- `scripts-ibc-offline` (lib + ibc_unit + ibc_golden + rebuild-from-public)
- `scripts-ibc-validate`
- lib tests: terp-auth, terp-account, crosslink-light-client, cw721-nips, terp-rs

## Notes

This is the **authoritative** local gate. act is optional YAML fidelity only
(`just act-wire` / `just act-core`) and must always tear down containers.
