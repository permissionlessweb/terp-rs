//! WebSocket transport listener (stub).
//!
//! Connects to a WebSocket endpoint and reads binary frames containing
//! protobuf-encoded `VoteExtensionHashData`.
//!
//! This is a minimal implementation using raw TCP + WebSocket framing.
//! For production use, consider adding `tokio-tungstenite`.

use crate::msg::VoteExtensionHashData;
use tokio::sync::mpsc;

pub async fn run(url: String, tx: mpsc::Sender<VoteExtensionHashData>) {
    // Stub: log and exit. A full implementation would do a WebSocket
    // upgrade handshake and read binary frames.
    eprintln!(
        "websocket transport: not yet implemented (url={url}). \
         Add tokio-tungstenite for full support."
    );

    // Keep the task alive so the transport doesn't immediately drop.
    // In a real implementation, this would be the read loop.
    let _ = tx; // keep sender alive
    tokio::signal::ctrl_c().await.ok();
}
