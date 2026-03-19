//! E2E tests for API key authentication over HTTP.
//!
//! These tests use a serial mutex since they manipulate the `API_KEY`
//! environment variable which is process-global.

#![expect(clippy::expect_used, deprecated_safe_2024)]

mod e2e_common;

use grammar_api::create_app_for_testing;
use reqwest::{Client, StatusCode};
use std::net::SocketAddr;
use std::sync::Mutex;
use tokio::net::TcpListener;

/// Global mutex to serialize tests that touch `API_KEY` env var.
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Start a server with a specific API key set.
async fn start_with_api_key(key: &str) -> (SocketAddr, Client) {
    // Set the env var BEFORE building the app
    std::env::set_var("API_KEY", key);
    let app = create_app_for_testing();
    std::env::remove_var("API_KEY");

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind");
    let addr = listener.local_addr().expect("addr");

    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("serve");
    });

    let client = Client::builder().no_proxy().build().expect("client");
    (addr, client)
}

fn url(addr: SocketAddr, path: &str) -> String {
    format!("http://{addr}{path}")
}

#[tokio::test]
async fn auth_valid_key_returns_200() {
    let _lock = ENV_LOCK.lock();
    let (addr, client) = start_with_api_key("test-secret-key").await;

    let resp = client
        .post(url(addr, "/v1/check"))
        .header("authorization", "Bearer test-secret-key")
        .json(&serde_json::json!({ "text": "Hello." }))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn auth_invalid_key_returns_401() {
    let _lock = ENV_LOCK.lock();
    let (addr, client) = start_with_api_key("test-secret-key").await;

    let resp = client
        .post(url(addr, "/v1/check"))
        .header("authorization", "Bearer wrong-key")
        .json(&serde_json::json!({ "text": "Hello." }))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_missing_header_returns_401() {
    let _lock = ENV_LOCK.lock();
    let (addr, client) = start_with_api_key("test-secret-key").await;

    let resp = client
        .post(url(addr, "/v1/check"))
        .json(&serde_json::json!({ "text": "Hello." }))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_empty_bearer_returns_401() {
    let _lock = ENV_LOCK.lock();
    let (addr, client) = start_with_api_key("test-secret-key").await;

    let resp = client
        .post(url(addr, "/v1/check"))
        .header("authorization", "Bearer ")
        .json(&serde_json::json!({ "text": "Hello." }))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_no_bearer_prefix_returns_401() {
    let _lock = ENV_LOCK.lock();
    let (addr, client) = start_with_api_key("test-secret-key").await;

    let resp = client
        .post(url(addr, "/v1/check"))
        .header("authorization", "test-secret-key")
        .json(&serde_json::json!({ "text": "Hello." }))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_basic_scheme_returns_401() {
    let _lock = ENV_LOCK.lock();
    let (addr, client) = start_with_api_key("test-secret-key").await;

    let resp = client
        .post(url(addr, "/v1/check"))
        .header("authorization", "Basic dGVzdDp0ZXN0")
        .json(&serde_json::json!({ "text": "Hello." }))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_401_returns_json_error_body() {
    let _lock = ENV_LOCK.lock();
    let (addr, client) = start_with_api_key("test-secret-key").await;

    let resp = client
        .post(url(addr, "/v1/check"))
        .header("authorization", "Bearer wrong")
        .json(&serde_json::json!({ "text": "Hello." }))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let json: serde_json::Value = resp.json().await.expect("parse json");
    assert!(json["error"].is_string(), "Should have error message");
    assert_eq!(json["code"].as_str(), Some("UNAUTHORIZED"));
}

#[tokio::test]
async fn auth_health_bypasses_auth() {
    let _lock = ENV_LOCK.lock();
    let (addr, client) = start_with_api_key("test-secret-key").await;

    // No auth header at all — health should still work
    let resp = client
        .get(url(addr, "/health"))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn auth_metrics_bypasses_auth() {
    let _lock = ENV_LOCK.lock();
    let (addr, client) = start_with_api_key("test-secret-key").await;

    let resp = client
        .get(url(addr, "/metrics"))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::OK);
}

// --- No API key configured = open access ---

#[tokio::test]
async fn no_api_key_configured_allows_all_requests() {
    // When no API_KEY is set, auth is disabled
    let _lock = ENV_LOCK.lock();
    std::env::remove_var("API_KEY");

    let app = create_app_for_testing();
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind");
    let addr = listener.local_addr().expect("addr");
    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("serve");
    });

    let client = Client::builder().no_proxy().build().expect("client");

    let resp = client
        .post(url(addr, "/v1/check"))
        .json(&serde_json::json!({ "text": "Hello." }))
        .send()
        .await
        .expect("send");

    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "No API_KEY = open access"
    );
}
