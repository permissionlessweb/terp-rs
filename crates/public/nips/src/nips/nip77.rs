//! NIP-77: Negentropy Syncing
//!
//! Efficient set reconciliation protocol for Nostr relays and clients.
//! <https://github.com/nostr-protocol/nips/blob/master/77.md>

use crate::error::{NipError, NipResult};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn generate_negentropy_session_id(pubkey: &str) -> String {
    use sha2::Digest as _;
    let timestamp = current_timestamp();
    let input = format!("neg:{}:{}", timestamp, pubkey);

    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();

    format!("neg-{}-{}", timestamp, hex::encode(&hash[0..8]))
}

pub fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ==================== Negentropy Message Types ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NegentropyMessage {
    Init {
        id: String,
        filter: Value,
        negentropy: String, // hex-encoded negentropy frame
    },
    Msg {
        id: String,
        negentropy: String,
    },
    Error {
        id: String,
        reason: String,
    },
}

// ==================== Core Trait (Workflow Style) ====================

pub trait NegentropySync {
    fn start_sync(
        &self,
        pubkey: &str,
        filter: Value,
        initial_known_ids: Vec<String>,
    ) -> NipResult<NegentropySession>;

    fn process_message(
        &mut self,
        message: NegentropyMessage,
    ) -> NipResult<Option<NegentropyMessage>>;

    fn get_missing_events(&self) -> Vec<String>;

    fn mark_reconciled(&mut self, event_ids: &[String]);
}

// ==================== Session State ====================

#[derive(Debug)]
pub struct NegentropySession {
    pub session_id: String,
    pub filter: Value,
    pub reconciled: Vec<String>,
    pub missing: Vec<String>,
}

impl NegentropySession {
    pub fn new(pubkey: &str, filter: Value) -> Self {
        Self {
            session_id: generate_negentropy_session_id(pubkey),
            filter,
            reconciled: vec![],
            missing: vec![],
        }
    }
}

impl NegentropySync for NegentropySession {
    fn start_sync(
        &self,
        pubkey: &str,
        filter: Value,
        initial_known_ids: Vec<String>,
    ) -> NipResult<NegentropySession> {
        let mut session = NegentropySession::new(pubkey, filter);
        session.reconciled = initial_known_ids;
        Ok(session)
    }

    fn process_message(
        &mut self,
        message: NegentropyMessage,
    ) -> NipResult<Option<NegentropyMessage>> {
        match message {
            NegentropyMessage::Init {
                id,
                filter,
                negentropy,
            } => {
                self.filter = filter;
                // TODO: In real implementation, decode negentropy and compute diff
                // For now, return a placeholder continuation message
                Ok(Some(NegentropyMessage::Msg {
                    id,
                    negentropy: format!("response-{}", current_timestamp()),
                }))
            }
            NegentropyMessage::Msg {
                id: _,
                negentropy: _,
            } => {
                // Continue reconciliation
                Ok(None)
            }
            NegentropyMessage::Error { id, reason } => {
                Err(NipError::Nip77(format!("Sync error [{}]: {}", id, reason)))
            }
        }
    }

    fn get_missing_events(&self) -> Vec<String> {
        self.missing.clone()
    }

    fn mark_reconciled(&mut self, event_ids: &[String]) {
        self.reconciled.extend_from_slice(event_ids);
        // Optionally remove from missing list
        self.missing.retain(|id| !event_ids.contains(id));
    }
}

// ==================== High-level Helpers ====================

/// Create initial sync message (client side)
pub fn create_init_message(
    pubkey: &str,
    filter: Value,
    known_event_ids: Vec<String>,
) -> NipResult<NegentropyMessage> {
    // In a real implementation you would use a proper negentropy library here
    Ok(NegentropyMessage::Init {
        id: generate_negentropy_session_id(pubkey),
        filter,
        negentropy: "initial_negentropy_frame".to_string(), // placeholder
    })
}

/// Run a full sync round with multiple messages
pub fn perform_sync_round<S: NegentropySync>(
    sync: &mut S,
    incoming_messages: Vec<NegentropyMessage>,
) -> NipResult<Vec<NegentropyMessage>> {
    let mut responses = Vec::new();

    for msg in incoming_messages {
        if let Some(response) = sync.process_message(msg)? {
            responses.push(response);
        }
    }

    Ok(responses)
}
