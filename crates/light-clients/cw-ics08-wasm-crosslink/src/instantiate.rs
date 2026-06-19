//! Instantiate logic for the Crosslink light client.

use cosmwasm_std::Storage;
use crosslink_light_client::client_state::ClientState as CrosslinkClientState;
use crosslink_light_client::consensus_state::ConsensusState as CrosslinkConsensusState;
use crosslink_light_client::{ZcashDeserialize, ZcashSerialize};
use ibc_proto::ibc::core::client::v1::Height;
use ibc_proto::ibc::lightclients::wasm::v1::ClientState as WasmClientState;

use crate::ContractError;
use crate::msg::InstantiateMsg;
use crate::state;

/// Initialize the client state and consensus state from the instantiate message.
///
/// # Errors
///
/// Returns [`ContractError`] if deserialization or storage fails.
pub fn client(storage: &mut dyn Storage, msg: InstantiateMsg) -> Result<(), ContractError> {
    let client_state: CrosslinkClientState =
        CrosslinkClientState::zcash_deserialize(std::io::Cursor::new(msg.client_state))?;

    let consensus_state: CrosslinkConsensusState =
        CrosslinkConsensusState::zcash_deserialize(std::io::Cursor::new(msg.consensus_state))?;

    // Consistency check: client state height must match consensus state height
    if client_state.latest_bft_height != consensus_state.bft_height {
        return Err(ContractError::ClientAndConsensusStateMismatch);
    }

    // Roster check: must have at least one finalizer with positive voting power
    if !client_state.validate_roster() {
        return Err(ContractError::InvalidClientMessage);
    }

    let wasm_client_state = WasmClientState {
        data: client_state.zcash_serialize_to_vec()?,
        checksum: msg.checksum.to_vec(),
        latest_height: Some(Height {
            revision_number: 0,
            revision_height: client_state.latest_bft_height.into(),
        }),
    };

    state::store_client_state(storage, &wasm_client_state)?;
    state::store_consensus_state(storage, consensus_state.bft_height, &consensus_state)?;

    Ok(())
}
