//! E2E tests for the /health endpoint.

#![expect(clippy::expect_used)]

mod e2e_common;

use e2e_common::TestServer;
use reqwest::StatusCode;

#[tokio::test]
async fn health_returns_200() {
    let srv = TestServer::start().await;
    let resp = srv.health().await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn health_body_is_ok() {
    let srv = TestServer::start().await;
    let body = srv.health().await.text().await.expect("read body");

    assert_eq!(body, "ok");
}

#[tokio::test]
async fn health_content_type_is_text() {
    let srv = TestServer::start().await;
    let resp = srv.health().await;
    let ct = resp
        .headers()
        .get("content-type")
        .expect("content-type header")
        .to_str()
        .expect("header to str");

    assert!(
        ct.contains("text/plain"),
        "Expected text/plain, got {ct}"
    );
}

#[tokio::test]
async fn health_does_not_require_auth_header() {
    let srv = TestServer::start().await;

    // No Authorization header at all
    let resp = srv
        .client
        .get(srv.url("/health"))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn health_post_returns_method_not_allowed() {
    let srv = TestServer::start().await;

    let resp = srv
        .client
        .post(srv.url("/health"))
        .send()
        .await
        .expect("send");

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}
