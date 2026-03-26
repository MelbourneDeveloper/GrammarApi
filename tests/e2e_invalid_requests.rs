//! E2E tests for invalid requests, bad payloads, and error responses.

#![expect(clippy::expect_used)]

mod e2e_common;

use e2e_common::TestServer;
use reqwest::StatusCode;

// --- Wrong HTTP method ---

#[tokio::test]
async fn check_get_returns_method_not_allowed() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .get(srv.url("/v1/check"))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn check_put_returns_method_not_allowed() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .put(srv.url("/v1/check"))
        .json(&serde_json::json!({ "text": "test" }))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn check_delete_returns_method_not_allowed() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .delete(srv.url("/v1/check"))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

// --- Missing / malformed body ---

#[tokio::test]
async fn check_no_body_returns_error() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .header("content-type", "application/json")
        .send()
        .await
        .expect("send");

    // Should be 4xx — either 400 or 422 depending on axum version
    assert!(
        resp.status().is_client_error(),
        "No body should be a client error, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn check_empty_json_object_returns_error() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .header("content-type", "application/json")
        .body("{}")
        .send()
        .await
        .expect("send");

    // Missing "text" field should fail
    assert!(
        resp.status().is_client_error(),
        "Empty JSON object missing 'text' should be client error, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn check_invalid_json_returns_error() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .header("content-type", "application/json")
        .body("{not valid json}")
        .send()
        .await
        .expect("send");

    assert!(
        resp.status().is_client_error(),
        "Invalid JSON should be client error, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn check_wrong_field_name_returns_error() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .json(&serde_json::json!({ "content": "hello" }))
        .send()
        .await
        .expect("send");

    assert!(
        resp.status().is_client_error(),
        "Wrong field name should be client error, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn check_text_as_number_returns_error() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .json(&serde_json::json!({ "text": 12345 }))
        .send()
        .await
        .expect("send");

    assert!(
        resp.status().is_client_error(),
        "text as number should be client error, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn check_text_as_array_returns_error() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .json(&serde_json::json!({ "text": ["hello", "world"] }))
        .send()
        .await
        .expect("send");

    assert!(
        resp.status().is_client_error(),
        "text as array should be client error, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn check_text_as_null_returns_error() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .json(&serde_json::json!({ "text": null }))
        .send()
        .await
        .expect("send");

    assert!(
        resp.status().is_client_error(),
        "text as null should be client error, got {}",
        resp.status()
    );
}

// --- Payload size limit ---

#[tokio::test]
async fn payload_under_limit_returns_200() {
    let srv = TestServer::start().await;
    // 50KB of text — well under 100KB limit
    let text = "a".repeat(50 * 1024);
    let resp = srv.check(&text).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn payload_over_limit_returns_413() {
    let srv = TestServer::start().await;
    // 101KB of text — over 100KB limit
    let text = "a".repeat(101 * 1024);
    let resp = srv.check(&text).await;

    assert_eq!(
        resp.status(),
        StatusCode::PAYLOAD_TOO_LARGE,
        "Text over 100KB should return 413"
    );
}

#[tokio::test]
async fn payload_over_limit_returns_json_error() {
    let srv = TestServer::start().await;
    let text = "a".repeat(101 * 1024);
    let resp = srv.check(&text).await;

    assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);

    let json: serde_json::Value = resp.json().await.expect("parse error json");

    assert!(json["error"].is_string(), "Error response should have 'error' string");
    assert_eq!(
        json["code"].as_str(),
        Some("PAYLOAD_TOO_LARGE"),
        "Error code should be PAYLOAD_TOO_LARGE"
    );
}

#[tokio::test]
async fn payload_at_exact_limit_returns_200() {
    let srv = TestServer::start().await;
    // Exactly 100KB
    let text = "a".repeat(100 * 1024);
    let resp = srv.check(&text).await;

    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "Text at exactly 100KB should pass"
    );
}

// --- Non-existent endpoints ---

#[tokio::test]
async fn unknown_path_returns_404() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .get(srv.url("/v1/nonexistent"))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn root_path_returns_404() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .get(srv.url("/"))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn check_without_version_returns_404() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/check"))
        .json(&serde_json::json!({ "text": "test" }))
        .send()
        .await
        .expect("send");

    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "/check without /v1 prefix should 404"
    );
}

// --- Content-Type edge cases ---

#[tokio::test]
async fn check_without_content_type_returns_error() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .body("{\"text\": \"hello\"}")
        .send()
        .await
        .expect("send");

    // Axum's Json extractor requires application/json content-type
    assert!(
        resp.status().is_client_error(),
        "Missing content-type should be client error, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn check_with_wrong_content_type_returns_error() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .header("content-type", "text/plain")
        .body("{\"text\": \"hello\"}")
        .send()
        .await
        .expect("send");

    assert!(
        resp.status().is_client_error(),
        "Wrong content-type should be client error, got {}",
        resp.status()
    );
}
