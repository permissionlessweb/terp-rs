// Legacy integration tests — Nostr relay publish/subscribe and chain event conversion.
//
// These tests connect to an external Nostr relay (ws://127.0.0.1:7777) directly
// and do NOT use the NostrTestEnv suite. See `nostr_orch_suite.rs` for the full
// suite lifecycle tests.

use std::time::Duration;

// ---------------------------------------------------------------------------
// Test 1: Publish a Nostr event and verify it's received back
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore] // requires Docker: `docker run -d -p 7777:7777 mattn/nostr-relay`
async fn test_nostr_relay_publish_and_subscribe() {
    let _ = env_logger::try_init();

    // Connect to a running Nostr relay
    let mut client = match scripts::nostr_env::NostrClient::connect("ws://127.0.0.1:7777").await {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Skipping test: Nostr relay not running on ws://127.0.0.1:7777");
            return;
        }
    };

    // Create a test event
    let event = scripts::nostr_env::NostrEvent {
        id: "test_event_id_001".to_string(),
        pubkey: "test_pubkey".to_string(),
        created_at: 1700000000,
        kind: 1,
        tags: vec![vec!["t".to_string(), "legacy-test".to_string()]],
        content: "Hello from legacy terp-rs integration test!".to_string(),
        sig: "00".repeat(32),
    };

    // Publish and verify OK response
    let ok = client
        .send_event(event)
        .await
        .expect("Failed to publish Nostr event");
    assert!(ok, "Nostr relay should accept the event");

    // Subscribe with a filter and receive the event back
    let filters = vec![serde_json::json!({
        "kinds": [1],
        "limit": 1,
    })];

    let sub_id = client
        .subscribe(filters)
        .await
        .expect("Failed to subscribe");
    eprintln!("Subscribed with id: {sub_id}");

    // Wait for the event back
    let received = tokio::time::timeout(Duration::from_secs(5), client.recv_event())
        .await
        .expect("Timeout waiting for Nostr event")
        .expect("Failed to receive Nostr event");

    assert_eq!(received.kind, 1, "Should receive kind 1 event");
    assert!(
        received.content.contains("integration test"),
        "Should contain test marker"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Chain event to Nostr event conversion
// ---------------------------------------------------------------------------

#[test]
fn test_chain_event_to_nostr_conversion() {
    use scripts::nostr_env::chain_event_to_nostr;

    let attrs = vec![
        ("action".into(), "wasm-execute".into()),
        ("_contract_address".into(), "terp1contract123".into()),
        ("sender".into(), "terp1sender456".into()),
    ];

    let nostr_ev = chain_event_to_nostr(
        &attrs,
        1001,
        Some("wasm-execute"),
        Some("terp1contract123"),
        "nostr_pubkey_test",
    );

    assert_eq!(nostr_ev.kind, 31922, "Calendar Date-Based Event kind");
    assert_eq!(nostr_ev.pubkey, "nostr_pubkey_test");
    assert!(!nostr_ev.id.is_empty(), "Event should have an id");

    let content: serde_json::Value =
        serde_json::from_str(&nostr_ev.content).expect("Content should be valid JSON");
    assert_eq!(content["chain_height"], 1001);
    assert_eq!(content["action"], "wasm-execute");
    assert_eq!(content["contract"], "terp1contract123");

    assert!(
        nostr_ev.tags.iter().any(|t| t.len() == 2 && t[0] == "t" && t[1] == "chain-event"),
        "Should have category tag"
    );
    assert!(
        nostr_ev.tags.iter().any(|t| t.len() == 2 && t[0] == "h" && t[1] == "1001"),
        "Should have height tag"
    );
}