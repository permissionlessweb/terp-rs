use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use std::collections::BTreeMap;

/// Reasonable upper bound for mint URLs (scheme + host + path).
pub const MAX_URL_LEN: usize = 512;
/// mint_id charset length cap (matches notes `validate_id`).
pub const MAX_MINT_ID_LEN: usize = 200;
/// Default max free-form metadata payload size (bytes, key+value sum).
pub const DEFAULT_MAX_METADATA_BYTES: u32 = 512;
/// Default page size / hard cap for ListMints.
pub const DEFAULT_LIMIT: u32 = 30;
pub const MAX_LIMIT: u32 = 100;

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    /// Cap on approximate metadata map size (sum of key + value lengths).
    pub max_metadata_bytes: u32,
    /// When true, any address may RegisterMint; updates/status remain gated.
    pub allow_public_register: bool,
}

/// Discovery status for a registered mint.
#[cw_serde]
#[derive(Eq, Hash)]
pub enum MintStatus {
    /// Discoverable; wallets may use subject to their own trust.
    Active,
    /// Temporarily hidden from default list queries.
    Paused,
    /// Tombstone; kept for audit, excluded from default list.
    Revoked,
}

impl MintStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MintStatus::Active => "active",
            MintStatus::Paused => "paused",
            MintStatus::Revoked => "revoked",
        }
    }
}

/// Canonical mint descriptor (v0) — discovery only, not an endorsement.
#[cw_serde]
pub struct MintDescriptor {
    pub mint_id: String,
    pub url: String,
    pub name: Option<String>,
    pub units: Vec<String>,
    pub keyset_ids: Vec<String>,
    /// Optional NUT feature map (string keys; values free-form short strings).
    pub nuts: BTreeMap<String, String>,
    pub pubkey: Option<String>,
    pub status: MintStatus,
    /// Optional 64-hex BUD pin of **public** mint-info only.
    pub content_sha256: Option<String>,
    /// Block time (unix seconds) at registration.
    pub registered_at: u64,
    /// Block time (unix seconds) of last update.
    pub updated_at: u64,
    pub registrar: Addr,
    /// Optional free-form metadata (size-capped by config).
    pub metadata: BTreeMap<String, String>,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// mint_id → MintDescriptor
pub const MINTS: Map<&str, MintDescriptor> = Map::new("mints");

/// normalized_url → mint_id (uniqueness index)
pub const URL_INDEX: Map<&str, String> = Map::new("url_idx");
