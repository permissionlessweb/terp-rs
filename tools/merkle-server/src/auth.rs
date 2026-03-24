//! Secp256k1 / ECDSA signature verification for write endpoints.
//!
//! The server verifies that write requests carry a valid ECDSA signature
//! produced by the holder of the configured admin private key.
//!
//! ## Canonical message
//!
//! The message signed is the raw bytes of:
//!
//!   "{METHOD}\n{PATH}\n{TIMESTAMP}\n{sha256hex(body)}"
//!
//! e.g.  "POST\n/tree/season-1\n1709500000\naabbccdd..."
//!
//! k256's `Verifier::verify` applies SHA256 to this string internally, so the
//! external signer must use standard ECDSA-SHA256 over the canonical bytes.
//!
//! ## Signature format
//!
//! 64-byte compact secp256k1 ECDSA (r ‖ s), hex-encoded.

use anyhow::Result;
use k256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AuthKey {
    vk: VerifyingKey,
}

impl AuthKey {
    /// Parse a compressed secp256k1 public key from a 66-char hex string (33 bytes).
    pub fn from_hex(pubkey_hex: &str) -> Result<Self> {
        let bytes = hex::decode(pubkey_hex)?;
        let vk = VerifyingKey::from_sec1_bytes(&bytes)?;
        Ok(Self { vk })
    }

    /// Generate a fresh keypair, returning (private_key_hex, public_key_hex).
    /// The private key is only needed to produce signatures externally.
    pub fn generate() -> Result<(String, String)> {
        use k256::ecdsa::SigningKey;
        use rand_core::OsRng;
        let sk = SigningKey::random(&mut OsRng);
        let pk_hex = hex::encode(sk.verifying_key().to_sec1_bytes());
        let sk_hex = hex::encode(sk.to_bytes());
        Ok((sk_hex, pk_hex))
    }

    /// Verify a write request.
    ///
    /// Returns `true` only when:
    ///  1. `timestamp_str` is within `tolerance_s` seconds of now (replay protection)
    ///  2. `sig_hex` is a valid compact secp256k1 signature over the canonical message
    pub fn verify(
        &self,
        method: &str,
        path: &str,
        timestamp_str: &str,
        body: &[u8],
        sig_hex: &str,
        tolerance_s: u64,
    ) -> bool {
        self.try_verify(method, path, timestamp_str, body, sig_hex, tolerance_s)
            .unwrap_or(false)
    }

    fn try_verify(
        &self,
        method: &str,
        path: &str,
        timestamp_str: &str,
        body: &[u8],
        sig_hex: &str,
        tolerance_s: u64,
    ) -> Result<bool> {
        // ── Timestamp freshness ──────────────────────────────────────────
        let ts: u64 = timestamp_str.trim().parse()?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let age = now.saturating_sub(ts).max(ts.saturating_sub(now));
        if age > tolerance_s {
            return Ok(false);
        }

        // ── Canonical message → signature verification ───────────────────
        let msg = canonical_message(method, path, timestamp_str, body);
        let sig_bytes = hex::decode(sig_hex.trim())?;
        let sig = Signature::try_from(sig_bytes.as_slice())?;
        // k256 Verifier::verify applies SHA256 to `msg` internally.
        self.vk.verify(msg.as_bytes(), &sig)?;
        Ok(true)
    }
}

/// Build the canonical message string for a given request.
///
/// The caller signs this string using ECDSA-SHA256 with their secp256k1 key.
/// The resulting compact (r‖s) 64-byte signature is hex-encoded and sent as
/// the `X-Signature` header.
pub fn canonical_message(method: &str, path: &str, timestamp: &str, body: &[u8]) -> String {
    let body_hash = hex::encode(Sha256::digest(body));
    format!("{method}\n{path}\n{timestamp}\n{body_hash}")
}

/// Current unix timestamp as a string. Used by `prepare` to anchor the message.
pub fn now_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        .to_string()
}
