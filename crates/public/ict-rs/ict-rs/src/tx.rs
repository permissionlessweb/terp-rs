use serde::{Deserialize, Serialize};

/// Output from executing a command in a container.
#[derive(Debug, Clone, Default)]
pub struct ExecOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub exit_code: i64,
}

impl ExecOutput {
    pub fn stdout_str(&self) -> String {
        String::from_utf8_lossy(&self.stdout).to_string()
    }

    pub fn stderr_str(&self) -> String {
        String::from_utf8_lossy(&self.stderr).to_string()
    }
}

/// A committed transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub height: u64,
    pub tx_hash: String,
    pub gas_spent: u64,
    pub packet: Option<Packet>,
}

/// An IBC packet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub sequence: u64,
    pub source_port: String,
    pub source_channel: String,
    pub dest_port: String,
    pub dest_channel: String,
    pub data: Vec<u8>,
    pub timeout_height: String,
    pub timeout_timestamp: u64,
}

/// A packet with its acknowledgement data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketAcknowledgement {
    pub packet: Packet,
    pub acknowledgement: Vec<u8>,
}

/// A timed-out packet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketTimeout {
    pub packet: Packet,
}

/// An amount of tokens associated with an address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAmount {
    pub address: String,
    pub denom: String,
    pub amount: u128,
}

/// Options for IBC transfers.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransferOptions {
    pub timeout_height: Option<u64>,
    pub timeout_timestamp: Option<u64>,
    pub memo: Option<String>,
}
