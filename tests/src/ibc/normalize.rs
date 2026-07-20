//! Ordering strings, alpha-side channel remap, Terp channel map builder.

use crate::ibc::routes::TerpChannelInfo;
use serde_json::{Value, json};
use std::collections::HashMap;

/// Convert IBC ordering int to schema-compliant string.
/// Cosmos SDK gRPC returns: ORDER_NONE_UNSPECIFIED=0, ORDER_UNORDERED=1, ORDER_ORDERED=2
pub fn ordering_to_str(ord_val: &serde_json::Value) -> String {
    match ord_val.as_str() {
        Some(s) if s == "ordered" || s == "unordered" => s.to_string(),
        _ => match ord_val.as_i64().unwrap_or(1) {
            1 => "unordered".to_string(),
            2 => "ordered".to_string(),
            _ => "unordered".to_string(),
        },
    }
}

/// Pure helper: given internal channel entries (chain_1 = terp side from query)
/// and the alpha decision, produce schema-compliant final channels with correct
/// side mapping + preferred tags.
///
/// `chain_1_name` is the alpha-ordered first chain name in the final entry.
/// Raw channels store terp on chain_1 and counterparty on chain_2.
pub fn finalize_channels_for_ibc_entry(
    raw_channels: &[Value],
    chain_1_name: &str,
    _counterparty_name: &str,
    client_status: &str,
) -> Vec<Value> {
    let mut has_preferred_transfer = false;
    let mut final_channels: Vec<Value> = Vec::new();
    for ch in raw_channels {
        let stored_terp = &ch["chain_1"];
        let stored_cp = &ch["chain_2"];
        let (out_c1, out_c2) = if chain_1_name == "terp" {
            (stored_terp.clone(), stored_cp.clone())
        } else {
            (stored_cp.clone(), stored_terp.clone())
        };
        let port_1 = out_c1["port_id"].as_str().unwrap_or("");
        let port_2 = out_c2["port_id"].as_str().unwrap_or("");
        let is_transfer = port_1 == "transfer" && port_2 == "transfer";
        let mut tags = ch["tags"].clone();
        if is_transfer && !has_preferred_transfer {
            if client_status == "Active" {
                tags["preferred"] = json!(true);
                has_preferred_transfer = true;
            } else {
                tags["preferred"] = json!(false);
            }
        } else if is_transfer {
            tags["preferred"] = json!(false);
        } else {
            tags["preferred"] = json!(true);
        }
        final_channels.push(json!({
            "chain_1": { "channel_id": out_c1["channel_id"], "port_id": out_c1["port_id"] },
            "chain_2": { "channel_id": out_c2["channel_id"], "port_id": out_c2["port_id"] },
            "ordering": ordering_to_str(&ch["ordering"]),
            "version": ch["version"],
            "tags": tags,
        }));
    }
    final_channels
}

/// Build a channel-to-chain map from ibc_data in state.json (Terp-centric).
pub fn build_channel_to_chain_map(
    ibc_data: &serde_json::Value,
) -> HashMap<String, TerpChannelInfo> {
    let mut map = HashMap::new();

    let obj = match ibc_data.as_object() {
        Some(o) => o,
        None => return map,
    };

    for (key, data) in obj {
        let (counterparty_name, terp_is_chain1) =
            if data["chain_1"].is_object() && data["chain_2"].is_object() {
                let chain_1_name = data["chain_1"]["chain_name"].as_str().unwrap_or("");
                let chain_2_name = data["chain_2"]["chain_name"].as_str().unwrap_or("");

                if chain_1_name == "terp" {
                    (chain_2_name.to_string(), true)
                } else if chain_2_name == "terp" {
                    (chain_1_name.to_string(), false)
                } else {
                    continue;
                }
            } else {
                let cp_name = key
                    .strip_suffix("-terp")
                    .or_else(|| key.strip_prefix("terp-"))
                    .unwrap_or(key);
                (cp_name.to_string(), false)
            };

        if counterparty_name.is_empty() {
            continue;
        }

        let channels = match data["channels"].as_array() {
            Some(c) => c,
            None => continue,
        };

        let preferred_channel = channels
            .iter()
            .find(|c| {
                let has_transfer_ports = if data["chain_1"].is_object() {
                    c["chain_1"]["port_id"].as_str() == Some("transfer")
                        && c["chain_2"]["port_id"].as_str() == Some("transfer")
                } else {
                    c[counterparty_name.as_str()]["port_id"].as_str() == Some("transfer")
                        && c["terp"]["port_id"].as_str() == Some("transfer")
                };
                has_transfer_ports && c["tags"]["preferred"].as_bool() == Some(true)
            })
            .or_else(|| {
                channels.iter().find(|c| {
                    if data["chain_1"].is_object() {
                        c["chain_1"]["port_id"].as_str() == Some("transfer")
                            && c["chain_2"]["port_id"].as_str() == Some("transfer")
                    } else {
                        c[counterparty_name.as_str()]["port_id"].as_str() == Some("transfer")
                            && c["terp"]["port_id"].as_str() == Some("transfer")
                    }
                })
            });

        let (terp_channel, cp_channel) = match preferred_channel {
            Some(c) => {
                if data["chain_1"].is_object() {
                    if terp_is_chain1 {
                        (
                            c["chain_1"]["channel_id"].as_str().unwrap_or("").to_string(),
                            c["chain_2"]["channel_id"].as_str().unwrap_or("").to_string(),
                        )
                    } else {
                        (
                            c["chain_2"]["channel_id"].as_str().unwrap_or("").to_string(),
                            c["chain_1"]["channel_id"].as_str().unwrap_or("").to_string(),
                        )
                    }
                } else {
                    (
                        c["terp"]["channel_id"].as_str().unwrap_or("").to_string(),
                        c[counterparty_name.as_str()]["channel_id"]
                            .as_str()
                            .unwrap_or("")
                            .to_string(),
                    )
                }
            }
            None => continue,
        };

        if !terp_channel.is_empty() {
            map.insert(
                counterparty_name.clone(),
                TerpChannelInfo {
                    terp_channel_id: terp_channel,
                    counterparty_channel_id: cp_channel,
                    counterparty_chain_name: counterparty_name,
                },
            );
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ibc::hash::compute_ibc_denom_hash;
    use serde_json::json;

    #[test]
    fn ordering_int_to_string() {
        assert_eq!(ordering_to_str(&json!(1)), "unordered");
        assert_eq!(ordering_to_str(&json!(2)), "ordered");
        assert_eq!(ordering_to_str(&json!("ordered")), "ordered");
        assert_eq!(ordering_to_str(&json!("unordered")), "unordered");
        assert_eq!(ordering_to_str(&json!(0)), "unordered");
    }

    #[test]
    fn finalize_channels_and_channel_map_roundtrip_akash_before_terp() {
        // Raw storage: terp as chain_1 from query; counterparty akash alpha-before terp
        let raw_chans = vec![json!({
            "chain_1": { "channel_id": "channel-6", "port_id": "transfer" },
            "chain_2": { "channel_id": "channel-115", "port_id": "transfer" },
            "ordering": 1,
            "version": "ics20-1",
            "tags": { "status": "ACTIVE" }
        })];
        let final_chs = finalize_channels_for_ibc_entry(&raw_chans, "akash", "akash", "Active");
        assert_eq!(final_chs.len(), 1);
        assert_eq!(final_chs[0]["chain_1"]["channel_id"], "channel-115");
        assert_eq!(final_chs[0]["chain_2"]["channel_id"], "channel-6");
        assert_eq!(final_chs[0]["tags"]["preferred"], true);
        assert_eq!(final_chs[0]["ordering"], "unordered");

        let ibc_entry = json!({
            "$schema": "../ibc_data.schema.json",
            "chain_1": {
                "chain_name": "akash",
                "chain_id": "akashnet-2",
                "client_id": "c1",
                "connection_id": "conn1"
            },
            "chain_2": {
                "chain_name": "terp",
                "chain_id": "morocco-1",
                "client_id": "c2",
                "connection_id": "conn2"
            },
            "channels": final_chs
        });
        let mut ibc_data = serde_json::Map::new();
        ibc_data.insert("akash".to_string(), ibc_entry);
        let map = build_channel_to_chain_map(&serde_json::Value::Object(ibc_data));
        assert_eq!(map.len(), 1);
        let info = map.get("akash").unwrap();
        assert_eq!(info.terp_channel_id, "channel-6");
        assert_eq!(info.counterparty_channel_id, "channel-115");

        for native in ["uterp", "uthiol"] {
            let path = format!("transfer/{}/{}", info.counterparty_channel_id, native);
            let hash = compute_ibc_denom_hash(&path);
            assert!(hash.starts_with("ibc/"));
        }
    }
}
