//! gRPC transport listener (stub).
//!
//! Listens on a TCP socket for length-prefixed protobuf `VoteExtensionHashData`
//! messages. This implements a simple framing protocol (4-byte big-endian length
//! prefix) rather than full gRPC, keeping dependencies minimal.

use crate::msg::VoteExtensionHashData;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub async fn run(listen_addr: String, tx: mpsc::Sender<VoteExtensionHashData>) {
    let listener = match TcpListener::bind(&listen_addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("grpc transport: failed to bind {listen_addr}: {e}");
            return;
        }
    };

    loop {
        let (mut stream, peer) = match listener.accept().await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("grpc transport: accept error: {e}");
                continue;
            }
        };

        let tx = tx.clone();
        tokio::spawn(async move {
            loop {
                // Read 4-byte length prefix
                let mut len_buf = [0u8; 4];
                if stream.read_exact(&mut len_buf).await.is_err() {
                    break;
                }
                let msg_len = u32::from_be_bytes(len_buf) as usize;
                if msg_len > 1_048_576 {
                    eprintln!("grpc transport: message too large from {peer}");
                    break;
                }

                let mut buf = vec![0u8; msg_len];
                if stream.read_exact(&mut buf).await.is_err() {
                    break;
                }

                match VoteExtensionHashData::decode(&buf) {
                    Ok(data) => {
                        if tx.send(data).await.is_err() {
                            break; // receiver dropped
                        }
                    }
                    Err(e) => {
                        eprintln!("grpc transport: decode error from {peer}: {e}");
                    }
                }
            }
        });
    }
}
