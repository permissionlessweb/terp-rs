# Harness run attestations

Live authenticity is **not** proven by default CI. After:

```sh
cargo test -p terp-scripts --test ibc_multihop_harness -- --ignored --nocapture
```

file a short note here (or as CI artifact) with:

- git commit SHA
- Docker image repo:tag (and digest if available)
- date / host
- scenario 1–6 pass/fail table
- any AUTH_MISMATCH logs

Without an attestation, treat multi-hop packet authenticity as **unproven**.
