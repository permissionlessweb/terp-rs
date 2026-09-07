use cosmwasm_std::Binary;
use cw_storage_plus::Item;

/// Registered passkey binding (origin + credential id). Full WebAuthn verify is
/// layered later; trait workflow stores registration data for origin checks.
#[cosmwasm_schema::cw_serde]
pub struct PasskeyRegistration {
    /// Expected clientData.origin (optional).
    pub origin: Option<String>,
    /// Credential id bytes (opaque).
    pub credential_id: Binary,
}

pub const REGISTRATION: Item<PasskeyRegistration> = Item::new("reg");
