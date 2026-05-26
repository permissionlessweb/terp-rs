#[cfg(feature = "nostr")]
pub mod nostr_env;

#[cfg(feature = "docker")]
pub mod quickspawn_env;
pub mod suite;