use std::collections::HashMap;

use anyhow::{anyhow, Context};
use cw_orch::{
    daemon::{
        networks::{chain_name_from_id, OSMOSIS_1, TERP_MAINNET},
        queriers::Ibc,
        Daemon, DaemonState,
    },
    environment::{ChainInfo, ChainKind, ChainState, NetworkInfo, QuerierGetter},
    prelude::*,
};
use cw_orch_interchain::prelude::*;
use serde_json::{json, Value};
use terp_rs::{ibc::lightclients::tendermint::v1::ClientState, Message};
use tracing::warn;

// ANCHOR: akash
pub const AKASH_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "akash",
    pub_address_prefix: "akash",
    coin_type: 118u32,
};

pub const AKASH_MAINNET: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "akash-1",
    gas_denom: "uakt",
    gas_price: 0.025,
    grpc_urls: &["https://grpc-akash.ecostake.com:443"],
    network_info: AKASH_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

// ANCHOR: atomone
pub const ATOMEONE_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "akash",
    pub_address_prefix: "akash",
    coin_type: 118u32,
};

pub const ATOMEONE_MAINNET: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "akash-1",
    gas_denom: "uakt",
    gas_price: 0.025,
    grpc_urls: &["https://grpc-akash.ecostake.com:443"],
    network_info: ATOMEONE_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

fn main() -> anyhow::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();
    env_logger::init();
    dotenv::dotenv().ok();

    println!("=== Method 1: Derive from state.json assets ===\n");

    derive_full_ibc_state()
    derive_full_ibc_asset_state()
}

/// Derive full ibc state for a given chain specified in the chain registry format:
/// - ibc_data schema defined client, channel, connection, port information
/// - asset_list: IBC denom hashes for foreign tokens defined in state.json assets list
fn derive_full_ibc_data_state() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let interchain = DaemonInterchain::new(
        vec![
            TERP_MAINNET.clone(),
            OSMOSIS_1.clone(),
            // ATOMEONE_MAINNET.clone(),
            // AKASH_MAINNET.clone(),
        ],
        &ChannelCreationValidator,
    )?;
    let terp: Daemon = interchain.get_chain("morocco-1")?;
    let osmosis: Daemon = interchain.get_chain("osmosis-1")?;
    // let akash: Daemon = interchain.get_chain("akash-1")?;
    // let atone: Daemon = interchain.get_chain("atomone-1")?;
    let ibc: Ibc = terp.querier();

    let state_terp = terp.state();
    let state_osmo = osmosis.state();
    // let state_akt = akt.state();
    // let state_atone = atone.state();

    let d = Vec::with_capacity(1);
    let mut assetlist = Vec::new();
    let assosmo = state_osmo.get("assets")?;
    let assosmo = assosmo.as_array().unwrap_or(&d);
      assetlist.extend(assosmo);
    // let assakt = state_akt.get("assets")?;
    // let assakt = assakt.as_array().unwrap_or(&d);
    // assetlist.extend(assakt);
    // let assatone = state_atone.get("assets")?;
    // let assatone = assatone.as_array().unwrap_or(&d);
    // assetlist.extend(assatone);

    let clients = terp.rt_handle.block_on(async {
        ibc._clients()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query clients: {}", e))
    })?;
    println!("{} IBC clients on {}\n",clients.len(),terp.chain_id());

    let mut ibc_by_chain: HashMap<String, serde_json::Value> = HashMap::new();

    for c in &clients {
        let (clientid,counterpartychainid, counterpartyname) = if let Some(raw) = &c.client_state {
            match ClientState::decode(raw.value.as_slice()) {
                Ok(tmstate) => (
                     &c.client_id,
                    tmstate.chain_id.clone(),
                    chain_name_from_id(&tmstate.chain_id),
                ),
                Err(_) => {
                    println!("  ⚠ Could not decode client state for {}", c.client_id);
                    continue;
                }
            }
        } else {
            println!("  ⚠ No client state for {}", c.client_id);
            continue;
        };

        // Get connections for this client
        let (ccons, e) = terp.rt_handle.block_on(async {
            match ibc._client_connections(clientid).await.map_err(|e| {
                anyhow::anyhow!("Failed to query connections for {}: {}", c.client_id, e)
            }) {
                Ok(cc) => (cc, String::default()),
                Err(e) => (vec![], e.to_string()),
            }
        });
        if ccons.is_empty() || !e.is_empty() {
            warn!("error_result:{}", e);
            warn!("connection count for {}:{}", clientid, ccons.len());
            continue;
        }

        for conid in &ccons {
            let ends = terp
                .rt_handle
                .block_on(async {
                    ibc._connection_end(conid)
                        .await
                        .map_err(|e| anyhow::anyhow!("Failed to query connection {}: {}", conid, e))
                })?
                .expect("no end");

            let (counterpartyclientid, counterpartyconnectionid) = match &ends.counterparty {
                Some(e) => (&e.client_id, &e.connection_id),
                None => {
                    println!(
                        "  ⚠ No client_id or connection_id for connection end {}",
                        ends.client_id
                    );
                    continue;
                }
            };

            let chans = terp.rt_handle.block_on(async {
                ibc._connection_channels(conid)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to query connection {}: {}", conid, e))
            })?;

            let mut channel_entries: Vec<Value> = Vec::new();

            for chan in chans {
                let channel_id = &chan.channel_id;
                let port_id = &chan.port_id;
                let counterparty = &chan.counterparty.clone().expect("");
                let counterpartychannelid = &counterparty.channel_id;
                let counterparty_port_id = &counterparty.port_id;
                let ord = &chan.ordering;
                let v = &chan.version;
                let state = chan.state().as_str_name();
                let status = if state == "STATE_OPEN" {
                    "ACTIVE"
                } else {
                    "INACTIVE"
                };
                println!(
                    "    Channel: {} ({}) <-> {} ({}) | {} | {} | {} | {}",
                    channel_id,
                    port_id,
                    counterpartychannelid,
                    counterparty_port_id,
                    ord,
                    v,
                    state,
                    status,
                );

                channel_entries.push(json!({
                    "terp": {
                        "channel_id": channel_id,
                        "port_id": port_id,
                    },
                    counterpartyname.clone(): {
                        "channel_id": counterpartychannelid,
                        "port_id": counterparty_port_id,
                    },
                    "ordering": ord,
                    "version": v,
                    "tags": {
                        "status": status,
                    }
                }));
            }
            // Build or merge IBC data for this counterparty chain
            let existing = ibc_by_chain.get(&counterpartyname.clone());
            let existing_channels = existing
                .and_then(|v| v["channels"].as_array().cloned())
                .unwrap_or_default();

            let all_channels: Vec<Value> = existing_channels
                .into_iter()
                .chain(channel_entries)
                .collect();

            ibc_by_chain.insert(
                counterpartyname.clone(),
                json!({
                    "counterparty_chain_id": counterpartychainid,
                    "terp_client_id": clientid,
                    "terp_connection_id": conid,
                    "counterparty_client_id": counterpartyclientid,
                    "counterparty_connection_id": counterpartyconnectionid,
                    "channels": all_channels,
                }),
            );
        }
    }

    // Step 2: Build final IBC data schema JSON for each counterparty chain
    let mut ibc_data_output: Vec<Value> = Vec::new();

    for (counterpartyname, raw_data) in ibc_by_chain.iter() {
        let counterparty_chain_id = raw_data["counterparty_chain_id"].as_str().unwrap_or("");
        let terp_client_id = raw_data["terp_client_id"].as_str().unwrap_or("");
        let terp_connection_id = raw_data["terp_connection_id"].as_str().unwrap_or("");
        let counterparty_client_id = raw_data["counterparty_client_id"].as_str().unwrap_or("");
        let counterparty_connection_id = raw_data["counterparty_connection_id"]
            .as_str()
            .unwrap_or("");

        // Determine alphabetical ordering for chain names
        let (chain_1_name, chain_2_name) = if "terp" < counterpartyname.as_str() {
            ("terp", counterpartyname.as_str())
        } else {
            (counterpartyname.as_str(), "terp")
        };

        // Build chain_1 and chain_2 data based on alphabetical order
        let (chain_1_data, chain_2_data) = if chain_1_name == "terp" {
            (
                json!({
                    "chain_name": "terp",
                    "client_id": terp_client_id,
                    "connection_id": terp_connection_id,
                    "chain_id": "morocco-1",
                }),
                json!({
                    "chain_name": counterpartyname,
                    "client_id": counterparty_client_id,
                    "connection_id": counterparty_connection_id,
                    "chain_id": counterparty_chain_id,
                }),
            )
        } else {
            (
                json!({
                    "chain_name": counterpartyname,
                    "client_id": counterparty_client_id,
                    "connection_id": counterparty_connection_id,
                    "chain_id": counterparty_chain_id,
                }),
                json!({
                    "chain_name": "terp",
                    "client_id": terp_client_id,
                    "connection_id": terp_connection_id,
                    "chain_id": "morocco-1",
                }),
            )
        };

        // Adjust channel entries based on alphabetical order and set preferred tags
        let mut has_preferred_transfer = false;
        let mut final_channels: Vec<Value> = Vec::new();

        for ch in raw_data["channels"].as_array().unwrap_or(&vec![]) {
            // Swap chain_1/chain_2 if terp is not chain_1
            let (ch_1, ch_2) = if chain_1_name == "terp" {
                (ch["terp"].clone(), ch[counterpartyname.clone()].clone())
            } else {
                (ch[counterpartyname.clone()].clone(), ch["terp"].clone())
            };

            let port_1 = ch_1["port_id"].as_str().unwrap_or("");
            let port_2 = ch_2["port_id"].as_str().unwrap_or("");
            let is_transfer = port_1 == "transfer" && port_2 == "transfer";

            let mut tags = ch["tags"].clone();

            // Only ONE transfer/transfer channel can be preferred
            if is_transfer && !has_preferred_transfer {
                tags["preferred"] = json!(true);
                has_preferred_transfer = true;
            } else if is_transfer {
                tags["preferred"] = json!(false);
            } else {
                // Non-transfer channels (ICA, CW20, etc.) can each be preferred
                tags["preferred"] = json!(true);
            }

            final_channels.push(json!({
                "terp": ch_1,
                counterpartyname.clone(): ch_2,
                "ordering": ch["ordering"],
                "version": ch["version"],
                "tags": tags,
            }));
        }

        // Build the full IBC data entry
        let filename = format!("{}-{}.json", chain_1_name, chain_2_name);

        let ibc_entry = json!({
            "$schema": "../ibc_data.schema.json",
            chain_1_name: chain_1_data,
            chain_2_name: chain_2_data,
            "channels": final_channels,
        });

        println!("\n=== {} ===", filename);
        println!("{}", serde_json::to_string_pretty(&ibc_entry)?);

        ibc_data_output.push(json!({
            "filename": filename,
            "data": ibc_entry,
        }));
    }

    // Step 3: Write to state file
    let mut state_mut = terp.state().clone();
    for entry in &ibc_data_output {
        let filename = entry["filename"].as_str().unwrap_or("unknown");
        let counterpartyname = filename.replace("terp-", "").replace(".json", "");
        state_mut.set("ibc_data", &counterpartyname, entry["data"].clone())?;
    }
    state_mut.force_write()?;

    println!(
        "\n✓ {} IBC data entries written to state.json",
        ibc_data_output.len()
    );

    // build ibc asset entries
    // build_ibc_asset_entry(assetlist)

    Ok(())
}


// fn build_ibc_asset_entry(
//     asset: &serde_json::Value,
//     ibc_denom: &str,
//     trace: &serde_json::Value,
//     trace_path: &str,
//     symbol: &str,
//     counterparty_chain: &str,
//     counterparty_denom: &str,
//     dest_channel: &str,
// ) -> serde_json::Value {
//     let display_denom = asset["display"].as_str().unwrap_or("");
//     let display_exponent = asset["denom_units"]
//         .as_array()
//         .and_then(|units| units.iter().find(|u| u["denom"] == display_denom))
//         .and_then(|u| u["exponent"].as_u64())
//         .unwrap_or(6);

//     serde_json::json!({
//         "description": asset["description"].as_str().unwrap_or(""),
//         "denom_units": [
//             {
//                 "denom": ibc_denom,
//                 "exponent": 0,
//                 "aliases": [counterparty_denom]
//             },
//             {
//                 "denom": display_denom,
//                 "exponent": display_exponent
//             }
//         ],
//         "type_asset": "ics20",
//         "base": ibc_denom,
//         "name": asset["name"].as_str().unwrap_or(""),
//         "display": display_denom,
//         "symbol": symbol,
//         "traces": [{
//             "type": "ibc",
//             "counterparty": {
//                 "chain_name": counterparty_chain,
//                 "base_denom": counterparty_denom,
//                 "channel_id": trace["counterparty"]["channel_id"].as_str().unwrap_or("")
//             },
//             "chain": {
//                 "channel_id": dest_channel,
//                 "path": trace_path
//             }
//         }]
//     })
// }

// /// Compute IBC denom hash locally from trace path
// /// The IBC spec defines the hash as SHA256(trace_path), formatted as uppercase hex
// fn compute_ibc_denom_hash(trace_path: &str) -> String {
//     use sha2::{Digest, Sha256};
//     let mut hasher = Sha256::new();
//     hasher.update(trace_path.as_bytes());
//     let result = hasher.finalize();
//     format!("ibc/{}", hex::encode(result).to_uppercase())
// }

// fn derive_ibc_asset_list() {
// let mut ibc_denoms: Vec<serde_json::Value> = vec![];
//     for asset in assetlist {
//         // Only process native IBC tokens (skip factory denoms)
//         let type_asset = asset["type_asset"].as_str().unwrap_or("");
//         if type_asset != "ics20" {
//             continue;
//         }

//         let symbol = asset["symbol"].as_str().unwrap_or("UNKNOWN");
//         let base_denom = asset["base"].as_str().unwrap_or("");

//         // Skip if already an ibc/ denom (already derived)
//         if base_denom.starts_with("ibc/") {
//             println!("⏭ {} - already has IBC denom: {}", symbol, base_denom);
//             continue;
//         }

//         // Get trace path from asset traces
//         let traces = asset["traces"].as_array().unwrap_or(&d);
//         if traces.is_empty() {
//             println!("⏭ {} - no traces found", symbol);
//             continue;
//         }

//         // Find the IBC trace (skip additional-mintage etc)
//         let ibc_trace = traces.iter().find(|t| t["type"] == "ibc");
//         let trace = match ibc_trace {
//             Some(t) => t,
//             None => {
//                 println!("⏭ {} - no IBC trace found", symbol);
//                 continue;
//             }
//         };

//         let trace_path = trace["chain"]["path"].as_str().unwrap_or("");
//         let counterparty_chain = trace["counterparty"]["chain_name"].as_str().unwrap_or("");
//         let counterparty_denom = trace["counterparty"]["base_denom"].as_str().unwrap_or("");
//         let dest_channel = trace["chain"]["channel_id"].as_str().unwrap_or("");

//         println!("Processing {} from {}...", symbol, counterparty_chain);
//         println!("  Trace path: {}", trace_path);
//         println!("  Channel: {}", dest_channel);
//         terp.rt_handle.block_on(async {
//             // Derive the IBC denom hash from the trace path
//             match ibc._denom_hash(trace_path.to_string()).await {
//                 Ok(hash) => {
//                     let ibc_denom = format!("ibc/{}", hash);
//                     println!("  ✓ IBC denom (on-chain): {}", ibc_denom);
//                     ibc_denoms.push(build_ibc_asset_entry(
//                         asset,
//                         &ibc_denom,
//                         trace,
//                         trace_path,
//                         symbol,
//                         counterparty_chain,
//                         counterparty_denom,
//                         dest_channel,
//                     ));
//                 }
//                 Err(e) => {
//                     let ibc_denom = compute_ibc_denom_hash(trace_path);
//                     println!("  ⚠ On-chain query failed: {}", e);
//                     println!("  ✓ IBC denom (derived locally): {}", ibc_denom);
//                     ibc_denoms.push(build_ibc_asset_entry(
//                         asset,
//                         &ibc_denom,
//                         trace,
//                         trace_path,
//                         symbol,
//                         counterparty_chain,
//                         counterparty_denom,
//                         dest_channel,
//                     ));
//                 }
//             }
//         });

//         println!();
//     }
//     // Write ibc_denoms back to state
//     // must write denoms in json scema spec of asasset list for chain registries, in the terp-chain state. this follows the design forspecifying tokens

//     if !ibc_denoms.is_empty() {
//         let mut state_mut = state_terp.clone();

//         // Merge with existing assets array
//         let mut existing_assets = state_terp
//             .get("assets")
//             .ok()
//             .and_then(|a| a.as_array().cloned())
//             .unwrap_or_default();

//         // Update existing IBC assets or append new ones
//         for new_asset in &ibc_denoms {
//             let base = new_asset["base"].as_str().unwrap_or("");
//             if let Some(pos) = existing_assets.iter().position(|a| a["base"] == base) {
//                 existing_assets[pos] = new_asset.clone();
//             } else {
//                 existing_assets.push(new_asset.clone());
//             }
//         }

//         state_mut.set("assets", "ibc", serde_json::Value::Array(existing_assets))?;
//         state_mut.force_write()?;
//         println!(
//             "✓ {} IBC assets written to state.json in assetlist schema format",
//             ibc_denoms.len()
//         );
//     }
// }
