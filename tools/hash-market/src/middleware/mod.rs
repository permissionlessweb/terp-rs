//! Authentication middleware dispatching on `X-Auth-Type` header.
//!
//! Supports:
//! - `secp256k1`: ECDSA signature verification using `k256::PrehashVerifier`
//! - `jwt`: HS256 JWT verification via `jsonwebtoken`
//!
//! ## secp256k1 Auth Flow
//!
//! The snap client:
//! 1. Computes `msg = "{timestamp}\n{pubkey_hex}"`
//! 2. Hashes: `hash = SHA256(msg)`
//! 3. Signs: `sig = secp256k1.sign(hash, sk)`
//! 4. Sends headers: X-Auth-Type, X-Pubkey, X-Timestamp, X-Signature
//!
//! The server MUST use `PrehashVerifier::verify_prehash()` (NOT `Verifier::verify()`)
//! because the client already hashed the message before signing.

pub mod auth;
pub mod notes_auth;

pub use crate::config::NotesAuthConfig;
pub use notes_auth::{AnyOfAuthVerifier, BearerTokenVerifier, SnapSecpVerifier};

// use anyhow::Result;
// use axum::{
//     extract::Request,
//     http::StatusCode,
//     middleware::Next,
//     response::Response,
// };
// use k256::ecdsa::{signature::hazmat::PrehashVerifier, Signature, VerifyingKey};
// use sha2::{Digest, Sha256};
// use std::time::{SystemTime, UNIX_EPOCH};

// /// Timestamp tolerance in seconds.
// const TIMESTAMP_TOLERANCE: u64 = 300;

// /// Axum middleware that verifies authentication.
// pub async fn auth_middleware(
//     request: Request,
//     next: Next,
// ) -> Result<Response, StatusCode> {
//     let headers = request.headers();

//     let auth_type = headers
//         .get("X-Auth-Type")
//         .and_then(|v| v.to_str().ok())
//         .unwrap_or("");

//     match auth_type {
//         "secp256k1" => verify_secp256k1(request, next).await,
//         "jwt" => verify_jwt(request, next).await,
//         _ => Err(StatusCode::UNAUTHORIZED),
//     }
// }

// async fn verify_secp256k1(
//     request: Request,
//     next: Next,
// ) -> Result<Response, StatusCode> {
//     let headers = request.headers();

//     let pubkey_hex = headers
//         .get("X-Pubkey")
//         .and_then(|v| v.to_str().ok())
//         .ok_or(StatusCode::UNAUTHORIZED)?;

//     let timestamp_str = headers
//         .get("X-Timestamp")
//         .and_then(|v| v.to_str().ok())
//         .ok_or(StatusCode::UNAUTHORIZED)?;

//     let sig_hex = headers
//         .get("X-Signature")
//         .and_then(|v| v.to_str().ok())
//         .ok_or(StatusCode::UNAUTHORIZED)?;

//     // Check timestamp freshness
//     let ts: u64 = timestamp_str.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;
//     let now = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
//         .as_secs();
//     let age = now.abs_diff(ts);
//     if age > TIMESTAMP_TOLERANCE {
//         return Err(StatusCode::UNAUTHORIZED);
//     }

//     // Reconstruct the message hash the client signed
//     // Client computes: SHA256("{timestamp}\n{pubkey_hex}")
//     let msg = format!("{timestamp_str}\n{pubkey_hex}");
//     let msg_hash = Sha256::digest(msg.as_bytes());

//     // Parse public key and signature
//     let pk_bytes = hex::decode(pubkey_hex).map_err(|_| StatusCode::UNAUTHORIZED)?;
//     let vk = VerifyingKey::from_sec1_bytes(&pk_bytes).map_err(|_| StatusCode::UNAUTHORIZED)?;
//     let sig_bytes = hex::decode(sig_hex).map_err(|_| StatusCode::UNAUTHORIZED)?;
//     let sig = Signature::try_from(sig_bytes.as_slice()).map_err(|_| StatusCode::UNAUTHORIZED)?;

//     // CRITICAL: Use verify_prehash, NOT verify.
//     // The client already hashed the message (SHA256) before signing.
//     // Using verify() would double-hash (SHA256(SHA256(msg))).
//     vk.verify_prehash(&msg_hash, &sig)
//         .map_err(|_| StatusCode::UNAUTHORIZED)?;

//     Ok(next.run(request).await)
// }

// async fn verify_jwt(
//     request: Request,
//     next: Next,
// ) -> Result<Response, StatusCode> {
//     let headers = request.headers();

//     let token = headers
//         .get("Authorization")
//         .and_then(|v| v.to_str().ok())
//         .and_then(|v| v.strip_prefix("Bearer "))
//         .ok_or(StatusCode::UNAUTHORIZED)?;

//     // JWT secret should come from config; for now use env var
//     let secret = std::env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
//     let key = jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());

//     jsonwebtoken::decode::<serde_json::Value>(token, &key, &validation)
//         .map_err(|_| StatusCode::UNAUTHORIZED)?;

//     Ok(next.run(request).await)
// }
