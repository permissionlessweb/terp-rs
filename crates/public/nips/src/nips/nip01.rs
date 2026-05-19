use crate::{
    error::{NipError, NipResult},
    types::*,
    NipKind, NipMetadata, RawNostrEvent, Tag,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::Digest as _;

// ==================== NIP-01 Kinds ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nip01Kind {
    Metadata,
    TextNote,
    RecommendRelay,
    ContactList,
    EncryptedDm,
    Deletion,
    Repost,
    Reaction,
}

impl NipKind for Nip01Kind {
    fn kind_value(&self) -> u16 {
        match self {
            Nip01Kind::Metadata => 0,
            Nip01Kind::TextNote => 1,
            Nip01Kind::RecommendRelay => 2,
            Nip01Kind::ContactList => 3,
            Nip01Kind::EncryptedDm => 4,
            Nip01Kind::Deletion => 5,
            Nip01Kind::Repost => 6,
            Nip01Kind::Reaction => 7,
        }
    }
}

// ==================== NIP-01 Metadata Types ====================

/// Kind 0: User metadata
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserMetadata {
    pub name: Option<String>,
    pub about: Option<String>,
    pub picture: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl NipMetadata for UserMetadata {
    type Kind = Nip01Kind;

    fn kind(&self) -> Self::Kind {
        Nip01Kind::Metadata
    }

    fn validate(&self) -> NipResult<()> {
        // At least one field should be set
        if self.name.is_none()
            && self.about.is_none()
            && self.picture.is_none()
            && self.extra.is_empty()
        {
            return Err(NipError::Validation(
                "User metadata must have at least one field".into(),
            ));
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        Vec::new() // Kind 0 events don't require specific tags
    }

    fn content(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip01Kind::Metadata.kind_value() {
            return Err(NipError::Nip01(format!(
                "Expected kind 0, got {}",
                event.kind
            )));
        }
        serde_json::from_str(&event.content).map_err(NipError::Serialization)
    }
}

/// Kind 1: Text note
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TextNote {
    pub content: String,
    pub event_references: Vec<EventReference>,
    pub pubkey_references: Vec<PubKeyReference>,
    pub hashtags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EventReference {
    pub event_id: String,
    pub relay_url: Option<String>,
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PubKeyReference {
    pub pubkey: String,
    pub relay_url: Option<String>,
}

impl NipMetadata for TextNote {
    type Kind = Nip01Kind;

    fn kind(&self) -> Self::Kind {
        Nip01Kind::TextNote
    }

    fn validate(&self) -> NipResult<()> {
        if self.content.is_empty() {
            return Err(NipError::Validation(
                "Text note content cannot be empty".into(),
            ));
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        let mut tags = Vec::new();

        for reference in &self.event_references {
            let mut tag = vec![String::from("e"), reference.event_id.clone()];
            if let Some(ref relay) = reference.relay_url {
                tag.push(relay.clone());
                if let Some(ref marker) = reference.marker {
                    tag.push(marker.clone());
                }
            }
            tags.push(Tag::new(tag));
        }

        for reference in &self.pubkey_references {
            let mut tag = vec![String::from("p"), reference.pubkey.clone()];
            if let Some(ref relay) = reference.relay_url {
                tag.push(relay.clone());
            }
            tags.push(Tag::new(tag));
        }

        for hashtag in &self.hashtags {
            tags.push(Tag::t(hashtag));
        }

        tags
    }

    fn content(&self) -> String {
        self.content.clone()
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip01Kind::TextNote.kind_value() {
            return Err(NipError::Nip01(format!(
                "Expected kind 1, got {}",
                event.kind
            )));
        }

        let mut event_references = Vec::new();
        let mut pubkey_references = Vec::new();
        let mut hashtags = Vec::new();

        for tag in &event.tags {
            if tag.len() < 2 {
                continue;
            }
            match tag[0].as_str() {
                "e" => {
                    let reference = EventReference {
                        event_id: tag[1].clone(),
                        relay_url: tag.get(2).cloned(),
                        marker: tag.get(3).cloned(),
                    };
                    event_references.push(reference);
                }
                "p" => {
                    let reference = PubKeyReference {
                        pubkey: tag[1].clone(),
                        relay_url: tag.get(2).cloned(),
                    };
                    pubkey_references.push(reference);
                }
                "t" => {
                    hashtags.push(tag[1].clone());
                }
                _ => {}
            }
        }

        Ok(TextNote {
            content: event.content.clone(),
            event_references,
            pubkey_references,
            hashtags,
        })
    }
}

/// Kind 5: Deletion event
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Deletion {
    pub event_ids: Vec<String>,
    pub reason: Option<String>,
}

impl NipMetadata for Deletion {
    type Kind = Nip01Kind;

    fn kind(&self) -> Self::Kind {
        Nip01Kind::Deletion
    }

    fn validate(&self) -> NipResult<()> {
        if self.event_ids.is_empty() {
            return Err(NipError::Validation(
                "Deletion must specify at least one event ID".into(),
            ));
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        self.event_ids.iter().map(|id| Tag::e(id)).collect()
    }

    fn content(&self) -> String {
        self.reason.clone().unwrap_or_default()
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip01Kind::Deletion.kind_value() {
            return Err(NipError::Nip01(format!(
                "Expected kind 5, got {}",
                event.kind
            )));
        }

        let event_ids: Vec<String> = event
            .tags
            .iter()
            .filter(|tag| tag.first().map(|n| n.as_str()) == Some("e"))
            .filter_map(|tag| tag.get(1).cloned())
            .collect();

        let reason = if event.content.is_empty() {
            None
        } else {
            Some(event.content.clone())
        };

        Ok(Deletion { event_ids, reason })
    }
}

// ==================== Helper Functions ====================

/// Parse a NIP-01 event from raw JSON
pub fn parse_event(json: &str) -> NipResult<RawNostrEvent> {
    serde_json::from_str(json).map_err(NipError::Serialization)
}

/// Serialize a NIP-01 event to JSON
pub fn serialize_event(event: &RawNostrEvent) -> NipResult<String> {
    serde_json::to_string(event).map_err(NipError::Serialization)
}

/// Compute event ID according to NIP-01
pub fn compute_event_id(
    pubkey: &str,
    created_at: u64,
    kind: u16,
    tags: &[Vec<String>],
    content: &str,
) -> NipResult<String> {
    let serialized = RawNostrEvent::serialize_for_id(pubkey, created_at, kind, tags, content);
    let hash = sha2::Sha256::digest(serialized.as_bytes());
    Ok(hex::encode(hash))
}
