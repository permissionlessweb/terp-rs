//! Consensus state update logic for the Crosslink light client.
//!
//! After a header is verified, this module derives the new consensus
//! state and optionally updates the client state.

use crate::client_state::ClientState;
use crate::consensus_state::ConsensusState;
use crate::error::CrosslinkIBCError;
use crate::header::CrosslinkHeader;

/// Update the consensus state given a verified header.
///
/// Returns `(updated_bft_height, new_consensus_state, optional_updated_client_state)`.
///
/// # Errors
///
/// Returns [`CrosslinkIBCError`] if the update logic fails.
pub fn update_consensus_state(
    _consensus_state: ConsensusState,
    client_state: ClientState,
    header: &CrosslinkHeader,
) -> Result<(u32, ConsensusState, Option<ClientState>), CrosslinkIBCError> {
    let bft_block = &header.bft_block;
    let new_bft_height = bft_block.height;
    let pow_anchor = bft_block.finalization_candidate();

    let new_consensus_state = ConsensusState {
        bft_height: new_bft_height,
        pow_anchor_height: bft_block.finalization_candidate_height,
        pow_anchor_hash: pow_anchor.hash,
        timestamp: pow_anchor.timestamp,
        state_commitment: [0u8; 32], // placeholder — populated from ZIP 222 tx
    };

    let mut new_client_state = client_state;
    new_client_state.latest_bft_block_hash = header.block_hash();
    new_client_state.latest_bft_height = new_bft_height;
    new_client_state.latest_finalized_pow_height = new_consensus_state.pow_anchor_height;
    new_client_state.latest_finalized_pow_hash = new_consensus_state.pow_anchor_hash;

    Ok((new_bft_height, new_consensus_state, Some(new_client_state)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{BftBlock, Blake3Hash, FatPointerToBftBlock2, PowHeader, PROTOTYPE_PARAMETERS};

    #[test]
    fn test_update_consensus_state_basic() {
        let client_state = ClientState::new(
            PROTOTYPE_PARAMETERS.clone(),
            Blake3Hash([0u8; 32]),
            0,
            0,
            [0u8; 32],
            vec![],
        );
        let consensus_state = ConsensusState {
            bft_height: 0,
            pow_anchor_height: 0,
            pow_anchor_hash: [0u8; 32],
            timestamp: 0,
            state_commitment: [0u8; 32],
        };

        let header = PowHeader {
            hash: [1u8; 32],
            timestamp: 1_700_000_000,
            height: 100,
        };

        let bft_block = BftBlock {
            version: 1,
            height: 1,
            previous_block_fat_ptr: FatPointerToBftBlock2::null(),
            finalization_candidate_height: 100,
            headers: vec![header.clone(), header.clone(), header],
        };

        let crosslink_header = CrosslinkHeader {
            trusted_bft_height: 0,
            bft_block,
            fat_pointer: FatPointerToBftBlock2::null(),
        };

        let result = update_consensus_state(consensus_state, client_state, &crosslink_header);
        assert!(result.is_ok());

        let (new_height, new_consensus, updated_client) = result.unwrap();
        assert_eq!(new_height, 1);
        assert_eq!(new_consensus.bft_height, 1);
        assert_eq!(new_consensus.pow_anchor_height, 100);
        assert!(updated_client.is_some());
        let client = updated_client.unwrap();
        assert_eq!(client.latest_bft_height, 1);
        assert_eq!(client.latest_finalized_pow_height, 100);
    }
}