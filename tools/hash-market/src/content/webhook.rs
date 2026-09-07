//! Outbound `content.available` fan-out (oline / Nostr bridge friendly).

use serde::{Deserialize, Serialize};

/// Unified event for S3 pins and BUD uploads.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentAvailableEvent {
    pub r#type: String,
    pub sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipfs_cid: Option<String>,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Origin kind: bud | s3 | ingest
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    pub labels: Vec<String>,
    pub urls: crate::content::registry::ContentUrls,
}

#[derive(Clone, Debug)]
pub struct WebhookFanout {
    urls: Vec<String>,
    bearer: Option<String>,
}

impl WebhookFanout {
    pub fn new(urls: Vec<String>, bearer: Option<String>) -> Self {
        Self { urls, bearer }
    }

    pub async fn fanout(&self, event: &ContentAvailableEvent) {
        if self.urls.is_empty() {
            return;
        }
        let body = match serde_json::to_vec(event) {
            Ok(b) => b,
            Err(e) => {
                tracing::warn!(error = %e, "content webhook serialize");
                return;
            }
        };
        let client = reqwest::Client::new();
        for url in &self.urls {
            let mut req = client
                .post(url)
                .header("Content-Type", "application/json")
                .body(body.clone());
            if let Some(t) = &self.bearer {
                req = req.header("Authorization", format!("Bearer {t}"));
            }
            match req.send().await {
                Ok(resp) if resp.status().is_success() => {
                    tracing::debug!(%url, "content.available delivered");
                }
                Ok(resp) => {
                    tracing::warn!(%url, status = %resp.status(), "content.available rejected");
                }
                Err(e) => {
                    tracing::warn!(%url, error = %e, "content.available POST failed");
                }
            }
        }
    }
}
