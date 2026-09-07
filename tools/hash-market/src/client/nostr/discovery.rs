use crate::client::nostr::HashMerchantRelayInfo;

/// Build a NIP-87-style discovery event tags from relay info.
pub fn build_discovery_tags(info: &HashMerchantRelayInfo) -> Vec<Vec<String>> {
    let mut tags: Vec<Vec<String>> = Vec::new();

    // d-tag — replaceable event identifier (relay pubkey)
    tags.push(vec!["d".to_string(), info.pubkey.clone()]);

    // relay tag — the Nostr relay URL
    tags.push(vec!["relay".to_string(), info.relay_url.clone()]);

    // chain tags — one per supported chain
    for chain in &info.chains {
        tags.push(vec!["chain".to_string(), chain.clone()]);
    }

    // algo tags — one per supported algorithm
    for algo in &info.algos {
        tags.push(vec!["algo".to_string(), algo.clone()]);
    }
    // ws tag (if configured)
    if let Some(ws) = &info.ws_url {
        tags.push(vec!["ws".to_string(), ws.clone()]);
    }

    // webhook tag (if configured)
    if let Some(wh) = &info.webhook_url {
        tags.push(vec!["webhook".to_string(), wh.clone()]);
    }

    // bud tag (if configured)
    if let Some(bud) = &info.bud_cid {
        tags.push(vec!["bud".to_string(), bud.clone()]);
    }

    tags
}

/// Parse a NIP-87 discovery event tags into a relay info struct.
pub fn parse_discovery_tags(pubkey: &str, tags: &[Vec<String>]) -> HashMerchantRelayInfo {
    let mut info = HashMerchantRelayInfo {
        relay_url: String::new(),
        chains: Vec::new(),
        algos: Vec::new(),
        ws_url: None,
        webhook_url: None,
        bud_cid: None,
        pubkey: pubkey.to_string(),
    };

    for tag in tags {
        if tag.len() < 2 {
            continue;
        }
        let name = &tag[0];
        let value = &tag[1];
        match name.as_str() {
            "relay" => info.relay_url = value.clone(),
            "chain" => info.chains.push(value.clone()),
            "algo" => info.algos.push(value.clone()),
            "ws" => info.ws_url = Some(value.clone()),
            "webhook" => info.webhook_url = Some(value.clone()),
            "bud" => info.bud_cid = Some(value.clone()),
            _ => {}
        }
    }

    info
}

/// Build the content JSON string for a discovery event.
pub fn build_discovery_content(name: &str, description: &str) -> String {
    serde_json::json!({
        "name": name,
        "description": description,
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_discovery_tags() {
        let info = HashMerchantRelayInfo {
            relay_url: "wss://hashmerchant.example.com".into(),
            chains: vec!["ethereum-mainnet".into(), "cosmoshub-4".into()],
            algos: vec!["keccak256".into()],
            ws_url: Some("ws://127.0.0.1:9444".into()),
            webhook_url: Some("https://example.com/webhook/root".into()),
            bud_cid: Some("QmTest123".into()),
            pubkey: "0".repeat(64),
        };

        let tags = build_discovery_tags(&info);
        let parsed = parse_discovery_tags(&info.pubkey, &tags);

        assert_eq!(parsed.relay_url, info.relay_url);
        assert_eq!(parsed.chains.len(), 2);
        assert!(parsed.chains.contains(&"ethereum-mainnet".to_string()));
        assert!(parsed.chains.contains(&"cosmoshub-4".to_string()));
        assert_eq!(parsed.algos, info.algos);
        assert_eq!(parsed.ws_url, info.ws_url);
        assert_eq!(parsed.webhook_url, info.webhook_url);
        assert_eq!(parsed.bud_cid, info.bud_cid);
        assert_eq!(parsed.pubkey, info.pubkey);
    }
}
