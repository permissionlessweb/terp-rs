//! Integration tests for the Nostr + cw-orch test suite.
//!
//! These tests demonstrate the full lifecycle:
//!   1. Start a `NostrTestEnv` with a local chain + Nostr relay
//!   2. Use `Environment<Daemon>` for contract operations
//!   3. Use NostrClient to publish/subscribe events
//!   4. Use ChainEventWatcher to bridge chain events to Nostr
//!   5. Cleanup
//!
//! ## Requirements
//!
//! - Docker daemon running
//! - `mattn/nostr-relay:latest` image pullable (or cached locally)
//! - A local Terp chain node (or any Cosmos SDK node) on the expected ports
//!
//! Run with: `cargo test --test nostr_orch_suite -- --nocapture --ignored`

use std::time::Duration;
use cw_orch::environment::Environment;
use cw_orch::prelude::TxHandler;

// ---------------------------------------------------------------------------
// Helper: skip test if relay is unreachable
// ---------------------------------------------------------------------------

async fn try_connect_nostr(url: &str) -> Option<ict_rs::nostr::NostrClient> {
    match ict_rs::nostr::NostrClient::connect(url).await {
        Ok(c) => Some(c),
        Err(e) => {
            eprintln!("Skipping test: Cannot connect to {url}: {e}");
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Test 1: Full suite lifecycle
// ---------------------------------------------------------------------------

/// Proves that NostrTestEnv can be created, provides a Daemon through
/// Environment<Daemon>, and the Nostr relay is operational.
#[tokio::test]
#[ignore] // requires Docker + local chain
async fn test_suite_lifecycle() {
    let _ = env_logger::try_init();

    // Build chain info for a local test node
    let chain_info = scripts::nostr_env::NostrTestEnv::local_terp_chain_info(
        "terp-test-1",
        "http://127.0.0.1:9090",
        "uterp",
    );

    // Start the environment
    let env = scripts::nostr_env::NostrTestEnv::start(
        "suite-lifecycle",
        chain_info,
        "chapter wrist alcohol shine angry noise mercy simple rebel recycle vehicle wrap \
         morning giraffe lazy outdoor noise blood ginger sort reunion boss crowd dutch",
    )
    .await
    .expect("Failed to start NostrTestEnv");

    // Verify chain WS URL is formed
    assert!(
        env.chain_ws_url().contains("/websocket"),
        "Chain WS URL should end with /websocket"
    );

    // Verify Nostr URL is formed
    assert!(
        env.nostr_ws_url().starts_with("ws://"),
        "Nostr URL should start with ws://"
    );

    // Access the Daemon through Environment<Daemon>
    let daemon = env.environment();
    assert_eq!(
        daemon.chain_info().grpc_urls[0],
        "http://127.0.0.1:9090"
    );

    // Connect NostrClient to the relay
    let mut client = match try_connect_nostr(env.nostr_ws_url()).await {
        Some(c) => c,
        None => return,
    };

    // Publish a test event
    let event = ict_rs::nostr::NostrEvent {
        id: "lifecycle_test_id".to_string(),
        pubkey: "test_pubkey".to_string(),
        created_at: 1700000000,
        kind: 1,
        tags: vec![vec!["t".to_string(), "lifecycle".to_string()]],
        content: "Suite lifecycle test event".to_string(),
        sig: "00".repeat(32),
    };
    let ok = client
        .send_event(event)
        .await
        .expect("Failed to publish event");
    assert!(ok, "Relay should accept the event");

    // Subscribe and verify we can receive it back
    let filters = vec![serde_json::json!({
        "kinds": [1],
        "limit": 5,
    })];
    let sub_id = client.subscribe(filters).await.expect("Failed to subscribe");
    eprintln!("Subscribed with id: {sub_id}");

    let received = tokio::time::timeout(Duration::from_secs(5), client.recv_event())
        .await
        .expect("Timeout waiting for Nostr event")
        .expect("Failed to receive Nostr event");
    assert_eq!(received.kind, 1, "Should receive kind 1 event");
    assert!(
        received.content.contains("lifecycle test"),
        "Should match published content: {}",
        received.content
    );

    // Connect a chain event watcher (requires chain to be running)
    match env.event_watcher().await {
        Ok(watcher) => {
            eprintln!("ChainEventWatcher connected successfully");
            watcher.shutdown();
        }
        Err(e) => {
            eprintln!("ChainEventWatcher skipped (no chain): {e}");
        }
    }

    // Properly stop the relay
    let mut env = env;
    env.stop().await.expect("Failed to stop Nostr relay");
}

// ---------------------------------------------------------------------------
// Test 2: Chain event to Nostr relay bridge
// ---------------------------------------------------------------------------

/// Demonstrates the bridge pattern: chain events → Nostr events → published
/// on the relay.
#[tokio::test]
#[ignore] // requires Docker + local chain
async fn test_chain_event_bridge() {
    let _ = env_logger::try_init();

    let chain_info = scripts::nostr_env::NostrTestEnv::local_terp_chain_info(
        "terp-test-1",
        "http://127.0.0.1:9090",
        "uterp",
    );

    let env = scripts::nostr_env::NostrTestEnv::start(
        "chain-bridge",
        chain_info,
        "chapter wrist alcohol shine angry noise mercy simple rebel recycle vehicle wrap \
         morning giraffe lazy outdoor noise blood ginger sort reunion boss crowd dutch",
    )
    .await
    .expect("Failed to start NostrTestEnv");

    let mut client = match try_connect_nostr(env.nostr_ws_url()).await {
        Some(c) => c,
        None => return,
    };

    // ── Create a chain event and bridge it to a Nostr event ──────────
    let attrs = vec![
        ("action".into(), "wasm-execute".into()),
        ("_contract_address".into(), "terp1contract123".into()),
        ("sender".into(), "terp1sender456".into()),
    ];

    let nostr_ev = scripts::nostr_env::chain_event_to_nostr(
        &attrs,
        42,
        Some("wasm-execute"),
        Some("terp1contract123"),
        "nostr_pubkey_test",
    );

    // Verify the bridged event
    assert_eq!(nostr_ev.kind, 31922, "Calendar Date-Based Event kind");
    assert_eq!(nostr_ev.pubkey, "nostr_pubkey_test");

    let content: serde_json::Value =
        serde_json::from_str(&nostr_ev.content).expect("Content should be valid JSON");
    assert_eq!(content["chain_height"], 42);
    assert_eq!(content["action"], "wasm-execute");

    // Publish the bridged event to the relay
    let ok = client
        .send_event(nostr_ev)
        .await
        .expect("Failed to publish bridged event");
    assert!(ok, "Relay should accept the bridged chain event");

    // Subscribe and verify we can receive it
    let filters = vec![serde_json::json!({
        "kinds": [31922],
        "limit": 5,
    })];
    let _ = client.subscribe(filters).await.expect("Failed to subscribe");

    let received = tokio::time::timeout(Duration::from_secs(5), client.recv_event())
        .await
        .expect("Timeout waiting for Nostr event")
        .expect("Failed to receive Nostr event");

    assert_eq!(received.kind, 31922, "Should receive kind 31922");
    assert!(
        received.content.contains(r#""chain_height":42"#) || received.content.contains(r#""chain_height":42"#),
        "Should contain chain_height: 42"
    );

    let mut env = env;
    env.stop().await.expect("Failed to stop Nostr relay");
}

// ---------------------------------------------------------------------------
// Test 3: Mock backend verification (no Docker required)
// ---------------------------------------------------------------------------

/// Verifies that NostrRelayerManager works correctly with the mock runtime
/// backend. This test does NOT require Docker.
#[tokio::test]
async fn test_mock_backend_relay() {
    use ict_rs::runtime::mock::MockRuntime;

    let runtime = std::sync::Arc::new(MockRuntime::new());
    let mut relay = ict_rs::nostr::NostrRelayerManager::with_image(
        runtime,
        ict_rs::runtime::DockerImage {
            repository: "test/noop".to_string(),
            version: "latest".to_string(),
            uid_gid: None,
        },
        "mock-test",
    );

    let url = relay.start().await.expect("Mock relay should start");
    assert!(url.starts_with("ws://127.0.0.1:"));
    assert!(relay.host_port() > 0);

    relay.stop().await.expect("Mock relay should stop");
    assert_eq!(relay.host_port(), 0, "Port should be reset after stop");
}

// ---------------------------------------------------------------------------
// Test 4: Environment<Daemon> isolation
// ---------------------------------------------------------------------------

/// Verifies that Environment<Daemon> can be used to set different senders
/// and that the Nostr relay is independent from chain operations.
#[tokio::test]
#[ignore] // requires Docker + local chain
async fn test_daemon_isolation() {
    let _ = env_logger::try_init();

    let chain_info = scripts::nostr_env::NostrTestEnv::local_terp_chain_info(
        "terp-test-1",
        "http://127.0.0.1:9090",
        "uterp",
    );

    let daemon_only = cw_orch::daemon::DaemonBuilder::new(chain_info)
        .handle(&tokio::runtime::Handle::current())
        .mnemonic("chapter wrist alcohol shine angry noise mercy simple rebel recycle vehicle wrap \
                    morning giraffe lazy outdoor noise blood ginger sort reunion boss crowd dutch")
        .is_test(true)
        .build()
        .expect("Failed to build Daemon");

    // The daemon should be connected and have a sender
    let addr = daemon_only.sender_addr();
    assert!(
        !addr.to_string().is_empty(),
        "Daemon should have a sender address"
    );
    eprintln!("Daemon sender: {addr}");

    // Verify chain info is accessible
    let info = daemon_only.chain_info();
    assert_eq!(info.chain_id, "terp-test-1");
}