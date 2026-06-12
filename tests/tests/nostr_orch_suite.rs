// //! Integration tests for the Nostr + cw-orch test suite.
// //!
// //! These tests demonstrate the full lifecycle:
// //!   1. Start a `NostrTestEnv` with a local chain + Nostr relay
// //!   2. Use `Environment<Daemon>` for contract operations
// //!   3. Use NostrClient to publish/subscribe events
// //!   4. Use ChainEventWatcher to bridge chain events to Nostr
// //!   5. Bridge a chain event to Nostr (kind 31922 calendar event)
// //!   6. Cleanup
// //!
// //! ## Requirements
// //!
// //! - Docker daemon running
// //! - `mattn/nostr-relay:latest` image pullable (or cached locally)
// //! - A local Terp chain node (or any Cosmos SDK node) on the expected ports
// //!
// //! Run with: `cargo test --test nostr_orch_suite -- --nocapture --ignored`

// use cw_orch::environment::Environment;
// use cw_orch::prelude::TxHandler;
// use std::time::Duration;

// // ---------------------------------------------------------------------------
// // Helper: skip test if relay is unreachable
// // ---------------------------------------------------------------------------

// async fn try_connect_nostr(url: &str) -> Option<ict_rs::nostr::NostrClient> {
//     match ict_rs::nostr::NostrClient::connect(url).await {
//         Ok(c) => Some(c),
//         Err(e) => {
//             eprintln!("Skipping test: Cannot connect to {url}: {e}");
//             None
//         }
//     }
// }

// // ---------------------------------------------------------------------------
// // Test 1: Full end-to-end lifecycle
// // ---------------------------------------------------------------------------

// /// Comprehensive end-to-end test that exercises the full nostr+calendar workflow:
// ///
// ///   - Creates a `NostrTestEnv` with a local chain + Nostr relay
// ///   - Verifies chain WS URL and Nostr URL formation
// ///   - Accesses `Environment<Daemon>` and verifies chain info + sender address
// ///   - Publishes a Nostr event and subscribes to receive it back
// ///   - Bridges a chain event (wasm-execute) to a Nostr kind-31922 calendar event
// ///     and publishes it on the relay
// ///   - Connects a `ChainEventWatcher` (gracefully skips if no chain is running)
// ///   - Stops the relay cleanly
// #[tokio::test]
// #[ignore] // requires Docker + local chain
// async fn test_full_lifecycle() {
//     let _ = env_logger::try_init();

//     // ── Build chain info for a local test node ──────────────────────
//     let chain_info = scripts::environments::nostr::NostrTestEnv::local_terp_chain_info(
//         "terp-test-1",
//         "http://127.0.0.1:9090",
//         "uterp",
//     );

//     // ── Start the environment ───────────────────────────────────────
//     let env = scripts::environments::nostr::NostrTestEnv::start(
//         "full-lifecycle",
//         chain_info,
//         "chapter wrist alcohol shine angry noise mercy simple rebel recycle vehicle wrap \
//          morning giraffe lazy outdoor noise blood ginger sort reunion boss crowd dutch",
//     )
//     .await
//     .expect("Failed to start NostrTestEnv");

//     // ── Verify chain WS URL ─────────────────────────────────────────
//     assert!(
//         env.chain_ws_url().contains("/websocket"),
//         "Chain WS URL should end with /websocket"
//     );

//     // ── Verify Nostr URL ────────────────────────────────────────────
//     assert!(
//         env.nostr_ws_url().starts_with("ws://"),
//         "Nostr URL should start with ws://"
//     );

//     // ── Access Daemon via Environment<Daemon> ───────────────────────
//     let daemon = env.environment();
//     assert_eq!(daemon.chain_info().grpc_urls[0], "http://127.0.0.1:9090");

//     // Verify the daemon has a sender address (daemon isolation check)
//     let addr = daemon.sender_addr();
//     assert!(
//         !addr.to_string().is_empty(),
//         "Daemon should have a sender address"
//     );
//     eprintln!("Daemon sender: {addr}");

//     // Verify chain info is accessible
//     assert_eq!(daemon.chain_info().chain_id, "terp-test-1");

//     // ── Connect NostrClient to the relay ────────────────────────────
//     let mut client = match try_connect_nostr(env.nostr_ws_url()).await {
//         Some(c) => c,
//         None => return,
//     };

//     // ── Publish a generic test event ─────────────────────────────────
//     let event = ict_rs::nostr::NostrEvent {
//         id: "lifecycle_test_id".to_string(),
//         pubkey: "test_pubkey".to_string(),
//         created_at: 1700000000,
//         kind: 1,
//         tags: vec![vec!["t".to_string(), "lifecycle".to_string()]],
//         content: "Full lifecycle test event".to_string(),
//         sig: "00".repeat(32),
//     };
//     let ok = client
//         .send_event(event)
//         .await
//         .expect("Failed to publish event");
//     assert!(ok, "Relay should accept the event");

//     // ── Subscribe and verify we can receive it back ─────────────────
//     let filters = vec![serde_json::json!({
//         "kinds": [1],
//         "limit": 5,
//     })];
//     let sub_id = client
//         .subscribe(filters)
//         .await
//         .expect("Failed to subscribe");
//     eprintln!("Subscribed with id: {sub_id}");

//     let received = tokio::time::timeout(Duration::from_secs(5), client.recv_event())
//         .await
//         .expect("Timeout waiting for Nostr event")
//         .expect("Failed to receive Nostr event");
//     assert_eq!(received.kind, 1, "Should receive kind 1 event");
//     assert!(
//         received.content.contains("lifecycle test"),
//         "Should match published content: {}",
//         received.content
//     );

//     // ── Bridge a chain event to a Nostr calendar event ──────────────
//     let attrs = vec![
//         ("action".into(), "wasm-execute".into()),
//         ("_contract_address".into(), "terp1contract123".into()),
//         ("sender".into(), "terp1sender456".into()),
//     ];

//     let bridged_ev = scripts::environments::nostr::chain_event_to_nostr(
//         &attrs,
//         42,
//         Some("wasm-execute"),
//         Some("terp1contract123"),
//         "nostr_pubkey_test",
//     );

//     // Verify the bridged event structure
//     assert_eq!(bridged_ev.kind, 31922, "Calendar Date-Based Event kind");
//     assert_eq!(bridged_ev.pubkey, "nostr_pubkey_test");

//     let content: serde_json::Value =
//         serde_json::from_str(&bridged_ev.content).expect("Content should be valid JSON");
//     assert_eq!(content["chain_height"], 42);
//     assert_eq!(content["action"], "wasm-execute");

//     // Publish the bridged event to the relay
//     let ok = client
//         .send_event(bridged_ev)
//         .await
//         .expect("Failed to publish bridged event");
//     assert!(ok, "Relay should accept the bridged chain event");

//     // Subscribe and verify we can receive the calendar event
//     let filters = vec![serde_json::json!({
//         "kinds": [31922],
//         "limit": 5,
//     })];
//     let _ = client
//         .subscribe(filters)
//         .await
//         .expect("Failed to subscribe");

//     let cal_received = tokio::time::timeout(Duration::from_secs(5), client.recv_event())
//         .await
//         .expect("Timeout waiting for Nostr calendar event")
//         .expect("Failed to receive Nostr calendar event");

//     assert_eq!(cal_received.kind, 31922, "Should receive kind 31922");
//     assert!(
//         cal_received.content.contains(r#""chain_height":42"#),
//         "Should contain chain_height: 42 in {}",
//         cal_received.content
//     );

//     // ── Connect a chain event watcher ───────────────────────────────
//     match env.event_watcher().await {
//         Ok(watcher) => {
//             eprintln!("ChainEventWatcher connected successfully");
//             watcher.shutdown();
//         }
//         Err(e) => {
//             eprintln!("ChainEventWatcher skipped (no chain): {e}");
//         }
//     }

//     // ── Stop the relay ──────────────────────────────────────────────
//     let mut env = env;
//     env.stop().await.expect("Failed to stop Nostr relay");
// }

// // ---------------------------------------------------------------------------
// // Test 2: Mock backend verification (no Docker required)
// // ---------------------------------------------------------------------------

// /// Verifies that NostrRelayerManager works correctly with the mock runtime
// /// backend. This test does NOT require Docker.
// #[tokio::test]
// async fn test_mock_backend_relay() {
//     use ict_rs::runtime::mock::MockRuntime;

//     let runtime = std::sync::Arc::new(MockRuntime::new());
//     let mut relay = ict_rs::nostr::NostrRelayerManager::with_image(
//         runtime,
//         ict_rs::runtime::DockerImage {
//             repository: "test/noop".to_string(),
//             version: "latest".to_string(),
//             uid_gid: None,
//         },
//         "mock-test",
//     );

//     let url = relay.start().await.expect("Mock relay should start");
//     assert!(url.starts_with("ws://127.0.0.1:"));
//     assert!(relay.host_port() > 0);

//     relay.stop().await.expect("Mock relay should stop");
//     assert_eq!(relay.host_port(), 0, "Port should be reset after stop");
// }

pub fn main() {}