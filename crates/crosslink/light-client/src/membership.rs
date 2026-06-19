//! Membership and non-membership proof verification for the Crosslink light client.
//!
//! For Crosslink, membership proofs relate to the finalized state commitment.
//! This is a stub until ZIP 222 integration defines the exact proof format.

use crate::client_state::ClientState;
use crate::consensus_state::ConsensusState;
use crate::error::CrosslinkIBCError;

/// Verify that a value exists at the given path under the state commitment.
///
/// # Errors
///
/// Returns [`CrosslinkIBCError::MembershipVerificationFailed`] if the proof is invalid.
pub fn verify_membership(
    _consensus_state: ConsensusState,
    _client_state: ClientState,
    _proof: Vec<u8>,
    _merkle_path: Vec<Vec<u8>>,
    _value: Vec<u8>,
) -> Result<(), CrosslinkIBCError> {
    // TODO: Implement merkle proof verification against the state commitment.
    // The proof format depends on the finalized state structure (ZIP 222).
    Err(CrosslinkIBCError::MembershipVerificationFailed(
        "not yet implemented".into(),
    ))
}

/// Verify that no value exists at the given path under the state commitment.
///
/// # Errors
///
/// Returns [`CrosslinkIBCError::MembershipVerificationFailed`] if the proof is invalid.
pub fn verify_non_membership(
    _consensus_state: ConsensusState,
    _client_state: ClientState,
    _proof: Vec<u8>,
    _merkle_path: Vec<Vec<u8>>,
) -> Result<(), CrosslinkIBCError> {
    Err(CrosslinkIBCError::MembershipVerificationFailed(
        "not yet implemented".into(),
    ))
}