// BUD-07 - Paid upload and download https://github.com/hzrd149/blossom/blob/master/buds/07.md
// client sends signed msg OR Tx hash that shows valid payment to relayers pubkey
//  default trait implementfor server./client traits, but requires enabling via .with_x402 || .with_bud07 (identical), setting default parameters

// ── Payment (BUD-07) ──────────────────────────────────────────────────────────

use crate::buds::{AuthResult, AuthScope, AuthVerifier};

pub trait PaymentVerifier: Send + Sync {
    fn verify_upload(&self, proof: &PaymentProof, size: u64) -> Result<(), PaymentError>;
    fn verify_download(&self, proof: &PaymentProof, hash: &[u8; 32]) -> Result<(), PaymentError>;
}

#[derive(Debug, Clone)]
pub struct PaymentProof {
    pub signature: String,
    pub pubkey: String,
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("invalid signature")]
    InvalidSignature,
    #[error("insufficient payment for size")]
    InsufficientPayment,
    #[error("payment expired")]
    Expired,
    #[error("{0}")]
    Other(String),
}

/// No-op payment verifier — allows everything.
pub struct NoopPaymentVerifier;

impl PaymentVerifier for NoopPaymentVerifier {
    fn verify_upload(&self, _proof: &PaymentProof, _size: u64) -> Result<(), PaymentError> {
        Ok(())
    }

    fn verify_download(&self, _proof: &PaymentProof, _hash: &[u8; 32]) -> Result<(), PaymentError> {
        Ok(())
    }
}

/// No-op auth verifier — allows everything.
pub struct NoopAuthVerifier;

impl AuthVerifier for NoopAuthVerifier {
    fn verify(&self, _headers: &axum::http::HeaderMap) -> AuthResult<crate::buds::AuthClaims> {
        Ok(crate::buds::AuthClaims {
            subject: "anonymous".to_string(),
            expires_at: u64::MAX,
            actions: vec![
                "get".to_string(),
                "upload".to_string(),
                "list".to_string(),
                "delete".to_string(),
            ],
            scope: AuthScope::Server,
        })
    }
}
