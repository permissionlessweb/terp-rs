//! Misbehaviour detection for the Crosslink light client.
//!
//! Detects equivocation: two conflicting fat pointers at the same BFT height
//! signed by the same finalizer set.

use crate::client_state::ClientState;
use crate::consensus_state::ConsensusState;
use crate::error::CrosslinkIBCError;
use crate::header::CrosslinkHeader;

/// Verify that two headers constitute misbehaviour (equivocation).
///
/// # Errors
///
/// Returns [`CrosslinkIBCError::MisbehaviourDetected`] if misbehaviour is confirmed.
pub fn verify_misbehaviour(
    _client_state: &ClientState,
    _consensus_state: &ConsensusState,
    _header_1: &CrosslinkHeader,
    _header_2: &CrosslinkHeader,
) -> Result<(), CrosslinkIBCError> {
    // TODO: Compare header_1 and header_2 at the same BFT height.
    // If both have valid signatures but different finalization candidates,
    // that's equivocation → freeze the client.
    Err(CrosslinkIBCError::MisbehaviourDetected)
}