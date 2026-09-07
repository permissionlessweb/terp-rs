use cosmwasm_std::{Addr, Binary};
use cw_storage_plus::{Item, Map};

use crate::msg::CircuitVerifyingKeys;

pub const CONFIG: Item<Config> = Item::new("config");
pub const SESSIONS: Map<&str, Session> = Map::new("sessions");
/// key = session_id bytes || 0x00 || nullifier bytes
pub const NULLIFIERS: Map<&[u8], bool> = Map::new("nullifiers");

#[cosmwasm_schema::cw_serde]
pub struct Config {
    pub admin: Addr,
    pub dao: Addr,
    pub circuit_vks: CircuitVerifyingKeys,
}

#[cosmwasm_schema::cw_serde]
pub struct Session {
    pub session_id: String,
    pub note_tree_root: Binary,
    pub proposal_ref: Option<String>,
    pub open: bool,
}
