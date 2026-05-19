use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::NipError;

// ==================== Primitive Types ====================

/// A 32-byte lowercase hex-encoded public key
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PubKey(String);

impl PubKey {
    pub fn new(hex: impl Into<String>) -> Result<Self, NipError> {
        let hex = hex.into();
        if hex.len() != 64 {
            return Err(NipError::InvalidPubKeyLength(hex.len()));
        }
        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(NipError::InvalidHexFormat);
        }
        Ok(Self(hex.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn to_bytes(&self) -> Result<[u8; 32], NipError> {
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(&self.0, &mut bytes)
            .map_err(NipError::HexDecodeError)?;
        Ok(bytes)
    }
}

impl fmt::Display for PubKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PubKey {
    type Err = NipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for PubKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// A 32-byte lowercase hex-encoded event ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(String);

impl EventId {
    pub fn new(hex: impl Into<String>) -> Result<Self, NipError> {
        let hex = hex.into();
        if hex.len() != 64 {
            return Err(NipError::InvalidEventIdLength(hex.len()));
        }
        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(NipError::InvalidHexFormat);
        }
        Ok(Self(hex.to_lowercase()))
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        Self(hex::encode(bytes))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_bytes(&self) -> Result<[u8; 32], NipError> {
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(&self.0, &mut bytes)
            .map_err(NipError::HexDecodeError)?;
        Ok(bytes)
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for EventId {
    type Err = NipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// A 64-byte lowercase hex-encoded Schnorr signature
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature(String);

impl Signature {
    pub fn new(hex: impl Into<String>) -> Result<Self, NipError> {
        let hex = hex.into();
        if hex.len() != 128 {
            return Err(NipError::InvalidSignatureLength(hex.len()));
        }
        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(NipError::InvalidHexFormat);
        }
        Ok(Self(hex.to_lowercase()))
    }

    pub fn from_bytes(bytes: &[u8; 64]) -> Self {
        Self(hex::encode(bytes))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_bytes(&self) -> Result<[u8; 64], NipError> {
        let mut bytes = [0u8; 64];
        hex::decode_to_slice(&self.0, &mut bytes)
            .map_err(NipError::HexDecodeError)?;
        Ok(bytes)
    }
}

/// Unix timestamp in seconds
pub type UnixTimestamp = u64;

/// Tag array - each tag is a vector of strings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag(Vec<String>);

impl Tag {
    pub fn new(items: Vec<String>) -> Self {
        Self(items)
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Create an "e" tag (event reference)
    pub fn e(event_id: impl Into<String>) -> Self {
        Self(vec![String::from("e"), event_id.into()])
    }

    /// Create an "e" tag with relay URL
    pub fn e_with_relay(event_id: impl Into<String>, relay: impl Into<String>) -> Self {
        Self(vec![String::from("e"), event_id.into(), relay.into()])
    }

    /// Create an "e" tag with relay URL and author pubkey
    pub fn e_full(
        event_id: impl Into<String>,
        relay: impl Into<String>,
        author: impl Into<String>,
    ) -> Self {
        Self(vec![
            String::from("e"),
            event_id.into(),
            relay.into(),
            author.into(),
        ])
    }

    /// Create a "p" tag (pubkey reference)
    pub fn p(pubkey: impl Into<String>) -> Self {
        Self(vec![String::from("p"), pubkey.into()])
    }

    /// Create a "p" tag with relay URL
    pub fn p_with_relay(pubkey: impl Into<String>, relay: impl Into<String>) -> Self {
        Self(vec![String::from("p"), pubkey.into(), relay.into()])
    }

    /// Create an "a" tag (addressable event reference)
    pub fn a(kind: u16, pubkey: impl Into<String>, d: impl Into<String>) -> Self {
        Self(vec![
            String::from("a"),
            format!("{}:{}:{}", kind, pubkey.into(), d.into()),
        ])
    }

    /// Create an "a" tag with relay URL
    pub fn a_with_relay(
        kind: u16,
        pubkey: impl Into<String>,
        d: impl Into<String>,
        relay: impl Into<String>,
    ) -> Self {
        Self(vec![
            String::from("a"),
            format!("{}:{}:{}", kind, pubkey.into(), d.into()),
            relay.into(),
        ])
    }

    /// Create a "d" tag (replaceable/addressable event identifier)
    pub fn d(value: impl Into<String>) -> Self {
        Self(vec![String::from("d"), value.into()])
    }

    /// Create a "t" tag (hashtag)
    pub fn t(value: impl Into<String>) -> Self {
        Self(vec![String::from("t"), value.into()])
    }

    /// Create an "alt" tag (alternative description)
    pub fn alt(value: impl Into<String>) -> Self {
        Self(vec![String::from("alt"), value.into()])
    }

    /// Create a "g" tag (geohash)
    pub fn g(value: impl Into<String>) -> Self {
        Self(vec![String::from("g"), value.into()])
    }

    /// Create an "r" tag (reference/link)
    pub fn r(value: impl Into<String>) -> Self {
        Self(vec![String::from("r"), value.into()])
    }

    /// Get the tag name (first element)
    pub fn name(&self) -> Option<&str> {
        self.0.first().map(|s| s.as_str())
    }

    /// Get the tag value (second element)
    pub fn value(&self) -> Option<&str> {
        self.0.get(1).map(|s| s.as_str())
    }

    /// Get additional elements after the value
    pub fn additional(&self) -> &[String] {
        if self.0.len() > 2 {
            &self.0[2..]
        } else {
            &[]
        }
    }

    pub fn into_inner(self) -> Vec<String> {
        self.0
    }

    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}

impl From<Vec<String>> for Tag {
    fn from(v: Vec<String>) -> Self {
        Self(v)
    }
}

impl From<Tag> for Vec<String> {
    fn from(tag: Tag) -> Self {
        tag.0
    }
}

impl AsRef<[String]> for Tag {
    fn as_ref(&self) -> &[String] {
        &self.0
    }
}

/// Event kind - integer between 0 and 65535
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Kind(u16);

impl Kind {
    // NIP-01 kinds
    pub const METADATA: Kind = Kind(0);
    pub const TEXT_NOTE: Kind = Kind(1);
    pub const RECOMMEND_RELAY: Kind = Kind(2);
    pub const CONTACT_LIST: Kind = Kind(3);
    pub const DM: Kind = Kind(4);
    pub const DELETION: Kind = Kind(5);
    pub const REPOST: Kind = Kind(6);
    pub const REACTION: Kind = Kind(7);

    // NIP-15 Marketplace kinds
    pub const SET_STALL: Kind = Kind(30017);
    pub const SET_PRODUCT: Kind = Kind(30018);
    pub const MARKETPLACE_UI: Kind = Kind(30019);
    pub const AUCTION_PRODUCT: Kind = Kind(30020);
    pub const BID: Kind = Kind(1021);
    pub const BID_CONFIRMATION: Kind = Kind(1022);

    // NIP-52 Calendar kinds
    pub const DATE_EVENT: Kind = Kind(31922);
    pub const TIME_EVENT: Kind = Kind(31923);
    pub const CALENDAR: Kind = Kind(31924);
    pub const RSVP: Kind = Kind(31925);

    pub fn new(value: u16) -> Self {
        Kind(value)
    }

    pub fn value(&self) -> u16 {
        self.0
    }

    /// Regular events: kinds 1000-9999, 4-44, 1, 2
    pub fn is_regular(self) -> bool {
        (1000..10000).contains(&self.0)
            || (4..45).contains(&self.0)
            || self.0 == 1
            || self.0 == 2
    }

    /// Replaceable events: kinds 10000-19999, 0, 3
    pub fn is_replaceable(self) -> bool {
        (10000..20000).contains(&self.0) || self.0 == 0 || self.0 == 3
    }

    /// Ephemeral events: kinds 20000-29999
    pub fn is_ephemeral(self) -> bool {
        (20000..30000).contains(&self.0)
    }

    /// Addressable events: kinds 30000-39999
    pub fn is_addressable(self) -> bool {
        (30000..40000).contains(&self.0)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for Kind {
    fn from(value: u16) -> Self {
        Kind(value)
    }
}

impl From<Kind> for u16 {
    fn from(kind: Kind) -> Self {
        kind.0
    }
}

/// Relay URL
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RelayUrl(String);

impl RelayUrl {
    pub fn new(url: impl Into<String>) -> Result<Self, NipError> {
        let url = url.into();
        if !url.starts_with("wss://") && !url.starts_with("ws://") {
            return Err(NipError::InvalidRelayUrl(url));
        }
        Ok(Self(url))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RelayUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}