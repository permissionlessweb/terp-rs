//! Consensus state for the Crosslink light client.
//!
//! Stores the finalized state commitment at a given TFL height.

use crate::types::{SerializationError, ZcashDeserialize, ZcashSerialize};

/// Consensus state at a specific BFT height.
#[derive(Clone, Debug, PartialEq)]
pub struct ConsensusState {
    /// BFT height.
    pub bft_height: u32,
    /// PoW anchor block height (the finalized block).
    pub pow_anchor_height: u32,
    /// PoW anchor block hash.
    pub pow_anchor_hash: [u8; 32],
    /// Unix timestamp (seconds) of the anchor block.
    pub timestamp: u64,
    /// State commitment root (e.g., the application state root).
    pub state_commitment: [u8; 32],
}

impl ZcashSerialize for ConsensusState {
    fn zcash_serialize<W: std::io::Write>(&self, mut writer: W) -> Result<(), std::io::Error> {
        writer.write_all(&self.bft_height.to_le_bytes())?;
        writer.write_all(&self.pow_anchor_height.to_le_bytes())?;
        writer.write_all(&self.pow_anchor_hash)?;
        writer.write_all(&self.timestamp.to_le_bytes())?;
        writer.write_all(&self.state_commitment)?;
        Ok(())
    }
}

impl ZcashDeserialize for ConsensusState {
    fn zcash_deserialize<R: std::io::Read>(
        mut reader: R,
    ) -> Result<Self, SerializationError> {
        let mut one = [0u8; 4];
        let mut two = [0u8; 4];
        let mut three = [0u8; 32];
        let mut four = [0u8; 8];
        let mut five = [0u8; 32];
        reader.read_exact(&mut one)?;
        reader.read_exact(&mut two)?;
        reader.read_exact(&mut three)?;
        reader.read_exact(&mut four)?;
        reader.read_exact(&mut five)?;
        Ok(Self {
            bft_height: u32::from_le_bytes(one),
            pow_anchor_height: u32::from_le_bytes(two),
            pow_anchor_hash: three,
            timestamp: u64::from_le_bytes(four),
            state_commitment: five,
        })
    }
}
