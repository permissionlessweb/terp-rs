//! MinioIpfsClient — HashMerchantClient impl using local Kubo RPC + MinIO.
//! TODOTODOTODO: this should accepts websocket or webhooks from minio server to partipate and stateful actions
//! Uses the local IPFS node (127.0.0.1:5001) for CID operations.
//! Roots are logged to stderr — MinIO integration for durable storage
//! is available as a hook.

use crate::client::nostr::{ClientError, ClientResult, HashMerchantRelayInfo, NegentropyFilter};
use crate::client::{HashMerchantClient, RootConfirmation};

/// Default Kubo RPC endpoint.
const KUBO_API: &str = "http://127.0.0.1:5001";

/// HashMerchantClient backed by a local IPFS node.
///
/// - `fetch_cid` / `pin_cid` use Kubo RPC at 127.0.0.1:5001
/// - `on_root_confirmed` logs the root to stderr
/// - `negentropy_sync` is a stub (requires a Nostr relay connection)
/// - `discover_relays` is a stub (requires a Nostr relay connection)
#[derive(Debug, Clone)]
pub struct MinioIpfsClient {
    kubo_api_url: String,
    http_client: reqwest::Client,
}

impl MinioIpfsClient {
    pub fn new() -> Self {
        Self {
            kubo_api_url: KUBO_API.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub fn with_kubo_url(kubo_api_url: impl Into<String>) -> Self {
        Self {
            kubo_api_url: kubo_api_url.into(),
            http_client: reqwest::Client::new(),
        }
    }
}

impl Default for MinioIpfsClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl HashMerchantClient for MinioIpfsClient {
    async fn on_root_confirmed(&self, confirmation: RootConfirmation) -> ClientResult<()> {
        let payload = &confirmation.payload;
        eprintln!(
            "[hashmerchant] root confirmed: chain={}, algo={}, height={}, attestations={}, root={}",
            payload.chain_uid,
            payload.algo,
            payload.height,
            payload.attestation_count,
            hex::encode(&payload.root).get(..16).unwrap_or("???"),
        );

        // Pin the associated CID if provided
        if let Some(cid) = &confirmation.cid {
            self.pin_cid(cid).await.ok();
        }

        Ok(())
    }

    async fn negentropy_sync(
        &self,
        _relay: &str,
        filter: &NegentropyFilter,
    ) -> ClientResult<Vec<RootConfirmation>> {
        // MinioIpfsClient doesn't connect to Nostr relays directly.
        // Use NostrRelayClient for that. This returns empty.
        eprintln!(
            "[hashmerchant] negentropy_sync: not supported on MinioIpfsClient. \
             Use NostrRelayClient for chain={}",
            filter.chains.join(","),
        );
        Ok(vec![])
    }

    async fn discover_relays(&self, _chain_uid: &str) -> ClientResult<Vec<HashMerchantRelayInfo>> {
        // MinioIpfsClient doesn't query Nostr relays.
        Ok(vec![])
    }

    async fn fetch_cid(&self, cid: &str) -> ClientResult<Vec<u8>> {
        let url = format!("{}/api/v0/cat?arg={cid}", self.kubo_api_url);
        let resp = self
            .http_client
            .post(&url)
            .send()
            .await
            .map_err(|e| ClientError::Ipfs(format!("Kubo cat failed: {e}")))?;

        if !resp.status().is_success() {
            return Err(ClientError::Ipfs(format!(
                "Kubo cat returned {}",
                resp.status()
            )));
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| ClientError::Ipfs(format!("read response failed: {e}")))?;
        Ok(bytes.to_vec())
    }

    async fn pin_cid(&self, cid: &str) -> ClientResult<()> {
        let url = format!("{}/api/v0/pin/add?arg={cid}", self.kubo_api_url);
        let resp = self
            .http_client
            .post(&url)
            .send()
            .await
            .map_err(|e| ClientError::Ipfs(format!("Kubo pin failed: {e}")))?;

        if !resp.status().is_success() {
            return Err(ClientError::Ipfs(format!(
                "Kubo pin returned {}",
                resp.status()
            )));
        }

        eprintln!("[hashmerchant] pinned CID: {cid}");
        Ok(())
    }
}
