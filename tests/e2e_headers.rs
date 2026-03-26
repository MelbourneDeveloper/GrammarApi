//! E2E tests for HTTP headers: x-request-id and CORS.

#![expect(clippy::expect_used)]

mod e2e_common;

use e2e_common::TestServer;
use reqwest::StatusCode;

// --- x-request-id ---

#[tokio::test]
async fn check_response_has_x_request_id() {
    let srv = TestServer::start().await;
    let resp = srv.check("Hello world.").await;

    assert!(
        resp.headers().get("x-request-id").is_some(),
        "Response should have x-request-id header"
    );
}

#[tokio::test]
async fn x_request_id_is_uuid_format() {
    let srv = TestServer::start().await;
    let resp = srv.check("Hello world.").await;

    let id = resp
        .headers()
        .get("x-request-id")
        .expect("x-request-id header")
        .to_str()
        .expect("header to str");

    // UUID v4 format: 8-4-4-4-12 hex chars
    assert_eq!(id.len(), 36, "UUID should be 36 chars, got {}", id.len());
    assert_eq!(
        id.chars().filter(|c| *c == '-').count(),
        4,
        "UUID should have 4 dashes"
    );
}

#[tokio::test]
async fn different_requests_get_different_request_ids() {
    let srv = TestServer::start().await;

    let resp1 = srv.check("Hello.").await;
    let id1 = resp1
        .headers()
        .get("x-request-id")
        .expect("id1")
        .to_str()
        .expect("str")
        .to_string();

    let resp2 = srv.check("World.").await;
    let id2 = resp2
        .headers()
        .get("x-request-id")
        .expect("id2")
        .to_str()
        .expect("str")
        .to_string();

    assert_ne!(id1, id2, "Each request should get a unique ID");
}

#[tokio::test]
async fn health_response_has_x_request_id() {
    let srv = TestServer::start().await;
    let resp = srv.health().await;

    assert!(
        resp.headers().get("x-request-id").is_some(),
        "Health response should have x-request-id"
    );
}

#[tokio::test]
async fn provided_x_request_id_is_propagated() {
    let srv = TestServer::start().await;

    let custom_id = "my-custom-request-id-12345";
    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .header("x-request-id", custom_id)
        .json(&serde_json::json!({ "text": "Hello." }))
        .send()
        .await
        .expect("send");

    let returned_id = resp
        .headers()
        .get("x-request-id")
        .expect("x-request-id")
        .to_str()
        .expect("str");

    assert_eq!(
        returned_id, custom_id,
        "Server should propagate provided x-request-id"
    );
}

// --- CORS (default permissive mode) ---

#[tokio::test]
async fn cors_allows_any_origin_by_default() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/v1/check"))
        .header("origin", "https://example.com")
        .json(&serde_json::json!({ "text": "Hello." }))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::OK);

    // Permissive CORS should echo back the origin or use *
    let acao = resp.headers().get("access-control-allow-origin");
    assert!(
        acao.is_some(),
        "Should have Access-Control-Allow-Origin header"
    );
}

#[tokio::test]
async fn cors_preflight_returns_ok() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .request(reqwest::Method::OPTIONS, srv.url("/v1/check"))
        .header("origin", "https://example.com")
        .header("access-control-request-method", "POST")
        .header("access-control-request-headers", "content-type")
        .send()
        .await
        .expect("send");

    assert!(
        resp.status().is_success(),
        "CORS preflight should succeed, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn cors_allows_content_type_header() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .request(reqwest::Method::OPTIONS, srv.url("/v1/check"))
        .header("origin", "https://example.com")
        .header("access-control-request-method", "POST")
        .header("access-control-request-headers", "content-type")
        .send()
        .await
        .expect("send");

    assert!(resp.status().is_success());

    let allowed_headers = resp
        .headers()
        .get("access-control-allow-headers")
        .map(|v| v.to_str().unwrap_or("").to_lowercase());

    // Permissive CORS allows all headers, or specifically lists content-type
    assert!(
        allowed_headers.is_some(),
        "Should have access-control-allow-headers"
    );
}

#[tokio::test]
async fn cors_allows_authorization_header() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .request(reqwest::Method::OPTIONS, srv.url("/v1/check"))
        .header("origin", "https://example.com")
        .header("access-control-request-method", "POST")
        .header("access-control-request-headers", "authorization")
        .send()
        .await
        .expect("send");

    assert!(
        resp.status().is_success(),
        "CORS should allow authorization header"
    );
}
