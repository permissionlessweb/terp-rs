//! Message types for the Crosslink light client contract.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

/// The instantiate message.
#[cw_serde]
pub struct InstantiateMsg {
    /// The serialized Crosslink [`ClientState`].
    pub client_state: Binary,
    /// The serialized Crosslink [`ConsensusState`].
    pub consensus_state: Binary,
    /// The checksum of the wasm bytecode.
    pub checksum: Binary,
}

/// The execute message (not used).
#[cw_serde]
pub enum ExecuteMsg {}

/// The query message.
#[cw_serde]
pub enum QueryMsg {
    /// Verify a client message.
    VerifyClientMessage(VerifyClientMessageMsg),
    /// Check for misbehaviour.
    CheckForMisbehaviour(CheckForMisbehaviourMsg),
    /// Get the timestamp at a given height.
    TimestampAtHeight(TimestampAtHeightMsg),
    /// Get the status of the client.
    Status(StatusMsg),
}

/// The sudo message.
#[cw_serde]
pub enum SudoMsg {
    /// Verify membership.
    VerifyMembership(VerifyMembershipMsg),
    /// Verify non-membership.
    VerifyNonMembership(VerifyNonMembershipMsg),
    /// Update the client state.
    UpdateState(UpdateStateMsg),
    /// Update state on misbehaviour.
    UpdateStateOnMisbehaviour(MisbehaviourMsg),
    /// Verify upgrade and update state.
    VerifyUpgradeAndUpdateState(VerifyUpgradeAndUpdateStateMsg),
    /// Migrate the client store.
    MigrateClientStore(MigrateClientStoreMsg),
}

/// The migrate message.
#[cw_serde]
pub struct MigrateMsg {
    /// The migration to perform.
    pub migration: Migration,
}

/// Migration variants.
#[cw_serde]
pub enum Migration {
    /// Code-only migration.
    CodeOnly,
    /// Re-instantiate the client.
    Reinstantiate(InstantiateMsg),
}

/// Verify client message request.
#[cw_serde]
pub struct VerifyClientMessageMsg {
    /// The serialized client message.
    pub client_message: Binary,
}

/// Check for misbehaviour request.
#[cw_serde]
pub struct CheckForMisbehaviourMsg {
    /// The serialized client message.
    pub client_message: Binary,
}

/// Timestamp at height request.
#[cw_serde]
pub struct TimestampAtHeightMsg {
    /// The height to query.
    pub height: Height,
}

/// Status request.
#[cw_serde]
pub struct StatusMsg {}

/// Verify membership request.
#[cw_serde]
pub struct VerifyMembershipMsg {
    /// The height.
    pub height: Height,
    /// The delay time period.
    pub delay_time_period: u64,
    /// The delay block period.
    pub delay_block_period: u64,
    /// The proof.
    pub proof: Binary,
    /// The merkle path.
    pub merkle_path: MerklePath,
    /// The value.
    pub value: Binary,
}

/// Verify non-membership request.
#[cw_serde]
pub struct VerifyNonMembershipMsg {
    /// The height.
    pub height: Height,
    /// The delay time period.
    pub delay_time_period: u64,
    /// The delay block period.
    pub delay_block_period: u64,
    /// The proof.
    pub proof: Binary,
    /// The merkle path.
    pub merkle_path: MerklePath,
}

/// Update state request.
#[cw_serde]
pub struct UpdateStateMsg {
    /// The serialized client message.
    pub client_message: Binary,
}

/// Misbehaviour request.
#[cw_serde]
pub struct MisbehaviourMsg {
    /// The serialized client message.
    pub client_message: Binary,
}

/// Verify upgrade and update state request.
#[cw_serde]
pub struct VerifyUpgradeAndUpdateStateMsg {
    /// The upgrade client state.
    pub upgrade_client_state: Binary,
    /// The upgrade consensus state.
    pub upgrade_consensus_state: Binary,
    /// The proof upgrade client.
    pub proof_upgrade_client: Binary,
    /// The proof upgrade consensus state.
    pub proof_upgrade_consensus_state: Binary,
}

/// Migrate client store request.
#[cw_serde]
pub struct MigrateClientStoreMsg {}

/// A height.
#[cw_serde]
pub struct Height {
    /// The revision number.
    pub revision_number: u32,
    /// The revision height.
    pub revision_height: u32,
}

/// A merkle path.
#[cw_serde]
pub struct MerklePath {
    /// The key path.
    pub key_path: Vec<Binary>,
}

/// The update state result.
#[cw_serde]
pub struct UpdateStateResult {
    /// The heights that were updated.
    pub heights: Vec<Height>,
}