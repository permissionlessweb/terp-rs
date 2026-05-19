pub mod error;
pub mod event;
pub mod nips;
pub mod tags;
pub mod types;

// Re-exports
pub use error::{NipError, NipResult};
pub use event::*;
use schemars::JsonSchema;
use sha2::Digest as _;
pub use tags::TagBuilder;
pub use types::*;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

// ==================== Core Traits ====================

/// Marker trait for NIP event kinds
pub trait NipKind:
    Copy + Clone + Eq + std::hash::Hash + std::fmt::Debug + Send + Sync + 'static
{
    fn kind_value(&self) -> u16;

    fn is_regular(&self) -> bool {
        let v = self.kind_value();
        (1000..10000).contains(&v) || (4..45).contains(&v) || v == 1 || v == 2
    }

    fn is_replaceable(&self) -> bool {
        let v = self.kind_value();
        (10000..20000).contains(&v) || v == 0 || v == 3
    }

    fn is_ephemeral(&self) -> bool {
        let v = self.kind_value();
        (20000..30000).contains(&v)
    }

    fn is_addressable(&self) -> bool {
        let v = self.kind_value();
        (30000..40000).contains(&v)
    }
}

/// Core trait for NIP metadata types
///
/// This trait defines the interface that all NIP-specific metadata types
/// must implement. It provides:
/// - Type-safe kind association
/// - Validation according to NIP-specific rules
/// - Conversion to/from event tags and content
/// - Parsing from raw Nostr events
pub trait NipMetadata: Serialize + DeserializeOwned + Clone + Send + Sync + 'static {
    /// The kind type for this NIP
    type Kind: NipKind;

    /// Get the kind for this metadata instance
    fn kind(&self) -> Self::Kind;

    /// Validate metadata according to NIP-specific rules
    fn validate(&self) -> NipResult<()>;

    /// Convert metadata to event tags
    fn to_tags(&self) -> Vec<Tag>;

    /// Convert metadata to event content
    fn content(&self) -> String;

    /// Parse metadata from a raw Nostr event
    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self>;

    /// Get the d-tag identifier for addressable events
    fn d_tag(&self) -> Option<String> {
        None
    }
}

/// A raw Nostr event as received from relays
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawNostrEvent {
    pub id: String,
    pub pubkey: String,
    pub created_at: u64,
    pub kind: u16,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

impl RawNostrEvent {
    /// Compute the event ID according to NIP-01
    pub fn compute_id(&self) -> NipResult<String> {
        let serialized = Self::serialize_for_id(
            &self.pubkey,
            self.created_at,
            self.kind,
            &self.tags,
            &self.content,
        );
        let hash = sha2::Sha256::digest(serialized.as_bytes());
        Ok(hex::encode(hash))
    }

    /// Serialize event data for ID computation
    pub fn serialize_for_id(
        pubkey: &str,
        created_at: u64,
        kind: u16,
        tags: &[Vec<String>],
        content: &str,
    ) -> String {
        let serialized = serde_json::json!([0, pubkey, created_at, kind, tags, content]);
        serialized.to_string()
    }

    /// Verify the event signature (requires cryptographic verification)
    pub fn verify_signature(&self) -> NipResult<bool> {
        // This would require schnorr signature verification
        // Implementation depends on crypto library choice
        Err(NipError::Crypto(
            "Signature verification not implemented".into(),
        ))
    }

    /// Parse tags by name
    pub fn tags_by_name(&self, name: &str) -> Vec<&Vec<String>> {
        self.tags
            .iter()
            .filter(|tag| tag.first().map(|n| n.as_str()) == Some(name))
            .collect()
    }

    /// Get the d-tag value for addressable events
    pub fn d_tag(&self) -> Option<&str> {
        self.tags
            .iter()
            .find(|tag| tag.first().map(|n| n.as_str()) == Some("d"))
            .and_then(|tag| tag.get(1))
            .map(|s| s.as_str())
    }
}

/// Builder for constructing Nostr events
pub struct NostrEventBuilder;

impl NostrEventBuilder {
    /// Build a raw Nostr event from metadata
    pub fn build<M: NipMetadata>(
        metadata: &M,
        pubkey: &PubKey,
        created_at: UnixTimestamp,
    ) -> NipResult<RawNostrEvent> {
        metadata.validate()?;

        let mut tags = metadata.to_tags();

        // Add d-tag for addressable events
        if let Some(d_tag) = metadata.d_tag() {
            tags.push(Tag::d(d_tag));
        }

        let kind = metadata.kind().kind_value();
        let content = metadata.content();

        let serialized = RawNostrEvent::serialize_for_id(
            pubkey.as_str(),
            created_at,
            kind,
            &tags
                .iter()
                .map(|t| t.clone().into_inner())
                .collect::<Vec<_>>(),
            &content,
        );

        let hash = sha2::Sha256::digest(serialized.as_bytes());
        let id = hex::encode(hash);

        Ok(RawNostrEvent {
            id,
            pubkey: pubkey.as_str().to_string(),
            created_at,
            kind,
            tags: tags.into_iter().map(|t| t.into_inner()).collect(),
            content,
            sig: String::new(), // Must be signed separately
        })
    }
}
