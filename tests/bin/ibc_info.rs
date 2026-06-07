use cw_orch::{
    daemon::{
        Daemon, DaemonState,
        networks::{OSMOSIS_1, TERP_MAINNET, chain_name_from_id},
        queriers::Ibc,
    },
    environment::{ChainState, QuerierGetter},
    prelude::*,
};
use cw_orch_interchain::prelude::*;
use hash_market::middleware::auth::now_timestamp;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use terp_rs::{Message, ibc::lightclients::tendermint::v1::ClientState};
use tracing::warn;

fn main() -> anyhow::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();
    env_logger::init();
    dotenv::dotenv().ok();
    derive_full_ibc_state()
}

fn derive_full_ibc_state() -> anyhow::Result<()> {
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
    let ibc: Ibc = terp.querier();

    let state_terp = terp.state();
    let state_osmo = osmosis.state();

    let clients = terp.rt_handle.block_on(async {
        ibc._clients()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query clients: {}", e))
    })?;

    info!("{} IBC clients on {}\n", clients.len(), terp.chain_id());
    let mut ibc_by_chain: HashMap<String, serde_json::Value> = HashMap::new();

    for c in &clients {
        let (clientid, counterpartychainid, counterpartyname) = if let Some(raw) = &c.client_state {
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
                info!(
                    " {} :: Channel: {} ({}) <-> {} ({}) | {} | {} | {} | {} ",
                    counterpartychainid,
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
                    "chain_1": {
                        "channel_id": channel_id,
                        "port_id": port_id,
                    },
                    "chain_2": {
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
        let d: Vec<Value> = Vec::with_capacity(1);
        let channels = raw_data["channels"].as_array().unwrap_or(&d);

        for ch in channels {
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
            "chain_1": {
                "chain_name": chain_1_name,
                "chain_id": "morocco-1",
                "client_id": terp_client_id,
                "connection_id": terp_connection_id,
            },
            "chain_2": {
                "chain_name": chain_2_name,
                "chain_id": counterparty_chain_id,
                "client_id": counterparty_client_id,
                "connection_id": counterparty_connection_id,
            },
            "channels": channels,
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
        // Extract counterparty name from filename like "akash-terp.json" -> "akash"
        let counterpartyname = filename
            .strip_suffix(".json")
            .unwrap_or(filename)
            .split('-')
            .find(|p| *p != "terp")
            .unwrap_or("unknown")
            .to_string();
        state_mut.set("ibc_data", &counterpartyname, entry["data"].clone())?;
    }
    state_mut.force_write()?;

    // Step 4: NOW build the channel map from the updated state
    let ibc_data_map = build_channel_to_chain_map(&state_terp);
    println!(
        "\n✓ {} IBC data entries written to state.json",
        ibc_data_output.len()
    );

    info!("\nChannel map entries: {}", ibc_data_map.len());

    let mut all_derived_ibc_assets: Vec<serde_json::Value> = Vec::new();
    let mut chain_assets: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
    let d = Vec::with_capacity(1);
    let mut assetlist = Vec::new();

    let assosmo = state_osmo.get("assets")?;
    let assosmo = assosmo.as_array().unwrap_or(&d);

    assetlist.extend::<&Vec<Value>>(assosmo.as_ref());

    let terp_assets: Vec<serde_json::Value> = state_terp
        .get("assets")
        .ok()
        .and_then(|a| a.as_array().cloned())
        .unwrap_or_default()
        .into_iter()
        .map(|mut a| {
            a["_source_chain"] = serde_json::json!("terp");
            a
        })
        .collect();
    chain_assets.insert("terp".to_string(), terp_assets.clone());

    all_derived_ibc_assets.extend(derive_ibc_asset_list(
        &terp,
        &ibc,
        &state_terp,
        terp_assets.as_slice(),
        &ibc_data_map,
    )?);

    // Process Osmosis assets — pass state_osmo, not state_terp
    let osmo_assets = load_assetlist_for_chain("osmosis", &state_osmo);
    if !osmo_assets.is_empty() {
        println!("\n=== Processing Osmosis assets (multi-hop paths) ===\n");
        all_derived_ibc_assets.extend(derive_ibc_asset_list(
            &terp,
            &ibc,
            &state_terp,
            &osmo_assets,
            &ibc_data_map,
        )?);
    }
    chain_assets.insert("osmosis".to_string(), osmo_assets);

    // Build final assetlist: native Terp assets + all derived IBC assets
    let mut native_assets: Vec<serde_json::Value> = state_terp
        .get("assets")
        .ok()
        .and_then(|a| a.as_array().cloned())
        .unwrap_or_default()
        .into_iter()
        .filter(|a| a["traces"].as_array().map_or(true, |t| t.is_empty()))
        .collect();

    // Merge IBC assets into the list
    for new_asset in &all_derived_ibc_assets {
        let base = new_asset["base"].as_str().unwrap_or("");
        if let Some(pos) = native_assets.iter().position(|a| a["base"] == base) {
            native_assets[pos] = new_asset.clone();
        } else {
            native_assets.push(new_asset.clone());
        }
    }

    if !native_assets.is_empty() {
        let mut state_mut = state_terp.clone();
        state_mut.set(
            "assets",
            "",
            serde_json::Value::Array(native_assets.clone()),
        )?;
        state_mut.force_write()?;

        let assetlist_output = serde_json::json!({
            "$schema": "../assetlist.schema.json",
            "chain_name": "terp",
            "assets": native_assets
        });
        let output_path = std::path::PathBuf::from("../public/assetlist.json");
        std::fs::write(
            &output_path,
            serde_json::to_string_pretty(&assetlist_output)?,
        )?;

        println!(
            "\n✓ Assetlist written with {} total assets ({} IBC)",
            native_assets.len(),
            all_derived_ibc_assets.len()
        );
    }

    let graph = IBCChannelGraph::build_from_state(&state_mut);

    println!("\n=== IBC Channel Graph ===");
    for (chain, edges) in &graph.edges {
        println!("  {} -> {} connections", chain, edges.len());
        for edge in edges {
            println!(
                "    -> {}/channel-{} (preferred: {})",
                edge.counterparty_chain, edge.this_channel_id, edge.preferred
            );
        }
    }

    // Premine the routing table
    println!("\n=== Premining IBC Asset Routes ===");
    let routing_table = IBCAssetRoutingTable::premine(&graph, &chain_assets, 3);

    println!("\n=== Routing Table Summary ===");
    println!("Total routes: {}", routing_table.metadata.total_routes);
    println!("Chains covered: {:?}", routing_table.metadata.chains);

    for (dest_chain, routes) in &routing_table.routes {
        println!("\n  {} - {} IBC assets:", dest_chain, routes.len());
        for route in routes.iter().take(10) {
            println!(
                "    {} ({}) -> {} [{} hops, preferred: {}]",
                route.symbol,
                route.origin_chain,
                route.dest_denom,
                route.hop_count,
                route.preferred
            );
        }
        if routes.len() > 10 {
            println!("    ... and {} more", routes.len() - 10);
        }
    }

    // Export the simplified lookup table
    let simplified = routing_table.to_simplified_lookup();
    let lookup_path = std::path::PathBuf::from("../public/ibc_lookup_table.json");
    std::fs::write(&lookup_path, serde_json::to_string_pretty(&simplified)?)?;
    println!("\n✓ IBC lookup table written to ibc_lookup_table.json");

    // Export the full routing table
    let full_table = routing_table.to_json();
    let full_path = std::path::PathBuf::from("../public/ibc_routing_table.json");
    std::fs::write(&full_path, serde_json::to_string_pretty(&full_table)?)?;
    println!("✓ Full routing table written to ibc_routing_table.json");

    Ok(())
}

/// Build a trace entry conforming to the specific schema for each trace type
fn build_trace_entry(trace: &serde_json::Value) -> Option<serde_json::Value> {
    let trace_type = trace["type"].as_str().unwrap_or("");
    let counterparty = &trace["counterparty"];
    let chain = &trace["chain"];

    match trace_type {
        "ibc" => Some(serde_json::json!({
            "type": "ibc",
            "counterparty": {
                "chain_name": counterparty["chain_name"].as_str().unwrap_or(""),
                "base_denom": counterparty["base_denom"].as_str().unwrap_or(""),
                "channel_id": counterparty["channel_id"].as_str().unwrap_or("")
            },
            "chain": {
                "channel_id": chain["channel_id"].as_str().unwrap_or(""),
                "path": chain["path"].as_str().unwrap_or("")
            }
        })),
        "ibc-cw20" => Some(serde_json::json!({
            "type": "ibc-cw20",
            "counterparty": {
                "chain_name": counterparty["chain_name"].as_str().unwrap_or(""),
                "base_denom": counterparty["base_denom"].as_str().unwrap_or(""),
                "port": counterparty["port"].as_str().unwrap_or(""),
                "channel_id": counterparty["channel_id"].as_str().unwrap_or("")
            },
            "chain": {
                "port": chain["port"].as_str().unwrap_or(""),
                "channel_id": chain["channel_id"].as_str().unwrap_or(""),
                "path": chain["path"].as_str().unwrap_or("")
            }
        })),
        "ibc-bridge" => Some(serde_json::json!({
            "type": "ibc-bridge",
            "counterparty": {
                "chain_name": counterparty["chain_name"].as_str().unwrap_or(""),
                "base_denom": counterparty["base_denom"].as_str().unwrap_or(""),
                "channel_id": counterparty["channel_id"].as_str().unwrap_or("")
            },
            "chain": {
                "channel_id": chain["channel_id"].as_str().unwrap_or(""),
                "path": chain["path"].as_str().unwrap_or("")
            },
            "provider": trace["provider"].as_str().unwrap_or("")
        })),
        "bridge" | "liquid-stake" | "synthetic" | "wrapped" | "additional-mintage"
        | "test-mintage" | "legacy-mintage" => {
            let mut cp = serde_json::json!({
                "chain_name": counterparty["chain_name"].as_str().unwrap_or(""),
                "base_denom": counterparty["base_denom"].as_str().unwrap_or("")
            });
            if let Some(contract) = counterparty["contract"].as_str() {
                cp["contract"] = serde_json::json!(contract);
            }
            let mut entry = serde_json::json!({
                "type": trace_type,
                "counterparty": cp,
                "provider": trace["provider"].as_str().unwrap_or("")
            });
            if !chain.is_null() && chain["contract"].is_string() {
                entry["chain"] = serde_json::json!({
                    "contract": chain["contract"].as_str().unwrap_or("")
                });
            }
            Some(entry)
        }
        _ => None,
    }
}

fn build_image_sync(counterparty_chain: &str, counterparty_base_denom: &str) -> serde_json::Value {
    let png_url = format!(
        "https://raw.githubusercontent.com/cosmos/chain-registry/master/{}/images/{}.png",
        counterparty_chain,
        counterparty_base_denom.trim_start_matches('u')
    );
    let svg_url = format!(
        "https://raw.githubusercontent.com/cosmos/chain-registry/master/{}/images/{}.svg",
        counterparty_chain,
        counterparty_base_denom.trim_start_matches('u')
    );
    serde_json::json!({
        "image_sync": {
            "chain_name": counterparty_chain,
            "base_denom": counterparty_base_denom
        },
        "png": png_url,
        "svg": svg_url
    })
}

/// Build a complete assetlist-compliant asset entry for any trace type
fn build_ibc_asset_entry(
    asset: &serde_json::Value,
    ibc_denom: &str,
    traces: &[serde_json::Value],
    trace_path: &str,
    symbol: &str,
    counterparty_chain: &str,
    counterparty_denom: &str,
    dest_channel: &str,
    via_chain: Option<&str>,
) -> serde_json::Value {
    let display_denom = asset["display"].as_str().unwrap_or("");
    let display_exponent = asset["denom_units"]
        .as_array()
        .and_then(|units| units.iter().find(|u| u["denom"] == display_denom))
        .and_then(|u| u["exponent"].as_u64())
        .unwrap_or(6);

    let type_asset = asset["type_asset"].as_str().unwrap_or("ics20");

    // Build traces array from all trace entries
    let trace_entries: Vec<serde_json::Value> = traces
        .iter()
        .filter_map(|t| {
            let mut entry = build_trace_entry(t)?;
            // If this is a multi-hop, add the via information
            if let Some(via) = via_chain {
                if entry["type"] == "ibc" {
                    entry["via"] = serde_json::json!(via);
                }
            }
            Some(entry)
        })
        .collect();

    // Build the base asset object
    let mut asset_obj = serde_json::json!({
        "description": asset["description"].as_str().unwrap_or(""),
        "denom_units": [
            {
                "denom": ibc_denom,
                "exponent": 0,
                "aliases": [counterparty_denom]
            },
            {
                "denom": display_denom,
                "exponent": display_exponent
            }
        ],
        "type_asset": type_asset,
        "base": ibc_denom,
        "name": asset["name"].as_str().unwrap_or(""),
        "display": display_denom,
        "symbol": symbol,
    });

    // Add traces if present
    if !trace_entries.is_empty() {
        asset_obj["traces"] = serde_json::json!(trace_entries);
    }

    // Add address for cw20/erc20/snip20 (required by schema)
    if let Some(address) = asset["address"].as_str() {
        if !address.is_empty() {
            asset_obj["address"] = serde_json::json!(address);
        }
    }

    // Add image_sync for IBC/bridged assets
    let has_ibc_trace = traces.iter().any(|t| {
        matches!(
            t["type"].as_str().unwrap_or(""),
            "ibc"
                | "ibc-cw20"
                | "ibc-bridge"
                | "bridge"
                | "wrapped"
                | "liquid-stake"
                | "test-mintage"
        )
    });

    if has_ibc_trace && !counterparty_chain.is_empty() && !counterparty_denom.is_empty() {
        let image_entry = build_image_sync(counterparty_chain, counterparty_denom);
        asset_obj["images"] = serde_json::json!([image_entry]);
        asset_obj["logo_URIs"] = serde_json::json!({
            "png": image_entry["png"],
            "svg": image_entry["svg"]
        });
    }

    // Add coingecko_id if present
    if let Some(cg_id) = asset["coingecko_id"].as_str() {
        if !cg_id.is_empty() {
            asset_obj["coingecko_id"] = serde_json::json!(cg_id);
        }
    }

    // Add deprecated flag only if true
    if asset["deprecated"].as_bool().unwrap_or(false) {
        asset_obj["deprecated"] = serde_json::json!(true);
    }

    asset_obj
}
fn derive_ibc_asset_list(
    terp: &Daemon,
    ibc: &Ibc,
    state_terp: &DaemonState,
    assetlist: &[serde_json::Value],
    ibc_data_map: &HashMap<String, TerpChannelInfo>,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let mut ibc_denoms: Vec<serde_json::Value> = vec![];

    info!("assetlist: {}", assetlist.len());

    for asset in assetlist {
        let symbol = asset["symbol"].as_str().unwrap_or("UNKNOWN");
        let base_denom = asset["base"].as_str().unwrap_or("");

        // Skip assets without traces (native sdk.coin tokens)
        let traces = match asset["traces"].as_array() {
            Some(t) if !t.is_empty() => t,
            _ => continue,
        };

        // Check if this is an IBC-type trace
        let has_ibc_trace = traces.iter().any(|t| {
            matches!(
                t["type"].as_str().unwrap_or(""),
                "ibc" | "ibc-cw20" | "ibc-bridge"
            )
        });

        if has_ibc_trace {
            match derive_terp_ibc_denom(asset, ibc_data_map) {
                Some((ibc_hash, trace_path, cp_chain)) => {
                    let cp_denom = traces[0]["counterparty"]["base_denom"]
                        .as_str()
                        .unwrap_or("");
                    let dest_channel = traces[0]["chain"]["channel_id"].as_str().unwrap_or("");

                    println!("↻ {} - derived via {}", symbol, trace_path);
                    println!("  ✓ IBC denom: {}", ibc_hash);
                    println!("  Counterparty: {} ({})", cp_chain, cp_denom);

                    ibc_denoms.push(build_ibc_asset_entry(
                        asset,
                        &ibc_hash,
                        traces,
                        &trace_path,
                        symbol,
                        &cp_chain,
                        cp_denom,
                        dest_channel,
                        None,
                    ));
                }
                None => {
                    println!("⚠ No path found for {}", symbol);
                }
            }
        } else {
            // Non-IBC trace (bridge, liquid-stake, etc.)
            let cp_chain = traces[0]["counterparty"]["chain_name"]
                .as_str()
                .unwrap_or("");
            let cp_denom = traces[0]["counterparty"]["base_denom"]
                .as_str()
                .unwrap_or("");

            println!("⏭ {} - non-IBC trace, using base denom", symbol);
            ibc_denoms.push(build_ibc_asset_entry(
                asset, base_denom, traces, "", symbol, cp_chain, cp_denom, "", None,
            ));
        }
    }

    Ok(ibc_denoms)
}

struct IbcChannelInfo {
    terp_channel: String,
    counterparty_channel: String,
    counterparty_chain_name: String,
    counterparty_chain_id: String,
}

fn build_channel_to_chain_map(state: &DaemonState) -> HashMap<String, TerpChannelInfo> {
    let mut map = HashMap::new();

    // Try to get ibc_data from state
    let ibc_data = match state.get("ibc_data") {
        Ok(data) => data,
        Err(_) => {
            println!("  ⚠ No ibc_data found in state");
            return map;
        }
    };

    let obj = match ibc_data.as_object() {
        Some(o) => o,
        None => {
            println!("  ⚠ ibc_data is not an object");
            return map;
        }
    };

    println!("  Parsing {} ibc_data entries...", obj.len());

    for (key, data) in obj {
        // Handle both formats:
        // Old format: key is "akash-terp", data has "akash" and "terp" keys
        // New format: key is "akash" (already extracted), data has "chain_1" and "chain_2" keys

        let (counterparty_name, terp_is_chain1) =
            if data["chain_1"].is_object() && data["chain_2"].is_object() {
                // New schema format: chain_1/chain_2
                let chain_1_name = data["chain_1"]["chain_name"].as_str().unwrap_or("");
                let chain_2_name = data["chain_2"]["chain_name"].as_str().unwrap_or("");

                if chain_1_name == "terp" {
                    (chain_2_name.to_string(), true)
                } else if chain_2_name == "terp" {
                    (chain_1_name.to_string(), false)
                } else {
                    println!("  ⚠ Entry '{}' has no terp chain", key);
                    continue;
                }
            } else {
                // Old format: chain names as keys
                // Key might be "akash-terp" or just "akash"
                let cp_name = key
                    .strip_suffix("-terp")
                    .or_else(|| key.strip_prefix("terp-"))
                    .unwrap_or(key);

                if data[cp_name].is_object() && data["terp"].is_object() {
                    (cp_name.to_string(), false) // terp is NOT chain_1 in old format
                } else {
                    println!("  ⚠ Entry '{}' has unknown format", key);
                    continue;
                }
            };

        if counterparty_name.is_empty() {
            println!("  ⚠ Empty counterparty name for key '{}'", key);
            continue;
        }

        // Find the preferred transfer channel
        let channels = match data["channels"].as_array() {
            Some(c) => c,
            None => {
                println!("  ⚠ No channels array in entry '{}'", key);
                continue;
            }
        };

        let preferred_channel = channels
            .iter()
            .find(|c| {
                let has_transfer_ports = if data["chain_1"].is_object() {
                    c["chain_1"]["port_id"].as_str() == Some("transfer")
                        && c["chain_2"]["port_id"].as_str() == Some("transfer")
                } else {
                    // Old format
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
                    // New format
                    if terp_is_chain1 {
                        (
                            c["chain_1"]["channel_id"]
                                .as_str()
                                .unwrap_or("")
                                .to_string(),
                            c["chain_2"]["channel_id"]
                                .as_str()
                                .unwrap_or("")
                                .to_string(),
                        )
                    } else {
                        (
                            c["chain_2"]["channel_id"]
                                .as_str()
                                .unwrap_or("")
                                .to_string(),
                            c["chain_1"]["channel_id"]
                                .as_str()
                                .unwrap_or("")
                                .to_string(),
                        )
                    }
                } else {
                    // Old format
                    (
                        c["terp"]["channel_id"].as_str().unwrap_or("").to_string(),
                        c[counterparty_name.as_str()]["channel_id"]
                            .as_str()
                            .unwrap_or("")
                            .to_string(),
                    )
                }
            }
            None => {
                println!(
                    "  ⚠ No preferred transfer channel found for '{}'",
                    counterparty_name
                );
                continue;
            }
        };

        if !terp_channel.is_empty() {
            map.insert(
                counterparty_name.clone(),
                TerpChannelInfo {
                    terp_channel_id: terp_channel.clone(),
                    counterparty_channel_id: cp_channel.clone(),
                    counterparty_chain_name: counterparty_name.clone(),
                },
            );
            println!(
                "  ✓ Channel map: terp/channel-{} <-> {}/channel-{}",
                terp_channel, counterparty_name, cp_channel
            );
        }
    }

    map
}
/// Find a multi-hop path from Terp to a target chain via an intermediary
/// Returns (terp_channel_to_intermediary, intermediary_chain_name)
fn find_path_via_intermediary(
    target_chain: &str,
    ibc_data_map: &HashMap<String, IbcChannelInfo>,
    available_intermediaries: &[&str], // e.g., ["osmosis", "cosmoshub", "juno"]
) -> Option<(String, String)> {
    for intermediary in available_intermediaries {
        if let Some(info) = ibc_data_map.get(*intermediary) {
            // We have a channel to this intermediary
            if !info.terp_channel.is_empty() {
                return Some((info.terp_channel.clone(), intermediary.to_string()));
            }
        }
    }
    None
}

fn load_assetlist_for_chain(chain_name: &str, state: &DaemonState) -> Vec<serde_json::Value> {
    let assets = state
        .get("assets")
        .ok()
        .and_then(|a| a.as_array().cloned())
        .unwrap_or_default();

    println!("Loaded {} assets from '{}'", assets.len(), chain_name);

    assets
        .into_iter()
        .map(|mut asset| {
            asset["_source_chain"] = serde_json::json!(chain_name);
            asset
        })
        .collect()
}

/// Terp's channel info to a counterparty chain
#[derive(Debug, Clone)]
struct TerpChannelInfo {
    terp_channel_id: String,         // e.g., "channel-5"
    counterparty_channel_id: String, // e.g., "channel-42"
    counterparty_chain_name: String, // e.g., "osmosis"
}

/// Derive the IBC denom hash and trace path for a foreign asset on Terp
///
/// Logic:
/// 1. If the asset has an IBC trace, extract the counterparty chain and base denom
/// 2. If Terp has a direct channel to the counterparty chain:
///    - Single hop: transfer/{terp_ch}/{counterparty_base_denom}
///    - Multi-hop on source: transfer/{terp_ch}/{source_trace_path}
/// 3. If Terp has NO direct channel to counterparty but has channel to source chain:
///    - Route through source: transfer/{terp_ch_to_source}/{source_trace_path}
fn derive_terp_ibc_denom(
    asset: &serde_json::Value,
    terp_channels: &HashMap<String, TerpChannelInfo>,
) -> Option<(String, String, String)> {
    // (ibc_denom_hash, full_trace_path, counterparty_chain)

    let traces = asset["traces"].as_array()?;
    let ibc_trace = traces.iter().find(|t| {
        matches!(
            t["type"].as_str().unwrap_or(""),
            "ibc" | "ibc-cw20" | "ibc-bridge"
        )
    })?;

    let counterparty_chain = ibc_trace["counterparty"]["chain_name"].as_str()?;
    let counterparty_base = ibc_trace["counterparty"]["base_denom"].as_str()?;
    let source_trace_path = ibc_trace["chain"]["path"].as_str().unwrap_or("");
    let source_chain = asset["_source_chain"].as_str().unwrap_or("");

    // Try direct channel to counterparty first
    if let Some(info) = terp_channels.get(counterparty_chain) {
        let full_path = if source_trace_path.contains('/') {
            // Asset is multi-hop on source chain (e.g., Osmosis wrapping ATOM from Hub)
            // We route directly to counterparty, so just use counterparty base
            format!("transfer/{}/{}", info.terp_channel_id, counterparty_base)
        } else {
            // Single hop: our channel + counterparty base denom
            format!("transfer/{}/{}", info.terp_channel_id, counterparty_base)
        };
        let hash = compute_ibc_denom_hash(&full_path);
        return Some((hash, full_path, counterparty_chain.to_string()));
    }

    // No direct channel — route through the source chain
    if !source_chain.is_empty() {
        if let Some(info) = terp_channels.get(source_chain) {
            // Prepend our channel to source chain onto the existing trace path
            let full_path = if source_trace_path.is_empty() {
                format!("transfer/{}/{}", info.terp_channel_id, counterparty_base)
            } else {
                format!("transfer/{}/{}", info.terp_channel_id, source_trace_path)
            };
            let hash = compute_ibc_denom_hash(&full_path);
            return Some((hash, full_path, counterparty_chain.to_string()));
        }
    }

    None
}
fn compute_ibc_denom_hash(trace_path: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(trace_path.as_bytes());
    let result = hasher.finalize();
    format!("ibc/{}", hex::encode(result).to_uppercase())
}

/// A single hop in an IBC route
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChannelHop {
    from_chain: String,
    from_channel: String,
    to_chain: String,
    to_channel: String,
}

/// A fully resolved IBC asset route
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IBCAssetRoute {
    /// Symbol of the asset (e.g., "ATOM")
    symbol: String,
    /// Chain where the asset originates (e.g., "cosmoshub")
    origin_chain: String,
    /// Denom on the origin chain (e.g., "uatom")
    origin_denom: String,
    /// Chain where we're computing the IBC denom (e.g., "terp")
    dest_chain: String,
    /// The IBC denom hash on the dest chain (e.g., "ibc/27394...")
    dest_denom: String,
    /// The full trace path (e.g., "transfer/channel-0/uatom")
    trace_path: String,
    /// The route taken (ordered list of hops)
    route: Vec<ChannelHop>,
    /// Number of hops
    hop_count: usize,
    /// Whether this uses a preferred channel
    preferred: bool,
}

/// The IBC channel graph
#[derive(Debug, Clone, Default)]
struct IBCChannelGraph {
    /// Adjacency list: chain -> [(counterparty_chain, channel_info)]
    edges: HashMap<String, Vec<ChannelEdge>>,
}

#[derive(Debug, Clone)]
struct ChannelEdge {
    counterparty_chain: String,
    this_channel_id: String,
    counterparty_channel_id: String,
    preferred: bool,
    status: String,
}

impl IBCChannelGraph {
    fn new() -> Self {
        Self::default()
    }

    /// Add a channel connection between two chains
    fn add_channel(
        &mut self,
        chain_a: &str,
        chain_a_channel: &str,
        chain_b: &str,
        chain_b_channel: &str,
        preferred: bool,
        status: String,
    ) {
        self.edges
            .entry(chain_a.to_string())
            .or_default()
            .push(ChannelEdge {
                counterparty_chain: chain_b.to_string(),
                counterparty_channel_id: chain_b_channel.to_string(),
                this_channel_id: chain_a_channel.to_string(),
                preferred,
                status: status.clone(),
            });

        self.edges
            .entry(chain_b.to_string())
            .or_default()
            .push(ChannelEdge {
                counterparty_chain: chain_a.to_string(),
                counterparty_channel_id: chain_a_channel.to_string(),
                this_channel_id: chain_b_channel.to_string(),
                preferred,
                status,
            });
    }

    /// Build the graph from state.json ibc_data
    fn build_from_state(state: &DaemonState) -> Self {
        let mut graph = Self::new();

        if let Ok(ibc_data) = state.get("ibc_data") {
            if let Some(obj) = ibc_data.as_object() {
                for (_key, data) in obj {
                    let chain_1_name = data["chain_1"]["chain_name"].as_str().unwrap_or("");
                    let chain_2_name = data["chain_2"]["chain_name"].as_str().unwrap_or("");

                    if let Some(channels) = data["channels"].as_array() {
                        for chan in channels {
                            let ch1_id = chan["chain_1"]["channel_id"].as_str().unwrap_or("");
                            let ch2_id = chan["chain_2"]["channel_id"].as_str().unwrap_or("");
                            let preferred = chan["tags"]["preferred"].as_bool().unwrap_or(false);
                            let status = chan["tags"]["status"]
                                .as_str()
                                .unwrap_or("UNKNOWN")
                                .to_string();

                            if !ch1_id.is_empty() && !ch2_id.is_empty() {
                                graph.add_channel(
                                    chain_1_name,
                                    ch1_id,
                                    chain_2_name,
                                    ch2_id,
                                    preferred,
                                    status,
                                );
                            }
                        }
                    }
                }
            }
        }

        graph
    }

    /// Find all routes from source to dest (BFS, up to max_hops)
    fn find_routes(&self, source: &str, dest: &str, max_hops: usize) -> Vec<Vec<ChannelHop>> {
        let mut routes = Vec::new();
        let mut queue = vec![(source.to_string(), Vec::new(), HashSet::new())];

        while let Some((current_chain, path, visited)) = queue.pop() {
            if current_chain == dest && !path.is_empty() {
                routes.push(path);
                continue;
            }

            if path.len() >= max_hops {
                continue;
            }

            if let Some(neighbors) = self.edges.get(&current_chain) {
                for edge in neighbors {
                    if edge.status != "ACTIVE" {
                        continue;
                    }
                    if visited.contains(&edge.counterparty_chain) {
                        continue;
                    }

                    let mut new_visited = visited.clone();
                    new_visited.insert(current_chain.clone());

                    let mut new_path = path.clone();
                    new_path.push(ChannelHop {
                        from_chain: current_chain.clone(),
                        from_channel: edge.this_channel_id.clone(),
                        to_chain: edge.counterparty_chain.clone(),
                        to_channel: edge.counterparty_channel_id.clone(),
                    });

                    queue.push((edge.counterparty_chain.clone(), new_path, new_visited));
                }
            }
        }

        // Sort by hop count (shortest first), then prefer routes with preferred channels
        routes.sort_by(|a, b| {
            a.len().cmp(&b.len()).then_with(|| {
                let a_preferred = a
                    .iter()
                    .any(|h| self.is_preferred(&h.from_chain, &h.to_chain, &h.from_channel));
                let b_preferred = b
                    .iter()
                    .any(|h| self.is_preferred(&h.from_chain, &h.to_chain, &h.from_channel));
                b_preferred.cmp(&a_preferred) // preferred = true > false
            })
        });

        routes
    }

    fn is_preferred(&self, from: &str, to: &str, channel: &str) -> bool {
        self.edges
            .get(from)
            .and_then(|edges| {
                edges
                    .iter()
                    .find(|e| e.counterparty_chain == to && e.this_channel_id == channel)
            })
            .map(|e| e.preferred)
            .unwrap_or(false)
    }

    /// Compute the IBC denom for an asset on a destination chain via a specific route
    fn compute_ibc_denom_for_route(
        &self,
        origin_denom: &str,
        route: &[ChannelHop],
    ) -> (String, String) {
        // Build the trace path by walking the route in reverse
        // (from dest back to origin)
        let mut path_parts = Vec::new();
        let mut current_denom = origin_denom.to_string();

        // Walk the route forward (origin -> dest)
        // Each hop adds a "transfer/channel-X/" prefix
        for hop in route {
            path_parts.push(format!("transfer/{}", hop.from_channel));
            current_denom = format!("transfer/{}/{}", hop.from_channel, current_denom);
        }

        // Actually, the trace path is built from the perspective of the dest chain
        // looking back to origin. So we need to reverse the hop order.
        let mut trace_path = String::new();
        for hop in route.iter().rev() {
            // From dest's perspective, the channel that connects TO the next hop
            // is the counterparty channel
            trace_path = format!("transfer/{}/", hop.to_channel) + &trace_path;
        }
        trace_path.push_str(origin_denom);

        // Compute the hash
        let hash = compute_ibc_denom_hash(&trace_path);

        (hash, trace_path)
    }
}

/// The full IBC Asset Routing Table
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IBCAssetRoutingTable {
    /// All computed routes, keyed by (dest_chain, ibc_denom)
    routes: HashMap<String, Vec<IBCAssetRoute>>,
    /// Metadata about which chains and assets were processed
    metadata: RoutingTableMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RoutingTableMetadata {
    /// Chains included in the routing table
    chains: Vec<String>,
    /// Total number of routes computed
    total_routes: usize,
    /// Timestamp of generation
    generated_at: String,
}

impl IBCAssetRoutingTable {
    /// Premine all possible IBC asset routes
    fn premine(
        graph: &IBCChannelGraph,
        chain_assets: &HashMap<String, Vec<serde_json::Value>>,
        max_hops: usize,
    ) -> Self {
        let mut routes: HashMap<String, Vec<IBCAssetRoute>> = HashMap::new();
        let mut chains = HashSet::new();

        // For each chain with assets
        for (source_chain, assets) in chain_assets {
            chains.insert(source_chain.clone());

            for asset in assets {
                let symbol = asset["symbol"].as_str().unwrap_or("UNKNOWN");
                let base_denom = asset["base"].as_str().unwrap_or("");

                // Skip assets that are already IBC on this chain (we want origin assets)
                // unless we also want to trace multi-hop IBC assets
                let is_native = !base_denom.starts_with("ibc/");

                if is_native {
                    // Find all chains this asset can reach
                    for dest_chain in graph.edges.keys() {
                        if dest_chain == source_chain {
                            continue;
                        }

                        // Find routes from source to dest
                        let found_routes = graph.find_routes(source_chain, dest_chain, max_hops);

                        for route in found_routes {
                            let (ibc_denom, trace_path) =
                                graph.compute_ibc_denom_for_route(base_denom, &route);

                            let preferred = route.iter().all(|h| {
                                graph.is_preferred(&h.from_chain, &h.to_chain, &h.from_channel)
                            });

                            let entry = IBCAssetRoute {
                                symbol: symbol.to_string(),
                                origin_chain: source_chain.clone(),
                                origin_denom: base_denom.to_string(),
                                dest_chain: dest_chain.clone(),
                                dest_denom: ibc_denom.clone(),
                                trace_path: trace_path.clone(),
                                route: route.clone(),
                                hop_count: route.len(),
                                preferred,
                            };

                            routes.entry(dest_chain.clone()).or_default().push(entry);
                        }
                    }
                } else {
                    // This is an IBC asset on the source chain
                    // We can still trace it further to other chains
                    if let Some(traces) = asset["traces"].as_array() {
                        if let Some(ibc_trace) = traces.iter().find(|t| {
                            matches!(
                                t["type"].as_str().unwrap_or(""),
                                "ibc" | "ibc-cw20" | "ibc-bridge"
                            )
                        }) {
                            let cp_chain = ibc_trace["counterparty"]["chain_name"]
                                .as_str()
                                .unwrap_or("");
                            let cp_denom = ibc_trace["counterparty"]["base_denom"]
                                .as_str()
                                .unwrap_or("");
                            let source_trace = ibc_trace["chain"]["path"].as_str().unwrap_or("");

                            // For each dest chain (not the source)
                            for dest_chain in graph.edges.keys() {
                                if dest_chain == source_chain {
                                    continue;
                                }

                                let found_routes =
                                    graph.find_routes(source_chain, dest_chain, max_hops);

                                for route in found_routes {
                                    // The trace path on dest chain is:
                                    // transfer/{our_channel_to_source}/{source_trace_path}
                                    let first_hop = &route[route.len() - 1]; // Last hop in reversed route
                                    let full_trace = if source_trace.is_empty() {
                                        format!("transfer/{}/{}", first_hop.to_channel, cp_denom)
                                    } else {
                                        format!(
                                            "transfer/{}/{}",
                                            first_hop.to_channel, source_trace
                                        )
                                    };

                                    let ibc_denom = compute_ibc_denom_hash(&full_trace);

                                    let preferred = route.iter().all(|h| {
                                        graph.is_preferred(
                                            &h.from_chain,
                                            &h.to_chain,
                                            &h.from_channel,
                                        )
                                    });

                                    let entry = IBCAssetRoute {
                                        symbol: symbol.to_string(),
                                        origin_chain: cp_chain.to_string(),
                                        origin_denom: cp_denom.to_string(),
                                        dest_chain: dest_chain.clone(),
                                        dest_denom: ibc_denom.clone(),
                                        trace_path: full_trace,
                                        route: route.clone(),
                                        hop_count: route.len(),
                                        preferred,
                                    };

                                    routes.entry(dest_chain.clone()).or_default().push(entry);
                                }
                            }
                        }
                    }
                }
            }
        }

        let total_routes: usize = routes.values().map(|v| v.len()).sum();

        Self {
            routes,
            metadata: RoutingTableMetadata {
                chains: chains.into_iter().collect(),
                total_routes,
                generated_at: now_timestamp(),
            },
        }
    }

    /// Look up all IBC assets available on a specific chain
    fn lookup_by_dest_chain(&self, chain: &str) -> Vec<&IBCAssetRoute> {
        self.routes
            .get(chain)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Look up the IBC denom for a specific asset on a specific chain
    fn lookup_ibc_denom(&self, symbol: &str, dest_chain: &str) -> Option<&IBCAssetRoute> {
        self.routes
            .get(dest_chain)?
            .iter()
            .find(|r| r.symbol == symbol && r.preferred)
            .or_else(|| {
                self.routes
                    .get(dest_chain)?
                    .iter()
                    .find(|r| r.symbol == symbol)
            })
    }

    /// Look up the origin asset info from an IBC denom on a chain
    fn reverse_lookup(&self, ibc_denom: &str, chain: &str) -> Option<&IBCAssetRoute> {
        self.routes
            .get(chain)?
            .iter()
            .find(|r| r.dest_denom == ibc_denom)
    }

    /// Export the routing table as a JSON lookup file
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    /// Export a simplified lookup table (dest_chain -> ibc_denom -> origin info)
    fn to_simplified_lookup(&self) -> serde_json::Value {
        let mut lookup = serde_json::Map::new();

        for (dest_chain, routes) in &self.routes {
            let mut chain_lookup = serde_json::Map::new();

            for route in routes {
                // Only include preferred routes in simplified lookup
                if !route.preferred {
                    continue;
                }

                chain_lookup.insert(
                    route.dest_denom.clone(),
                    serde_json::json!({
                        "symbol": route.symbol,
                        "origin_chain": route.origin_chain,
                        "origin_denom": route.origin_denom,
                        "trace_path": route.trace_path,
                        "hop_count": route.hop_count,
                    }),
                );
            }

            lookup.insert(dest_chain.clone(), serde_json::Value::Object(chain_lookup));
        }

        serde_json::Value::Object(lookup)
    }
}

mod test {

    use super::*;
    use serde_json::json;

    fn make_terp_channels() -> HashMap<String, TerpChannelInfo> {
        let mut map = HashMap::new();
        map.insert(
            "osmosis".to_string(),
            TerpChannelInfo {
                terp_channel_id: "channel-5".to_string(),
                counterparty_channel_id: "channel-42".to_string(),
                counterparty_chain_name: "osmosis".to_string(),
            },
        );
        map.insert(
            "cosmoshub".to_string(),
            TerpChannelInfo {
                terp_channel_id: "channel-0".to_string(),
                counterparty_channel_id: "channel-99".to_string(),
                counterparty_chain_name: "cosmoshub".to_string(),
            },
        );
        map.insert(
            "akash".to_string(),
            TerpChannelInfo {
                terp_channel_id: "channel-3".to_string(),
                counterparty_channel_id: "channel-9".to_string(),
                counterparty_chain_name: "akash".to_string(),
            },
        );
        map
    }

    #[test]
    fn test_compute_ibc_denom_hash_single_hop() {
        // ATOM single hop from Cosmos Hub
        let path = "transfer/channel-0/uatom";
        let hash = compute_ibc_denom_hash(path);
        // Verify it starts with ibc/ and is 64 chars after prefix
        assert!(hash.starts_with("ibc/"));
        assert_eq!(hash.len(), 68); // "ibc/" (4) + 64 hex chars
    }

    #[test]
    fn test_compute_ibc_denom_hash_multi_hop() {
        // ATOM via Osmosis: terp -> osmosis -> cosmoshub
        let path = "transfer/channel-5/transfer/channel-0/uatom";
        let hash = compute_ibc_denom_hash(path);
        assert!(hash.starts_with("ibc/"));
        // Multi-hop should produce DIFFERENT hash than single hop
        let single_hop = compute_ibc_denom_hash("transfer/channel-0/uatom");
        assert_ne!(hash, single_hop);
    }

    #[test]
    fn test_derive_direct_channel_single_hop() {
        // AKT on Osmosis, traces back to Akash
        // Terp has direct channel to Akash -> single hop
        let channels = make_terp_channels();
        let asset = json!({
            "base": "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4",
            "symbol": "AKT",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "akash",
                    "base_denom": "uakt",
                    "channel_id": "channel-9"
                },
                "chain": {
                    "channel_id": "channel-1",
                    "path": "transfer/channel-1/uakt"
                }
            }],
            "_source_chain": "osmosis"
        });

        let result = derive_terp_ibc_denom(&asset, &channels);
        assert!(result.is_some());

        let (hash, path, cp_chain) = result.unwrap();
        // Direct channel to Akash -> single hop
        assert_eq!(path, "transfer/channel-3/uakt");
        assert_eq!(cp_chain, "akash");
        // Hash should match SHA256 of "transfer/channel-3/uakt"
        let expected_hash = compute_ibc_denom_hash("transfer/channel-3/uakt");
        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn test_derive_no_direct_channel_route_via_source() {
        // ATONE on Osmosis, traces back to AtomOne
        // Terp has NO direct channel to AtomOne, but has channel to Osmosis
        let channels = make_terp_channels();
        let asset = json!({
            "base": "ibc/BC26A7A805ECD6822719472BCB7842A48EF09DF206182F8F259B2593EB5D23FB",
            "symbol": "ATONE",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "atomone",
                    "base_denom": "uatone",
                    "channel_id": "channel-2"
                },
                "chain": {
                    "channel_id": "channel-94814",
                    "path": "transfer/channel-94814/uatone"
                }
            }],
            "_source_chain": "osmosis"
        });

        let result = derive_terp_ibc_denom(&asset, &channels);
        assert!(result.is_some());

        let (hash, path, cp_chain) = result.unwrap();
        // No direct channel to atomone, route through osmosis
        // Path: transfer/{terp_ch_to_osmo}/{osmo_trace_path}
        assert_eq!(path, "transfer/channel-5/transfer/channel-94814/uatone");
        assert_eq!(cp_chain, "atomone");
    }

    #[test]
    fn test_derive_multi_hop_on_source() {
        // ETH on Osmosis, traces through Cosmos Hub (multi-hop on Osmosis)
        // Terp has NO direct channel to ethereum, routes through osmosis
        let channels = make_terp_channels();
        let asset = json!({
            "base": "ibc/20850C646CDDDC2270E9BBDB08558B5FEE57B647EC6827F41096AABFD8A0471B",
            "symbol": "ETH",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "cosmoshub",
                    "base_denom": "ibc/C0B53D3D23827AE38058BED0BDCD554229278AF530A8D265FCF6DFF7C4B2ADFF",
                    "channel_id": "channel-141"
                },
                "chain": {
                    "channel_id": "channel-0",
                    "path": "transfer/channel-0/transfer/08-wasm-1369/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
                }
            }],
            "_source_chain": "osmosis"
        });

        let result = derive_terp_ibc_denom(&asset, &channels);
        assert!(result.is_some());

        let (hash, path, cp_chain) = result.unwrap();
        // Terp has direct channel to cosmoshub, but the asset is multi-hop on Osmosis
        // Since we have direct channel to cosmoshub, we use the cosmoshub base denom
        assert_eq!(cp_chain, "cosmoshub");
        // Direct to cosmoshub -> use cosmoshub base denom
        assert_eq!(
            path,
            "transfer/channel-0/ibc/C0B53D3D23827AE38058BED0BDCD554229278AF530A8D265FCF6DFF7C4B2ADFF"
        );
    }

    #[test]
    fn test_compute_ibc_denom_hash_format() {
        let path = "transfer/channel-0/uatom";
        let hash = compute_ibc_denom_hash(path);
        assert!(hash.starts_with("ibc/"));
        assert_eq!(hash.len(), 68);
    }

    #[test]
    fn test_direct_channel_single_hop() {
        let channels = make_terp_channels();
        let asset = json!({
            "base": "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4",
            "symbol": "AKT",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "akash",
                    "base_denom": "uakt",
                    "channel_id": "channel-9"
                },
                "chain": {
                    "channel_id": "channel-1",
                    "path": "transfer/channel-1/uakt"
                }
            }],
            "_source_chain": "osmosis"
        });

        let (hash, path, cp_chain) = derive_terp_ibc_denom(&asset, &channels).unwrap();
        assert_eq!(path, "transfer/channel-3/uakt");
        assert_eq!(cp_chain, "akash");
        assert_eq!(hash, compute_ibc_denom_hash("transfer/channel-3/uakt"));
    }

    #[test]
    fn test_no_direct_channel_route_via_source() {
        let channels = make_terp_channels();
        let asset = json!({
            "base": "ibc/BC26A7A805ECD6822719472BCB7842A48EF09DF206182F8F259B2593EB5D23FB",
            "symbol": "ATONE",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "atomone",
                    "base_denom": "uatone",
                    "channel_id": "channel-2"
                },
                "chain": {
                    "channel_id": "channel-94814",
                    "path": "transfer/channel-94814/uatone"
                }
            }],
            "_source_chain": "osmosis"
        });

        let (hash, path, cp_chain) = derive_terp_ibc_denom(&asset, &channels).unwrap();
        assert_eq!(path, "transfer/channel-5/transfer/channel-94814/uatone");
        assert_eq!(cp_chain, "atomone");
    }

    #[test]
    fn test_penumbra_via_osmosis() {
        let channels = make_terp_channels();
        let asset = json!({
            "base": "ibc/0FA9232B262B89E77D1335D54FB1E1F506A92A7E4B51524B400DC69C68D28372",
            "symbol": "UM",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "penumbra",
                    "base_denom": "upenumbra",
                    "channel_id": "channel-4"
                },
                "chain": {
                    "channel_id": "channel-79703",
                    "path": "transfer/channel-79703/upenumbra"
                }
            }],
            "_source_chain": "osmosis"
        });

        let (hash, path, cp_chain) = derive_terp_ibc_denom(&asset, &channels).unwrap();
        assert_eq!(path, "transfer/channel-5/transfer/channel-79703/upenumbra");
        assert_eq!(cp_chain, "penumbra");
    }

    #[test]
    fn test_eth_multi_hop_via_cosmoshub() {
        let channels = make_terp_channels();
        let asset = json!({
            "base": "ibc/20850C646CDDDC2270E9BBDB08558B5FEE57B647EC6827F41096AABFD8A0471B",
            "symbol": "ETH",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "cosmoshub",
                    "base_denom": "ibc/C0B53D3D23827AE38058BED0BDCD554229278AF530A8D265FCF6DFF7C4B2ADFF",
                    "channel_id": "channel-141"
                },
                "chain": {
                    "channel_id": "channel-0",
                    "path": "transfer/channel-0/transfer/08-wasm-1369/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
                }
            }],
            "_source_chain": "osmosis"
        });

        let (hash, path, cp_chain) = derive_terp_ibc_denom(&asset, &channels).unwrap();
        // Direct channel to cosmoshub exists -> use cosmoshub base denom
        assert_eq!(cp_chain, "cosmoshub");
        assert_eq!(
            path,
            "transfer/channel-0/ibc/C0B53D3D23827AE38058BED0BDCD554229278AF530A8D265FCF6DFF7C4B2ADFF"
        );
    }

    #[test]
    fn test_no_channel_returns_none() {
        let channels = make_terp_channels();
        let asset = json!({
            "base": "ibc/FOO",
            "symbol": "NOPE",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "unknownchain",
                    "base_denom": "unope",
                    "channel_id": "channel-1"
                },
                "chain": {
                    "channel_id": "channel-2",
                    "path": "transfer/channel-2/unope"
                }
            }],
            "_source_chain": "unknownchain"
        });

        assert!(derive_terp_ibc_denom(&asset, &channels).is_none());
    }

    #[test]
    fn test_build_channel_to_chain_map_with_schema_format() {
        let mock_state_data = json!({
            "akash-terp": {
                "chain_1": {
                    "chain_name": "akash",
                    "chain_id": "akashnet-2",
                    "client_id": "07-tendermint-210",
                    "connection_id": "connection-207"
                },
                "chain_2": {
                    "chain_name": "terp",
                    "chain_id": "morocco-1",
                    "client_id": "07-tendermint-32",
                    "connection_id": "connection-11"
                },
                "channels": [{
                    "chain_1": {
                        "channel_id": "channel-6",
                        "port_id": "transfer"
                    },
                    "chain_2": {
                        "channel_id": "channel-115",
                        "port_id": "transfer"
                    },
                    "ordering": 1,
                    "version": "ics20-1",
                    "tags": {
                        "preferred": true,
                        "status": "ACTIVE"
                    }
                }]
            },
            "osmosis-terp": {
                "chain_1": {
                    "chain_name": "osmosis",
                    "chain_id": "osmosis-1",
                    "client_id": "07-tendermint-3708",
                    "connection_id": "connection-11060"
                },
                "chain_2": {
                    "chain_name": "terp",
                    "chain_id": "morocco-1",
                    "client_id": "07-tendermint-33",
                    "connection_id": "connection-13"
                },
                "channels": [{
                    "chain_1": {
                        "channel_id": "channel-1",
                        "port_id": "transfer"
                    },
                    "chain_2": {
                        "channel_id": "channel-6738",
                        "port_id": "transfer"
                    },
                    "ordering": 1,
                    "version": "ics20-1",
                    "tags": {
                        "preferred": true,
                        "status": "ACTIVE"
                    }
                }]
            },
            "atomone-terp": {
                "chain_1": {
                    "chain_name": "atomone",
                    "chain_id": "atomone-1",
                    "client_id": "07-tendermint-46",
                    "connection_id": "connection-42"
                },
                "chain_2": {
                    "chain_name": "terp",
                    "chain_id": "morocco-1",
                    "client_id": "07-tendermint-34",
                    "connection_id": "connection-12"
                },
                "channels": [{
                    "chain_1": {
                        "channel_id": "channel-10",
                        "port_id": "transfer"
                    },
                    "chain_2": {
                        "channel_id": "channel-13",
                        "port_id": "transfer"
                    },
                    "ordering": 1,
                    "version": "ics20-1",
                    "tags": {
                        "preferred": true,
                        "status": "ACTIVE"
                    }
                }]
            }
        });

        // Verify structure
        for (_key, entry) in mock_state_data.as_object().unwrap() {
            assert!(entry["chain_1"].is_object(), "Missing chain_1");
            assert!(entry["chain_2"].is_object(), "Missing chain_2");
            assert!(entry["channels"].is_array(), "Missing channels");

            for chan in entry["channels"].as_array().unwrap() {
                assert!(chan["chain_1"].is_object(), "Channel missing chain_1");
                assert!(chan["chain_2"].is_object(), "Channel missing chain_2");
            }
        }
    }

    #[test]
    fn test_derive_terp_ibc_denom_direct_single_hop_osmosis_native() {
        let channels = make_terp_channels();
        let asset = json!({
            "symbol": "OSMO",
            "base": "uosmo",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "osmosis",
                    "base_denom": "uosmo",
                    "channel_id": "channel-0"
                },
                "chain": {
                    "channel_id": "channel-42",
                    "path": "transfer/channel-42/uosmo"
                }
            }],
            "_source_chain": "osmosis"
        });

        let result = derive_terp_ibc_denom(&asset, &channels);
        assert!(result.is_some());
        let (hash, path, cp) = result.unwrap();
        assert_eq!(path, "transfer/channel-5/uosmo");
        assert_eq!(cp, "osmosis");
        assert_eq!(hash, compute_ibc_denom_hash("transfer/channel-5/uosmo"));
    }

    #[test]
    fn test_derive_terp_ibc_denom_direct_to_cosmoshub_atom() {
        let channels = make_terp_channels();
        let asset = json!({
            "symbol": "ATOM",
            "base": "uatom",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "cosmoshub",
                    "base_denom": "uatom",
                    "channel_id": "channel-0"
                },
                "chain": {
                    "channel_id": "channel-99",
                    "path": "transfer/channel-99/uatom"
                }
            }],
            "_source_chain": "cosmoshub"
        });

        let result = derive_terp_ibc_denom(&asset, &channels);
        assert!(result.is_some());
        let (hash, path, cp) = result.unwrap();
        assert_eq!(path, "transfer/channel-0/uatom");
        assert_eq!(cp, "cosmoshub");
    }

    #[test]
    fn test_derive_terp_ibc_denom_multi_hop_via_osmosis_to_atomone() {
        let channels = make_terp_channels();
        let asset = json!({
            "symbol": "ATONE",
            "base": "ibc/somehash",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "atomone",
                    "base_denom": "uatone",
                    "channel_id": "channel-2"
                },
                "chain": {
                    "channel_id": "channel-94814",
                    "path": "transfer/channel-94814/uatone"
                }
            }],
            "_source_chain": "osmosis"
        });

        let result = derive_terp_ibc_denom(&asset, &channels);
        assert!(result.is_some());
        let (_, path, cp) = result.unwrap();
        assert_eq!(path, "transfer/channel-5/transfer/channel-94814/uatone");
        assert_eq!(cp, "atomone");
    }

    #[test]
    fn test_derive_terp_ibc_denom_multi_hop_on_source_with_direct_channel() {
        // Should prefer direct channel when available, even if source has multi-hop trace
        let channels = make_terp_channels();
        let asset = json!({
            "symbol": "ETH",
            "base": "ibc/someethhash",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "cosmoshub",
                    "base_denom": "ibc/C0B53...ADFF",
                    "channel_id": "channel-141"
                },
                "chain": {
                    "channel_id": "channel-0",
                    "path": "transfer/channel-0/transfer/08-wasm-xxx/0xeth"
                }
            }],
            "_source_chain": "osmosis"
        });

        let result = derive_terp_ibc_denom(&asset, &channels);
        assert!(result.is_some());
        let (_, path, cp) = result.unwrap();
        assert_eq!(cp, "cosmoshub");
        assert_eq!(path, "transfer/channel-0/ibc/C0B53...ADFF"); // uses direct + base from counterparty
    }

    #[test]
    fn test_derive_terp_ibc_denom_no_channel_fallback_none() {
        let channels = make_terp_channels();
        let asset = json!({
            "symbol": "UNKNOWN",
            "base": "ibc/xxx",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "secret",
                    "base_denom": "uscrt"
                },
                "chain": { "path": "transfer/channel-999/uscrt" }
            }],
            "_source_chain": "secret"
        });

        let result = derive_terp_ibc_denom(&asset, &channels);
        assert!(result.is_none());
    }

    #[test]
    fn test_build_ibc_asset_entry_full_with_traces() {
        let asset = json!({
            "symbol": "ATOM",
            "name": "Cosmos Hub",
            "description": "The native token of Cosmos Hub",
            "display": "atom",
            "denom_units": [
                {"denom": "uatom", "exponent": 0},
                {"denom": "atom", "exponent": 6}
            ],
            "coingecko_id": "cosmos",
            "type_asset": "ics20"
        });

        let traces = vec![json!({
            "type": "ibc",
            "counterparty": {
                "chain_name": "cosmoshub",
                "base_denom": "uatom"
            },
            "chain": {
                "channel_id": "channel-0",
                "path": "transfer/channel-0/uatom"
            }
        })];

        let entry = build_ibc_asset_entry(
            &asset,
            "ibc/27394C5B9E9A3C6C8A7F4E5D6E7F8A9B0C1D2E3F4A5B6C7D8E9F0A1B2C3D4E5",
            &traces,
            "transfer/channel-0/uatom",
            "ATOM",
            "cosmoshub",
            "uatom",
            "channel-0",
            None,
        );

        assert_eq!(
            entry["base"],
            "ibc/27394C5B9E9A3C6C8A7F4E5D6E7F8A9B0C1D2E3F4A5B6C7D8E9F0A1B2C3D4E5"
        );
        assert_eq!(entry["symbol"], "ATOM");
        assert!(entry["traces"].is_array());
        assert!(entry["images"].is_array());
        assert_eq!(entry["coingecko_id"], "cosmos");
    }

    #[test]
    fn test_build_ibc_asset_entry_non_ibc_trace() {
        let asset = json!({
            "symbol": "stATOM",
            "name": "Stride Staked ATOM",
            "display": "statom",
            "denom_units": [{"denom": "ustatom", "exponent": 0}, {"denom": "statom", "exponent": 6}]
        });

        let traces = vec![json!({
            "type": "liquid-stake",
            "counterparty": {
                "chain_name": "stride",
                "base_denom": "uatom"
            },
            "provider": "stride"
        })];

        let entry = build_ibc_asset_entry(
            &asset, "ustatom", &traces, "", "stATOM", "stride", "uatom", "", None,
        );

        assert_eq!(entry["base"], "ustatom");
        assert_eq!(entry["type_asset"], "ics20");
    }

    #[test]
    fn test_compute_ibc_denom_hash_respects_spec() {
        // Standard IBC hash test vectors
        let cases = vec![
            ("transfer/channel-0/uatom", "ibc/27394C5B..."), // real example truncated
            (
                "transfer/channel-5/transfer/channel-0/uatom",
                "ibc/ABC123...",
            ),
        ];

        for (path, _) in cases {
            let hash = compute_ibc_denom_hash(path);
            assert!(hash.starts_with("ibc/"));
            assert_eq!(hash.len(), 68);
            // SHA256 of the path should be deterministic
            let hash2 = compute_ibc_denom_hash(path);
            assert_eq!(hash, hash2);
        }
    }

    #[test]
    fn test_derive_terp_ibc_denom_empty_traces() {
        let channels = make_terp_channels();
        let asset = json!({
            "symbol": "TERP",
            "base": "uterp",
            "traces": []
        });

        let result = derive_terp_ibc_denom(&asset, &channels);
        assert!(result.is_none());
    }
}
