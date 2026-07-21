use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::msg::IssuerConfig;

pub const CONFIG: Item<Config> = Item::new("config");
pub const ISSUERS: Map<&str, IssuerConfig> = Map::new("issuers");
/// claim_commitment (as Binary) → account that registered it
pub const CLAIMS: Map<&[u8], Addr> = Map::new("claims");
/// nullifier → spent height (replay protection on Track)
pub const NULLIFIERS: Map<&[u8], u64> = Map::new("nullifiers");

#[cosmwasm_schema::cw_serde]
pub struct Config {
    pub admin: Addr,
    pub require_registered_claim: bool,
}
