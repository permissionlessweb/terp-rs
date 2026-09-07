//! Golden circom-JWT fixtures (gateway-style pyramid).
//!
//! ## L1 (codec / fail-closed)
//! - [`FULL_CIRCOM_PUBLICS_JSON`] — real snarkjs public signals (40 Fr)
//! - [`SPARSE_CIRCOM_PUBLICS_JSON`] — too few publics → fail closed
//!
//! ## L2 (crypto)
//! - `l2/proof.json` + `l2/jwt-auth_vkey.json` + `l2/public.json`
//! - Verify: `node scripts/l2_snarkjs_verify.mjs` (snarkjs groth16.verify)
//!
//! Pattern mirrors o-line gateway fixtures under `auth_plane/fixtures/`.
//!
//! ## Regenerating L2
//! ```text
//! # 1) yarn install in crates/zk-jwt
//! # 2) yarn gen-input --account-code 0x… --input-file …/input.json
//! # 3) download zkey/wasm from GCS demo-18-12-2024 (see CIRCOM_CODEC_PYRAMID.md)
//! # 4) snarkjs.groth16.fullProve → copy proof.json public.json vkey → fixtures/l2/
//! # 5) cp public.json → full_circom_publics.json
//! # 6) node scripts/l2_snarkjs_verify.mjs && cargo test -p terp-zkjwt --lib circom
//! ```

/// Full production jwt-auth public vector (40 signals) from snarkjs fullProve.
/// Indices: 4 = jwtNullifier, 26 = accountSalt (codec v1 claim).
pub const FULL_CIRCOM_PUBLICS_JSON: &str = include_str!("full_circom_publics.json");

/// Sparse-only (too few publics) — must fail decode / soundness.
pub const SPARSE_CIRCOM_PUBLICS_JSON: &str = include_str!("sparse_circom_publics_only.json");

/// L2: snarkjs proof (do not strip).
pub const L2_PROOF_JSON: &str = include_str!("l2/proof.json");
/// L2: verification key.
pub const L2_VKEY_JSON: &str = include_str!("l2/jwt-auth_vkey.json");
/// L2: public signals (same as FULL when regenerated together).
pub const L2_PUBLIC_JSON: &str = include_str!("l2/public.json");
