//! Default authenticator trait alias (legacy Bitsong name kept for contract compatibility).

/// Historical name used across `terp-passkey`, `terp-dao`, etc.
///
/// Canonical definition: [`terp_auth::TerpAccountTrait`].
pub use terp_auth::TerpAccountTrait as BtsgAccountTrait;

/// Preferred modern name.
pub use terp_auth::TerpAccountTrait;
